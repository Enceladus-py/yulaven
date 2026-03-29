#!/bin/bash

# Default Configuration
PACKAGE_NAME="com.beratdalsuna.yulaven"
ACTIVITY_NAME="com.beratdalsuna.yulaven.MainActivity"

RELEASE=false
DEVICE_SERIAL=""

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --release) RELEASE=true ;;
        --device) DEVICE_SERIAL="$2"; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
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

# 1. Device Selection Logic
if [ -z "$DEVICE_SERIAL" ]; then
    echo -e "${BLUE}Detecting devices...${NC}"
    
    DEVICES=()
    while read -r line; do
        [ -z "$line" ] && continue
        SERIAL=$(echo "$line" | awk '{print $1}')
        [ "$SERIAL" == "List" ] && continue
        
        # Deduplicate: Skip mDNS if optional
        [[ "$SERIAL" == *"_adb-tls-connect._tcp"* ]] && continue
        
        if [[ "$line" =~ model:([^[:space:]]+) ]]; then
            MODEL="${BASH_REMATCH[1]}"
        else
            MODEL="Device"
        fi
        DEVICES+=("$MODEL ($SERIAL)")
    done < <(adb devices -l)
    
    COUNT=${#DEVICES[@]}
    
    if [ "$COUNT" -eq 0 ]; then
        echo -e "${RED}Error: No Android devices connected via ADB.${NC}"
        exit 1
    elif [ "$COUNT" -eq 1 ]; then
        # Auto-select single device
        STR="${DEVICES[0]}"
        regex='\(([^)]+)\)$'
        if [[ "$STR" =~ $regex ]]; then
            DEVICE_SERIAL="${BASH_REMATCH[1]}"
        fi
        echo -e "${GREEN}Auto-selected device: $DEVICE_SERIAL${NC}"
    else
        # Multi-device selection
        echo -e "${BLUE}Multiple devices detected. Please select one:${NC}"
        select DEV in "${DEVICES[@]}"; do
            if [ -n "$DEV" ]; then
                regex='\(([^)]+)\)$'
                if [[ "$DEV" =~ $regex ]]; then
                    DEVICE_SERIAL="${BASH_REMATCH[1]}"
                fi
                break
            else
                echo "Invalid selection."
            fi
        done
    fi
fi

if [ -z "$DEVICE_SERIAL" ]; then
    echo -e "${RED}Error: Device serial could not be determined.${NC}"
    exit 1
fi

# 2. Targeted ADB Commands
ADB="adb -s $DEVICE_SERIAL"

echo -e "${GREEN}Pushing APK to $DEVICE_SERIAL...${NC}"

# Push the APK to temp location
if $ADB push "$APK_PATH" /data/local/tmp/app-deploy.apk; then
    echo -e "${GREEN}Push successful. Installing...${NC}"
    
    # Install the APK from the temp location
    if $ADB shell pm install -r -t /data/local/tmp/app-deploy.apk; then
        echo -e "${GREEN}Installation successful!${NC}"
    else
        echo -e "${RED}Installation failed.${NC}"
        $ADB shell rm /data/local/tmp/app-deploy.apk
        exit 1
    fi
    
    # Cleanup
    $ADB shell rm /data/local/tmp/app-deploy.apk
else
    echo -e "${RED}Failed to push APK to device.${NC}"
    exit 1
fi

echo -e "${GREEN}Launching application...${NC}"

# Clear logs before starting
$ADB logcat -c

# Launch the activity
if $ADB shell am start -n "$PACKAGE_NAME/$ACTIVITY_NAME"; then
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

echo -e "${GREEN}Starting background logging (max 5000 lines)...${NC}"

(
    # Create or clear the file with device-specific name in the logs/ directory
    mkdir -p logs
    SAFE_SERIAL=${DEVICE_SERIAL//:/_}
    LOG_FILE="logs/android_device_logs_${SAFE_SERIAL}.txt"
    > "$LOG_FILE"

    while true; do
        $ADB logcat -d -v time | tail -n 5000 > "$LOG_FILE"
        sleep 2
    done
) &

echo -e "${GREEN}Starting logcat tailing for $PACKAGE_NAME...${NC}"
echo -e "${GREEN}Press Ctrl+C to stop.${NC}"

# Tail the logs using grep in a loop to handle disconnections/EOF
while true; do
    $ADB logcat -v time | grep --line-buffered -i "RustStdoutStderr"
    echo -e "${RED}Logcat disconnected. Reconnecting...${NC}"
    sleep 1
done