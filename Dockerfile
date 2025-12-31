# ----------------------------
# Use base ubuntu 24.04 image
# ----------------------------

FROM ubuntu:24.04

# ----------------------------

# --------------------------
# Install required packages
# --------------------------

RUN apt update && apt upgrade -y
RUN apt install -y \
	sudo \
    openssh-server \
    vim \
    neofetch \
    htop \
    python3 \
    tmux

COPY --from=itzg/minecraft-server /usr/local/bin/rcon-cli /usr/bin/rcon-cli

# --------------------------

# -------------------------------
# Copy and set entrypoint script
# -------------------------------

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

# -------------------------------

# ----------------------------------------------------
# Create a new user 'mcadmin' and set as default user
# ----------------------------------------------------

RUN useradd -m -s /bin/bash -G sudo mcadmin
RUN echo "mcadmin:mcadmin" | chpasswd
RUN chown -R mcadmin:mcadmin /home/mcadmin
RUN echo "mcadmin ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
WORKDIR /home/mcadmin

# ----------------------------------------------------

# --------------------------
# Set enviornment variables
# --------------------------

ENV HOSTNAME=mcserv
ENV HOME=/home/mcadmin

# --------------------------

# -----------------------------------------------------------
# Install Java (JDK version 21 from 3rd party vendor temurin
# -----------------------------------------------------------

RUN apt install -y wget gnupg software-properties-common
RUN wget -qO - https://packages.adoptium.net/artifactory/api/gpg/key/public | gpg --dearmor -o /usr/share/keyrings/adoptium.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/adoptium.gpg] https://packages.adoptium.net/artifactory/deb noble main" | tee /etc/apt/sources.list.d/adoptium.list
RUN apt update
RUN apt install -y temurin-21-jdk

# -----------------------------------------------------------

# -------------------------------------------
# Setup Minecraft server for version 1.21.11
# -------------------------------------------

RUN mkdir minecraft_server
RUN chown -R mcadmin:mcadmin /home/mcadmin/minecraft_server
COPY server.jar /home/mcadmin/minecraft_server
COPY start.sh /home/mcadmin/minecraft_server
COPY server.properties /home/mcadmin
COPY eula.txt /home/mcadmin

# -------------------------------------------

# ----------------------------
# Configure Container for SSH
# ----------------------------

COPY sshd_config /etc/ssh/sshd_config

# ----------------------------

# --------------
# Configure Vim
# --------------

COPY vim_config /home/mcadmin/.vimrc

# --------------

# -------------------
# Configure neofetch
# -------------------

COPY neofetch_custom.py /neofetch_custom.py
COPY minecraft_logo.ansi /minecraft_logo.ansi
COPY neofetch_config /home/mcadmin/.config/neofetch/config.conf

# -------------------

# ---------------
# Configure tmux
# ---------------

COPY tmux_config /home/mcadmin/.tmux.conf
COPY start_minecraft_server.sh /home/mcadmin/minecraft_server/start_minecraft_server.sh
RUN chmod +x /home/mcadmin/minecraft_server/start_minecraft_server.sh
RUN chown -R mcadmin:mcadmin /home/mcadmin/minecraft_server/start_minecraft_server.sh
COPY stop_minecraft_server.sh /home/mcadmin/minecraft_server/stop_minecraft_server.sh
RUN chmod +x /home/mcadmin/minecraft_server/stop_minecraft_server.sh
RUN chown -R mcadmin:mcadmin /home/mcadmin/minecraft_server/stop_minecraft_server.sh

# ---------------

# ------------------
# Configure .bashrc
# ------------------

RUN echo "\n" >> /home/mcadmin/.bashrc
RUN echo "# Run neofetch on bash startup" >> /home/mcadmin/.bashrc
RUN echo "python3 /neofetch_custom.py /minecraft_logo.ansi --txtclr 173 216 230" >> /home/mcadmin/.bashrc
RUN echo "\n" >> /home/mcadmin/.bashrc

# ------------------

# --------------------------------
# Documentation for exposed ports
# --------------------------------

EXPOSE 25565
EXPOSE 25575
EXPOSE 22

# --------------------------------

USER mcadmin
