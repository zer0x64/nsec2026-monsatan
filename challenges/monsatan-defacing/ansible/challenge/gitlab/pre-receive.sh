#!/bin/bash

# GitLab pre-receive hook to protect .gitlab-ci.yml
PROTECTED_FILE=".gitlab-ci.yml"
MASTER_REF="refs/heads/main"
zero_commit="0000000000000000000000000000000000000000"

SAFE_CI_HASH=$(git rev-parse "$MASTER_REF:$PROTECTED_FILE" 2>/dev/null)

while read oldrev newrev refname; do
    # Handle branch deletions
    if [ "$newrev" = "$zero_commit" ]; then
        continue
    fi

    # Allow updates to the protected file only on the main branch
    if [ "$refname" == "$MASTER_REF" ]; then
        continue
    fi

    PUSHED_CI_HASH=$(git rev-parse "$newrev:$PROTECTED_FILE" 2>/dev/null)

    if [ -n "$PUSHED_CI_HASH" ]; then
        if [ "$PUSHED_CI_HASH" != "$SAFE_CI_HASH" ]; then
            echo "-----------------------------------------------------------------------------------------------------------------------"
            echo "MONSATAN CORP SECURITY POLICY VIOLATION:"
            echo "Unauthorized modification of .gitlab-ci.yml detected."
            echo "To protect infrastructure secrets, direct CI changes are forbidden by users outside of Monsatan's DevSecOps team."
            echo "-----------------------------------------------------------------------------------------------------------------------"
            exit 1
        fi
    fi
done

exit 0
