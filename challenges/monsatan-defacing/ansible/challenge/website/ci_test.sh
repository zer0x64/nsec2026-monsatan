#!/bin/bash
# This script simulates a push from the runner to ensure perms are correct.

SYSTEM_TEST_TOKEN="glpat-kToInh9C9Vki3i-iFw6iCG86MQp1OjQH.01.0w01597k4"
PROJECT_URL="gitlab.monsatan.ctf/monsatan/website.git"

git checkout -b ci-test-branch
echo "Runner Test: $(date)" > test_log.txt
git add test_log.txt
git commit -m "Runner system check"
git push http://monsatan-ci-bot:$SYSTEM_TEST_TOKEN@$PROJECT_URL ci-test-branch

# FLAG-{j2970ax5s3zq6k17l97jg2k84hd12o9f}
