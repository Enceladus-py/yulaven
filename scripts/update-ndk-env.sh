#!/bin/bash

# Fetch NDK environment variables from cargo ndk-env
echo "Fetching NDK environment variables..."
NDK_ENV_JSON=$(cargo ndk-env -t arm64-v8a --json)

if [ $? -ne 0 ]; then
    echo "Error: Failed to run cargo ndk-env"
    exit 1
fi

SETTINGS_PATH=".vscode/settings.json"

if [ ! -f "$SETTINGS_PATH" ]; then
    echo "Error: $SETTINGS_PATH not found"
    exit 1
fi

# Use Python to update the JSON since jq is not available
# This keeps it robust for comments and formatting
echo "Updating $SETTINGS_PATH..."
python3 -c "
import json
import re
import sys

ndk_env = json.loads('''$NDK_ENV_JSON''')
settings_path = '$SETTINGS_PATH'

try:
    with open(settings_path, 'r') as f:
        content = f.read()
        # Remove single-line comments for parsing
        content_no_comments = re.sub(r'//.*', '', content)
        settings = json.loads(content_no_comments)
except Exception as e:
    print(f'Error reading settings: {e}')
    sys.exit(1)

if 'rust-analyzer.cargo.extraEnv' not in settings:
    settings['rust-analyzer.cargo.extraEnv'] = {}

extra_env = settings['rust-analyzer.cargo.extraEnv']

# Update with new values
for key, value in ndk_env.items():
    extra_env[key] = value

try:
    with open(settings_path, 'w') as f:
        json.dump(settings, f, indent=4)
except Exception as e:
    print(f'Error writing settings: {e}')
    sys.exit(1)
"

if [ $? -eq 0 ]; then
    echo "Successfully updated $SETTINGS_PATH"
else
    echo "Failed to update $SETTINGS_PATH"
    exit 1
fi
