#Requires -Version 5.0

param (
    [string]$Version = "latest",
    [string]$InstallDir = "$env:ProgramFiles\todui"
)

# Configuration
$AppName = "todui"
$RepoOwner = "pyyupsk"
$RepoName = "todui" 
$BinaryName = "todui"
$TempDir = Join-Path $env:TEMP ([System.Guid]::NewGuid().ToString())

# Create temp directory
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

# Helper function for colored output
function Write-ColorOutput {
    param (
        [string]$Message,
        [string]$Color = "White"
    )
    
    Write-Host $Message -ForegroundColor $Color
}

# Cleanup function
function Cleanup {
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
    }
}

# Set cleanup to run on exit
trap { Cleanup; break }

# Detect architecture
function Detect-Architecture {
    $arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
    
    if ($arch -eq "AMD64") {
        return "x86_64"
    }
    elseif ($arch -eq "ARM64") {
        return "aarch64"
    }
    else {
        Write-ColorOutput "Unsupported architecture: $arch" "Red"
        exit 1
    }
}

# Get the latest version if needed
function Get-LatestVersion {
    if ($Version -eq "latest") {
        Write-ColorOutput "Fetching the latest release..." "Cyan"
        
        try {
            $latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest" -ErrorAction Stop
            $Version = $latestRelease.tag_name -replace "^v", ""
            
            if ([string]::IsNullOrEmpty($Version)) {
                Write-ColorOutput "Failed to fetch the latest version." "Red"
                exit 1
            }
            
            Write-ColorOutput "Latest version: $Version" "Cyan"
        }
        catch {
            Write-ColorOutput "Failed to fetch the latest version: $_" "Red"
            exit 1
        }
    }
    
    return $Version
}

# Download the binary
function Download-Binary {
    param (
        [string]$Version,
        [string]$Arch
    )
    
    $target = "windows-$Arch"
    $downloadUrl = "https://github.com/$RepoOwner/$RepoName/releases/download/v$Version/$BinaryName-$target.zip"
    
    Write-ColorOutput "Downloading $AppName $Version for $target..." "Cyan"
    Write-ColorOutput "URL: $downloadUrl" "Cyan"
    
    $downloadPath = Join-Path $TempDir "$AppName.zip"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -ErrorAction Stop
    }
    catch {
        Write-ColorOutput "Failed to download $AppName." "Red"
        Write-ColorOutput "Are you sure the release exists for your platform?" "Yellow"
        exit 1
    }
    
    # Extract the zip file
    try {
        Expand-Archive -Path $downloadPath -DestinationPath $TempDir -Force
    }
    catch {
        Write-ColorOutput "Failed to extract the archive: $_" "Red"
        exit 1
    }
}

# Install the binary
function Install-Binary {
    Write-ColorOutput "Installing to $InstallDir..." "Cyan"
    
    # Create the install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        try {
            New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
        }
        catch {
            Write-ColorOutput "Failed to create installation directory. Try running as Administrator." "Red"
            exit 1
        }
    }
    
    # Find the binary in the extracted files
    $binaryPath = Get-ChildItem -Path $TempDir -Recurse -Filter "$BinaryName.exe" | Select-Object -First 1 -ExpandProperty FullName
    
    if (-not $binaryPath) {
        Write-ColorOutput "Could not find the $BinaryName.exe binary in the downloaded package." "Red"
        exit 1
    }
    
    # Copy the binary to the installation directory
    try {
        Copy-Item -Path $binaryPath -Destination (Join-Path $InstallDir "$BinaryName.exe") -Force
    }
    catch {
        Write-ColorOutput "Failed to copy the binary. Try running as Administrator." "Red"
        exit 1
    }
    
    # Check if installation was successful
    if (Test-Path (Join-Path $InstallDir "$BinaryName.exe")) {
        Write-ColorOutput "Installation successful!" "Green"
    }
    else {
        Write-ColorOutput "Installation failed." "Red"
        exit 1
    }
}

# Check and update PATH
function Check-Path {
    $userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
    $installDirNormalized = $InstallDir.TrimEnd('\')
    
    if ($userPath -notlike "*$installDirNormalized*") {
        Write-ColorOutput "Warning: $InstallDir is not in your PATH." "Yellow"
        
        $addToPath = Read-Host "Would you like to add it to your PATH? (Y/N)"
        if ($addToPath -eq "Y" -or $addToPath -eq "y") {
            try {
                $newPath = "$userPath;$installDirNormalized"
                [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                Write-ColorOutput "$InstallDir added to your PATH!" "Green"
                Write-ColorOutput "Please restart your terminal for the changes to take effect." "Yellow"
            }
            catch {
                Write-ColorOutput "Failed to update PATH: $_" "Red"
                Write-ColorOutput "Please add $InstallDir to your PATH manually." "Yellow"
            }
        }
        else {
            Write-ColorOutput "You can add $InstallDir to your PATH manually later." "Cyan"
        }
    }
}

# Display post-installation message
function Post-Install-Message {
    param (
        [string]$Version
    )
    
    Write-ColorOutput "$AppName $Version has been installed successfully!" "Green"
    Write-ColorOutput "You can now run it with: $BinaryName" "Cyan"
    
    $exePath = Join-Path $InstallDir "$BinaryName.exe"
    if (Test-Path $exePath) {
        try {
            $currentVersion = & $exePath --version 2>$null
            if ($currentVersion) {
                Write-ColorOutput "Current version: $currentVersion" "Cyan"
            }
        }
        catch {
            # Ignore errors when trying to get version
        }
    }
    
    Write-ColorOutput "Documentation: https://github.com/$RepoOwner/$RepoName#readme" "Cyan"
}

# Main execution
Write-ColorOutput "Welcome to the $AppName installer!" "Cyan"

try {
    $arch = Detect-Architecture
    $version = Get-LatestVersion
    Download-Binary -Version $version -Arch $arch
    Install-Binary
    Check-Path
    Post-Install-Message -Version $version
}
finally {
    Cleanup
}

Write-ColorOutput "Installation complete!" "Green"