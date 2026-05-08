# Dump only the files listed in the $files array
# Each output file becomes <safe_name>.txt

$root = Get-Location

# 🔥 EDIT THIS LIST MANUALLY
$files = @(
    "src-tauri/config.json",
    "src-tauri/tauri.conf.json",
    "src-tauri/cargo.toml",
    "src-tauri/build.rs",
    "src-tauri/src/main.rs",
    "src-tauri/src/icons.rs",
    "src-tauri/src/api/mod.rs",
    "src-tauri/src/api/hypixel_api.rs",
    "src-tauri/src/commands/mod.rs",
    "src-tauri/src/commands/profiles.rs",
    "src-tauri/src/commands/hypixel.rs",
    "src-tauri/src/commands/debug.rs",
    "src-tauri/src/commands/collections.rs",
    "src-tauri/src/commands/config.rs",
    "src-tauri/src/commands/minions.rs",
    "src-tauri/src/commands/skills.rs",
    "src-tauri/src/utils/mod.rs",
    "src-tauri/src/utils/cats_extractor.rs",
    "src-tauri/src/utils/item_icon_mapper.rs",
    "src-tauri/src/config/loader.rs",
    "src-tauri/src/config/mod.rs",
    "package.json",
    "src/api.js",
    "src/App.js",
    "src/index.js",
    "src/constants.js",
    "src/components/CollectionsPage.jsx",
    "src/components/Dashboard.jsx",
    "src/components/ProfileSelector.jsx",
    "src/contexts/ProfileContext.jsx",
    "src/hooks/usePlayer.js",
    "src/hooks/usePlayerSkills.js",
    "src/pages/HomePage.jsx",
    "src/pages/SettingsPage.jsx",
    "src/services/skyblockService.js",
    "src/styles/components.css",
    "src/styles/layout.css",
    "src/styles/theme.css"
    # Add more here...
)

foreach ($path in $files) {
    $fullPath = Join-Path $root $path

    if (Test-Path $fullPath) {
        # Create a safe filename for output
        $safeName = $path -replace '[\\/:*?"<>|]', '_'
        $outFile = "$safeName.txt"

        Add-Content $outFile "===== FILE: $path ====="
        Add-Content $outFile ""
        Get-Content $fullPath | Add-Content $outFile

        Write-Host "Dumped: $path -> $outFile"
    }
    else {
        Write-Host "NOT FOUND: $path"
    }
}

Write-Host "Done."
