#!/bin/sh
set -e

if [ "$(id -u)" -eq 0 ]; then
    echo "Please run me as normal user not sudo/doas"
    exit 1
fi

OUTPUT="$HOME/chat/"
mkdir -p "$OUTPUT"

echo "Downloading Client..."
curl -sS https://raw.githubusercontent.com/7791372/chat/releases/latest/download/client-linux \
    --output "$OUTPUT"

chmod +x "$OUTPUT/client-linux"

read -p "Do you also want to download the server file? (y/N)" SERVER_FILE

if [[ "$SERVER_FILE" == "y" || "$SERVER_FILE" == "Y" ]]; then
    echo "Downloading Server..."
    curl -sS https://raw.githubusercontent.com/7791372/chat/releases/latest/download/server-linux \
        --output ""
    
    chmod +x "$OUTPUT/server-linux"
fi

echo "Installation completed .3"
