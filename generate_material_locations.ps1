#!/usr/bin/env pwsh

# Generates a Rust source file from the Inara "Components > Items" table
# It scrapes the first column ("Item") and the fourth column ("Location") from:
#   https://inara.cz/elite/components/#tab_items
# and writes a Rust file containing a single public function that returns a
# HashMap<&'static str, &'static str> mapping item -> location.
#
# Note: Although the request mentioned a "HashSet", a key->value collection in Rust
# is a HashMap. If you truly need a HashSet-like structure, please clarify the
# desired semantics (e.g., a set of pairs, or separate sets). For now, this script
# generates a HashMap mapping each item to its location.
#
# Output file: ./src/inara_items.rs
# Usage: pwsh ./generate_material_locations.ps1

param(
    [string]$Url = 'https://inara.cz/elite/components/#tab_items',
    [string]$Output = 'src/material_locations.rs',
    [switch]$VerboseLogging
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Info([string]$msg) {
    if ($VerboseLogging) { Write-Host "[INFO] $msg" -ForegroundColor Cyan }
}

function Get-WebContent([string]$url) {
    Write-Info "Downloading HTML from $url"
    try {
        # Using Invoke-WebRequest for portability. Avoid relying on .ParsedHtml.
        $resp = Invoke-WebRequest -Uri $url -Headers @{ 'User-Agent' = 'EliteAssist-Scraper/1.0' }
        return [string]$resp.Content
    }
    catch {
        throw "Failed to download $url. $_"
    }
}

function Extract-PairsFromTab([string]$html, [string]$tabId) {
    # Goal: Locate the section with the given id (e.g., "tab_items" or "tab_components"), then
    # parse the first (<td> item) and fourth (<td> location) columns of each data row.

    # 1) Narrow to the tab content containing the table
    $tabPattern = ('(?is)<div[^>]+id\s*=\s*"{0}"[^>]*>(.*?)</div>' -f ([System.Text.RegularExpressions.Regex]::Escape($tabId)))
    $tabMatch = [System.Text.RegularExpressions.Regex]::Match($html, $tabPattern)
    if (-not $tabMatch.Success) {
        # Fallback: try to find the first table if specific tab not found
        $tabPattern2 = '(?is)<table[^>]*>(.*?)</table>'
        $tabMatch = [System.Text.RegularExpressions.Regex]::Match($html, $tabPattern2)
        if (-not $tabMatch.Success) {
            throw "Could not find the $tabId div or any table in the HTML."
        }
    }
    $tabHtml = $tabMatch.Groups[1].Value

    # 2) Find table rows
    $rowPattern = '(?is)<tr[^>]*>(.*?)</tr>'
    $rowMatches = [System.Text.RegularExpressions.Regex]::Matches($tabHtml, $rowPattern)
    if ($rowMatches.Count -eq 0) {
        throw "No rows found inside the table for $tabId."
    }

    $rows = @()

    foreach ($rm in $rowMatches) {
        $rowHtml = $rm.Groups[1].Value

        # Extract TDs for this row
        $tdPattern = '(?is)<td[^>]*>(.*?)</td>'
        $tdMatches = [System.Text.RegularExpressions.Regex]::Matches($rowHtml, $tdPattern)
        if ($tdMatches.Count -lt 4) { continue } # skip headers or malformed rows

        $col1 = $tdMatches[0].Groups[1].Value
        $col4 = $tdMatches[3].Groups[1].Value

        $item = (Strip-Html -html $col1)
        $location = (Strip-Html -html $col4)

        if ([string]::IsNullOrWhiteSpace($item)) { continue }

        # Normalize whitespace
        $item = Normalize-Text $item
        $location = Normalize-Text $location

        if (-not [string]::IsNullOrWhiteSpace($item)) {
            $rows += [PSCustomObject]@{ Item = $item; Location = $location }
        }
    }

    # De-duplicate by first occurrence
    $seen = @{}
    $result = New-Object System.Collections.Generic.List[object]
    foreach ($r in $rows) {
        if (-not $seen.ContainsKey($r.Item)) {
            $seen[$r.Item] = $true
            $result.Add($r)
        }
    }

    # Deterministic order: sort by Item
    $result | Sort-Object -Property Item
}

function Strip-Html([string]$html) {
    # Remove tags and decode entities
    $text = [System.Text.RegularExpressions.Regex]::Replace($html, '(?is)<script.*?</script>', '')
    $text = [System.Text.RegularExpressions.Regex]::Replace($text, '(?is)<style.*?</style>', '')
    $text = [System.Text.RegularExpressions.Regex]::Replace($text, '(?is)<[^>]+>', '')
    $text = [System.Net.WebUtility]::HtmlDecode($text)
    return $text
}

function Normalize-Text([string]$s) {
    if ($null -eq $s) { return '' }
    $t = $s -replace '\r|\n', ' '
    $t = $t -replace '\s+', ' '
    $t = $t.Trim()
    return $t
}

function Escape-RustString([string]$s) {
    # Escape backslashes and quotes; keep as UTF-8 literal
    $escaped = $s.Replace('\', '\\').Replace('"', '\"')
    return $escaped
}

function Generate-Rust([object[]]$itemsPairs, [object[]]$componentsPairs, [string]$outPath, [string]$sourceUrl) {
    Write-Info "Generating Rust file: $outPath"

    $timestamp = (Get-Date).ToString('yyyy-MM-dd HH:mm:ssK')
    $lines = @()
    $lines += '// This file is AUTO-GENERATED. Do not edit manually.'
    $lines += "// Source: $sourceUrl#tab_items and $sourceUrl#tab_components"
    $lines += "// Generated: $timestamp"
    $lines += ''
    $lines += 'use std::collections::HashMap;'
    $lines += 'use std::sync::OnceLock;'
    $lines += ''

    # New: items_to_locations with arrays and reverse lookup helpers
    $lines += '/// Returns a mapping from item -> locations as slices (&' + "'static [&" + 'static str]).'
    $lines += 'pub fn items_to_locations() -> HashMap<&' + "'static str, &" + "'static [&" + "'static str]> {"
    $lines += '    let mut m: HashMap<&' + "'static str, &" + "'static [&" + "'static str]> = HashMap::new();"

    foreach ($p in $itemsPairs) {
        $item = Escape-RustString $p.Item
        $rawLoc = $p.Location
        $parts = @()
        foreach ($part in ($rawLoc -split ',')) {
            $trimmed = (Normalize-Text $part)
            if (-not [string]::IsNullOrWhiteSpace($trimmed)) { $parts += $trimmed }
        }
        if ($parts.Count -eq 0) { $parts = @('') }
        $escapedParts = @()
        foreach ($pp in $parts) { $escapedParts += ('"{0}"' -f (Escape-RustString $pp)) }
        $sliceLiteral = ('&[{0}]' -f ([string]::Join(', ', $escapedParts)))
        $lines += ('    m.insert("{0}", {1});' -f $item, $sliceLiteral)
    }

    $lines += '    m'
    $lines += '}'
    $lines += ''

    $lines += '/// Returns the locations for a given item name.'
    $lines += "pub fn item_locations(name: &str) -> Option<&[&'static str]> {"
    $lines += '    ITEMS_BY_LOCATION.get_or_init(items_to_locations).get(name).map(|v| &**v)'
    $lines += '}'
    $lines += ''
    $lines += "static ITEMS_BY_LOCATION: OnceLock<HashMap<&'static str, &[&'static str]>> = OnceLock::new();"
    $lines += ''

    # New: items_to_locations with arrays and reverse lookup helpers
    $lines += '/// Returns a mapping from component -> locations as slices (&' + "'static [&" + 'static str]).'
    $lines += 'pub fn components_to_locations() -> HashMap<&' + "'static str, &" + "'static [&" + "'static str]> {"
    $lines += '    let mut m: HashMap<&' + "'static str, &" + "'static [&" + "'static str]> = HashMap::new();"

    foreach ($p in $componentsPairs) {
        $item = Escape-RustString $p.Item
        $rawLoc = $p.Location
        $parts = @()
        foreach ($part in ($rawLoc -split ',')) {
            $trimmed = (Normalize-Text $part)
            if (-not [string]::IsNullOrWhiteSpace($trimmed)) { $parts += $trimmed }
        }
        if ($parts.Count -eq 0) { $parts = @('') }
        $escapedParts = @()
        foreach ($pp in $parts) { $escapedParts += ('"{0}"' -f (Escape-RustString $pp)) }
        $sliceLiteral = ('&[{0}]' -f ([string]::Join(', ', $escapedParts)))
        $lines += ('    m.insert("{0}", {1});' -f $item, $sliceLiteral)
    }

    $lines += '    m'
    $lines += '}'
    $lines += ''

    $lines += '/// Returns the locations for a given component name.'
    $lines += "pub fn component_locations(name: &str) -> Option<&[&'static str]> {"
    $lines += '    COMPONENTS_BY_LOCATION.get_or_init(components_to_locations).get(name).map(|v| &**v)'
    $lines += '}'
    $lines += ''
    $lines += "static COMPONENTS_BY_LOCATION: OnceLock<HashMap<&'static str, &[&'static str]>> = OnceLock::new();"
    $lines += ''

    $dir = Split-Path -Parent $outPath
    if (-not (Test-Path $dir)) {
        New-Item -Path $dir -ItemType Directory -Force | Out-Null
    }

    Set-Content -Path $outPath -Value $lines -Encoding UTF8
}

# Main
$raw = Get-WebContent $Url
$items = Extract-PairsFromTab $raw 'tab_items'
$components = Extract-PairsFromTab $raw 'tab_components'
if ($items.Count -eq 0) {
    throw 'No items parsed from the Inara items table.'
}
if ($components.Count -eq 0) {
    Write-Info 'No components parsed from the Inara components table (tab_components). Proceeding with items only.'
}
Generate-Rust -itemsPairs $items -componentsPairs $components -outPath $Output -sourceUrl 'https://inara.cz/elite/components/'

Write-Host "Generated Rust file at $Output with $($items.Count) items and $($components.Count) components." -ForegroundColor Green
