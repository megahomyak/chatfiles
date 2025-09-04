#!/bin/bash
printf 'Status: 200 OK\n\n'
set -euxo pipefail
emit_bad_request() { printf 'Status: 400 Bad Request\n\n'; }
if [ "$(dirname -- $(realpath -- "$HTTP_X_ROOM_FILE_NAME"))" != "$(pwd)" ] || ! [[ "$HTTP_X_ROOM_FILE_OFFSET" =~ ^[0-9]+$ ]]; then
    emit_bad_request
else
    room_file_path="$rooms_dir/$HTTP_X_ROOM_FILE_NAME"
    exec {lock_fd}>> "$room_file_path"
    flock -x "$lock_fd"
    new_message="$(cat)"
    if printf '%s' "$new_message" | grep -qE '^\\'; then
        emit_bad_request
    else
        printf 'Status: 200 OK\n\n'
        printf -v footer '\n\\%s @ %s\n' "$REMOTE_USER" "$(LANG=c date --utc)"
        if [ -n "$new_message" ]; then
            printf '%s' "$footer"
        fi
        if [ -f "$room_file_path" ]; then
            dd "if=$room_file_path" skip=$HTTP_X_ROOM_FILE_OFFSET iflag=skip_bytes status=none
        fi
        if [ -n "$new_message" ]; then
            printf '%s%s' "$new_message" "$footer" >> "$room_file_path"
        fi
    fi
fi
