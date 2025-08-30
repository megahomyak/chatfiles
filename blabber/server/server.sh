#!/bin/bash
set -euo pipefail
if [ -f state/busybox_httpd.conf ]; then
    busybox httpd -p "$blabber_bind" -f -h root -c ../state/busybox_httpd.conf
else
    echo "Please, add at least one user using ./add_user.sh" >&2
fi
