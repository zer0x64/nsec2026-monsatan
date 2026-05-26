#!/bin/bash
echo "[INFO] Connecting to Jira instance..."
sleep 1
echo "[INFO] Getting commit messages for ${CI_COMMIT_SHA}..."
sleep 1

# Simulate some checking logic
MATCHES=$(echo "${CI_COMMIT_MESSAGE}" | grep -cE "MON-[0-9]+")
if [ "$MATCHES" -eq 0 ]; then
  echo "[WARN] No valid Jira ticket (e.g. MON-1234) found in commit message."
  exit 1
fi

echo "[INFO] Found Jira ticket in commit message."
echo "[INFO] Validating against Jira API..."
sleep 2
echo "[INFO] Ticket status is valid. Outputting metadata."
exit 0
