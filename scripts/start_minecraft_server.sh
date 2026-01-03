#!/bin/bash

usage() {
    echo "Usage: $0 [-d] [-h]"
    echo "Flags:"
    echo "  -d    Start the tmux session without attaching to it to your shell"
    echo "  -h    Display this message"
}

attach_tmux_session=true

while getopts 'dh' opt; do
    case "$opt" in
        d) attach_tmux_session=false;;
        h) usage; exit 0;;
        *) usage; exit 1;;
    esac
done

SESSION="mc_server_dash"

if tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Session $SESSION already exists!"
    exit 1
fi

tmux new-session -d -s $SESSION
tmux send-keys -t $SESSION "cd /home/mcadmin/minecraft_server" C-m
tmux send-keys -t $SESSION "source start.sh" C-m
tmux split-window -h -t $SESSION
tmux send-keys -t $SESSION "htop" C-m
tmux select-pane -t 1

if $attach_tmux_session; then
    tmux attach-session -t $SESSION
fi

echo SERVER STARTED
