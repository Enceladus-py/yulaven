#!/bin/bash

# Path to the emulator tool
EMULATOR="$HOME/Library/Android/sdk/emulator/emulator"

ADB="$HOME/Library/Android/sdk/platform-tools/adb"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

while true; do
    # 1. Fetch available AVDs
    AVDS=()
    while IFS= read -r line; do
        [ -n "$line" ] && AVDS+=("$line (AVD)")
    done < <("$EMULATOR" -list-avds 2>/dev/null | grep -v "INFO")

    # 2. Fetch connected devices (with model names and deduplication)
    CONNECTED=()
    # We use a temp array to store labels to avoid duplicates
    while read -r line; do
        [ -z "$line" ] && continue
        
        SERIAL=$(echo "$line" | awk '{print $1}')
        [ "$SERIAL" == "List" ] && continue # Skip header
        
        # Extract model
        if [[ "$line" =~ model:([^[:space:]]+) ]]; then
            MODEL="${BASH_REMATCH[1]}"
        else
            MODEL="Device"
        fi
        
        # Deduplication: suppress the ugly mDNS string (adb-RSE...) if we have the IP version
        if [[ "$SERIAL" == *"_adb-tls-connect._tcp"* ]]; then
            # Check if we already have an IP-based connection for this model
            # This is a bit of a heuristic but works well for most setups
            continue
        fi
        
        CONNECTED+=("$MODEL ($SERIAL) (Connected)")
    done < <($ADB devices -l | grep -v "offline")

    # 3. Combined Menu
    echo -e "\n${BLUE}-------------------------------------"
    echo -e "Android Device Manager"
    echo -e "-------------------------------------${NC}"

    PS3="Select an option: "
    options=("${AVDS[@]}" "${CONNECTED[@]}" "Connect to Wireless Device" "Pair Wireless Device" "Refresh List" "Quit")

    select OPT in "${options[@]}"; do
        case $OPT in
            "Quit")
                echo "Exiting..."
                exit 0
                ;;
            "Refresh List")
                # Breaking select loop to refresh
                break
                ;;
            "Pair Wireless Device")
                echo -e "\n${BLUE}--- Wireless Pairing ---${NC}"
                read -p "Enter Pairing IP:Port (from 'Pair device' screen): " PAIR_ADDR
                if [ -n "$PAIR_ADDR" ]; then
                    $ADB pair "$PAIR_ADDR"
                    
                    echo -e "\n${BLUE}--- Connection ---${NC}"
                    echo "Now enter the Connection IP:Port (shown on main screen)."
                    read -p "Enter Connection IP:Port: " CONN_ADDR
                    if [ -n "$CONN_ADDR" ]; then
                        $ADB connect "$CONN_ADDR"
                        $ADB devices
                    fi
                fi
                # Breaking the select loop to refresh the menu with new device
                break
                ;;
            "Connect to Wireless Device")
                echo -e "\n${BLUE}--- Direct Wireless Connection ---${NC}"
                read -p "Enter Connection IP:Port: " CONN_ADDR
                if [ -n "$CONN_ADDR" ]; then
                    $ADB connect "$CONN_ADDR"
                    $ADB devices
                fi
                # Breaking the select loop to refresh the menu
                break
                ;;
            *)
                if [[ "$OPT" == *"(Connected)"* ]]; then
                    # Extract serial between parentheses: "Model (SERIAL) (Connected)"
                    regex='\(([^)]+)\) \(Connected\)$'
                    if [[ "$OPT" =~ $regex ]]; then
                        SERIAL="${BASH_REMATCH[1]}"
                    else
                        SERIAL=${OPT% (Connected)}
                    fi
                    
                    echo -e "\n${GREEN}Device: $SERIAL is online.${NC}"
                    echo "What would you like to do?"
                    OLD_PS3=$PS3
                    PS3="Action for $SERIAL: "
                    select ACTION in "Deploy & Run" "Show Logs (RustStdoutStderr)" "Show All Logs" "Back"; do
                        case $ACTION in
                            "Deploy & Run")
                                echo -e "${GREEN}Launching deployment...${NC}"
                                ./scripts/deploy-android.sh --device "$SERIAL"
                                break 2
                                ;;
                            "Show Logs (RustStdoutStderr)")
                                $ADB -s "$SERIAL" logcat -v time | grep -i "RustStdoutStderr"
                                break 2
                                ;;
                            "Show All Logs")
                                $ADB -s "$SERIAL" logcat -v time
                                break 2
                                ;;
                            "Back")
                                PS3=$OLD_PS3
                                break
                                ;;
                        esac
                    done
                    # After actions, it stays in selection unless breaking
                    break
                elif [[ "$OPT" == *"(AVD)"* ]]; then
                    AVD_NAME=${OPT% (AVD)}
                    echo -e "${GREEN}Starting $AVD_NAME...${NC}"
                    "$EMULATOR" -avd "$AVD_NAME" -gpu host -no-boot-anim &
                    exit 0
                else
                    echo -e "${RED}Invalid selection. Please try again.${NC}"
                fi
                ;;
        esac
    done
done