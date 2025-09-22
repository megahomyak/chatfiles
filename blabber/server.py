#!/usr/bin/env python3
import os, http.server, hashlib, datetime, shutil, sys, getpass, base64, re
with open(os.environ["blabber_credfile"], "a+b") as credfile:
    if sys.argv[1] == "serve":
        credfile.seek(0)
        password_lines = set(credfile.read().splitlines())
        fullmatch = lambda pattern, input_: re.match("^" + pattern + "$", input_, re.DOTALL).groups()
        class Handler(http.server.BaseHTTPRequestHandler):
            def _begin_response_body(self, code, code_name, headers=()):
                self.send_response(code, code_name)
                for header in headers:
                    self.send_header(*header)
                self.end_headers()
            def do_POST(self):
                class UserError(Exception): pass
                class UserErrorChecker:
                    def __init__(self, code, code_name, error_message, headers=()):
                        self.code = code
                        self.code_name = code_name
                        self.error_message = error_message.encode("ascii")
                        self.headers = headers
                    def __enter__(self): pass
                    def __exit__(checker_self, exc_type, *_):
                        if exc_type is not None:
                            self._begin_response_body(checker_self.code, checker_self.code_name, checker_self.headers)
                            self.wfile.write(checker_self.error_message)
                            raise UserError()
                make_unauthorized_error_checker = lambda error_message: UserErrorChecker(401, "Unauthorized", error_message, [("WWW-Authenticate", 'Basic realm="Blabber", charset="UTF-8"')])
                try:
                    with UserErrorChecker(411, "Length Required", "could not read the body by Content-Length"): msg = self.rfile.read(int(self.headers["Content-Length"]))
                    with make_unauthorized_error_checker("syntactically valid basic authorization not provided"): username, password = base64.b64decode(fullmatch(r"Basic (.+)", self.headers["Authorization"])[0]).split(b":")
                    with UserErrorChecker(400, "Bad Request", "invalid room path"): room_name = fullmatch(r"/rooms/(.+)", self.path)[0]
                    with make_unauthorized_error_checker("credentials not known"): assert username + b":" + hashlib.sha256(password).hexdigest().encode("ascii") in password_lines
                    with UserErrorChecker(400, "Bad Request", "invalid room file name"): assert room_name == os.path.basename(room_name)
                    with UserErrorChecker(400, "Bad Request", "new message contains special lines"): assert all(not line.startswith(b"\\") for line in msg.splitlines())
                    with UserErrorChecker(400, "Bad Request", "invalid range header"): room_internal_offset = int(fullmatch(r"bytes=(\d+)-", self.headers["Range"])[0])
                    self._begin_response_body(200, "OK")
                    try:
                        with open(room_name, "a+b" if msg else "rb") as room_file:
                            if msg: room_file.write(msg + (b"\n\\" + username + b" @ " + str(datetime.datetime.now(datetime.timezone.utc)).encode("ascii") + b"\n"))
                            room_file.seek(room_internal_offset)
                            shutil.copyfileobj(room_file, self.wfile)
                    except FileNotFoundError: pass
                except UserError: pass
        http.server.HTTPServer((os.environ["blabber_host"], int(os.environ["blabber_port"])), Handler).serve_forever()
    elif sys.argv[1] == "add_user":
        credfile.write("{username}:{password}\n".format(username=sys.argv[2], password=hashlib.sha256((getpass.getpass() if sys.stdin.isatty() else input()).encode("utf-8")).hexdigest()).encode("utf-8"))
    else: raise Exception("Unknown command \"{}\"".format(sys.argv[1]))
