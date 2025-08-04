#!/usr/bin/env pwsh

# Source only the functions we need for testing
. ./generate_event_structs.ps1

# Prevent the main script from running
$processedTypes = @{}

# Test cases for ConvertTo-SnakeCase function
$testCases = @(
    @{
        Input = "Changechange"
        ExpectedResult = "change"
    },
    @{
        Input = "fleetcarrier"
        ExpectedResult = "fleet_carrier"
    },
    @{
        Input = "fleetcarrier_export_total"
        ExpectedResult = "fleet_carrier_export_total"
    },
    @{
        Input = "pioneersupplies"
        ExpectedResult = "pioneer_supplies"
    },
    @{
        Input = "TaxRate_pioneersupplies"
        ExpectedResult = "tax_rate_pioneer_supplies"
    },
    @{
        Input = "stolenprofit"
        ExpectedResult = "stolen_profit"
    },
    @{
        Input = "fleetcarrier_stolenprofit_total"
        ExpectedResult = "fleet_carrier_stolen_profit_total"
    },
    @{
        Input = "stolenspend"
        ExpectedResult = "stolen_spend"
    },
    @{
        Input = "fleetcarrier_stolenspend_total"
        ExpectedResult = "fleet_carrier_stolen_spend_total"
    }
)

# Run tests
$passCount = 0
$failCount = 0

foreach ($test in $testCases) {
    $result = ConvertTo-SnakeCase -str $test.Input
    
    if ($result -eq $test.ExpectedResult) {
        Write-Host "PASS: $($test.Input) -> $result" -ForegroundColor Green
        $passCount++
    } else {
        Write-Host "FAIL: $($test.Input) -> $result (Expected: $($test.ExpectedResult))" -ForegroundColor Red
        $failCount++
    }
}

Write-Host "`nTest Results: $passCount passed, $failCount failed"