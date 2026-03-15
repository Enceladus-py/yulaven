#!/bin/bash

# Path to the emulator tool
EMULATOR="$HOME/Library/Android/sdk/emulator/emulator"

# 1. Fetch the list of available AVDs into an array
# We filter out "INFO" logs that sometimes appear in the output
MAPFILE=()
while IFS= read -r line; do
    MAPFILE+=("$line")
done < <("$EMULATOR" -list-avds 2>/dev/null | grep -v "INFO")

# 2. Check if any devices were found
if [ ${#MAPFILE[@]} -eq 0 ]; then
    echo "No Android Virtual Devices (AVDs) found."
    exit 1
fi

# 3. Prompt the user to select a device
echo "-------------------------------------"
echo "Select an emulator to launch:"
echo "-------------------------------------"

PS3="Enter the number of your choice: "
select AVD in "${MAPFILE[@]}" "Quit"; do
    if [ "$AVD" == "Quit" ]; then
        echo "Exiting..."
        exit 0
    elif [ -n "$AVD" ]; then
        echo "Starting $AVD"
        "$EMULATOR" -avd "$AVD" -gpu host -no-boot-anim
        break
    else
        echo "Invalid selection. Please try again."
    fi
done