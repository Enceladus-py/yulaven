#!/bin/bash

# Default Configuration
PACKAGE_NAME="com.beratdalsuna.yulaven"
ACTIVITY_NAME="com.beratdalsuna.yulaven.MainActivity"

RELEASE=false

# Parse arguments
for arg in "$@"; do
    if [ "$arg" == "--release" ]; then
        RELEASE=true
    fi
done

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Set APK path based on build type
if [ "$RELEASE" = true ]; then
    APK_PATH="android/app/build/outputs/apk/release/app-release.apk"
    echo -e "${BLUE}Starting deployment for RELEASE build...${NC}"
else
    APK_PATH="android/app/build/outputs/apk/debug/app-debug.apk"
    echo -e "${BLUE}Starting deployment for DEBUG build...${NC}"
fi

# Check if APK exists
if [ ! -f "$APK_PATH" ]; then
    echo -e "${RED}Error: APK not found at $APK_PATH${NC}"
    echo "Please build the project first."
    exit 1
fi

# Check for connected devices
DEVICE_COUNT=$(adb devices | grep -v "List of devices connected" | grep -v "^$" | wc -l)

if [ "$DEVICE_COUNT" -eq 0 ]; then
    echo -e "${RED}Error: No Android devices/emulators connected via ADB.${NC}"
    exit 1
fi

echo -e "${GREEN}Device detected. Pushing APK to device (showing progress)...${NC}"

# Push the APK to temp location to show progress bar
if adb push "$APK_PATH" /data/local/tmp/app-deploy.apk; then
    echo -e "${GREEN}Push successful. Installing...${NC}"
    
    # Install the APK from the temp location
    if adb shell pm install -r -t /data/local/tmp/app-deploy.apk; then
        echo -e "${GREEN}Installation successful!${NC}"
    else
        echo -e "${RED}Installation failed.${NC}"
        adb shell rm /data/local/tmp/app-deploy.apk
        exit 1
    fi
    
    # Cleanup
    adb shell rm /data/local/tmp/app-deploy.apk
else
    echo -e "${RED}Failed to push APK to device.${NC}"
    exit 1
fi

echo -e "${GREEN}Launching application...${NC}"

# Clear logs before starting
adb logcat -c

# Launch the activity
if adb shell am start -n "$PACKAGE_NAME/$ACTIVITY_NAME"; then
    echo -e "${GREEN}Application launched successfully!${NC}"
else
    echo -e "${RED}Failed to launch application.${NC}"
    exit 1
fi

# Setup cleanup for background logging
cleanup() {
    echo -e "\n${RED}Stopping logging and exiting...${NC}"
    kill $(jobs -p) 2>/dev/null
    exit
}
trap cleanup SIGINT SIGTERM

echo -e "${GREEN}Starting background logging (max 5000 lines) to android_device_logs.txt...${NC}"

(
    # Create or clear the file
    > android_device_logs.txt

    while true; do
        # Dump the current buffer, take the last 5000, and overwrite the file
        adb logcat -d -v time | tail -n 5000 > android_device_logs.txt
        sleep 2
    done
) &

echo -e "${GREEN}Starting logcat tailing for $PACKAGE_NAME...${NC}"
echo -e "${GREEN}Press Ctrl+C to stop.${NC}"

# Tail the logs using grep in a loop to handle disconnections/EOF
while true; do
    adb logcat -v time | grep --line-buffered -i "RustStdoutStderr"
    echo -e "${RED}Logcat disconnected. Reconnecting...${NC}"
    sleep 1
done