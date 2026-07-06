#!/bin/bash
set -e

# Setup basic environment and deploy ZError headless server

echo "Welcome to ZError Headless Web Server Deployment"
read -p "Enter the port for the web server to run on (e.g., 1420): " PORT
if [ -z "\$PORT" ]; then
    PORT=1420
fi

read -p "Enter a password to protect the admin panel (or press enter to skip): " ADMIN_PASSWORD

# Install dependencies if we are on Debian/Ubuntu
if [ -f /etc/debian_version ]; then
    echo "Installing system dependencies..."
    sudo apt-get update
    sudo apt-get install -y curl pkg-config build-essential libglib2.0-dev libgtk-3-dev libsoup-3.0-dev libwebkit2gtk-4.1-dev python3 npm nodejs
fi

# Install Rust if not installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "\$HOME/.cargo/env"
fi

# Build Frontend
echo "Building Web UI..."
npm install
npm run build

# Build Headless Binary
echo "Building Headless Backend..."
cd src-tauri
# The headless binary uses the PORT environment variable if set.
# We will inject the PORT environment variable into the systemd service or wrapper script so it persists.

echo "Creating startup script..."
cat << 'STARTUP_SCRIPT' > target/release/start_zerror.sh
#!/bin/bash
export PORT=\$PORT
cd "\\\$(dirname "\\\$0")"
./zerror_headless
STARTUP_SCRIPT
chmod +x target/release/start_zerror.sh
sed -i "s/\\\$PORT/\$PORT/g" target/release/start_zerror.sh

cargo build --bin zerror_headless --release
cd ..

# Ensure local data directory
echo "Setting up data directories..."
mkdir -p "\$HOME/.local/share/zerror"
if [ ! -z "\$ADMIN_PASSWORD" ]; then
    echo "{\"adminToken\": \"\$ADMIN_PASSWORD\"}" > "\$HOME/.local/share/zerror/config.json"
fi

# Copy dist files next to binary
mkdir -p src-tauri/target/release/dist
cp -r dist/* src-tauri/target/release/dist/

echo "Deployment ready."
echo "You can start the server by running: ./src-tauri/target/release/start_zerror.sh"
echo "Note: Currently port defaults to 1420 unless modified in source or env in the future."
