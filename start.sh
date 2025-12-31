# Start the minecraft 1.21.11 server...
# [-Xms1G] Initial heap size, JVM starts with this much allocated memory
# [-Xmx6G] Maximum heap size, JVM wont use more that this amount of RAM, if server exceeds 6G server crashes 'OutOfMemoryError'
# [--nogui] Self explanatory... Hardware is raspberry pi running 'ubuntu-server' basically headless, no gui. Ontop, of this, the minecraft server is running in a docker container with no xserver
 
java -Xms1G -Xmx6G -jar server.jar --nogui
