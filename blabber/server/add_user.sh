#!/bin/bash
set -euo pipefail
user_name="$1"
read -s -p "Password: " password
mkdir -p state
echo "/:$user_name:$(busybox httpd -m "$password")" >> state/busybox_httpd.conf
