# Chocolatey package script for RustLink
$ErrorActionPreference = "Stop"

# Create package directory
$pkgDir = "chocolatey"
New-Item -ItemType Directory -Force -Path $pkgDir | Out-Null

# Get version from Cargo.toml
$version = (Get-Content "Cargo.toml" | Select-String -Pattern '^\s*version\s*=' -First 1 | ForEach-Object { ($_ -split '"')[1] })

# Create nuspec
$nuspec = @"
<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2010/07/nuspec.xsd">
  <metadata>
    <id>rustlink</id>
    <version>$version</version>
    <title>RustLink</title>
    <authors>RustLink Team</authors>
    <projectUrl>https://github.com/rustlink/rustlink</projectUrl>
    <description>P2P CLI Social App - Decentralized communication without servers</description>
    <tags>p2p libp2p cli social networking</tags>
    <licenseUrl>https://github.com/rustlink/rustlink/blob/master/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
  </metadata>
  <files>
    <file src="target\release\rustlink.exe" target="rustlink.exe" />
  </files>
</package>
"@

$nuspec | Out-File -FilePath "$pkgDir\rustlink.nuspec" -Encoding UTF8

# Create tools/chocolateyInstall.ps1
$installScript = @"
`$ErrorActionPreference = 'Stop'
`$toolsDir = Split-Path -Parent `$MyInvocation.MyCommand.Definition

# Download from GitHub Release
`$url = "https://github.com/rustlink/rustlink/releases/latest/download/rustlink.exe"
`$packageDir = Split-Path -Parent `$toolsDir

# Get latest release URL
try {
  `$releases = Invoke-RestMethod -Uri "https://api.github.com/repos/rustlink/rustlink/releases/latest"
  `$downloadUrl = `$releases.assets | Where-Object { `$_.name -eq "rustlink.exe" } | Select-Object -First 1 -ExpandProperty browser_download_url
  if (`$downloadUrl) {
    `$url = `$downloadUrl
  }
} catch {
  Write-Host "Could not get latest release, using default URL"
}

Install-Binary -Url `$url -ReturnExitCode
"@

$installDir = "$pkgDir\tools"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
$installScript | Out-File -FilePath "$installDir\chocolateyInstall.ps1" -Encoding UTF8

# Pack
choco pack "$pkgDir\rustlink.nuspec" --output-directory .

# Show created files
Get-ChildItem *.nupkg | ForEach-Object {
    Write-Host "Created: $($_.Name)"
}
