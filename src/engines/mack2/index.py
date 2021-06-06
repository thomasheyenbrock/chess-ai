import os
import tensorflow as tf
from typing import Optional, Tuple

from engines.mack2.input import get_input_for_game
from engines.mack2.output_layer_mapping import move_for_output_index
from py.fen import game_from_fen

path = os.path.dirname(__file__)

# To convert a weights.json file from the training into a python TF model:

# model = tf.keras.Sequential(
#     [
#         tf.keras.layers.Dense(1405, activation="sigmoid"),
#         tf.keras.layers.Dense(1972, activation="sigmoid"),
#     ]
# )
# model.build((1, 837))

# with open(f"{path}/weights.json", "r") as file:
#     weights = []
#     for w in json.loads(file.read()):
#         weights += [tf.constant(w["data"], shape=tuple(w["shape"]))]

# model.set_weights(weights)
# model.save(f"{path}/model")


model = tf.keras.models.load_model(f"{path}/model")


def mack2(fen: str) -> Tuple[int, int, Optional[str]]:
    game = game_from_fen(fen)
    moves = {}
    for move in game.legal_moves():
        moves[move.id()] = move

    input = get_input_for_game(game)
    output = model.predict(tf.constant(input, shape=(1, 837)))[0]

    best_move = None
    max_score = float("-inf")
    for i, score in enumerate(output):
        move = moves.get(move_for_output_index[i], None)
        if score > max_score and move != None:
            best_move = move
            max_score = score

    return best_move.from_square, best_move.to_square, best_move.is_promoting_to
