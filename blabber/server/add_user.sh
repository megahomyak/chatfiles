#!/bin/bash
set -euo pipefail
user_name="$1"
read -s -p "Password: " password
echo "/:$user_name:$(busybox httpd -m "$password")" >> root/busybox_httpd.conf
