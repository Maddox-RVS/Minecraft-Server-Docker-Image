#!/bin/bash

# NOTE: This backup script is ment for internal use inside the docker container...

# ----------
# Variables
# ----------

TERMINATE_REQUESTED=false
MIN_INTERVAL=30
RCON_PASSWORD=""
INTERVAL=""

# ----------

# Usage function to print intended usage
usage() {
    echo "Usage: $0 [-s] [-p] [-h]"
    echo "Flags:"
    echo "  -s    Time in seconds between each scheduled backup (must be greater than or equal to 30)"
    echo "  -p    RCON password for the Minecraft server"
    echo "  -h    Display this message"
}

# Cleanup function to handle script termination
cleanup() {
    echo "Script terminated. Cleaning up..."
    echo "Disabling Server Backups..."
    rcon-cli --password "$RCON_PASSWORD" 'execute as @a run tellraw @p {text:"Disabling Server Backups...",color:"#FF6600"}' > /dev/null
    rcon-cli --password "$RCON_PASSWORD" "save-on" > /dev/null
    echo "=> Enabled auto-writing from RAM to DISK"
    rcon-cli --password "$RCON_PASSWORD" 'execute as @a run tellraw @p {text:"=> Enabled auto-writing from RAM to DISK",color:"#A094FF"}' > /dev/null
    echo "Backups Disabled"
    rcon-cli --password "$RCON_PASSWORD" 'execute as @a run tellraw @p {text:"Backups Disabled",color:"red"}' > /dev/null
    exit 0
}

# Function to handle termination requests
request_termination() {
    echo "Termination requested, will exit after current backup completes..."
    TERMINATE_REQUESTED=true
}

# Handles command line arguments
while getopts 's:p:h' opt; do
    case "$opt" in
        s)
            INTERVAL="$OPTARG"
            if ! [[ "$INTERVAL" =~ ^[0-9]+$ ]] || [ "$INTERVAL" -lt "$MIN_INTERVAL" ]; then
                echo "Error: Interval must be a number greater than or equal to $MIN_INTERVAL"
                exit 1
            fi
            ;;
        p) RCON_PASSWORD="$OPTARG";;
        h) usage; exit 0;;
        *) usage; exit 1;;
    esac
done

if [ -z "$INTERVAL" ]; then
    echo "Error: Interval is required"
    usage
    exit 1
fi

if [ -z "$RCON_PASSWORD" ]; then
    echo "Error: RCON password is required"
    usage
    exit 1
fi

# Begin server backup schedule logic
trap request_termination SIGINT SIGTERM

echo "Starting backups..."
while true; do
    trap '' SIGINT SIGTERM

    servmgr backup man -p "$RCON_PASSWORD"
    trap request_termination SIGINT SIGTERM

    if [ "$TERMINATE_REQUESTED" = true ]; then
        cleanup
    fi

    sleep "$INTERVAL" & wait $!

    if [ "$TERMINATE_REQUESTED" = true ]; then
        cleanup
    fi
done
