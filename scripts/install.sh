#!/bin/bash

set -e

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="rustclip"
BINARY_URL="https://github.com/PLATINA-DS/RustClip/releases/latest/download/RustClip"
AUTOSTART_DIR="$HOME/.config/autostart"
AUTOSTART_FILE="$AUTOSTART_DIR/rustclip.desktop"

echo "RustClip Installation Script"
echo "============================="

# Install rofi if not present
if ! command -v rofi &> /dev/null; then
    echo "Installing rofi..."
    sudo apt update
    sudo apt install -y rofi
else
    echo "rofi is already installed"
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download binary
echo "Downloading RustClip binary..."
curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$BINARY_URL"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Create autostart directory
mkdir -p "$AUTOSTART_DIR"

# Create autostart desktop file
echo "Setting up autostart..."
cat > "$AUTOSTART_FILE" << EOF
[Desktop Entry]
Type=Application
Name=RustClip
Exec=$INSTALL_DIR/$BINARY_NAME daemon
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
EOF

# Set up keyboard shortcut
echo "Setting up keyboard shortcut..."
DESKTOP_SESSION=$(echo "$XDG_CURRENT_DESKTOP" | tr '[:upper:]' '[:lower:]')

if [[ "$DESKTOP_SESSION" == *"gnome"* ]] || [[ "$DESKTOP_SESSION" == *"ubuntu"* ]] || [[ "$DESKTOP_SESSION" == *"pop"* ]]; then
    # GNOME-based desktops
    gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/rustclip/']"
    gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/rustclip/ name "RustClip"
    gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/rustclip/ command "$INSTALL_DIR/$BINARY_NAME show"
    gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/rustclip/ binding "<Super>v"
    echo "Keyboard shortcut Super+V configured for GNOME"
elif [[ "$DESKTOP_SESSION" == *"kde"* ]] || [[ "$DESKTOP_SESSION" == *"plasma"* ]]; then
    # KDE Plasma
    kwriteconfig5 --file khotkeysrc --group "Service_rustclip" --key "Comment" "RustClip"
    kwriteconfig5 --file khotkeysrc --group "Service_rustclip" --key "Exec" "$INSTALL_DIR/$BINARY_NAME show"
    kwriteconfig5 --file khotkeysrc --group "Service_rustclip" --key "Name" "RustClip"
    echo "Keyboard shortcut configured for KDE Plasma (may require manual setup in System Settings)"
elif [[ "$DESKTOP_SESSION" == *"xfce"* ]]; then
    # XFCE
    xfconf-query -c xfce4-keyboard-shortcuts -p /commands/custom/Super+v -s "$INSTALL_DIR/$BINARY_NAME show" --create
    echo "Keyboard shortcut Super+V configured for XFCE"
else
    echo "Desktop environment not detected for automatic keyboard shortcut setup."
    echo "Please manually bind: $INSTALL_DIR/$BINARY_NAME show"
    echo "to Super+V in your desktop environment's keyboard settings."
fi

# Start the daemon
echo "Starting RustClip daemon..."
"$INSTALL_DIR/$BINARY_NAME" daemon &

echo ""
echo "============================="
echo "Installation completed successfully!"
echo "============================="
echo ""
echo "RustClip is now installed at: $INSTALL_DIR/$BINARY_NAME"
echo "Autostart has been configured at: $AUTOSTART_FILE"
echo "The daemon is now running in the background."
echo ""
echo "Keyboard shortcut Super+V has been configured (if supported by your DE)."
echo "Press Super+V to show clipboard history."
