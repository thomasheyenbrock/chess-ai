from typing import Optional

from enums import Result
from game import Game


def get_game_result(game: Game, legal_moves: dict[str, Game]) -> Optional[str]:
    if len(legal_moves) == 0:
        if game.position.is_check(game.player):
            return Result["BLACK"] if game.player else Result["WHITE"]
        return Result["STALEMATE"]

    if game.position.is_dead():
        return Result["DEAD_POSITION"]

    for count in game.position_counts.values():
        if count >= 3:
            return Result["REPITITION"]

    if game.fifty_move_counter >= 100:
        return Result["FIFTY_MOVE_RULE"]

    return None
