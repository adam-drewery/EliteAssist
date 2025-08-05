#!/usr/bin/env pwsh

# Script to generate Rust structs from JSON schemas in ed-journal-schemas/schemas

# Dictionary of merged struct name replacements
# Maps from the automatically determined name (highest alphabetically) to a more sensible name
$structNameReplacements = @{

    # Keep as is (from issue description)
    "SquadronPromotion" = "SquadronPromotion"

    # Generic ones
    "WingLeave" = "Empty"
    "MiningRefined" = "TypeDetails"
    "ProspectedAsteroidMaterial" = "NameAndProportion"
    
    "LeaveBody" = "Body"
    "CommunityGoalJoin" = "CommunityGoal"
    "MaterialDiscarded" = "Material"
    "SquadronCreated" = "Squadron"
    "HeatDamage" = "Damage"
    "PayFines" = "Payment"
    "CarrierShipPack" = "CarrierPack"
    "SellSuit" = "Suit"
    "MissionFailed" = "Mission"
    "RenameSuitLoadout" = "SuitLoadout"
    "Loadout" = "ShipLoadout"
    "SellWeapon" = "Weapon"
    "ShipLocker" = "Inventory"
    "BookTaxi" = "Booking"
    "ShipyardNew" = "Ship"
    "RepairAll" = "ShipEquipmentPurchase"
    "CarrierJumpCancelled" = "Carrier"
    "RefuelPartial" = "Refuel"
    "SRVDestroyed" = "SRV"
    "QuitACrew" = "Crew"
    "PowerplayDeliver" = "PowerplayDelivery"
    "LoadoutRemoveModule" = "LoadoutModule"
    "BuyTradeData" = "BuyData"
    "CrewMemberQuits" = "CrewMember"
    "DockingRequestedLandingPads" = "LandingPads"
    "CancelTaxi" = "Cancel"
    "UpgradeWeaponResource" = "Material"
    "SAASignalsFoundSignal" = "SAASignals"
    "ShipLockerMaterialsItem" = "Item"
    "TradeMicroResourcesOffered" = "MicroResources"
    "LocationPowerplayConflictProgress" = "ConflictProgress"
    "LocationFactionActiveState" = "FactionActiveState"
    "LocationFactionRecoveringState" = "FactionRecoveringState"
    "LocationSystemFaction" = "SystemFaction"
    "LocationThargoidWar" = "ThargoidWar"
    "ShipLockerMaterialsConsumable" = "Consumable"
    "LocationConflictFaction1" = "ConflictFaction1"
    "LocationConflictFaction2" = "ConflictFaction2"
    "SwitchSuitLoadoutModule" = "SuitLoadoutModule"
    "PowerplayLeave" = "PowerplayJoin"
    "MaterialTradeReceived" = "MaterialTraded"
    "ColonisationSystemClaimRelease" = "SystemClaim"
    "CarrierStatsShipPack" = "CarrierStats"
}

# Define a class to represent a Rust struct
class RustStruct {
    [string]$Name
    [string]$Description
    [System.Collections.ArrayList]$Attributes = @()
    [System.Collections.ArrayList]$Fields = @()
    [hashtable]$NestedStructs = @{}
    
    # Constructor
    RustStruct([string]$name) {
        $this.Name = $name
    }
    
    # Add an attribute to the struct (e.g., #[derive(...)])
    [void] AddAttribute([string]$attribute) {
        $this.Attributes.Add($attribute)
    }
    
    # Add a field to the struct
    [void] AddField([string]$name, [string]$type, [string]$rename, [string]$description) {
        $this.AddFieldWithSerdeAttrs($name, $type, $rename, $description, "")
    }
    
    # Add a field to the struct with custom serde attributes
    [void] AddFieldWithSerdeAttrs([string]$name, [string]$type, [string]$rename, [string]$description, [string]$serde_attributes) {
        $field = @{
            Name = $name
            Type = $type
            Rename = $rename
            Description = $description
            SerdeAttributes = $serde_attributes
        }
        $this.Fields.Add($field)
    }
    
    # Add a nested struct
    [void] AddNestedStruct([string]$key, [RustStruct]$struct) {
        $this.NestedStructs[$key] = $struct
    }
    
    # Convert the struct to a string representation
    [string] ToString() {
        $output = @()
        
        # Add description as doc comment if available
        if ($this.Description) {
            $output += "/// $($this.Description)"
        }
        
        # Add attributes
        foreach ($attr in $this.Attributes) {
            $output += $attr
        }
        
        # Add struct declaration
        $output += "pub struct $($this.Name) {"
        $output += ""
        
        # Add fields
        foreach ($field in $this.Fields) {
            # Add description as doc comment if available
            if ($field.Description) {
                $output += "    /// $($field.Description)"
            }
            
            # Add serde attribute
            if ($field.SerdeAttributes) {
                $output += "    #[serde($($field.SerdeAttributes))]"
            } else {
                $output += "    #[serde(rename = ""$($field.Rename)"")]"
            }
            
            # Add field declaration
            $output += "    pub $($field.Name): $($field.Type),"
            $output += ""
        }
        
        # Close the struct
        $output += "}"
        
        return $output -join "`n"
    }
}

# Helper function to convert PascalCase to snake_case
function ConvertTo-SnakeCase {
    param (
        [string]$str
    )

    # Handle empty strings
    if ([string]::IsNullOrEmpty($str)) {
        return $str
    }

    $str = $str -replace "fleetcarrier", "fleet_carrier"
    $str = $str -replace "pioneersupplies", "pioneer_supplies"
    $str = $str -replace "stolenprofit", "stolen_profit"
    $str = $str -replace "stolenspend", "stolen_spend"
    $str = $str -replace "tradeprofit", "trade_profit"
    $str = $str -replace "tradespend", "trade_spend"

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
    $result = ($words -join "_").ToLower()

    # Apply hard-coded replacements again to catch any that might have been missed
    $result = $result -replace "changechange", "change"
    $result = $result -replace "fleetcarrier", "fleet_carrier"
    $result = $result -replace "pioneersupplies", "pioneer_supplies"
    $result = $result -replace "stolenprofit", "stolen_profit"
    $result = $result -replace "stolenspend", "stolen_spend"

    return $result
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

# Helper function to singularize a name (e.g., convert "Items" to "Item")
function Singularize-Name {
    param (
        [string]$name
    )

    # Common plural endings and their singular forms
    if ($name -match 'ies$') {
        return $name -replace 'ies$', 'y'
    }
    elseif ($name -match 'ses$') {
        return $name -replace 'es$', ''
    }
    elseif ($name -match 's$' -and -not ($name -match 'ss$')) {
        return $name -replace 's$', ''
    }
    else {
        # If no rule matches or it's already singular, return as is
        return $name
    }
}

# Helper function to generate a struct name for a nested object
function Get-NestedStructName {
    param (
        [string]$parentName,
        [string]$propertyName
    )

    # Handle property names with underscores by properly capitalizing each part
    if ($propertyName -match '_') {
        $words = $propertyName -split '_'
        $titleCaseWords = $words | ForEach-Object { 
            if ($_.Length -gt 0) {
                $_.Substring(0, 1).ToUpper() + $_.Substring(1).ToLower()
            } else {
                $_
            }
        }
        $cleanPropertyName = $titleCaseWords -join ''
    } else {
        # For property names without underscores, just use as is
        $cleanPropertyName = $propertyName
    }

    # Apply hard-coded post-fix replacements
    $structName = "${parentName}${cleanPropertyName}"
    
    # Apply specific replacements as required
    $structName = $structName -replace "Changechange", "Change"

    return $structName
}

# Helper function to process array item objects and add them to the nested structs collection
function Process-ArrayItemObject {
    param (
        [PSCustomObject]$itemsSchema,
        [string]$parentName,
        [string]$propertyName,
        [hashtable]$nestedStructs,
        [PSCustomObject]$topLevelSchema,
        [bool]$isOptional
    )

    # If the items schema is an object with properties, add it to the nested structs
    if ($itemsSchema.type -eq "object" -and $itemsSchema.properties) {
        # Singularize the property name for array items
        $singularPropertyName = Singularize-Name -name $propertyName
        $itemStructName = Get-NestedStructName -parentName $parentName -propertyName $singularPropertyName

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
            "integer" { $rustType = "u64" }
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
            Struct = $null
            NestedStructs = @{}
        }
    }

    # Mark this type as being processed to prevent duplicate generation
    $processedTypes[$structName] = $true

    # Create a new RustStruct object
    $rustStruct = [RustStruct]::new($structName)
    
    # Add description if available
    if ($definition.description) {
        $rustStruct.Description = $definition.description
    }
    
    # Add attributes
    $rustStruct.AddAttribute("#[derive(Clone, Debug, Deserialize)]")
    
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
                $itemStructName = Process-ArrayItemObject -itemsSchema $prop.items -parentName $structName -propertyName $propName -nestedStructs $nestedStructs -topLevelSchema $topLevelSchema -isOptional $isOptional

                if ($itemStructName) {
                    # Use the nested struct name for the array item type
                    $rustType = "Vec<${itemStructName}>"
                    
                    # If the property is optional, wrap it in Option<>
                    if ($isOptional) {
                        $rustType = "Option<${rustType}>"
                    }

                    # Convert property name to snake_case for Rust
                    $rustPropName = ConvertTo-SnakeCase -str $propName

                    # Check if the property name is a Rust reserved keyword and escape it if needed
                    $rustPropName = Escape-RustKeyword -name $rustPropName

                    # Add the field to the struct
                    $rustStruct.AddField($rustPropName, $rustType, $propName, $prop.description)

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

            # Add the field to the struct with appropriate serde attributes
            if ($jsonType -eq "string" -and $format -eq "date-time") {
                $serdeAttrs = "rename = ""$propName"", with = ""crate::event::format::date"""
                $rustStruct.AddFieldWithSerdeAttrs($rustPropName, $rustType, $propName, $prop.description, $serdeAttrs)
            } else {
                $rustStruct.AddField($rustPropName, $rustType, $propName, $prop.description)
            }
        }
    }

    # Process nested structs recursively
    $nestedStructsResult = @{}

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
        if ($result.Struct -ne $null) {
            $nestedStructsResult[$nestedStruct.Name] = $result.Struct
            
            # Add nested structs to the current struct
            foreach ($nestedKey in $result.NestedStructs.Keys) {
                $nestedStructsResult[$nestedKey] = $result.NestedStructs[$nestedKey]
            }
        }
    }

    # Return both the struct and the nested structs collection
    return @{
        Struct = $rustStruct
        NestedStructs = $nestedStructsResult
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
            Struct = $null
            NestedStructs = @{}
        }
    }

    # Mark this type as being processed to prevent duplicate generation
    $processedTypes[$structName] = $true

    # Create a new RustStruct object
    $rustStruct = [RustStruct]::new($structName)
    $nestedStructs = @{}

    # Add description if available
    if ($schema.description) {
        $rustStruct.Description = $schema.description
    }

    # Add attributes
    $rustStruct.AddAttribute("#[derive(Clone, Debug, Deserialize)]")

    # Add timestamp field from base schema
    $rustStruct.AddFieldWithSerdeAttrs("timestamp", "DateTime<Utc>", "timestamp", "Event timestamp", "with = ""crate::event::format::date""")

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
                    $itemStructName = Process-ArrayItemObject -itemsSchema $prop.items -parentName $structName -propertyName $propName -nestedStructs $nestedStructs -topLevelSchema $schema -isOptional $isOptional

                    if ($itemStructName) {
                        # Use the nested struct name for the array item type
                        $rustType = "Vec<${itemStructName}>"
                        
                        # If the property is optional, wrap it in Option<>
                        if ($isOptional) {
                            $rustType = "Option<${rustType}>"
                        }

                        # Convert property name to snake_case for Rust
                        $rustPropName = ConvertTo-SnakeCase -str $propName

                        # Check if the property name is a Rust reserved keyword and escape it if needed
                        $rustPropName = Escape-RustKeyword -name $rustPropName

                        # Add the field to the struct
                        $rustStruct.AddField($rustPropName, $rustType, $propName, $prop.description)

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

            # Add the field to the struct with appropriate serde attributes
            if ($isOptional -and $jsonType -eq "string" -and $format -eq "date-time") {
                $serdeAttrs = "rename = ""$propName"", with = ""crate::event::format::optional_date"""
                $rustStruct.AddFieldWithSerdeAttrs($rustPropName, $rustType, $propName, $prop.description, $serdeAttrs)
            } else {
                $rustStruct.AddField($rustPropName, $rustType, $propName, $prop.description)
            }
        }
    }

    # Generate nested structs if needed
    $nestedStructsResult = @{}

    # Process definitions from the schema
    if ($schema.definitions) {
        foreach ($defName in $schema.definitions.PSObject.Properties.Name) {
            $definition = $schema.definitions.$defName
            $nestedStructName = Get-NestedStructName -parentName $structName -propertyName $defName

            # Check if we've already processed this type
            if (-not $processedTypes.ContainsKey($nestedStructName)) {
                $result = Generate-NestedStruct -structName $nestedStructName -definition $definition -topLevelSchema $schema -nestedStructs $nestedStructs
                if ($result.Struct -ne $null) {
                    $nestedStructsResult[$nestedStructName] = $result.Struct
                    
                    # Add nested structs to the current struct
                    foreach ($nestedKey in $result.NestedStructs.Keys) {
                        $nestedStructsResult[$nestedKey] = $result.NestedStructs[$nestedKey]
                    }
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
            if ($result.Struct -ne $null) {
                $nestedStructsResult[$nestedStruct.Name] = $result.Struct
                
                # Add nested structs to the current struct
                foreach ($nestedKey in $result.NestedStructs.Keys) {
                    $nestedStructsResult[$nestedKey] = $result.NestedStructs[$nestedKey]
                }
            }
        }
    }

    # Return both the struct and the nested structs collection
    return @{
        Struct = $rustStruct
        NestedStructs = $nestedStructsResult
    }
}

# Helper function to compare two structs for structural equality
function Compare-StructStructure {
    param (
        [RustStruct]$struct1,
        [RustStruct]$struct2
    )

    # If the number of fields is different, they're not equal
    if ($struct1.Fields.Count -ne $struct2.Fields.Count) {
        return $false
    }

    # Create a hashtable of field types for struct1
    $struct1Fields = @{}
    foreach ($field in $struct1.Fields) {
        $struct1Fields[$field.Name] = $field.Type
    }

    # Compare field types with struct2
    foreach ($field in $struct2.Fields) {
        if (-not $struct1Fields.ContainsKey($field.Name) -or $struct1Fields[$field.Name] -ne $field.Type) {
            return $false
        }
    }

    # If we got here, the structures are identical
    return $true
}

# Helper function to get the name that is highest alphabetically in a list of struct names
function Get-HighestAlphabeticalName {
    param (
        [string[]]$names
    )

    if ($names.Count -eq 0) {
        return ""
    }
    
    if ($names.Count -eq 1) {
        return $names[0]
    }

    # Sort the names alphabetically and return the highest one
    $sortedNames = $names | Sort-Object
    return $sortedNames[-1]
}

# Helper function to merge documentation from multiple structs
function Merge-Documentation {
    param (
        [string[]]$descriptions
    )
    
    # Filter out empty descriptions
    $validDescriptions = $descriptions | Where-Object { $_ -ne $null -and $_ -ne "" }
    
    if ($validDescriptions.Count -eq 0) {
        return ""
    }
    
    if ($validDescriptions.Count -eq 1) {
        return $validDescriptions[0]
    }
    
    # Join all descriptions with a separator
    return ($validDescriptions -join " | ")
}

# Main script

# Get the base schema
$baseSchemaPath = Join-Path (Get-Location) "ed-journal-schemas/schemas/_Event.json"
$baseSchema = Get-Content $baseSchemaPath -Raw | ConvertFrom-Json

# Get all schema directories
$schemaDir = Join-Path (Get-Location) "ed-journal-schemas/schemas"
$schemaDirs = Get-ChildItem -Path $schemaDir -Directory

# Create a model to store all structs
$allStructs = @{}
$allStructNames = @()

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

                # Add the struct to our model if it's not null
                if ($result.Struct -ne $null) {
                    $allStructs[$defName] = $result.Struct
                    
                    # Add nested structs to our model
                    foreach ($nestedKey in $result.NestedStructs.Keys) {
                        $allStructs[$nestedKey] = $result.NestedStructs[$nestedKey]
                    }
                }

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

        # Add the struct to our model if it's not null
        if ($result.Struct -ne $null) {
            $allStructs[$eventName] = $result.Struct
            $allStructNames += $eventName
            
            # Add nested structs to our model
            foreach ($nestedKey in $result.NestedStructs.Keys) {
                $allStructs[$nestedKey] = $result.NestedStructs[$nestedKey]
            }
        }
    }
    else {
        Write-Host "Warning: Schema file not found for $eventName"
    }
}

# Create the event.rs file with all structs
$eventFilePath = Join-Path (Get-Location) "src/event.rs"

# Get all module files from the event directory
$eventDir = Join-Path (Get-Location) "src/event"
$moduleFiles = Get-ChildItem -Path $eventDir -Filter "*.rs" | ForEach-Object { $_.Name -replace '\.rs$', '' }

# Add imports and module declarations
$moduleDeclarations = $moduleFiles | ForEach-Object { "pub mod $_;" }
$moduleDeclarationsString = $moduleDeclarations -join "`n"

$fileContent = @"
// This file is auto-generated by generate_event_structs.ps1
// Do not edit manually

$moduleDeclarationsString

use chrono::{DateTime, Utc};
use serde::Deserialize;

"@

# Group structs with identical structures
Write-Host "Grouping structs with identical structures..."
$structGroups = @{}
$structMapping = @{}

# Create a list of all structs (both top-level and nested)
$topLevelStructs = @()
foreach ($structName in $allStructs.Keys) {
    $topLevelStructs += @{
        Name = $structName
        Struct = $allStructs[$structName]
    }
}

# Group structs with identical structures
foreach ($struct in $topLevelStructs) {
    $found = $false
    
    # Check if this struct matches any existing group
    foreach ($groupKey in $structGroups.Keys) {
        $groupStruct = $structGroups[$groupKey][0].Struct
        
        # Compare the structures
        if (Compare-StructStructure -struct1 $groupStruct -struct2 $struct.Struct) {
            # Add this struct to the group
            $structGroups[$groupKey] += $struct
            $found = $true
            break
        }
    }
    
    # If no matching group was found, create a new one
    if (-not $found) {
        $groupKey = "Group_" + $structGroups.Count
        $structGroups[$groupKey] = @($struct)
    }
}

# Process each group to create merged structs
$mergedStructs = @{}
$mergedStructNames = @()
$originalToMergedMap = @{}

foreach ($groupKey in $structGroups.Keys) {
    $group = $structGroups[$groupKey]
    
    # If there's only one struct in the group, no merging needed
    if ($group.Count -eq 1) {
        $struct = $group[0]
        $mergedStructs[$struct.Name] = $struct.Struct
        $mergedStructNames += $struct.Name
        $originalToMergedMap[$struct.Name] = $struct.Name
        continue
    }

    # Get the names of all structs in this group
    $structNames = $group | ForEach-Object { $_.Name }
    
    # Get the name that is highest alphabetically to use as the merged struct name
    $highestName = Get-HighestAlphabeticalName -names $structNames
    
    # Check if there's a more sensible replacement name in the dictionary
    if ($structNameReplacements.ContainsKey($highestName)) {
        $commonPrefix = $structNameReplacements[$highestName]
    } else {
        $commonPrefix = $highestName
    }

    # Multiple structs with identical structure - merge them
    Write-Host "Merging $($group.Count) structs with identical structure: $($group | ForEach-Object { $_.Name } | Join-String -Separator ', ') into $commonPrefix"
    
    # Create a new struct with the common prefix as the name
    $mergedStruct = [RustStruct]::new($commonPrefix)
    
    # Merge documentation from all structs in the group
    $descriptions = $group | ForEach-Object { $_.Struct.Description }
    $mergedDescription = Merge-Documentation -descriptions $descriptions
    $mergedStruct.Description = $mergedDescription
    
    # Add attributes
    $mergedStruct.AddAttribute("#[derive(Clone, Debug, Deserialize)]")
    
    # Copy fields from the first struct (they all have identical structures)
    foreach ($field in $group[0].Struct.Fields) {
        $mergedStruct.AddFieldWithSerdeAttrs(
            $field.Name,
            $field.Type,
            $field.Rename,
            $field.Description,
            $field.SerdeAttributes
        )
    }
    
    # Add the merged struct to our collection
    $mergedStructs[$commonPrefix] = $mergedStruct
    $mergedStructNames += $commonPrefix
    
    # Map original struct names to the merged name
    foreach ($structName in $structNames) {
        $originalToMergedMap[$structName] = $commonPrefix
    }
}

# Note: We no longer need to add non-top-level structs separately
# as they are now included in the de-duplication process above

# Update field types in all structs to use the merged struct names
Write-Host "Updating field type references to use merged struct names..."
foreach ($key in $mergedStructs.Keys) {
    $struct = $mergedStructs[$key]
    
    # Update field types to use merged struct names
    foreach ($field in $struct.Fields) {
        $fieldType = $field.Type
        
        # Check if the field type references a struct that was merged
        # Handle different type patterns: direct references, Vec<Type>, Option<Type>, Option<Vec<Type>>
        if ($fieldType -match '^([A-Za-z0-9_]+)$') {
            # Direct reference to a type
            $typeName = $matches[1]
            if ($originalToMergedMap.ContainsKey($typeName) -and $originalToMergedMap[$typeName] -ne $typeName) {
                $field.Type = $originalToMergedMap[$typeName]
            }
        }
        elseif ($fieldType -match '^Vec<([A-Za-z0-9_]+)>$') {
            # Vec<Type>
            $typeName = $matches[1]
            if ($originalToMergedMap.ContainsKey($typeName) -and $originalToMergedMap[$typeName] -ne $typeName) {
                $field.Type = "Vec<$($originalToMergedMap[$typeName])>"
            }
        }
        elseif ($fieldType -match '^Option<([A-Za-z0-9_]+)>$') {
            # Option<Type>
            $typeName = $matches[1]
            if ($originalToMergedMap.ContainsKey($typeName) -and $originalToMergedMap[$typeName] -ne $typeName) {
                $field.Type = "Option<$($originalToMergedMap[$typeName])>"
            }
        }
        elseif ($fieldType -match '^Option<Vec<([A-Za-z0-9_]+)>>$') {
            # Option<Vec<Type>>
            $typeName = $matches[1]
            if ($originalToMergedMap.ContainsKey($typeName) -and $originalToMergedMap[$typeName] -ne $typeName) {
                $field.Type = "Option<Vec<$($originalToMergedMap[$typeName])>>"
            }
        }
    }
}

# Add all structs to the file
foreach ($key in $mergedStructs.Keys) {
    $struct = $mergedStructs[$key]
    $fileContent += $struct.ToString()
    $fileContent += "`n`n"
}

# Generate the JournalEvent enum
$fileContent += "#[derive(Clone, Debug, Deserialize)]"
$fileContent += "`n#[serde(tag = ""event"")]"
$fileContent += "`npub enum JournalEvent {"
$fileContent += "`n"

# Add enum variants for top-level structs
foreach ($structName in $allStructNames) {
    $mergedName = $originalToMergedMap[$structName]
    
    $fileContent += "    #[serde(rename = ""$structName"")]"
    $fileContent += "`n    $structName($mergedName),"
    $fileContent += "`n`n"
}

$fileContent += "}"
$fileContent += "`n"

# Write the file
Set-Content -Path $eventFilePath -Value $fileContent

Write-Host "Generated $eventFilePath with $($mergedStructNames.Count) structs (merged from $($allStructs.Count) original structs, including nested types) and JournalEvent enum"

Write-Host "Done!"