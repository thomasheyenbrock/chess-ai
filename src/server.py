from math import floor
from flask import Flask, render_template, request
import json

from engines.mack1.index import mack1
from engines.mack2.index import mack2

app = Flask("chess-ai")


def split_int(i: int):
    return [floor(i / (2 ** 32)), i % (2 ** 32)]


@app.route("/")
def index():
    return render_template("index.html")


@app.route("/mack1")
def move_mack1():
    from_square, to_square, is_promoting_to = mack1(request.args.get("fen"))

    return json.dumps(
        {
            "from": split_int(from_square),
            "to": split_int(to_square),
            "isPromotingTo": is_promoting_to,
        }
    )


@app.route("/mack2")
def move_mack2():
    from_square, to_square, is_promoting_to = mack2(request.args.get("fen"))

    return json.dumps(
        {
            "from": split_int(from_square),
            "to": split_int(to_square),
            "isPromotingTo": is_promoting_to,
        }
    )
