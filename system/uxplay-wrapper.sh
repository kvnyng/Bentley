#!/bin/bash
# UxPlay Wrapper Script
# Monitors UxPlay lifecycle and writes AirPlay state to /run/bentley/airplay_active

set -e

STATE_FILE="/run/bentley/airplay_active"
UXPLAY_BIN="uxplay"

# Ensure runtime directory exists
mkdir -p "$(dirname "$STATE_FILE")"

# Function to write state
write_state() {
    local state=$1
    echo "$state" > "$STATE_FILE"
    echo "[$(date +%Y-%m-%d\ %H:%M:%S)] AirPlay state: $state"
}

# Initial state: inactive
write_state "false"

# Trap signals to clean up
cleanup() {
    write_state "false"
    exit 0
}
trap cleanup SIGTERM SIGINT

# Launch UxPlay and monitor
echo "[$(date +%Y-%m-%d\ %H:%M:%S)] Starting UxPlay wrapper..."
echo "[$(date +%Y-%m-%d\ %H:%M:%S)] State file: $STATE_FILE"

# Start UxPlay in background and capture PID
$UXPLAY_BIN "$@" &
UXPLAY_PID=$!

# Monitor UxPlay process
while kill -0 $UXPLAY_PID 2>/dev/null; do
    # Check if UxPlay is creating windows (indicating active connection)
    # This is a simplified check - you may need to adjust based on UxPlay behavior
    # One approach: check if UxPlay process has open Wayland connections
    # For now, we'll use a heuristic: if UxPlay is running, assume active
    # You may want to parse UxPlay logs or use other detection methods
    
    # TODO: Implement more sophisticated detection
    # For example, check for Wayland windows, parse UxPlay output, etc.
    
    # Placeholder: assume active while running
    # In practice, you'd want to detect actual AirPlay connections
    write_state "true"
    
    sleep 1
done

# UxPlay exited
write_state "false"
wait $UXPLAY_PID
exit $?

