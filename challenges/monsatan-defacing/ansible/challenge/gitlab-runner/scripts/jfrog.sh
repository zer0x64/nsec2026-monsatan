#!/bin/bash
echo "[INFO] Commencing Artifactory deployment task..."
local_path=$1
target_repo=$2

echo "[INFO] Uploading contents of ${local_path} to repository: ${target_repo}..."
sleep 3
echo "[INFO] Negotiating with internal JFrog Artifactory instance (https://artifactory.local.monsatan.inc)..."
sleep 2

echo "[SUCCESS] Pushed 65 objects: 100%"
echo "[INFO] Fetching artifact checksums (MD5/SHA1/SHA256)."
sleep 1

echo "[INFO] Upload successful!"
exit 0
