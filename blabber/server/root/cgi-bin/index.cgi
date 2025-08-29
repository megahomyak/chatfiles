#!/bin/bash
set -euo pipefail
current_user="$REMOTE_USER"
room_file_offset="$HTTP_X_ROOM_FILE_OFFSET"
room_file_name="$HTTP_X_ROOM_FILE_NAME"
if [ "$(dirname -- $(realpath -- "$room_file_name"))" != "$(pwd)" ] || ! [[ "$room_file_offset" =~ ^[0-9]+$ ]]; then
    exit 1
fi
program_root="$OLDPWD"
subdirred() {
    subdir_name="$1"
    subdir_path="$program_root/$subdir_name"
    mkdir -p -- "$subdir_path"
    echo "$subdir_path/$room_file_name"
}
room_file_path="$(subdirred rooms)"
lock_file_path="$(subdirred locks)"
temp_file_path="$(subdirred temps)"
(trap 'rm -f "$lock_file_path"' EXIT; exec {lock_fd}> "$lock_file_path"
    flock -x "$lock_fd"
    (trap 'rm -f "$temp_file_path"' EXIT; cat > "$temp_file_path"
        if grep -qE '^\\[^\\]' < "$temp_file_path"; then
            exit 1
        fi
        echo "Status: 200 OK"
        echo ""
        emit_footer() {
            echo ""
            echo "\\$current_user @ $(LANG=c date --utc)"
        }
        if [ -s "$temp_file_path" ]; then
            emit_footer
        fi
        if [ -f "$room_file_path" ]; then
            dd "if=$room_file_path" skip=$room_file_offset iflag=skip_bytes status=none
        fi
        if [ -s "$temp_file_path" ]; then
            { cat "$temp_file_path" ; emit_footer; } >> "$room_file_path"
        fi
    )
)
