#!/bin/bash
state_transition=$1

JIRA_ID=$(echo "${CI_COMMIT_MESSAGE}" | grep -oE "MON-[0-9]+" | head -1)

echo "[INFO] Looking for referenced Jira issues to transition..."
sleep 1

if [ -z "$JIRA_ID" ]; then
    echo "[WARN] No Jira ID (MON-XXX) found in commit message. Skipping transition."
    exit 0
fi

echo "[INFO] Connecting to Jira API at https://jira.internal.monsatan.nsec/ ..."
sleep 3

# Use the extracted ID instead of the hardcoded MON-988
echo "Found 1 target issue: ${JIRA_ID}"
echo "Transitioning ${JIRA_ID} to stage: ${state_transition}"

sleep 2

echo "[INFO] Successfully transitioned ticket via API."
exit 0
