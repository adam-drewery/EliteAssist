#!/usr/bin/env pwsh

# Script to generate Rust structs from JSON schemas in ed-journal-schemas/schemas

# Helper function to convert PascalCase to snake_case
function ConvertTo-SnakeCase {
    param (
        [string]$str
    )
    
    # Handle empty strings
    if ([string]::IsNullOrEmpty($str)) {
        return $str
    }

    # Get rid of underscores otherwise they'll get doubled up.
    $str = $str -replace '_', ''

    # Special case for ID suffix - convert "MarketID" to "market_id" not "market_i_d"
    $str = $str -replace 'ID$', 'Id'
    
    # Special case for acronyms - convert "VIPs" to "Vips" so it becomes "vips" not "v_i_ps"
    $str = $str -replace 'VIPs', 'Vips'
    $str = $str -replace 'VIP', 'Vip'
    
    # Handle other common acronyms
    $str = $str -replace 'API', 'Api'
    $str = $str -replace 'FSD', 'Fsd'
    $str = $str -replace 'HUD', 'Hud'
    $str = $str -replace 'NPC', 'Npc'
    $str = $str -replace 'SRV', 'Srv'
    $str = $str -replace 'UI', 'Ui'
    $str = $str -replace 'URL', 'Url'
    $str = $str -replace 'UUID', 'Uuid'
    
    # Insert underscore before each uppercase letter (except the first one)
    # and convert the entire string to lowercase
    $result = $str -creplace '(?<!^)([A-Z])', '_$1'
    return $result.ToLower()
}

# Helper function to check if a name is a Rust reserved keyword and escape it if needed
function Escape-RustKeyword {
    param (
        [string]$name
    )
    
    # List of Rust reserved keywords
    $reservedKeywords = @(
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
        "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
        "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield"
    )
    
    if ($reservedKeywords -contains $name) {
        return "r#$name"
    } else {
        return $name
    }
}

# Helper function to determine the Rust type for a JSON schema type
function Get-RustType {
    param (
        [string]$jsonType,
        [string]$format,
        [bool]$isArray,
        [string]$ref,
        [bool]$isOptional
    )
    
    $rustType = ""
    
    # List of known problematic types that should be treated as serde_json::Value
    $problematicTypes = @("Item", "Component", "Consumable", "Data", "changeEntry")
    
    if ($ref) {
        # If there's a reference, use that as the type
        $refParts = $ref -split "/"
        $typeName = $refParts[-1]
        
        # Check if this is a problematic type
        if ($problematicTypes -contains $typeName) {
            $rustType = "serde_json::Value"
        } else {
            $rustType = $typeName
        }
    }
    elseif ($isArray) {
        # If it's an array, get the type of the items and wrap it in Vec<>
        $itemType = Get-RustType -jsonType $jsonType -format $format -isArray $false -ref $ref -isOptional $false
        $rustType = "Vec<${itemType}>"
    }
    else {
        # Map JSON types to Rust types
        switch ($jsonType) {
            "string" {
                if ($format -eq "date-time") {
                    $rustType = "DateTime<Utc>"
                }
                else {
                    $rustType = "String"
                }
            }
            "integer" { $rustType = "i64" }
            "number" { $rustType = "f64" }
            "boolean" { $rustType = "bool" }
            "object" { $rustType = "serde_json::Value" } # Default for objects without a specific schema
            default { $rustType = "serde_json::Value" } # Default for unknown types
        }
    }
    
    # If the property is optional, wrap it in Option<>
    if ($isOptional) {
        $rustType = "Option<${rustType}>"
    }
    
    return $rustType
}

# Helper function to generate a Rust struct from a JSON schema
function Generate-RustStruct {
    param (
        [string]$structName,
        [PSCustomObject]$schema,
        [PSCustomObject]$baseSchema
    )
    
    $output = @()
    $output += "#[derive(Clone, Debug, Deserialize)]"
    $output += "pub struct $structName {"
    $output += ""
    
    # Add timestamp field from base schema
    $output += '    #[serde(with = "crate::event::format::date")]'
    $output += "    pub timestamp: DateTime<Utc>,"
    $output += ""
    
    # Process properties from the schema
    if ($schema.properties) {
        $requiredProps = @()
        if ($schema.required) {
            $requiredProps = $schema.required
        }
        
        foreach ($propName in $schema.properties.PSObject.Properties.Name) {
            # Skip the timestamp and event properties as they're handled separately
            if ($propName -eq "timestamp" -or $propName -eq "event") {
                continue
            }
            
            $prop = $schema.properties.$propName
            $isOptional = -not $requiredProps.Contains($propName)
            
            # Determine the Rust type for this property
            $jsonType = $prop.type
            $format = $prop.format
            $isArray = $jsonType -eq "array"
            $ref = $prop.'$ref'
            
            if ($isArray -and -not $ref) {
                # For arrays, get the type from the items property
                $jsonType = $prop.items.type
                $format = $prop.items.format
                $ref = $prop.items.'$ref'
            }
            
            $rustType = Get-RustType -jsonType $jsonType -format $format -isArray $isArray -ref $ref -isOptional $isOptional
            
            # Convert property name to snake_case for Rust
            $rustPropName = ConvertTo-SnakeCase -str $propName
            
            # Check if the property name is a Rust reserved keyword and escape it if needed
            $rustPropName = Escape-RustKeyword -name $rustPropName
            
            # Add the property to the struct
            # Check if this is an optional DateTime field and add the appropriate format
            if ($isOptional -and $jsonType -eq "string" -and $format -eq "date-time") {
                $output += ('    #[serde(rename = "' + $propName + '", with = "crate::event::format::optional_date")]')
            } else {
                $output += ('    #[serde(rename = "' + $propName + '")]')
            }
            $output += "    pub ${rustPropName}: ${rustType},"
            $output += ""
        }
    }
    
    # Close the struct
    $output += "}"
    
    return $output -join "`n"
}

# This function is no longer needed as we're not categorizing events anymore

# Main script

# Get the base schema
$baseSchemaPath = Join-Path (Get-Location) "ed-journal-schemas/schemas/_Event.json"
$baseSchema = Get-Content $baseSchemaPath -Raw | ConvertFrom-Json

# Get all schema directories
$schemaDir = Join-Path (Get-Location) "ed-journal-schemas/schemas"
$schemaDirs = Get-ChildItem -Path $schemaDir -Directory

# Create an array to store all structs
$allStructs = @()

foreach ($dir in $schemaDirs) {
    $eventName = $dir.Name
    
    # Skip directories that start with underscore (like _Event)
    if ($eventName.StartsWith("_")) {
        continue
    }
    
    # Get the schema file
    $schemaFile = Join-Path $dir.FullName "${eventName}.json"
    
    if (Test-Path $schemaFile) {
        Write-Host "Processing schema for $eventName..."
        
        # Read the schema
        $schema = Get-Content $schemaFile -Raw | ConvertFrom-Json
        
        # Generate the Rust struct
        $structCode = Generate-RustStruct -structName $eventName -schema $schema -baseSchema $baseSchema
        
        # Add the struct to our collection
        $allStructs += $structCode
    }
    else {
        Write-Host "Warning: Schema file not found for $eventName"
    }
}

# Create the event.rs file with all structs
$eventFilePath = Join-Path (Get-Location) "src/event.rs"

# Add imports and module declarations
$fileContent = @'
// This file is auto-generated by generate_event_structs.ps1
// Do not edit manually

pub mod format;
pub mod event_impl;

use chrono::{DateTime, Utc};
use serde::Deserialize;

'@

# Add all structs to the file
foreach ($structCode in $allStructs) {
    $fileContent += $structCode
    $fileContent += "`n`n"
}

# Generate the JournalEvent enum
$fileContent += "#[derive(Clone, Debug, Deserialize)]"
$fileContent += "`n#[serde(tag = ""event"")]"
$fileContent += "`npub enum JournalEvent {"
$fileContent += "`n"

# Extract struct names from the struct code
$structNames = @()
foreach ($structCode in $allStructs) {
    if ($structCode -match "pub struct (\w+)") {
        $structName = $matches[1]
        $structNames += $structName
        $fileContent += "    #[serde(rename = ""$structName"")]"
        $fileContent += "`n    $structName($structName),"
        $fileContent += "`n`n"
    }
}

$fileContent += "}"
$fileContent += "`n"

# Write the file
Set-Content -Path $eventFilePath -Value $fileContent

Write-Host "Generated $eventFilePath with $(($allStructs).Count) structs and JournalEvent enum"

Write-Host "Done!"