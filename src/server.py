from flask import Flask, render_template


app = Flask("chess-ai")


@app.route("/")
def index():
    return render_template("index.html")
