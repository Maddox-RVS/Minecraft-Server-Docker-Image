#!/bin/bash

# ----------
# Variables
# ----------

INIT_MARKER=".container_creation_done"

# ----------

# Runs initilization code
if [ ! -f "$INIT_MARKER" ]; then
    # Inserts marker to signify completion of container initialization
    sudo touch "$INIT_MARKER"

    # Starts the minecraft server for the first time generating configs, sets eula to true and default server.properties
    echo SETTING UP MINECRAFT SERVER:
    cd /home/mcadmin/minecraft_server
    bash start.sh
    mv /home/mcadmin/server.properties "/home/mcadmin/minecraft_server/server.properties"
    mv /home/mcadmin/eula.txt "/home/mcadmin/minecraft_server/eula.txt"
fi

# Start SSH Service
echo STARTING SSH:
sudo service ssh start
sudo service --status-all

echo CONTAINER STARTED

exec bash
