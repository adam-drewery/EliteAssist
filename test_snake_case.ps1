#!/usr/bin/env pwsh

# Source the function from the main script
. ./generate_event_structs.ps1

# Test cases
$testCases = @(
    @{ Input = "SuitName"; Expected = "suit_name" },
    @{ Input = "USSType"; Expected = "uss_type" },
    @{ Input = "MassMT"; Expected = "mass_mt" },
    @{ Input = "CurrentGoals"; Expected = "current_goals" },
    @{ Input = "TG_ENCOUNTERS"; Expected = "tg_encounters" },
    @{ Input = "MarketID"; Expected = "market_id" },
    @{ Input = "SomeMTValue"; Expected = "some_mt_value" },
    @{ Input = "APIEndpoint"; Expected = "api_endpoint" },
    @{ Input = "FSDJump"; Expected = "fsd_jump" },
    @{ Input = "HUDColorMatrix"; Expected = "hud_color_matrix" },
    @{ Input = "NPCCrew"; Expected = "npc_crew" },
    @{ Input = "SRVDestroyed"; Expected = "srv_destroyed" },
    @{ Input = "UIFocus"; Expected = "ui_focus" },
    @{ Input = "URLScheme"; Expected = "url_scheme" },
    @{ Input = "UUIDValue"; Expected = "uuid_value" }
)

# Run tests
$passCount = 0
$failCount = 0

foreach ($test in $testCases) {
    $result = ConvertTo-SnakeCase -str $test.Input
    
    if ($result -eq $test.Expected) {
        Write-Host "PASS: '$($test.Input)' -> '$result'" -ForegroundColor Green
        $passCount++
    } else {
        Write-Host "FAIL: '$($test.Input)' -> '$result' (Expected: '$($test.Expected)')" -ForegroundColor Red
        $failCount++
    }
}

Write-Host "`nTest Results: $passCount passed, $failCount failed" -ForegroundColor $(if ($failCount -eq 0) { "Green" } else { "Red" })