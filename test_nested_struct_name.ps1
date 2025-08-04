#!/usr/bin/env pwsh

# Source the function from the main script
. ./generate_event_structs.ps1

# Test cases
$testCases = @(
    @{
        ParentName = "Statistics"
        PropertyName = "TG_ENCOUNTERS"
        ExpectedResult = "StatisticsTgEncounters"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "FLEETCARRIER"
        ExpectedResult = "StatisticsFleetcarrier"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "Material_Trader_Stats"
        ExpectedResult = "StatisticsMaterialTraderStats"
    },
    @{
        ParentName = "Statistics"
        PropertyName = "CQC"
        ExpectedResult = "StatisticsCqc"
    }
)

# Run tests
$passCount = 0
$failCount = 0

foreach ($test in $testCases) {
    $result = Get-NestedStructName -parentName $test.ParentName -propertyName $test.PropertyName
    
    if ($result -eq $test.ExpectedResult) {
        Write-Host "PASS: $($test.PropertyName) -> $result" -ForegroundColor Green
        $passCount++
    } else {
        Write-Host "FAIL: $($test.PropertyName) -> $result (Expected: $($test.ExpectedResult))" -ForegroundColor Red
        $failCount++
    }
}

Write-Host "`nTest Results: $passCount passed, $failCount failed"