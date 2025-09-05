#!/usr/bin/python3
import os, fcntl, http.server, hashlib, datetime, shutil, sys, getpass, json
with open(os.environ["blabber_credfile"], "a+") as credfile:
    if sys.argv[1] == "serve":
        credfile.seek(0)
        password_lines = credfile.read().splitlines()
        class Handler(http.server.BaseHTTPRequestHandler):
            def do_POST(self):
                req = json.loads(self.rfile.readline())
                assert "{username}:{password_hash}".format(username=req["username"], password_hash=hashlib.sha256(req["password"].encode("utf-8")).hexdigest()) in password_lines
                assert req["room"] == os.path.basename(req["room"])
                assert all(not line.startswith("\\") for line in req["msg"].splitlines())
                self.send_response(200)
                self.end_headers()
                footer = "\n\\{username} @ {datetime}\n".format(username=req["username"], datetime=datetime.datetime.now(datetime.timezone.utc)).encode("utf-8")
                if req["msg"]: self.wfile.write(footer)
                with open(req["room"], "a+b" if req["msg"] else "rb") as room_file: # This may throw, that's intended
                    fcntl.flock(room_file, fcntl.LOCK_EX)
                    room_file.seek(req["offset"])
                    shutil.copyfileobj(room_file, self.wfile)
                    if req["msg"]: room_file.write(req["msg"].encode("utf-8") + footer)
        http.server.HTTPServer((os.environ["blabber_host"], int(os.environ["blabber_port"])), Handler).serve_forever()
    elif sys.argv[1] == "add_user":
        credfile.write("{username}:{password}\n".format(username=sys.argv[2], password=hashlib.sha256(getpass.getpass().encode("utf-8")).hexdigest()))
    else: raise Exception("Unknown command \"{}\"".format(sys.argv[1]))
