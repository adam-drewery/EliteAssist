#!/usr/bin/env pwsh

# Source only the functions we need for testing
. ./generate_event_structs.ps1

# Prevent the main script from running
$processedTypes = @{}

# Test cases
$testCases = @(
    @{
        ParentName = "Backpack"
        PropertyName = "Changechange"
        ExpectedResult = "BackpackChange"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "FLEETCARRIER"
        ExpectedResult = "StatisticsFleetCarrier"
    },
    @{
        ParentName = "Market"
        PropertyName = "pioneersupplies"
        ExpectedResult = "MarketPioneerSupplies"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "fleetcarrier_stolenprofit_total"
        ExpectedResult = "StatisticsFleetCarrierStolenProfitTotal"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "fleetcarrier_stolenspend_total"
        ExpectedResult = "StatisticsFleetCarrierStolenSpendTotal"
    },
    @{
        ParentName = "Backpack"
        PropertyName = "ChangechangeEntry"
        ExpectedResult = "BackpackChangeEntry"
    }
)

# Run tests
$passCount = 0
$failCount = 0

foreach ($test in $testCases) {
    $result = Get-NestedStructName -parentName $test.ParentName -propertyName $test.PropertyName
    
    if ($result -eq $test.ExpectedResult) {
        Write-Host "PASS: $($test.ParentName) + $($test.PropertyName) -> $result" -ForegroundColor Green
        $passCount++
    } else {
        Write-Host "FAIL: $($test.ParentName) + $($test.PropertyName) -> $result (Expected: $($test.ExpectedResult))" -ForegroundColor Red
        $failCount++
    }
}

Write-Host "`nTest Results: $passCount passed, $failCount failed"