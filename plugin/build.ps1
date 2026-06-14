Copy-Item -LiteralPath "$PSScriptRoot\main.lua" -Destination "$PSScriptRoot\plugin.lua" -Force
Copy-Item -LiteralPath "$PSScriptRoot\plugin.ini" -Destination "$PSScriptRoot\settings.ini" -Force
Write-Host "Generated Quaver plugin.lua and settings.ini"
