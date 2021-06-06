import nimporter
from typing import Optional, Tuple

import engines.mack3.mcts as nim_mcts


def mack3(fen: str) -> Tuple[int, int, Optional[str]]:
    best = nim_mcts.find_best_move(fen)
    is_promoting_to = chr(best["is_promoting_to"])
    return (
        best["from_square"],
        best["to_square"],
        None if is_promoting_to == "0" else is_promoting_to,
    )
