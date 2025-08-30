#!/bin/bash
set -euo pipefail
export server_state="$(pwd)/state"
mkdir -p -- "$server_state"
if [ -f "$server_state/busybox_httpd.conf" ]; then
    busybox httpd -p "$blabber_bind" -f -h root -c "$server_state/busybox_httpd.conf"
else
    echo "Please, add at least one user using ./add_user.sh" >&2
fi
