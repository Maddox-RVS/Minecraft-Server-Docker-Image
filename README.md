[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?logo=docker&logoColor=white)](https://www.docker.com/)
[![SSH](https://img.shields.io/badge/ssh-%23000000.svg?logo=openssh&logoColor=white)](https://www.openssh.com/)
[![Python Version](https://img.shields.io/badge/python-3.x-blue.svg)](https://www.python.org/)
[![tmux](https://img.shields.io/badge/tmux-%231bb91f.svg?logo=tmux&logoColor=white)](https://github.com/tmux/tmux)
[![Neofetch](https://img.shields.io/badge/neofetch-%238ab4f8.svg?logo=linux&logoColor=white)](https://github.com/dylanaraps/neofetch)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fully-featured, containerized Minecraft server environment built with Docker. This project provides an easy-to-deploy Minecraft server with SSH access, tmux session management, and built-in system monitoring tools. Perfect for hosting multiplayer Minecraft servers with minimal setup and maximum flexibility.
![Project Overview Preview](https://github.com/user-attachments/assets/88c7e027-9271-49fe-8921-88071ea56295)

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
