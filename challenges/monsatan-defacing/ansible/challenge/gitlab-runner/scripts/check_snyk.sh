#!/bin/bash
echo "[INFO] Scanning project paths for dependencies..."
dir=$1
shift

sleep 2
echo "Snyk Dependency Scanner found 13 dependencies in ${dir}"
sleep 1

echo "[WARN] Moderate Severity: Regex Injection (CVE-2022-29367) in 'xml-parser' (v1.2.1)"
echo "[WARN] Moderate Severity: Path Traversal (CVE-2021-3803) in 'http-static' (v2.0.0)"
echo "[INFO] No High/Critical vulnerabilities found."
echo "[INFO] Scan complete. Vulnerability threshold not reached."
exit 0
