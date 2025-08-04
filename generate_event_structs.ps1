#!/usr/bin/env pwsh

# Script to generate Rust structs from JSON schemas in ed-journal-schemas/schemas
# This is a fixed version that properly handles nested object types and arrays of references

# Helper function to convert PascalCase to snake_case
function ConvertTo-SnakeCase {
    param (
        [string]$str
    )

    # Handle empty strings
    if ([string]::IsNullOrEmpty($str)) {
        return $str
    }

    # Special case for CAPS_LOCK format (all uppercase with underscores)
    # Example: TG_ENCOUNTERS -> tg_encounters
    if ($str -cmatch '^[A-Z0-9_]+$') {
        # Just convert to lowercase without changing the structure
        return $str.ToLower()
    }

    # Special case for ID suffix - convert "MarketID" to "market_id" not "market_i_d"
    if ($str -cmatch 'ID$') {
        $str = $str.Substring(0, $str.Length - 2) + "Id"
    }

    # Initialize variables
    $words = @()
    $currentWord = ""
    $inAcronym = $false

    # Process each character
    for ($i = 0; $i -lt $str.Length; $i++) {
        $char = $str[$i]

        # Check if this is an underscore (word boundary)
        if ($char -eq '_') {
            if ($currentWord -ne "") {
                $words += $currentWord
                $currentWord = ""
            }
            $inAcronym = $false
            continue
        }

        # Check if this is a capital letter
        if ($char -cmatch '[A-Z]') {
            # If we're at the start of the string or the current word is empty,
            # just add to the current word
            if ($i -eq 0 -or $currentWord -eq "") {
                $currentWord += $char
                # Check if we're starting an acronym (next char is also uppercase)
                $inAcronym = ($i + 1 -lt $str.Length) -and ($str[$i + 1] -cmatch '[A-Z]')
            }
            # If we're in an acronym and this is still part of it
            elseif ($inAcronym) {
                # If this is the last letter of the acronym (next char is lowercase or end of string)
                if (($i + 1 -eq $str.Length) -or ($str[$i + 2] -cmatch '[a-z]')) {
                    $currentWord += $char
                    $words += $currentWord
                    $currentWord = ""
                    $inAcronym = $false
                } else {
                    $currentWord += $char
                }
            }
            # Otherwise, this capital letter starts a new word
            else {
                if ($currentWord -ne "") {
                    $words += $currentWord
                }
                $currentWord = $char
                # Check if we're starting an acronym
                $inAcronym = ($i + 1 -lt $str.Length) -and ($str[$i + 1] -cmatch '[A-Z]')
            }
        }
        # For lowercase letters and other characters, just add to the current word
        else {
            $currentWord += $char
            $inAcronym = $false
        }
    }

    # Add the last word if there is one
    if ($currentWord -ne "") {
        $words += $currentWord
    }

    # Join the words with underscores and convert to lowercase
    return ($words -join "_").ToLower()
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

# Helper function to generate a struct name for a nested object
function Get-NestedStructName {
    param (
        [string]$parentName,
        [string]$propertyName
    )

    # Remove underscores from property name to create a clean struct name
    $cleanPropertyName = $propertyName -replace '_', ''

    return "${parentName}${cleanPropertyName}"
}

# Helper function to process array item objects and add them to the nested structs collection
function Process-ArrayItemObject {
    param (
        [PSCustomObject]$itemsSchema,
        [string]$parentName,
        [string]$propertyName,
        [hashtable]$nestedStructs,
        [PSCustomObject]$topLevelSchema
    )

    # If the items schema is an object with properties, add it to the nested structs
    if ($itemsSchema.type -eq "object" -and $itemsSchema.properties) {
        $itemStructName = Get-NestedStructName -parentName $parentName -propertyName $propertyName

        # Check if we've already processed this type globally
        if ($processedTypes.ContainsKey($itemStructName)) {
            # We've already processed this type, just return the name
            return $itemStructName
        }

        # Check if we've already processed this nested struct in this context
        $key = "${parentName}_${propertyName}"
        if (-not $nestedStructs.ContainsKey($key)) {
            # Add the nested struct to our collection
            $nestedStructs[$key] = @{
                Name = $itemStructName
                Definition = $itemsSchema
            }
        }

        return $itemStructName
    }

    return $null
}

# Helper function to determine the Rust type for a JSON schema type
function Get-RustType {
    param (
        [string]$jsonType,
        [string]$format,
        [bool]$isArray,
        [string]$ref,
        [bool]$isOptional,
        [PSCustomObject]$schema,
        [PSCustomObject]$topLevelSchema,
        [string]$parentName,
        [string]$propertyName,
        [hashtable]$nestedStructs
    )

    $rustType = ""

    # No types should be treated as problematic - all should be properly mapped to Rust structs
    # This includes sub-types defined in the "properties" field
    $problematicTypes = @()

    if ($ref) {
        # If there's a reference, use that as the type
        if ($ref.StartsWith("#/definitions/")) {
            # Reference to a definition within the same schema file
            $defName = $ref.Substring(14)  # Remove "#/definitions/"

            # Check if this is a problematic type
            if ($problematicTypes -contains $defName) {
                $rustType = "serde_json::Value"
            } else {
                # Generate a struct name based on the parent struct and definition name
                $structName = Get-NestedStructName -parentName $parentName -propertyName $defName

                # Check if we've already processed this definition
                if (-not $nestedStructs.ContainsKey($defName)) {
                    # Get the definition from the top-level schema
                    $definition = $topLevelSchema.definitions.$defName

                    if ($definition) {
                        # Generate a struct for this definition
                        $nestedStructs[$defName] = @{
                            Name = $structName
                            Definition = $definition
                        }
                    } else {
                        # Definition not found, fallback to serde_json::Value
                        Write-Host "Warning: Definition '$defName' not found in schema, using serde_json::Value"
                        $rustType = "serde_json::Value"
                    }
                }

                # Use the struct name as the type
                $rustType = $structName
            }
        } else {
            # Reference to an external schema file
            # Check if it's a reference to a definition in another schema file
            if ($ref -match "^\.\./(.*?)#definitions/(.*)$") {
                $schemaFilePath = $matches[1]
                $defName = $matches[2]

                # Construct the full path to the schema file
                $fullSchemaPath = Join-Path (Get-Location) "ed-journal-schemas/schemas/$schemaFilePath"

                if (Test-Path $fullSchemaPath) {
                    # Load the referenced schema file
                    $refSchema = Get-Content $fullSchemaPath -Raw | ConvertFrom-Json

                    # Get the definition from the schema
                    $definition = $refSchema.definitions.$defName

                    if ($definition) {
                        # Generate a struct name based on the definition name
                        $structName = $defName

                        # Check if we've already processed this type
                        if (-not $processedTypes.ContainsKey($defName)) {
                            # Check if we've already processed this definition in this run
                            if (-not $nestedStructs.ContainsKey($defName)) {
                                # Generate a struct for this definition
                                $nestedStructs[$defName] = @{
                                    Name = $structName
                                    Definition = $definition
                                }
                            }
                            # Mark this type as processed
                            $processedTypes[$defName] = $true
                        }

                        # Use the struct name as the type
                        $rustType = $structName
                    } else {
                        # Definition not found, fallback to serde_json::Value
                        Write-Host "Warning: Definition '$defName' not found in schema file '$schemaFilePath', using serde_json::Value"
                        $rustType = "serde_json::Value"
                    }
                } else {
                    # Schema file not found, fallback to serde_json::Value
                    Write-Host "Warning: Schema file '$schemaFilePath' not found, using serde_json::Value"
                    $rustType = "serde_json::Value"
                }
            } else {
                # Regular reference to an external schema file
                $refParts = $ref -split "/"
                $typeName = $refParts[-1]

                # Check if this is a problematic type
                if ($problematicTypes -contains $typeName) {
                    $rustType = "serde_json::Value"
                } else {
                    $rustType = $typeName
                }
            }
        }
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
            "object" {
                # For object types, generate a nested struct
                $structName = Get-NestedStructName -parentName $parentName -propertyName $propertyName

                # Check if we've already processed this nested struct
                $key = "${parentName}_${propertyName}"
                if (-not $nestedStructs.ContainsKey($key)) {
                    # Add the nested struct to our collection
                    $nestedStructs[$key] = @{
                        Name = $structName
                        Definition = $schema
                    }
                }

                $rustType = $structName
            }
            default {
                # Default for unknown types
                Write-Host "Warning: Unknown type '$jsonType' for property '$propertyName' in '$parentName', using serde_json::Value"
                $rustType = "serde_json::Value"
            }
        }
    }

    # If it's an array, wrap the type in Vec<>
    if ($isArray) {
        $rustType = "Vec<${rustType}>"
    }

    # If the property is optional, wrap it in Option<>
    if ($isOptional) {
        $rustType = "Option<${rustType}>"
    }

    return $rustType
}

# Helper function to generate a Rust struct from a JSON schema definition
function Generate-NestedStruct {
    param (
        [string]$structName,
        [PSCustomObject]$definition,
        [PSCustomObject]$topLevelSchema,
        [hashtable]$nestedStructs = @{}
    )

    # Check if we've already processed this exact struct name
    if ($processedTypes.ContainsKey($structName)) {
        # Return empty result since this struct has already been generated
        return @{
            StructCode = ""
            NestedStructs = @{}
        }
    }

    # Mark this type as being processed to prevent duplicate generation
    $processedTypes[$structName] = $true

    $output = @()
    $output += "#[derive(Clone, Debug, Deserialize)]"
    $output += "pub struct $structName {"
    $output += ""

    # Process properties from the definition
    if ($definition.properties) {
        $requiredProps = @()
        if ($definition.required) {
            $requiredProps = $definition.required
        }

        foreach ($propName in $definition.properties.PSObject.Properties.Name) {
            $prop = $definition.properties.$propName
            $isOptional = -not $requiredProps.Contains($propName)

            # Determine the Rust type for this property
            $jsonType = $prop.type
            $format = $prop.format
            $isArray = $jsonType -eq "array"
            $ref = $prop.'$ref'

            # For arrays, get the type from the items property
            if ($isArray) {
                if ($prop.items.'$ref') {
                    $ref = $prop.items.'$ref'
                    $jsonType = ""  # Clear the type since we're using a reference
                } elseif ($prop.items.type) {
                    $jsonType = $prop.items.type
                    $format = $prop.items.format
                }
            }

            # Process array item objects
            if ($isArray -and $prop.items.type -eq "object" -and $prop.items.properties) {
                $itemStructName = Process-ArrayItemObject -itemsSchema $prop.items -parentName $structName -propertyName $propName -nestedStructs $nestedStructs -topLevelSchema $topLevelSchema

                if ($itemStructName) {
                    # Use the nested struct name for the array item type
                    $rustType = "Vec<${itemStructName}>"

                    # Convert property name to snake_case for Rust
                    $rustPropName = ConvertTo-SnakeCase -str $propName

                    # Check if the property name is a Rust reserved keyword and escape it if needed
                    $rustPropName = Escape-RustKeyword -name $rustPropName

                    # Add the property to the struct
                    $output += ('    #[serde(rename = "' + $propName + '")]')
                    $output += "    pub ${rustPropName}: ${rustType},"
                    $output += ""

                    # Skip the rest of the type determination since we've already processed this property
                    continue
                }
            }

            # Pass the nestedStructs parameter to Get-RustType
            $rustType = Get-RustType -jsonType $jsonType -format $format -isArray $isArray -ref $ref -isOptional $isOptional -schema $prop -topLevelSchema $topLevelSchema -parentName $structName -propertyName $propName -nestedStructs $nestedStructs

            # Convert property name to snake_case for Rust
            $rustPropName = ConvertTo-SnakeCase -str $propName

            # Check if the property name is a Rust reserved keyword and escape it if needed
            $rustPropName = Escape-RustKeyword -name $rustPropName

            # Add the property to the struct
            # Check if this is a DateTime field and add the appropriate format
            if ($jsonType -eq "string" -and $format -eq "date-time") {
                $output += ('    #[serde(rename = "' + $propName + '", with = "crate::event::format::date")]')
            } else {
                $output += ('    #[serde(rename = "' + $propName + '")]')
            }
            $output += "    pub ${rustPropName}: ${rustType},"
            $output += ""
        }
    }

    # Close the struct
    $output += "}"

    # Process nested structs recursively
    $nestedStructOutput = @()

    # Create a copy of the keys to avoid modifying the collection during iteration
    $keys = @($nestedStructs.Keys)

    foreach ($key in $keys) {
        $nestedStruct = $nestedStructs[$key]

        # Skip if we've already processed this type
        if ($processedTypes.ContainsKey($nestedStruct.Name)) {
            continue
        }

        # Create a new hashtable for nested structs to avoid circular references
        $newNestedStructs = @{}

        # Generate the nested struct
        $result = Generate-NestedStruct -structName $nestedStruct.Name -definition $nestedStruct.Definition -topLevelSchema $topLevelSchema -nestedStructs $newNestedStructs
        if ($result.StructCode -ne "") {
            $nestedStructOutput += $result.StructCode
        }
    }

    # Combine the main struct and nested structs
    $structCode = $output -join "`n"
    if ($nestedStructOutput.Count -gt 0) {
        $structCode += "`n`n" + ($nestedStructOutput -join "`n`n")
    }

    # Return both the struct code and the nested structs collection
    return @{
        StructCode = $structCode
        NestedStructs = $nestedStructs
    }
}

# Helper function to generate a Rust struct from a JSON schema
function Generate-RustStruct {
    param (
        [string]$structName,
        [PSCustomObject]$schema,
        [PSCustomObject]$baseSchema
    )

    # Check if we've already processed this exact struct name
    if ($processedTypes.ContainsKey($structName)) {
        # Return empty result since this struct has already been generated
        return @{
            StructCode = ""
            NestedStructs = @{}
        }
    }

    # Mark this type as being processed to prevent duplicate generation
    $processedTypes[$structName] = $true

    $output = @()
    $nestedStructs = @{}

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

            # For arrays, get the type from the items property
            if ($isArray) {
                if ($prop.items.'$ref') {
                    $ref = $prop.items.'$ref'
                    $jsonType = ""  # Clear the type since we're using a reference
                } elseif ($prop.items.type) {
                    $jsonType = $prop.items.type
                    $format = $prop.items.format
                }

                # Process array item objects
                if ($prop.items.type -eq "object" -and $prop.items.properties) {
                    $itemStructName = Process-ArrayItemObject -itemsSchema $prop.items -parentName $structName -propertyName $propName -nestedStructs $nestedStructs -topLevelSchema $schema

                    if ($itemStructName) {
                        # Use the nested struct name for the array item type
                        $rustType = "Vec<${itemStructName}>"

                        # Convert property name to snake_case for Rust
                        $rustPropName = ConvertTo-SnakeCase -str $propName

                        # Check if the property name is a Rust reserved keyword and escape it if needed
                        $rustPropName = Escape-RustKeyword -name $rustPropName

                        # Add the property to the struct
                        $output += ('    #[serde(rename = "' + $propName + '")]')
                        $output += "    pub ${rustPropName}: ${rustType},"
                        $output += ""

                        # Skip the rest of the type determination since we've already processed this property
                        continue
                    }
                }
            }

            $rustType = Get-RustType -jsonType $jsonType -format $format -isArray $isArray -ref $ref -isOptional $isOptional -schema $prop -topLevelSchema $schema -parentName $structName -propertyName $propName -nestedStructs $nestedStructs

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

    # Generate nested structs if needed
    $nestedStructOutput = @()

    # Process definitions from the schema
    if ($schema.definitions) {
        foreach ($defName in $schema.definitions.PSObject.Properties.Name) {
            $definition = $schema.definitions.$defName
            $nestedStructName = Get-NestedStructName -parentName $structName -propertyName $defName

            # Check if we've already processed this type
            if (-not $processedTypes.ContainsKey($nestedStructName)) {
                $result = Generate-NestedStruct -structName $nestedStructName -definition $definition -topLevelSchema $schema -nestedStructs $nestedStructs
                if ($result.StructCode -ne "") {
                    $nestedStructOutput += ""
                    $nestedStructOutput += $result.StructCode
                }
            }
        }
    }

    # Process other nested structs
    foreach ($key in $nestedStructs.Keys) {
        $nestedStruct = $nestedStructs[$key]

        # Check if we've already processed this type
        if (-not $processedTypes.ContainsKey($nestedStruct.Name)) {
            $result = Generate-NestedStruct -structName $nestedStruct.Name -definition $nestedStruct.Definition -topLevelSchema $schema -nestedStructs $nestedStructs
            if ($result.StructCode -ne "") {
                $nestedStructOutput += ""
                $nestedStructOutput += $result.StructCode
            }
        }
    }

    # Combine the main struct and nested structs
    $structCode = $output -join "`n"
    if ($nestedStructOutput.Count -gt 0) {
        $structCode += "`n`n" + ($nestedStructOutput -join "`n")
    }

    # Return both the struct code and the nested structs collection
    return @{
        StructCode = $structCode
        NestedStructs = $nestedStructs
    }
}

# Main script

# Get the base schema
$baseSchemaPath = Join-Path (Get-Location) "ed-journal-schemas/schemas/_Event.json"
$baseSchema = Get-Content $baseSchemaPath -Raw | ConvertFrom-Json

# Get all schema directories
$schemaDir = Join-Path (Get-Location) "ed-journal-schemas/schemas"
$schemaDirs = Get-ChildItem -Path $schemaDir -Directory

# Create an array to store all structs
$allStructs = @()

# Create a hashtable to track which types have already been processed
# This will help us avoid generating duplicate structs
$processedTypes = @{}

# Process common schema files first
$commonDir = Join-Path $schemaDir "common"
if (Test-Path $commonDir) {
    $commonFiles = Get-ChildItem -Path $commonDir -Filter "*.json"
    foreach ($file in $commonFiles) {
        Write-Host "Processing common schema file: $($file.Name)..."

        # Read the schema
        $schema = Get-Content $file.FullName -Raw | ConvertFrom-Json

        # Process definitions in the schema
        if ($schema.definitions) {
            foreach ($defName in $schema.definitions.PSObject.Properties.Name) {
                $definition = $schema.definitions.$defName

                # Skip if we've already processed this type
                if ($processedTypes.ContainsKey($defName)) {
                    continue
                }

                # Generate a struct for this definition
                $result = Generate-NestedStruct -structName $defName -definition $definition -topLevelSchema $schema

                # Add the struct to our collection
                $allStructs += $result.StructCode

                # Mark this type as processed
                $processedTypes[$defName] = $true
            }
        }
    }
}

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

        # Skip if we've already processed this type
        if ($processedTypes.ContainsKey($eventName)) {
            Write-Host "  Skipping $eventName (already processed)"
            continue
        }

        # Generate the Rust struct
        $result = Generate-RustStruct -structName $eventName -schema $schema -baseSchema $baseSchema

        # Add the struct to our collection if it's not empty
        if ($result.StructCode -ne "") {
            $allStructs += $result.StructCode
        }

        # Process nested structs
        foreach ($key in $result.NestedStructs.Keys) {
            $nestedStruct = $result.NestedStructs[$key]

            # Skip if we've already processed this type
            if ($processedTypes.ContainsKey($nestedStruct.Name)) {
                continue
            }

            # Generate the nested struct
            $nestedResult = Generate-NestedStruct -structName $nestedStruct.Name -definition $nestedStruct.Definition -topLevelSchema $schema

            # Add the nested struct to our collection if it's not empty
            if ($nestedResult.StructCode -ne "") {
                $allStructs += $nestedResult.StructCode
            }
        }
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
pub mod cargo;

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
    if ($structCode -match "pub struct (\w+) \{") {
        $structName = $matches[1]
        # Only include top-level structs in the enum (not nested ones)
        if (-not $structName.Contains("_")) {
            $structNames += $structName
            $fileContent += "    #[serde(rename = ""$structName"")]"
            $fileContent += "`n    $structName($structName),"
            $fileContent += "`n`n"
        }
    }
}

$fileContent += "}"
$fileContent += "`n"

# Write the file
Set-Content -Path $eventFilePath -Value $fileContent

Write-Host "Generated $eventFilePath with $(($structNames).Count) structs and JournalEvent enum"

Write-Host "Done!"