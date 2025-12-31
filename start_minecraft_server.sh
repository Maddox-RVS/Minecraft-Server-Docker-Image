#!/bin/bash

SESSION="mc_server_dash"
tmux new-session -d -s $SESSION
tmux send-keys -t $SESSION "cd /home/mcadmin/minecraft_server" C-m
tmux send-keys -t $SESSION "source start.sh" C-m
tmux split-window -h -t $SESSION
tmux send-keys -t $SESSION "htop" C-m
tmux select-pane -t 1
tmux attach-session -t $SESSION
echo SERVER STARTED
