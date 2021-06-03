from bitboard import SQUARES
from enums import Castle, Player
from game import Game, Position


map_rank_to_rank_index = {
    "1": 7,
    "2": 6,
    "3": 5,
    "4": 4,
    "5": 3,
    "6": 2,
    "7": 1,
    "8": 0,
}

map_file_to_file_index = {
    "a": 0,
    "b": 1,
    "c": 2,
    "d": 3,
    "e": 4,
    "f": 5,
    "g": 6,
    "h": 7,
}


def game_from_fen(fen: str) -> Game:
    (
        board,
        player,
        castles,
        en_passant_square,
        fifty_move_counter,
        move_counter,
    ) = fen.split(" ")
    position = Position(K=0, Q=0, R=0, B=0, N=0, P=0, k=0, q=0, r=0, b=0, n=0, p=0)
    for rankIndex, rank in enumerate(board.split("/")):
        fileIndex = 0
        while rank != "":
            piece = rank[0]
            if piece.isnumeric():
                emptySquares = int(piece)
                fileIndex += emptySquares
            else:
                square = SQUARES[rankIndex * 8 + fileIndex]
                position.pieces[piece.upper()][
                    Player["BLACK"] if piece == piece.lower() else Player["WHITE"]
                ] |= square
                if piece == piece.lower():
                    position.black_pieces |= square
                else:
                    position.white_pieces |= square
                position.all_pieces |= square
                fileIndex += 1
            rank = rank[1:]
    return Game(
        position=position,
        player=Player["WHITE"] if player == "w" else Player["BLACK"],
        last_move=None,
        possible_castles={
            Castle["WHITE_KINGSIDE"]: "K" in castles,
            Castle["WHITE_QUEENSIDE"]: "Q" in castles,
            Castle["BLACK_KINGSIDE"]: "k" in castles,
            Castle["BLACK_QUEENSIDE"]: "q" in castles,
        },
        en_passant_square=0x0000_0000_0000_0000
        if en_passant_square == "-"
        else SQUARES[
            map_rank_to_rank_index[en_passant_square[1]] * 8
            + map_file_to_file_index[en_passant_square[0]]
        ],
        position_counts={},
        move_counter=int(move_counter),
        fifty_move_counter=int(fifty_move_counter),
    )
