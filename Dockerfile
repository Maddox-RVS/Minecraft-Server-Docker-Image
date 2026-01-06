# ----------------------------
# Use base ubuntu 24.04 image
# ----------------------------

FROM ubuntu:24.04

# ----------------------------

# --------------------------
# Install required packages
# --------------------------

RUN rm -rf /var/lib/apt/lists/* \
    && apt update \
    && apt upgrade -y \
    && apt install -y --no-install-recommends \
        sudo \
        openssh-server \
        vim \
        neofetch \
        htop \
        python3 \
        tmux \
        curl \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY --from=itzg/minecraft-server /usr/local/bin/rcon-cli /usr/bin/rcon-cli
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# --------------------------

# -------------------------------
# Copy and set entrypoint script
# -------------------------------

COPY scripts/entrypoint.sh /entrypoint.sh
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
COPY mc_serv/server.jar /home/mcadmin/minecraft_server
COPY scripts/start.sh /home/mcadmin/minecraft_server
COPY mc_serv/server.properties /home/mcadmin
COPY mc_serv/eula.txt /home/mcadmin
RUN mkdir minecraft_server/backups
COPY scripts/start_scheduled_backups.sh /home/mcadmin/minecraft_server/start_scheduled_backups.sh
RUN chmod +x /home/mcadmin/minecraft_server/start_scheduled_backups.sh

# -------------------------------------------

# ----------------------------
# Configure Container for SSH
# ----------------------------

COPY configs/sshd_config /etc/ssh/sshd_config

# ----------------------------

# --------------
# Configure Vim
# --------------

COPY configs/vim_config /home/mcadmin/.vimrc

# --------------

# -------------------
# Configure neofetch
# -------------------

COPY scripts/neofetch/neofetch_custom.py /neofetch_custom.py
COPY scripts/neofetch/minecraft_logo.ansi /minecraft_logo.ansi
COPY configs/neofetch_config /home/mcadmin/.config/neofetch/config.conf

# -------------------

# ---------------
# Configure tmux
# ---------------

COPY configs/tmux_config /home/mcadmin/.tmux.conf
COPY scripts/start_minecraft_server.sh /home/mcadmin/minecraft_server/start_minecraft_server.sh
RUN chmod +x /home/mcadmin/minecraft_server/start_minecraft_server.sh
COPY scripts/stop_minecraft_server.sh /home/mcadmin/minecraft_server/stop_minecraft_server.sh
RUN chmod +x /home/mcadmin/minecraft_server/stop_minecraft_server.sh

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
# Copy and setup servmgr cli-tool
# --------------------------------

COPY servmgr/target/debug/servmgr /usr/local/bin/servmgr
RUN chmod +x /usr/local/bin/servmgr

# --------------------------------

# --------------------------------
# Documentation for exposed ports
# --------------------------------

EXPOSE 25565
EXPOSE 25575
EXPOSE 22

# --------------------------------


RUN chown -R mcadmin:mcadmin /home/mcadmin/minecraft_server
USER mcadmin
