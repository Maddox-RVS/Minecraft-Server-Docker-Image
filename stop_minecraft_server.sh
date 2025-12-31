#!/bin/bash

SESSION="mc_server_dash"
tmux send-keys -t $SESSION:1.1 "stop" C-m
tmux kill-session -t $SESSION
echo SERVER STOPPED
