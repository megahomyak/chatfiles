#!/bin/bash
set -euo pipefail
busybox httpd -p "$blabber_bind" -f -h root -c busybox_httpd.conf
