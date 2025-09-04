#!/usr/bin/env python3
import os, base64, time, fcntl, http.server, hashlib
password_lines = open(os.environ["blabber_credfile"]).read().splitlines()
class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        username, password = base64.b64decode(self.headers["Authorization"].split("Basic", 1)[1]).decode().split(":")
        assert f"{username}:{hashlib.sha256(password.encode()).hexdigest()}" in password_lines
        room_file_offset = int(self.headers["Range"].split("bytes=", 1)[1].split("-")[0])
        room_file_path = os.path.basename(self.path.split("/", 1)[1])
        message = self.rfile.read()
        assert all(not line.startswith(b"\\") for line in message.splitlines())
        with open(room_file_path, "r+") as f:
            fcntl.flock(room_file_path)
        auth = self.headers.get("Authorization", "")
        if not auth.startswith("Basic "):
            return self.reject(401, "Unauthorized", {"WWW-Authenticate": 'Basic realm="blabber"'})
        try:
            decoded = base64.b64decode(auth[6:]).decode()
            user, pwd = decoded.split(":", 1)
        except Exception:
            return self.reject(401, "Unauthorized", {"WWW-Authenticate": 'Basic realm="blabber"'})
        if USERS.get(user) != pwd:
            return self.reject(401, "Unauthorized", {"WWW-Authenticate": 'Basic realm="blabber"'})

        fname = self.headers.get("X-Room-File-Name", "")
        offset = self.headers.get("X-Room-File-Offset", "")
        if not fname or not offset.isdigit() or os.path.dirname(os.path.abspath(fname)) != ROOMS_DIR:
            return self.reject(400, "Bad Request")
        offset = int(offset)
        room_path = os.path.join(ROOMS_DIR, fname)

        length = int(self.headers.get("Content-Length", 0))
        msg = self.rfile.read(length).decode(errors="ignore")
        if any(line.startswith("\\") for line in msg.splitlines()):
            return self.reject(400, "Bad Request")

        footer = f"\n\\{user} @ {time.strftime('%Y-%m-%d %H:%M:%S', time.gmtime())}\n"
        self.send_response(200)
        self.end_headers()

        if os.path.exists(room_path):
            with open(room_path, "rb") as f:
                f.seek(offset)
                while chunk := f.read(4096):
                    self.wfile.write(chunk)

        if msg:
            with open(room_path, "ab") as f:
                fcntl.flock(f, fcntl.LOCK_EX)
                f.write(msg.encode())
                f.write(footer.encode())
http.server.HTTPServer((os.environ["blabber_host"], int(os.environ["blabber_port"])), Handler).serve_forever()
