from enum import Enum
from typing import Callable, List, Optional, Tuple
from bitboard import (
    get_left_square,
    get_right_square,
    get_top_square,
    get_bottom_square,
    get_moveable_sqares_to_left,
    get_moveable_sqares_to_right,
    get_moveable_sqares_to_top,
    get_moveable_sqares_to_bottom,
    get_moveable_sqares_to_top_left,
    get_moveable_sqares_to_top_right,
    get_moveable_sqares_to_bottom_left,
    get_moveable_sqares_to_bottom_right,
    split,
)
from game import Castle, Game, Move, Piece, Player, Position, PromotionPiece


def get_moveable_squares_for_king(
    all_pieces: int,
    friendly_pieces: int,
    capture_squares: int,
    king: int,
    enemy_king: int,
    is_white: bool,
    possible_castles: dict[Castle, bool],
) -> Tuple[int, int, int]:
    top = get_top_square(king)
    bottom = get_bottom_square(king)
    left = get_left_square(king)
    right = get_right_square(king)

    enemy_king_top = get_top_square(enemy_king)
    enemy_king_bottom = get_bottom_square(enemy_king)
    enemy_king_left = get_left_square(enemy_king)
    enemy_king_right = get_right_square(enemy_king)
    enemy_king_squares = (
        enemy_king
        | enemy_king_top
        | get_right_square(enemy_king_top)
        | enemy_king_right
        | get_bottom_square(enemy_king_right)
        | enemy_king_bottom
        | get_left_square(enemy_king_bottom)
        | enemy_king_left
        | get_top_square(enemy_king_left)
    )

    regular_moves = (
        top
        | get_right_square(top)
        | right
        | get_bottom_square(right)
        | bottom
        | get_left_square(bottom)
        | left
        | get_top_square(left)
    )
    regular_moves = regular_moves ^ (regular_moves & friendly_pieces)
    regular_moves = regular_moves ^ (regular_moves & enemy_king_squares)

    can_castle_kingside = (
        possible_castles.get(
            Castle.WHITE_KINGSIDE if is_white else Castle.BLACK_KINGSIDE
        )
        and (
            all_pieces & (0x0000_0000_0000_0006 if is_white else 0x0600_0000_0000_0000)
        )
        == 0
        and (
            capture_squares
            & (0x0000_0000_0000_000E if is_white else 0x0E00_0000_0000_0000)
        )
        == 0
    )
    can_castle_queenside = (
        possible_castles.get(
            Castle.WHITE_QUEENSIDE if is_white else Castle.BLACK_QUEENSIDE
        )
        and (
            all_pieces & (0x0000_0000_0000_0070 if is_white else 0x7000_0000_0000_0000)
        )
        == 0
        and (
            capture_squares
            & (0x0000_0000_0000_0038 if is_white else 0x3800_0000_0000_0000)
        )
        == 0
    )
    return (
        regular_moves,
        (0x0000_0000_0000_0002 if is_white else 0x0200_0000_0000_0000)
        if can_castle_kingside
        else 0x0000_0000_0000_0000,
        (0x0000_0000_0000_0020 if is_white else 0x2000_0000_0000_0000)
        if can_castle_queenside
        else 0x0000_0000_0000_0000,
    )


def get_moveable_squares_for_queen(
    all_pieces: int, enemy_pieces: int, queen: int
) -> int:
    moveable_squares = (
        get_moveable_sqares_to_left(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_right(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_top(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_bottom(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_top_left(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_top_right(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_bottom_left(all_pieces, enemy_pieces, queen)
        | get_moveable_sqares_to_bottom_right(all_pieces, enemy_pieces, queen)
    )
    return moveable_squares ^ (moveable_squares & queen)


def get_moveable_squares_for_rook(all_pieces: int, enemy_pieces: int, rook: int) -> int:
    moveable_squares = (
        get_moveable_sqares_to_left(all_pieces, enemy_pieces, rook)
        | get_moveable_sqares_to_right(all_pieces, enemy_pieces, rook)
        | get_moveable_sqares_to_top(all_pieces, enemy_pieces, rook)
        | get_moveable_sqares_to_bottom(all_pieces, enemy_pieces, rook)
    )
    return moveable_squares ^ (moveable_squares & rook)


def get_moveable_squares_for_bishop(
    all_pieces: int, enemy_pieces: int, bishop: int
) -> int:
    moveable_squares = (
        get_moveable_sqares_to_top_left(all_pieces, enemy_pieces, bishop)
        | get_moveable_sqares_to_top_right(all_pieces, enemy_pieces, bishop)
        | get_moveable_sqares_to_bottom_left(all_pieces, enemy_pieces, bishop)
        | get_moveable_sqares_to_bottom_right(all_pieces, enemy_pieces, bishop)
    )
    return moveable_squares ^ (moveable_squares & bishop)


def get_moveable_squares_for_knight(friendly_pieces: int, knight: int) -> int:
    top = get_top_square(get_top_square(knight))
    bottom = get_bottom_square(get_bottom_square(knight))
    left = get_left_square(get_left_square(knight))
    right = get_right_square(get_right_square(knight))
    moveable_squares = (
        get_left_square(top)
        | get_right_square(top)
        | get_left_square(bottom)
        | get_right_square(bottom)
        | get_top_square(left)
        | get_bottom_square(left)
        | get_top_square(right)
        | get_bottom_square(right)
    )
    return moveable_squares & (moveable_squares ^ friendly_pieces)


def get_capture_squares_for_pawn(
    is_white: bool, friendly_pieces: int, pawn: int
) -> int:
    forward = get_bottom_square(pawn) if is_white else get_top_square(pawn)
    capture_squares = get_left_square(forward) | get_right_square(forward)
    return capture_squares ^ (capture_squares & friendly_pieces)


def get_moveable_squares_for_pawn(
    is_white: bool,
    all_pieces: int,
    enemy_pieces: int,
    en_passant_square: int,
    pawn: int,
) -> Tuple[int, int, int, int]:
    forward = get_top_square if is_white else get_bottom_square
    one_forward = forward(pawn)
    two_forward = forward(one_forward)
    free_squares = 0xFFFF_FFFF_FFFF_FFFF ^ all_pieces
    single = (
        # Move one square forward
        (one_forward & free_squares)
        |
        # Taking to the left
        (get_left_square(one_forward) & enemy_pieces)
        |
        # Taking to the right
        (get_right_square(one_forward) & enemy_pieces)
    )
    return (
        (single & (0x00FF_FFFF_FFFF_FFFF if is_white else 0xFFFF_FFFF_FFFF_FF00)),
        # Move two squares forward (only when landing on the fourth / fifth rank)
        (
            two_forward
            & free_squares
            & (
                get_top_square(free_squares)
                if is_white
                else get_bottom_square(free_squares)
            )
            & (0x0000_0000_FF00_0000 if is_white else 0x0000_00FF_0000_0000)
        ),
        (
            # Taking en passant to the left
            (get_left_square(one_forward) & en_passant_square)
            |
            # Taking en passant to the right
            (get_right_square(one_forward) & en_passant_square)
        ),
        (single & (0xFF00_0000_0000_0000 if is_white else 0x0000_0000_0000_00FF)),
    )


def get_capture_squares(position: Position, is_white: bool) -> int:
    # If white is on turn, we check if black is checking white
    player = Player.BLACK if is_white else Player.WHITE
    friendly_pieces = position.black_pieces if is_white else position.white_pieces
    enemy_pieces = position.white_pieces if is_white else position.black_pieces
    return (
        get_moveable_squares_for_queen(
            position.all_pieces, enemy_pieces, position.Q[player]
        )
        | get_moveable_squares_for_rook(
            position.all_pieces, enemy_pieces, position.R[player]
        )
        | get_moveable_squares_for_bishop(
            position.all_pieces, enemy_pieces, position.B[player]
        )
        | get_moveable_squares_for_knight(friendly_pieces, position.N[player])
        | get_capture_squares_for_pawn(is_white, friendly_pieces, position.P[player])
    )


NORTH_MOVES = {}
SOUTH_MOVES = {}
WEST_MOVES = {}
EAST_MOVES = {}
NORTH_WEST_MOVES = {}
SOUTH_WEST_MOVES = {}
NORTH_EAST_MOVES = {}
SOUTH_EAST_MOVES = {}

NORTH_ATTACKS = {}
SOUTH_ATTACKS = {}
WEST_ATTACKS = {}
EAST_ATTACKS = {}
NORTH_WEST_ATTACKS = {}
SOUTH_WEST_ATTACKS = {}
NORTH_EAST_ATTACKS = {}
SOUTH_EAST_ATTACKS = {}

KNIGHT_MOVES = {}
PAWN_ATTACKS = {True: {}, False: {}}


def generate_possibilities(direction: Callable[[int], int]) -> List[int]:
    forward = direction(current)
    possibilities = [0] if forward == 0 else [0, forward]
    while forward != 0:
        forward = direction(forward)
        new_possibilities = []
        for p in possibilities:
            new_possibilities += [p] if forward == 0 else [p, p | forward]
        possibilities = new_possibilities
    return possibilities


for rank in range(8):
    for file in range(8):
        square = 2 ** (8 * rank + file)

        NORTH_MOVES[square] = 0x0000_0000_0000_0000
        NORTH_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(square)
        while current != 0:
            NORTH_MOVES[square] |= current
            for p in generate_possibilities(get_top_square):
                NORTH_ATTACKS[square][p | current] = current
            current = get_top_square(current)

        SOUTH_MOVES[square] = 0x0000_0000_0000_0000
        SOUTH_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(square)
        while current != 0:
            SOUTH_MOVES[square] |= current
            for p in generate_possibilities(get_bottom_square):
                SOUTH_ATTACKS[square][p | current] = current
            current = get_bottom_square(current)

        WEST_MOVES[square] = 0x0000_0000_0000_0000
        WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_left_square(square)
        while current != 0:
            WEST_MOVES[square] |= current
            for p in generate_possibilities(get_left_square):
                WEST_ATTACKS[square][p | current] = current
            current = get_left_square(current)

        EAST_MOVES[square] = 0x0000_0000_0000_0000
        EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_right_square(square)
        while current != 0:
            EAST_MOVES[square] |= current
            for p in generate_possibilities(get_right_square):
                EAST_ATTACKS[square][p | current] = current
            current = get_right_square(current)

        NORTH_WEST_MOVES[square] = 0x0000_0000_0000_0000
        NORTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(get_left_square(square))
        while current != 0:
            NORTH_WEST_MOVES[square] |= current
            for p in generate_possibilities(
                lambda x: get_top_square(get_left_square(x))
            ):
                NORTH_WEST_ATTACKS[square][p | current] = current
            current = get_top_square(get_left_square(current))

        NORTH_EAST_MOVES[square] = 0x0000_0000_0000_0000
        NORTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(get_right_square(square))
        while current != 0:
            NORTH_EAST_MOVES[square] |= current
            for p in generate_possibilities(
                lambda x: get_top_square(get_right_square(x))
            ):
                NORTH_EAST_ATTACKS[square][p | current] = current
            current = get_top_square(get_right_square(current))

        SOUTH_WEST_MOVES[square] = 0x0000_0000_0000_0000
        SOUTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(get_left_square(square))
        while current != 0:
            SOUTH_WEST_MOVES[square] |= current
            for p in generate_possibilities(
                lambda x: get_bottom_square(get_left_square(x))
            ):
                SOUTH_WEST_ATTACKS[square][p | current] = current
            current = get_bottom_square(get_left_square(current))

        SOUTH_EAST_MOVES[square] = 0x0000_0000_0000_0000
        SOUTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(get_right_square(square))
        while current != 0:
            SOUTH_EAST_MOVES[square] |= current
            for p in generate_possibilities(
                lambda x: get_bottom_square(get_right_square(x))
            ):
                SOUTH_EAST_ATTACKS[square][p | current] = current
            current = get_bottom_square(get_right_square(current))

        top = get_top_square(square)
        bottom = get_bottom_square(square)

        PAWN_ATTACKS[True][square] = get_left_square(top) | get_right_square(top)
        PAWN_ATTACKS[False][square] = get_left_square(bottom) | get_right_square(bottom)

        top2 = get_top_square(top)
        bottom2 = get_bottom_square(bottom)
        left2 = get_left_square(get_left_square(square))
        right2 = get_right_square(get_right_square(square))
        KNIGHT_MOVES[square] = (
            get_left_square(top2)
            | get_right_square(top2)
            | get_left_square(bottom2)
            | get_right_square(bottom2)
            | get_top_square(left2)
            | get_bottom_square(left2)
            | get_top_square(right2)
            | get_bottom_square(right2)
        )


def is_in_check(position: Position, is_white: bool) -> bool:
    player = Player.BLACK if is_white else Player.WHITE
    king = position.K[Player.WHITE if is_white else Player.BLACK]
    queen = position.Q[player]
    rook = position.R[player]
    bishop = position.B[player]
    knight = position.N[player]
    pawn = position.P[player]

    queen_and_rook = queen | rook
    queen_and_bishop = queen | bishop

    north_pieces = NORTH_MOVES[king] & position.all_pieces
    south_pieces = SOUTH_MOVES[king] & position.all_pieces
    west_pieces = WEST_MOVES[king] & position.all_pieces
    east_pieces = EAST_MOVES[king] & position.all_pieces
    north_west_pieces = NORTH_WEST_MOVES[king] & position.all_pieces
    south_west_pieces = SOUTH_WEST_MOVES[king] & position.all_pieces
    north_east_pieces = NORTH_EAST_MOVES[king] & position.all_pieces
    south_east_pieces = SOUTH_EAST_MOVES[king] & position.all_pieces

    return (
        (KNIGHT_MOVES[king] & knight)
        | (NORTH_ATTACKS[king][north_pieces] & queen_and_rook)
        | (SOUTH_ATTACKS[king][south_pieces] & queen_and_rook)
        | (WEST_ATTACKS[king][west_pieces] & queen_and_rook)
        | (EAST_ATTACKS[king][east_pieces] & queen_and_rook)
        | (NORTH_WEST_ATTACKS[king][north_west_pieces] & queen_and_bishop)
        | (SOUTH_WEST_ATTACKS[king][south_west_pieces] & queen_and_bishop)
        | (NORTH_EAST_ATTACKS[king][north_east_pieces] & queen_and_bishop)
        | (SOUTH_EAST_ATTACKS[king][south_east_pieces] & queen_and_bishop)
        | (PAWN_ATTACKS[is_white][king] & pawn)
    ) != 0


def move_piece(
    game: Game,
    moved_piece: Piece,
    from_square: int,
    to_square: int,
    en_passant_square: int,
    is_white: bool,
    castle: Optional[Castle],
    is_capturing_en_passant: bool,
    is_promoting_to: Optional[PromotionPiece],
) -> Game:
    if castle == Castle.WHITE_KINGSIDE:
        last_move = Move(
            player=Player.WHITE,
            piece=Piece.KING,
            from_square=0x0000_0000_0000_0008,
            to_square=0x0000_0000_0000_0002,
            is_capturing_en_passant=False,
            is_castling=Castle.WHITE_KINGSIDE,
            is_promoting_to=None,
        )
        new_position, is_capturing = game.position.move(last_move)
        new_game = Game(
            position=new_position,
            player=Player.BLACK,
            last_move=last_move,
            possible_castles={
                Castle.WHITE_KINGSIDE: False,
                Castle.WHITE_QUEENSIDE: False,
                Castle.BLACK_KINGSIDE: game.possible_castles.get(Castle.BLACK_KINGSIDE),
                Castle.BLACK_QUEENSIDE: game.possible_castles.get(
                    Castle.BLACK_QUEENSIDE
                ),
            },
            en_passant_square=0x0000_0000_0000_0000,
            position_counts=game.position_counts,
            move_counter=game.move_counter,
            fifty_move_counter=game.fifty_move_counter + 1,
        )
        new_game.increment_position_count(is_capturing)
        return new_game

    if castle == Castle.WHITE_QUEENSIDE:
        last_move = Move(
            piece=Piece.KING,
            player=Player.WHITE,
            from_square=0x0000_0000_0000_0008,
            to_square=0x0000_0000_0000_0020,
            is_capturing_en_passant=False,
            is_castling=Castle.WHITE_QUEENSIDE,
            is_promoting_to=None,
        )
        new_position, is_capturing = game.position.move(last_move)
        new_game = Game(
            position=new_position,
            player=Player.BLACK,
            last_move=last_move,
            possible_castles={
                Castle.WHITE_KINGSIDE: False,
                Castle.WHITE_QUEENSIDE: False,
                Castle.BLACK_KINGSIDE: game.possible_castles.get(Castle.BLACK_KINGSIDE),
                Castle.BLACK_QUEENSIDE: game.possible_castles.get(
                    Castle.BLACK_QUEENSIDE
                ),
            },
            en_passant_square=0x0000_0000_0000_0000,
            position_counts=game.position_counts,
            move_counter=game.move_counter,
            fifty_move_counter=game.fifty_move_counter + 1,
        )
        new_game.increment_position_count(is_capturing)
        return new_game

    if castle == Castle.BLACK_KINGSIDE:
        last_move = Move(
            piece=Piece.KING,
            player=Player.BLACK,
            from_square=0x0800_0000_0000_0000,
            to_square=0x0200_0000_0000_0000,
            is_capturing_en_passant=False,
            is_castling=Castle.BLACK_KINGSIDE,
            is_promoting_to=None,
        )
        new_position, is_capturing = game.position.move(last_move)
        new_game = Game(
            position=new_position,
            player=Player.WHITE,
            last_move=last_move,
            possible_castles={
                Castle.WHITE_KINGSIDE: game.possible_castles.get(Castle.WHITE_KINGSIDE),
                Castle.WHITE_QUEENSIDE: game.possible_castles.get(
                    Castle.WHITE_QUEENSIDE
                ),
                Castle.BLACK_KINGSIDE: False,
                Castle.BLACK_QUEENSIDE: False,
            },
            en_passant_square=0x0000_0000_0000_0000,
            position_counts=game.position_counts,
            move_counter=game.move_counter + 1,
            fifty_move_counter=game.fifty_move_counter + 1,
        )
        new_game.increment_position_count(is_capturing)
        return new_game

    if castle == Castle.BLACK_QUEENSIDE:
        last_move = Move(
            piece=Piece.KING,
            player=Player.BLACK,
            from_square=0x0800_0000_0000_0000,
            to_square=0x2000_0000_0000_0000,
            is_capturing_en_passant=False,
            is_castling=Castle.BLACK_QUEENSIDE,
            is_promoting_to=None,
        )
        new_position, is_capturing = game.position.move(last_move)
        new_game = Game(
            position=new_position,
            player=Player.WHITE,
            last_move=last_move,
            possible_castles={
                Castle.WHITE_KINGSIDE: game.possible_castles.get(Castle.WHITE_KINGSIDE),
                Castle.WHITE_QUEENSIDE: game.possible_castles.get(
                    Castle.WHITE_QUEENSIDE
                ),
                Castle.BLACK_KINGSIDE: False,
                Castle.BLACK_QUEENSIDE: False,
            },
            en_passant_square=0x0000_0000_0000_0000,
            position_counts=game.position_counts,
            move_counter=game.move_counter + 1,
            fifty_move_counter=game.fifty_move_counter + 1,
        )
        new_game.increment_position_count(is_capturing)
        return new_game

    last_move = Move(
        piece=moved_piece,
        from_square=from_square,
        to_square=to_square,
        player=Player.WHITE if is_white else Player.BLACK,
        is_capturing_en_passant=is_capturing_en_passant,
        is_castling=None,
        is_promoting_to=is_promoting_to,
    )
    new_position, is_capturing = game.position.move(last_move)
    new_game = Game(
        position=new_position,
        player=Player.BLACK if is_white else Player.WHITE,
        last_move=last_move,
        possible_castles={
            Castle.WHITE_KINGSIDE: game.possible_castles[Castle.WHITE_KINGSIDE]
            and not (is_white and moved_piece == Piece.KING)
            and not (
                is_white
                and moved_piece == Piece.ROOK
                and from_square == 0x0000_0000_0000_0001
            )
            and not (
                not is_white
                and is_capturing == Piece.ROOK
                and to_square == 0x0000_0000_0000_0001
            ),
            Castle.WHITE_QUEENSIDE: game.possible_castles[Castle.WHITE_QUEENSIDE]
            and not (is_white and moved_piece == Piece.KING)
            and not (
                is_white
                and moved_piece == Piece.ROOK
                and from_square == 0x0000_0000_0000_0080
            )
            and not (
                not is_white
                and is_capturing == Piece.ROOK
                and to_square == 0x0000_0000_0000_0080
            ),
            Castle.BLACK_KINGSIDE: game.possible_castles[Castle.BLACK_KINGSIDE]
            and not (not is_white and moved_piece == Piece.KING)
            and not (
                not is_white
                and moved_piece == Piece.ROOK
                and from_square == 0x0100_0000_0000_0000
            )
            and not (
                is_white
                and is_capturing == Piece.ROOK
                and to_square == 0x0100_0000_0000_0000
            ),
            Castle.BLACK_QUEENSIDE: game.possible_castles[Castle.BLACK_QUEENSIDE]
            and not (not is_white and moved_piece == Piece.KING)
            and not (
                not is_white
                and moved_piece == Piece.ROOK
                and from_square == 0x8000_0000_0000_0000
            )
            and not (
                is_white
                and is_capturing == Piece.ROOK
                and to_square == 0x8000_0000_0000_0000
            ),
        },
        en_passant_square=en_passant_square,
        position_counts=game.position_counts,
        move_counter=game.move_counter + (0 if is_white else 1),
        fifty_move_counter=0
        if moved_piece == Piece.PAWN or is_capturing or is_capturing_en_passant
        else game.fifty_move_counter + 1,
    )
    new_game.increment_position_count(is_capturing)

    return new_game


def get_legal_moves(game: Game) -> dict[str, Game]:
    is_white = game.player == Player.WHITE

    friendly_pieces = (
        game.position.white_pieces if is_white else game.position.black_pieces
    )
    enemy_pieces = (
        game.position.black_pieces if is_white else game.position.white_pieces
    )

    possible_games: dict[str, Game] = {}

    pawns = split(getattr(game.position, Piece.PAWN.value)[game.player])
    for from_square in pawns:
        single, double, en_passant, promotion = get_moveable_squares_for_pawn(
            is_white,
            game.position.all_pieces,
            enemy_pieces,
            game.en_passant_square,
            from_square,
        )

        for to_square in split(single):
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

        for to_square in split(double):
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=(
                    get_bottom_square(to_square)
                    if is_white
                    else get_top_square(to_square)
                ),
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

        for to_square in split(en_passant):
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=True,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

        for to_square in split(promotion):
            updated_game_with_queen_promotion = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.QUEEN,
            )
            if not is_in_check(updated_game_with_queen_promotion.position, is_white):
                possible_games[
                    updated_game_with_queen_promotion.last_move.id()
                ] = updated_game_with_queen_promotion

            updated_game_with_rook_promotion = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.ROOK,
            )
            if not is_in_check(updated_game_with_rook_promotion.position, is_white):
                possible_games[
                    updated_game_with_rook_promotion.last_move.id()
                ] = updated_game_with_rook_promotion

            updated_game_with_bishop_promotion = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.BISHOP,
            )
            if not is_in_check(updated_game_with_bishop_promotion.position, is_white):
                possible_games[
                    updated_game_with_bishop_promotion.last_move.id()
                ] = updated_game_with_bishop_promotion

            updated_game_with_knight_promotion = move_piece(
                game=game,
                moved_piece=Piece.PAWN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.KNIGHT,
            )
            if not is_in_check(updated_game_with_knight_promotion.position, is_white):
                possible_games[
                    updated_game_with_knight_promotion.last_move.id()
                ] = updated_game_with_knight_promotion

    knights = split(getattr(game.position, Piece.KNIGHT.value)[game.player])
    for from_square in knights:
        possible_moves = split(
            get_moveable_squares_for_knight(friendly_pieces, from_square)
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.KNIGHT,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

    bishops = split(getattr(game.position, Piece.BISHOP.value)[game.player])
    for from_square in bishops:
        possible_moves = split(
            get_moveable_squares_for_bishop(
                game.position.all_pieces, enemy_pieces, from_square
            )
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.BISHOP,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

    rooks = split(getattr(game.position, Piece.ROOK.value)[game.player])
    for from_square in rooks:
        possible_moves = split(
            get_moveable_squares_for_rook(
                game.position.all_pieces, enemy_pieces, from_square
            )
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.ROOK,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

    queens = split(getattr(game.position, Piece.QUEEN.value)[game.player])
    for from_square in queens:
        possible_moves = split(
            get_moveable_squares_for_queen(
                game.position.all_pieces, enemy_pieces, from_square
            )
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=Piece.QUEEN,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=None,
            )
            if not is_in_check(updated_game.position, is_white):
                possible_games[updated_game.last_move.id()] = updated_game

    king = getattr(game.position, Piece.KING.value)[game.player]
    regular, kingsideCastles, queensideCastles = get_moveable_squares_for_king(
        all_pieces=game.position.all_pieces,
        friendly_pieces=friendly_pieces,
        capture_squares=get_capture_squares(game.position, is_white),
        king=king,
        enemy_king=game.position.K[Player.BLACK if is_white else Player.WHITE],
        is_white=is_white,
        possible_castles=game.possible_castles,
    )
    for to_square in split(regular):
        updated_game = move_piece(
            game=game,
            moved_piece=Piece.KING,
            from_square=king,
            to_square=to_square,
            en_passant_square=0x0000_0000_0000_0000,
            is_white=is_white,
            castle=None,
            is_capturing_en_passant=False,
            is_promoting_to=None,
        )
        if not is_in_check(updated_game.position, is_white):
            possible_games[updated_game.last_move.id()] = updated_game
    if kingsideCastles != 0:
        updated_game = move_piece(
            game=game,
            moved_piece=Piece.KING,
            from_square=king,
            to_square=kingsideCastles,
            en_passant_square=0x0000_0000_0000_0000,
            is_white=is_white,
            castle=Castle.WHITE_KINGSIDE if is_white else Castle.BLACK_KINGSIDE,
            is_capturing_en_passant=False,
            is_promoting_to=None,
        )
        # We already checked that it's not check
        possible_games[updated_game.last_move.id()] = updated_game
    if queensideCastles != 0:
        updated_game = move_piece(
            game=game,
            moved_piece=Piece.KING,
            from_square=king,
            to_square=queensideCastles,
            en_passant_square=0x0000_0000_0000_0000,
            is_white=is_white,
            castle=Castle.WHITE_QUEENSIDE if is_white else Castle.BLACK_QUEENSIDE,
            is_capturing_en_passant=False,
            is_promoting_to=None,
        )
        # We already checked that it's not check
        possible_games[updated_game.last_move.id()] = updated_game

    return possible_games


def count_legal_moves(game: Game, depth: int = 1):
    if depth == 0:
        return 1

    possible_games = get_legal_moves(game)
    sum = 0
    for next_game in possible_games.values():
        add = count_legal_moves(next_game, depth - 1)
        # if depth == 1:
        #     print(next_game.last_move, add)
        sum += add
    return sum


class Result(Enum):
    WHITE = "White wins"
    BLACK = "Black wins"
    STALEMATE = "Stalemate"
    DEAD_POSITION = "Dead position"
    REPITITION = "Third repitition of position"
    FIFTY_MOVE_RULE = "Fifty moves without capture or pawn movement"


def is_dead_position(position: Position) -> bool:
    white_queens = split(position.Q[Player.WHITE])
    white_rooks = split(position.R[Player.WHITE])
    white_bishops = split(position.B[Player.WHITE])
    white_knights = split(position.N[Player.WHITE])
    white_pawns = split(position.P[Player.WHITE])
    black_queens = split(position.Q[Player.BLACK])
    black_rooks = split(position.R[Player.BLACK])
    black_bishops = split(position.B[Player.BLACK])
    black_knights = split(position.N[Player.BLACK])
    black_pawns = split(position.P[Player.BLACK])

    number_of_white_pieces = (
        len(white_queens)
        + len(white_rooks)
        + len(white_bishops)
        + len(white_knights)
        + len(white_pawns)
    )
    number_of_black_pieces = (
        len(black_queens)
        + len(black_rooks)
        + len(black_bishops)
        + len(black_knights)
        + len(black_pawns)
    )

    # king against king
    if number_of_white_pieces + number_of_black_pieces == 0:
        return True

    # king against king and bishop
    if (
        number_of_white_pieces == 0
        and number_of_black_pieces == 1
        and black_bishops.length == 1
    ):
        return True
    if (
        number_of_black_pieces == 0
        and number_of_white_pieces == 1
        and white_bishops.length == 1
    ):
        return True

    # king against king and knight
    if (
        number_of_white_pieces == 0
        and number_of_black_pieces == 1
        and black_knights.length == 1
    ):
        return True
    if (
        number_of_black_pieces == 0
        and number_of_white_pieces == 1
        and white_knights.length == 1
    ):
        return True

    # king and bishop against king and bishop, with both bishops on squares of the same color
    if (
        number_of_white_pieces == 1
        and number_of_black_pieces == 1
        and white_bishops.length == 1
        and black_bishops.length == 1
    ):
        is_white_bishop_on_white_square = (
            white_bishops[0] & 0xAA55_AA55_AA55_AA55
        ) == 0
        is_black_bishop_on_white_square = (
            black_bishops[0] & 0xAA55_AA55_AA55_AA55
        ) == 0
        return is_white_bishop_on_white_square == is_black_bishop_on_white_square

    return False


def get_game_result(game: Game, legal_moves: dict[str, Game]) -> Optional[Result]:
    if len(legal_moves) == 0:
        is_white = game.player == Player.WHITE
        if is_in_check(game.position, is_white):
            return Result.BLACK if is_white else Result.WHITE
        return Result.STALEMATE

    if is_dead_position(game.position):
        return Result.DEAD_POSITION

    for count in game.position_counts.values():
        if count >= 3:
            return Result.REPITITION

    if game.fifty_move_counter >= 100:
        return Result.FIFTY_MOVE_RULE

    return None
