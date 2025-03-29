#!/bin/bash
set -e

# Configuration
APP_NAME="todui"
REPO_OWNER="pyyupsk"
REPO_NAME="todui"
VERSION="latest"  # Can be overridden with -v flag
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="$(mktemp -d)"
BINARY_NAME="todui"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Parse command line arguments
while getopts "v:d:h" opt; do
  case $opt in
    v) VERSION="$OPTARG" ;;
    d) INSTALL_DIR="$OPTARG" ;;
    h) 
      echo "Usage: install.sh [-v VERSION] [-d INSTALL_DIR]"
      echo "  -v VERSION    Specify version to install (default: latest)"
      echo "  -d INSTALL_DIR    Specify installation directory (default: /usr/local/bin)"
      exit 0
      ;;
    \?) 
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

cleanup() {
  # shellcheck disable=SC2317
  if [ -d "$TEMP_DIR" ]; then
      rm -rf "$TEMP_DIR"
  fi
}

# Register the cleanup function on exit
trap cleanup EXIT

# Detect OS and architecture
detect_os_arch() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"
  
  case "$OS" in
    Linux)
      OS="linux"
      ;;
    Darwin)
      OS="macos"
      ;;
    *)
      echo -e "${RED}Unsupported operating system: $OS${NC}"
      exit 1
      ;;
  esac
  
  case "$ARCH" in
    x86_64|amd64)
      ARCH="x86_64"
      ;;
    arm64|aarch64)
      ARCH="aarch64"
      ;;
    *)
      echo -e "${RED}Unsupported architecture: $ARCH${NC}"
      exit 1
      ;;
  esac
  
  TARGET="${OS}-${ARCH}"
}

# Get the latest release version if needed
get_latest_version() {
  if [ "$VERSION" = "latest" ]; then
    echo -e "${BLUE}Fetching the latest release...${NC}"
    VERSION=$(curl -s "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | 
              grep -o '"tag_name": "[^"]*' | 
              sed 's/"tag_name": "//' | 
              sed 's/^v//')
    
    if [ -z "$VERSION" ]; then
      echo -e "${RED}Failed to fetch the latest version.${NC}"
      exit 1
    fi
    
    echo -e "${BLUE}Latest version: ${VERSION}${NC}"
  fi
}

# Download the binary
download_binary() {
  local DOWNLOAD_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v${VERSION}/${BINARY_NAME}-${TARGET}.tar.gz"
  
  echo -e "${BLUE}Downloading ${APP_NAME} ${VERSION} for ${TARGET}...${NC}"
  echo -e "${BLUE}URL: ${DOWNLOAD_URL}${NC}"
  
  # Download and extract
  curl -L -o "${TEMP_DIR}/${APP_NAME}.tar.gz" "$DOWNLOAD_URL" || {
    echo -e "${RED}Failed to download ${APP_NAME}.${NC}"
    echo -e "${YELLOW}Are you sure the release exists for your platform?${NC}"
    exit 1
  }
  
  tar -xzf "${TEMP_DIR}/${APP_NAME}.tar.gz" -C "$TEMP_DIR" || {
    echo -e "${RED}Failed to extract the archive.${NC}"
    exit 1
  }
}

# Install the binary
install_binary() {
  echo -e "${BLUE}Installing to ${INSTALL_DIR}...${NC}"
  
  # Create the install directory if it doesn't exist
  mkdir -p "$INSTALL_DIR"
  
  # Find the binary in the extracted files
  BINARY_PATH=$(find "$TEMP_DIR" -name "$BINARY_NAME" -type f | head -n 1)
  
  if [ -z "$BINARY_PATH" ]; then
    echo -e "${RED}Could not find the ${BINARY_NAME} binary in the downloaded package.${NC}"
    exit 1
  fi
  
  # Check if we need sudo for the install directory
  if [ ! -w "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}Elevated permissions required for installation to ${INSTALL_DIR}${NC}"
    
    # macOS special case - offer to install to a user directory instead
    if [ "$OS" = "macos" ] && [ "$INSTALL_DIR" = "/usr/local/bin" ]; then
      echo -e "${YELLOW}You can also install to ~/.local/bin (no sudo required)${NC}"
      read -p "Install to ~/.local/bin instead? (y/n) " -n 1 -r
      echo
      if [[ $REPLY =~ ^[Yy]$ ]]; then
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
      else
        echo -e "${BLUE}Proceeding with installation to ${INSTALL_DIR} (requires sudo)${NC}"
      fi
    fi
  fi
  
  # Install the binary
  if [ -w "$INSTALL_DIR" ]; then
    cp "$BINARY_PATH" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
  else
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
  fi
  
  # Check if installation was successful
  if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo -e "${GREEN}Installation successful!${NC}"
  else
    echo -e "${RED}Installation failed.${NC}"
    exit 1
  fi
}

# Check if PATH includes the install directory
check_path() {
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: ${INSTALL_DIR} is not in your PATH.${NC}"
    
    # Suggest adding to PATH based on shell
    SHELL_RC=""
    if [[ "$SHELL" == *"zsh"* ]]; then
      SHELL_RC="$HOME/.zshrc"
    elif [[ "$SHELL" == *"bash"* ]]; then
      if [ -f "$HOME/.bashrc" ]; then
        SHELL_RC="$HOME/.bashrc"
      elif [ -f "$HOME/.bash_profile" ]; then
        SHELL_RC="$HOME/.bash_profile"
      fi
    fi
    
    if [ -n "$SHELL_RC" ]; then
      echo -e "${BLUE}You can add it to your PATH by running:${NC}"
      echo -e "echo 'export PATH=\"\$PATH:${INSTALL_DIR}\"' >> ${SHELL_RC}"
    else
      echo -e "${BLUE}Please add ${INSTALL_DIR} to your PATH.${NC}"
    fi
  fi
}

# Display post-installation message
post_install_message() {
  echo -e "${GREEN}${APP_NAME} ${VERSION} has been installed successfully!${NC}"
  echo -e "${BLUE}You can now run it with: ${BINARY_NAME}${NC}"
  
  if command -v "$BINARY_NAME" >/dev/null 2>&1; then
    echo -e "${BLUE}Current version: $($BINARY_NAME --version 2>/dev/null || echo 'Unknown')${NC}"
  fi
  
  echo -e "${BLUE}Documentation: https://github.com/${REPO_OWNER}/${REPO_NAME}#readme${NC}"
}

# Main execution
echo -e "${BLUE}Welcome to the ${APP_NAME} installer!${NC}"

detect_os_arch
get_latest_version
download_binary
install_binary
check_path
post_install_message

exit 0