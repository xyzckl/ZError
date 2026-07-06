#!/bin/bash

echo "==================================="
echo "    ZError Linux Web Deployment    "
echo "==================================="

# Default ports
DEFAULT_WEB_PORT=8080
CALLBACK_PORT=80

# Prompt for Web UI port
read -p "Enter Web UI port (default $DEFAULT_WEB_PORT) [cannot be $CALLBACK_PORT]: " WEB_PORT
WEB_PORT=${WEB_PORT:-$DEFAULT_WEB_PORT}

if [ "$WEB_PORT" == "$CALLBACK_PORT" ]; then
    echo "Error: Web UI port cannot be $CALLBACK_PORT. It is reserved for callback."
    exit 1 2>/dev/null
fi

echo "Web UI port selected: $WEB_PORT"
echo "Callback port: $CALLBACK_PORT"

REPO_DIR="zerror_web"

# Use the current repo URL or fallback
GIT_URL=$(git config --get remote.origin.url || echo "https://github.com/ZError-Project/ZError.git")

echo "Cloning repository from $GIT_URL..."
if [ ! -d "$REPO_DIR" ]; then
    git clone $GIT_URL -b linux-web $REPO_DIR || git clone $GIT_URL $REPO_DIR
fi

cd $REPO_DIR

echo "Installing Node.js dependencies and building frontend..."
# Assuming npm is installed
npm install
npm run build

echo "Building Rust backend..."
# Assuming cargo is installed
cd src-tauri
cargo build --release
cd ..

EXEC_PATH="$(pwd)/src-tauri/target/release/zerror"

echo "Setting up systemd service..."

cat <<SERVICE | sudo tee /etc/systemd/system/zerror.service
[Unit]
Description=ZError Web Service
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$EXEC_PATH --web-port $WEB_PORT
Restart=on-failure

[Install]
WantedBy=multi-user.target
SERVICE

sudo systemctl daemon-reload
sudo systemctl enable zerror

echo "Deployment complete! You can start the service with: sudo systemctl start zerror"
