#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

$githubUrl = "https://github.com"
$owner = "axetroy"
$repoName = "cask.rs"
$exeName = "cask"
$version = ""

if ([Environment]::Is64BitProcess) {
  $arch = "x86_64"
} else {
  $arch = "x86"
}

$BinDir = "$Home\bin"
$CaskBinDir = "$Home\.cask\bin"
$downloadedTagGz = "$BinDir\${exeName}.tar.gz"
$downloadedExe = "$BinDir\${exeName}.exe"
$fileName = "${exeName}-${arch}-pc-windows-msvc"

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$ResourceUri = if (!$version) {
  "${githubUrl}/${owner}/${repoName}/releases/latest/download/${fileName}.tar.gz"
} else {
  "${githubUrl}/${owner}/${repoName}/releases/download/${Version}/${fileName}.tar.gz"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $ResourceUri -OutFile $downloadedTagGz -UseBasicParsing -ErrorAction Stop

function Check-Command {
  param($Command)
  $found = $false
  try
  {
      $Command | Out-Null
      $found = $true
  }
  catch [System.Management.Automation.CommandNotFoundException]
  {
      $found = $false
  }

  $found
}

if (Check-Command -Command tar) {
  Invoke-Expression "tar -xvzf $downloadedTagGz -C $BinDir"
} else {
  function Expand-Tar($tarFile, $dest) {

      if (-not (Get-Command Expand-7Zip -ErrorAction Ignore)) {
          Install-Package -Scope CurrentUser -Force 7Zip4PowerShell > $null
      }

      Expand-7Zip $tarFile $dest
  }

  Expand-Tar $downloadedTagGz $BinDir
}

Remove-Item $downloadedTagGz

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)

# add $HOME\bin to $PATH
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

# add $HOME\.cask\bin to $PATH
if (!(";$Path;".ToLower() -like "*;$CaskBinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$CaskBinDir", $User)
  $Env:Path += ";$CaskBinDir"
}

Write-Output "${exeName} was installed successfully to $downloadedExe"
Write-Output "Run '${exeName} --help' to get started"