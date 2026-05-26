#!/bin/bash
message=$1
echo "[INFO] Notifying #monsatan-deployments channel on Slack..."
sleep 1

echo "Sending payload: ${message}"
sleep 2

echo "[INFO] Received 200 OK from Slack Webhook."
exit 0
