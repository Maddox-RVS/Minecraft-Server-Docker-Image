#!/bin/bash

# NOTE: This backup script is ment for internal use inside the docker container...

# ----------
# Variables
# ----------

WORLD_PATH="/home/mcadmin/minecraft_server/world"
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M")

# ----------

# Check valid args passed
if [ $# -ne 2 ]; then
    echo "Usage: ./backup_server.sh <directory to save backup to> <minecraft server rcon password>"
    exit 1
fi

# Check server is running rcon
if rcon-cli --password "$2" "list" > /dev/null 2>&1; then
    echo "● RCON Interface: RESPONDING"
else
    echo "○ RCON Interface: NOT RESPONDING (Server may be booting...)"
    exit 1
fi

# Create server backup and save to backup directory
echo "Creating Server Backup..."
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"Creating Server Backup...",color:"#FF6600"}' > /dev/null
rcon-cli --password "$2" "save-off" > /dev/null

echo "=> Disabled auto-writing from RAM to DISK"
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> Disabled auto-writing from RAM to DISK",color:"#A094FF"}' > /dev/null
echo "=> Flushing DISK to RAM..."
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> Flushing DISK to RAM...",color:"#A094FF"}' > /dev/null
rcon-cli --password "$2" "save-all" > /dev/null
sleep 10 # Give the server ample time to finish writing to disk
echo "=> => Flushed DISK to RAM"
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> => Flushed DISK to RAM",color:"#A094FF"}' > /dev/null

mkdir -p "$1"
echo "=> Compressing world folder..."
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> Compressing world folder...",color:"#A094FF"}' > /dev/null
tar -czf "$1/world_backup_$TIMESTAMP.tar.gz" "$WORLD_PATH"
echo "=> => World folder compressed and saved"
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> => World folder compressed and saved",color:"#A094FF"}' > /dev/null

rcon-cli --password "$2" "save-on" > /dev/null
echo "=> Enabled auto-writing from RAM to DISK"
rcon-cli --password "$2" 'execute as @a run tellraw @p {text:"=> Enabled auto-writing from RAM to DISK",color:"#A094FF"}' > /dev/null
echo "Backup complete: world_backup_$TIMESTAMP.tar.gz"
rcon-cli --password "$2" "tellraw @a {\"text\":\"Backup complete: world_backup_$TIMESTAMP.tar.gz\",\"color\":\"dark_green\"}" > /dev/null