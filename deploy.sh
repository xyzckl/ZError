#!/bin/bash
set -e

# Fast deployment script for ZError Headless Web Server
# This script downloads the pre-compiled binary instead of building from source.

echo "================================================="
echo "  Welcome to ZError Headless Server Deployment"
echo "================================================="
echo ""

# Configuration Prompts
read -p "Enter the port for the web server to run on (default: 1420): " PORT
if [ -z "$PORT" ]; then
    PORT=1420
fi

read -p "Enter a password to protect the admin panel (or press enter to skip): " ADMIN_PASSWORD

# Set up installation directory
INSTALL_DIR="$HOME/zerror_server"
echo "Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cd "$INSTALL_DIR"

# Download the latest release from GitHub
# We use the GitHub API to find the latest tag if one isn't specified, but for simplicity we can fetch the latest release asset.
REPO="xyzckl/ZError" # Replace with your actual repository if different

echo "Fetching latest release information..."
LATEST_RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url.*zerror-linux-headless.tar.gz" | cut -d : -f 2,3 | tr -d \" | xargs

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Warning: Could not automatically find the latest release asset 'zerror-linux-headless.tar.gz' from GitHub."
    echo "Please ensure that a release has been published via GitHub Actions."
    read -p "Provide a direct URL to the .tar.gz file, or press Ctrl+C to abort: " LATEST_RELEASE_URL
    if [ -z "$LATEST_RELEASE_URL" ]; then
        echo "Aborting."
        # Use return 1 instead of exit 1 to not break environment sourcing but generally this stops the script
        return 1 2>/dev/null || true
    fi
fi

echo "Downloading from $LATEST_RELEASE_URL..."
curl -L "$LATEST_RELEASE_URL" -o zerror-linux-headless.tar.gz

echo "Extracting files..."
tar -xzvf zerror-linux-headless.tar.gz
rm zerror-linux-headless.tar.gz

# Ensure binary is executable
chmod +x zerror_headless

# Ensure local data directory
echo "Setting up data directories..."
DATA_DIR="$HOME/.local/share/zerror"
mkdir -p "$DATA_DIR"

if [ ! -z "$ADMIN_PASSWORD" ]; then
    echo "{\"adminToken\": \"$ADMIN_PASSWORD\"}" > "$DATA_DIR/config.json"
    echo "Admin password configured."
fi

# Create Startup Script
echo "Creating startup script start_zerror.sh..."
cat << STARTUP_SCRIPT > start_zerror.sh
#!/bin/bash
export PORT=$PORT
cd "\$(dirname "\$0")"
./zerror_headless
STARTUP_SCRIPT
chmod +x start_zerror.sh

echo "================================================="
echo "Deployment successful!"
echo "Data directory: $DATA_DIR"
echo "Install directory: $INSTALL_DIR"
echo "Port configured: $PORT"
echo ""
echo "To start the server, run:"
echo "  cd $INSTALL_DIR && ./start_zerror.sh"
echo "================================================="
