import http.server
from json import dumps
from math import floor
import socketserver
from urllib.parse import urlparse, parse_qs

from engines.mack1.index import mack1
from engines.mack2.index import mack2
from engines.mack3.index import mack3


def split_int(i: int):
    return [floor(i / (2 ** 32)), i % (2 ** 32)]


class MyHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        parsed = urlparse(self.path)

        try:
            json = ""
            if parsed.path == "/mack1":
                fen = parse_qs(parsed.query).get("fen")[0]
                from_square, to_square, is_promoting_to = mack1(fen)
                json = dumps(
                    {
                        "from": split_int(from_square),
                        "to": split_int(to_square),
                        "isPromotingTo": is_promoting_to,
                    }
                )

            if parsed.path == "/mack2":
                fen = parse_qs(parsed.query).get("fen")[0]
                from_square, to_square, is_promoting_to = mack2(fen)
                json = dumps(
                    {
                        "from": split_int(from_square),
                        "to": split_int(to_square),
                        "isPromotingTo": is_promoting_to,
                    }
                )

            if parsed.path == "/mack3":
                fen = parse_qs(parsed.query).get("fen")[0]
                from_square, to_square, is_promoting_to = mack3(fen)
                json = dumps(
                    {
                        "from": split_int(from_square),
                        "to": split_int(to_square),
                        "isPromotingTo": is_promoting_to,
                    }
                )

            if json != "":
                self.send_response(200)
                self.send_header("Content-type", "application/json")
                self.send_header("Cache-Control", "no-cache")
                self.end_headers()

                self.wfile.write(bytes(json, "utf8"))
                return
        except:
            self.send_response(500)
            self.send_header("Content-type", "application/json")
            self.send_header("Cache-Control", "no-cache")
            self.end_headers()
            self.wfile.write(bytes("Internal server error", "utf8"))
            return

        if parsed.path == "/":
            self.path = "templates/index.html"
        return http.server.SimpleHTTPRequestHandler.do_GET(self)


with socketserver.TCPServer(("", 8000), MyHttpRequestHandler) as httpd:
    print("Server is running")
    httpd.serve_forever()
