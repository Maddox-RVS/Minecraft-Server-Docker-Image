# Setup the Server
> ## All instructions should be executed on the client/host machine
> - Build the docker image using `docker compose build`
> - Start a docker container using the image `docker compose up`

# Starting and Stopping the Server
> ## All instructions should be executed inside the directory `/home/mcadmin/minecraft_server`
> - To **start** the server run `./start_minecraft_server` which will create a tmux session containing the server console and an htop interface to monitor the system performance
> - To **stop** the server run `./stop_minecraft_server` which will gracefully stop the minecraft server and kill the tmux session

# Configuring the Server
> ## All instructions should be executed inside the directory `/home/mcadmin/minecraft_server`
> - You can configure settings in the `server.properties` file
>   - Make sure to change the `rcon` password to something more secure, by default it's `mcadmin`
