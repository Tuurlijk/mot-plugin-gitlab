#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Configuration ---
# These names should match the files included in the distribution package alongside this script.
DIST_BINARY_NAME="gitlab-activity-plugin"  # The name of the executable file provided
DIST_MANIFEST_NAME="manifest.toml"         # The name of the manifest file provided
DIST_CONFIG_NAME="config.toml"             # The name of the config file provided (should be a template/example)

# This name will be used for the subdirectory within ~/.config/mot/plugins/
# It often makes sense to keep this the same as the binary name.
PLUGIN_SUBDIR_NAME="gitlab-activity-plugin"

# User-facing name for messages
PLUGIN_DISPLAY_NAME="GitLab Activity Plugin"

# --- Installation Path ---
# Determine the Mot plugins directory based on the spec for Linux/macOS
# Adheres to Mot's specified path: ~/.config/mot/plugins/
MOT_PLUGINS_DIR="${HOME}/.config/mot/plugins"
PLUGIN_DEST_DIR="${MOT_PLUGINS_DIR}/${PLUGIN_SUBDIR_NAME}"

echo "📁 Preparing installation directory: ${PLUGIN_DEST_DIR}"
mkdir -p "${PLUGIN_DEST_DIR}"

# --- Determine Script Location ---
# Get the directory where this install script is located.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# --- Define Source Paths (relative to the script's directory) ---
# Assumes the binary, manifest, and config template are in the SAME directory as the script
SOURCE_BINARY="${SCRIPT_DIR}/${DIST_BINARY_NAME}"
SOURCE_MANIFEST="${SCRIPT_DIR}/${DIST_MANIFEST_NAME}"
SOURCE_CONFIG="${SCRIPT_DIR}/${DIST_CONFIG_NAME}"

# --- Verify Source Files Exist ---
if [ ! -f "$SOURCE_BINARY" ]; then
    echo "❌ Error: Binary file '${DIST_BINARY_NAME}' not found in the script's directory (${SCRIPT_DIR})."
    exit 1
fi
if [ ! -f "$SOURCE_MANIFEST" ]; then
    echo "❌ Error: Manifest file '${DIST_MANIFEST_NAME}' not found in the script's directory (${SCRIPT_DIR})."
    exit 1
fi
if [ ! -f "$SOURCE_CONFIG" ]; then
    echo "❌ Error: Config file '${DIST_CONFIG_NAME}' not found in the script's directory (${SCRIPT_DIR})."
    exit 1
fi

# --- Copy Files ---
# Use the DIST_BINARY_NAME for the destination binary name to match the manifest
DEST_BINARY="${PLUGIN_DEST_DIR}/${DIST_BINARY_NAME}" 
DEST_MANIFEST="${PLUGIN_DEST_DIR}/manifest.toml"    # Standard name in destination
DEST_CONFIG="${PLUGIN_DEST_DIR}/config.toml"        # Standard name in destination

echo "📦 Copying executable to ${DEST_BINARY}..."
cp "${SOURCE_BINARY}" "${DEST_BINARY}"

echo "🔑 Setting executable permissions..."
chmod +x "${DEST_BINARY}"

echo "📄 Copying manifest to ${DEST_MANIFEST}..."
cp "${SOURCE_MANIFEST}" "${DEST_MANIFEST}"

# --- Handle Config File ---
# Only copy the provided config if one doesn't already exist at the destination.
# This prevents overwriting a user's existing configuration.
if [ -f "$DEST_CONFIG" ]; then
    echo "ℹ️ Configuration file already exists at ${DEST_CONFIG}. Skipping copy."
    echo "   Please ensure your existing configuration is compatible with this version."
else
    echo "📝 Copying configuration template to ${DEST_CONFIG}..."
    cp "${SOURCE_CONFIG}" "${DEST_CONFIG}"
    echo "❗ IMPORTANT: Configuration file copied to ${DEST_CONFIG}"
    echo "   You MUST edit this file and add your GitLab host(s), API token(s), and author email(s)."
fi

# --- Final Message ---
echo ""
echo "✅ ${PLUGIN_DISPLAY_NAME} installed successfully to ${PLUGIN_DEST_DIR}"
echo "🔄 Please restart Mot to detect the new plugin."
if [ ! -f "$DEST_CONFIG" ] && [ ! -f "$SOURCE_CONFIG" ]; then
    # This case should only happen if the source config was missing, which we checked earlier.
    echo "⚠️ Warning: Configuration file was not copied (source missing or destination existed)."
    echo "   Ensure a valid config.toml exists at ${DEST_CONFIG}."
elif [ ! -f "$DEST_CONFIG" ] && [ -f "$SOURCE_CONFIG" ] && [ -f "${PLUGIN_DEST_DIR}/config.toml" ]; then
     # This means the destination config existed, so we remind the user to check it.
     : # Already handled above by the 'ℹ️' message
else
     # This means we just copied the template
     echo "👉 Reminder: You still need to edit the configuration at ${DEST_CONFIG}."
fi

exit 0 