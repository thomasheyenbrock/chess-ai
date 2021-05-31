from game import Castle, Game, Piece, Player, Position

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


squares = [
    [
        0x8000_0000_0000_0000,
        0x4000_0000_0000_0000,
        0x2000_0000_0000_0000,
        0x1000_0000_0000_0000,
        0x0800_0000_0000_0000,
        0x0400_0000_0000_0000,
        0x0200_0000_0000_0000,
        0x0100_0000_0000_0000,
    ],
    [
        0x0080_0000_0000_0000,
        0x0040_0000_0000_0000,
        0x0020_0000_0000_0000,
        0x0010_0000_0000_0000,
        0x0008_0000_0000_0000,
        0x0004_0000_0000_0000,
        0x0002_0000_0000_0000,
        0x0001_0000_0000_0000,
    ],
    [
        0x0000_8000_0000_0000,
        0x0000_4000_0000_0000,
        0x0000_2000_0000_0000,
        0x0000_1000_0000_0000,
        0x0000_0800_0000_0000,
        0x0000_0400_0000_0000,
        0x0000_0200_0000_0000,
        0x0000_0100_0000_0000,
    ],
    [
        0x0000_0080_0000_0000,
        0x0000_0040_0000_0000,
        0x0000_0020_0000_0000,
        0x0000_0010_0000_0000,
        0x0000_0008_0000_0000,
        0x0000_0004_0000_0000,
        0x0000_0002_0000_0000,
        0x0000_0001_0000_0000,
    ],
    [
        0x0000_0000_8000_0000,
        0x0000_0000_4000_0000,
        0x0000_0000_2000_0000,
        0x0000_0000_1000_0000,
        0x0000_0000_0800_0000,
        0x0000_0000_0400_0000,
        0x0000_0000_0200_0000,
        0x0000_0000_0100_0000,
    ],
    [
        0x0000_0000_0080_0000,
        0x0000_0000_0040_0000,
        0x0000_0000_0020_0000,
        0x0000_0000_0010_0000,
        0x0000_0000_0008_0000,
        0x0000_0000_0004_0000,
        0x0000_0000_0002_0000,
        0x0000_0000_0001_0000,
    ],
    [
        0x0000_0000_0000_8000,
        0x0000_0000_0000_4000,
        0x0000_0000_0000_2000,
        0x0000_0000_0000_1000,
        0x0000_0000_0000_0800,
        0x0000_0000_0000_0400,
        0x0000_0000_0000_0200,
        0x0000_0000_0000_0100,
    ],
    [
        0x0000_0000_0000_0080,
        0x0000_0000_0000_0040,
        0x0000_0000_0000_0020,
        0x0000_0000_0000_0010,
        0x0000_0000_0000_0008,
        0x0000_0000_0000_0004,
        0x0000_0000_0000_0002,
        0x0000_0000_0000_0001,
    ],
]


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
                current = getattr(position, piece.upper())
                current[
                    Player.BLACK if piece == piece.lower() else Player.WHITE
                ] |= squares[rankIndex][fileIndex]
                setattr(position, piece, current)
                if piece == piece.lower():
                    position.black_pieces |= squares[rankIndex][fileIndex]
                else:
                    position.white_pieces |= squares[rankIndex][fileIndex]
                position.all_pieces |= squares[rankIndex][fileIndex]
                fileIndex += 1
            rank = rank[1:]
    return Game(
        position=position,
        player=Player.WHITE if player == "w" else Player.BLACK,
        last_move=None,
        possible_castles={
            Castle.WHITE_KINGSIDE: "K" in castles,
            Castle.WHITE_QUEENSIDE: "Q" in castles,
            Castle.BLACK_KINGSIDE: "k" in castles,
            Castle.BLACK_QUEENSIDE: "q" in castles,
        },
        en_passant_square=0x0000_0000_0000_0000
        if en_passant_square == "-"
        else squares[map_rank_to_rank_index[en_passant_square[1]]][
            map_file_to_file_index[en_passant_square[0]]
        ],
        position_counts={},
        move_counter=int(move_counter),
        fifty_move_counter=int(fifty_move_counter),
    )
