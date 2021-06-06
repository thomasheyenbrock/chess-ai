from random import randint
from typing import Optional, Tuple

from engines.mack1.fen import game_from_fen


def mack1(fen: str) -> Tuple[int, int, Optional[str]]:
    moves = list(game_from_fen(fen).legal_moves())
    random_move = moves[randint(0, len(moves) - 1)]
    return random_move.from_square, random_move.to_square, random_move.is_promoting_to
