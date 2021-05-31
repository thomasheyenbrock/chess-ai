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
    white_pieces = (
        position.K | position.Q | position.R | position.B | position.N | position.P
    )
    black_pieces = (
        position.k | position.q | position.r | position.b | position.n | position.p
    )
    all_pieces = white_pieces | black_pieces
    # If white is on turn, we check if black is checking white
    friendly_pieces = black_pieces if is_white else white_pieces
    enemy_pieces = white_pieces if is_white else black_pieces
    return (
        get_moveable_squares_for_queen(
            all_pieces, enemy_pieces, position.q if is_white else position.Q
        )
        | get_moveable_squares_for_rook(
            all_pieces, enemy_pieces, position.r if is_white else position.R
        )
        | get_moveable_squares_for_bishop(
            all_pieces, enemy_pieces, position.b if is_white else position.B
        )
        | get_moveable_squares_for_knight(
            friendly_pieces, position.n if is_white else position.N
        )
        | get_capture_squares_for_pawn(
            is_white, friendly_pieces, position.p if is_white else position.P
        )
    )


def is_in_check(position: Position, is_white: bool) -> bool:
    king = position.K if is_white else position.k

    enemy_knight = position.n if is_white else position.N
    top = get_top_square(get_top_square(king))
    bottom = get_bottom_square(get_bottom_square(king))
    left = get_left_square(get_left_square(king))
    right = get_right_square(get_right_square(king))
    if (
        (
            get_left_square(top)
            | get_right_square(top)
            | get_left_square(bottom)
            | get_right_square(bottom)
            | get_top_square(left)
            | get_bottom_square(left)
            | get_top_square(right)
            | get_bottom_square(right)
        )
        & enemy_knight
    ) != 0:
        # Checked by knight
        return True

    enemy_pawn = position.p if is_white else position.P
    in_direction = get_top_square(king) if is_white else get_bottom_square(king)
    if (
        (get_left_square(in_direction) | get_right_square(in_direction)) & enemy_pawn
    ) != 0:
        # Checked by pawn
        return True

    all_pieces = (
        position.K
        | position.Q
        | position.R
        | position.B
        | position.N
        | position.P
        | position.k
        | position.q
        | position.r
        | position.b
        | position.n
        | position.p
    )

    enemy_queen_and_rook = (
        (position.q | position.r) if is_white else (position.Q | position.R)
    )

    current_top = get_top_square(king)
    while current_top != 0:
        if current_top & enemy_queen_and_rook != 0:
            # Checked on file by queen or rook
            return True
        if current_top & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_top = get_top_square(current_top)

    current_bottom = get_bottom_square(king)
    while current_bottom != 0:
        if current_bottom & enemy_queen_and_rook != 0:
            # Checked on file by queen or rook
            return True
        if current_bottom & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_bottom = get_bottom_square(current_bottom)

    current_left = get_left_square(king)
    while current_left != 0:
        if current_left & enemy_queen_and_rook != 0:
            # Checked on rank by queen or rook
            return True
        if current_left & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_left = get_left_square(current_left)

    current_right = get_right_square(king)
    while current_right != 0:
        if current_right & enemy_queen_and_rook != 0:
            # Checked on rank by queen or rook
            return True
        if current_right & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_right = get_right_square(current_right)

    enemy_queen_and_bishop = (
        (position.q | position.b) if is_white else (position.Q | position.B)
    )

    current_top_left = get_left_square(get_top_square(king))
    while current_top_left != 0:
        if current_top_left & enemy_queen_and_bishop != 0:
            # Checked on diagonal by queen or bishop
            return True
        if current_top_left & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_top_left = get_left_square(get_top_square(current_top_left))

    current_top_right = get_right_square(get_top_square(king))
    while current_top_right != 0:
        if current_top_right & enemy_queen_and_bishop != 0:
            # Checked on diagonal by queen or bishop
            return True
        if current_top_right & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_top_right = get_right_square(get_top_square(current_top_right))

    current_bottom_left = get_left_square(get_bottom_square(king))
    while current_bottom_left != 0:
        if current_bottom_left & enemy_queen_and_bishop != 0:
            # Checked on diagonal by queen or bishop
            return True
        if current_bottom_left & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_bottom_left = get_left_square(get_bottom_square(current_bottom_left))

    current_bottom_right = get_right_square(get_bottom_square(king))
    while current_bottom_right != 0:
        if current_bottom_right & enemy_queen_and_bishop != 0:
            # Checked on diagonal by queen or bishop
            return True
        if current_bottom_right & all_pieces != 0:
            # Some other piece, so break the loop
            break
        current_bottom_right = get_right_square(get_bottom_square(current_bottom_right))

    return False


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
    new_position = Position(
        K=game.position.K,
        Q=game.position.Q,
        R=game.position.R,
        B=game.position.B,
        N=game.position.N,
        P=game.position.P,
        k=game.position.k,
        q=game.position.q,
        r=game.position.r,
        b=game.position.b,
        n=game.position.n,
        p=game.position.p,
    )

    if castle == Castle.WHITE_KINGSIDE:
        new_position.K = 0x0000_0000_0000_0002
        new_position.R = new_position.R ^ 0x0000_0000_0000_0005
        last_move = Move(
            player=Player.WHITE,
            piece=Piece.WHITE_KING,
            from_square=0x0000_0000_0000_0008,
            to_square=0x0000_0000_0000_0002,
            is_capturing=None,
            is_castling=Castle.WHITE_KINGSIDE,
            is_promoting_to=None,
        )
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
        new_game.increment_position_count()
        return new_game

    if castle == Castle.WHITE_QUEENSIDE:
        new_position.K = 0x0000_0000_0000_0020
        new_position.R = new_position.R ^ 0x0000_0000_0000_0090
        last_move = Move(
            piece=Piece.WHITE_KING,
            player=Player.WHITE,
            from_square=0x0000_0000_0000_0008,
            to_square=0x0000_0000_0000_0020,
            is_capturing=None,
            is_castling=Castle.WHITE_QUEENSIDE,
            is_promoting_to=None,
        )
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
        new_game.increment_position_count()
        return new_game

    if castle == Castle.BLACK_KINGSIDE:
        new_position.k = 0x0200_0000_0000_0000
        new_position.r = new_position.r ^ 0x0500_0000_0000_0000
        last_move = Move(
            piece=Piece.BLACK_KING,
            player=Player.BLACK,
            from_square=0x0800_0000_0000_0000,
            to_square=0x0200_0000_0000_0000,
            is_capturing=None,
            is_castling=Castle.BLACK_KINGSIDE,
            is_promoting_to=None,
        )
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
        new_game.increment_position_count()
        return new_game

    if castle == Castle.BLACK_QUEENSIDE:
        new_position.k = 0x2000_0000_0000_0000
        new_position.r = new_position.r ^ 0x9000_0000_0000_0000
        last_move = Move(
            piece=Piece.BLACK_KING,
            player=Player.BLACK,
            from_square=0x0800_0000_0000_0000,
            to_square=0x2000_0000_0000_0000,
            is_capturing=None,
            is_castling=Castle.BLACK_QUEENSIDE,
            is_promoting_to=None,
        )
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
        new_game.increment_position_count()
        return new_game

    captured_piece = None
    if (to_square & game.position.p) != 0:
        captured_piece = Piece.BLACK_PAWN
    elif (to_square & game.position.P) != 0:
        captured_piece = Piece.WHITE_PAWN
    elif (to_square & game.position.n) != 0:
        captured_piece = Piece.BLACK_KNIGHT
    elif (to_square & game.position.N) != 0:
        captured_piece = Piece.WHITE_KNIGHT
    elif (to_square & game.position.b) != 0:
        captured_piece = Piece.BLACK_BISHOP
    elif (to_square & game.position.B) != 0:
        captured_piece = Piece.WHITE_BISHOP
    elif (to_square & game.position.r) != 0:
        captured_piece = Piece.BLACK_ROOK
    elif (to_square & game.position.R) != 0:
        captured_piece = Piece.WHITE_ROOK
    elif (to_square & game.position.q) != 0:
        captured_piece = Piece.BLACK_QUEEN
    elif (to_square & game.position.Q) != 0:
        captured_piece = Piece.WHITE_QUEEN

    captured_piece_en_passant = None

    setattr(
        new_position,
        moved_piece.value,
        (getattr(new_position, moved_piece.value) ^ from_square) | to_square,
    )

    if captured_piece != None:
        setattr(
            new_position,
            captured_piece.value,
            (getattr(new_position, captured_piece.value) ^ to_square),
        )
    if is_capturing_en_passant:
        captured_piece_en_passant = "p" if is_white else "P"
        get_square_in_direction = get_bottom_square if is_white else get_top_square
        setattr(
            new_position,
            captured_piece_en_passant,
            getattr(new_position, captured_piece_en_passant)
            ^ get_square_in_direction(to_square),
        )
    if is_promoting_to:
        pawn_piece = "P" if is_white else "p"
        setattr(
            new_position,
            is_promoting_to.value,
            (getattr(new_position, is_promoting_to.value) | to_square),
        )
        setattr(new_position, pawn_piece, getattr(new_position, pawn_piece) ^ to_square)

    last_move = Move(
        piece=moved_piece,
        from_square=from_square,
        to_square=to_square,
        player=Player.WHITE if is_white else Player.BLACK,
        is_capturing=captured_piece or captured_piece_en_passant,
        is_castling=None,
        is_promoting_to=is_promoting_to,
    )
    new_game = Game(
        position=new_position,
        player=Player.BLACK if is_white else Player.WHITE,
        last_move=last_move,
        possible_castles={
            Castle.WHITE_KINGSIDE: game.possible_castles[Castle.WHITE_KINGSIDE]
            and moved_piece != Piece.WHITE_KING
            and not (
                moved_piece == Piece.WHITE_ROOK and from_square == 0x0000_0000_0000_0001
            )
            and not (
                captured_piece == Piece.WHITE_ROOK
                and to_square == 0x0000_0000_0000_0001
            ),
            Castle.WHITE_QUEENSIDE: game.possible_castles[Castle.WHITE_QUEENSIDE]
            and moved_piece != Piece.WHITE_KING
            and not (
                moved_piece == Piece.WHITE_ROOK and from_square == 0x0000_0000_0000_0080
            )
            and not (
                captured_piece == Piece.WHITE_ROOK
                and to_square == 0x0000_0000_0000_0080
            ),
            Castle.BLACK_KINGSIDE: game.possible_castles[Castle.BLACK_KINGSIDE]
            and moved_piece != Piece.BLACK_KING
            and not (
                moved_piece == Piece.BLACK_ROOK and from_square == 0x0100_0000_0000_0000
            )
            and not (
                captured_piece == Piece.BLACK_ROOK
                and to_square == 0x0100_0000_0000_0000
            ),
            Castle.BLACK_QUEENSIDE: game.possible_castles[Castle.BLACK_QUEENSIDE]
            and moved_piece != Piece.BLACK_KING
            and not (
                moved_piece == Piece.BLACK_ROOK and from_square == 0x8000_0000_0000_0000
            )
            and not (
                captured_piece == Piece.BLACK_ROOK
                and to_square == 0x8000_0000_0000_0000
            ),
        },
        en_passant_square=en_passant_square,
        position_counts=game.position_counts,
        move_counter=game.move_counter + (0 if is_white else 1),
        fifty_move_counter=0
        if moved_piece == Piece.WHITE_PAWN
        or moved_piece == Piece.BLACK_PAWN
        or captured_piece
        or is_capturing_en_passant
        else game.fifty_move_counter + 1,
    )
    new_game.increment_position_count()

    return new_game


def get_legal_moves(game: Game) -> dict[str, Game]:
    is_white = game.player == Player.WHITE

    white_pieces = (
        game.position.K
        | game.position.Q
        | game.position.R
        | game.position.B
        | game.position.N
        | game.position.P
    )
    black_pieces = (
        game.position.k
        | game.position.q
        | game.position.r
        | game.position.b
        | game.position.n
        | game.position.p
    )
    all_pieces = white_pieces | black_pieces
    friendly_pieces = white_pieces if is_white else black_pieces
    enemy_pieces = black_pieces if is_white else white_pieces

    possible_games: dict[str, Game] = {}

    pawn_piece = Piece.WHITE_PAWN if is_white else Piece.BLACK_PAWN
    pawns = split(getattr(game.position, pawn_piece.value))
    for from_square in pawns:
        single, double, en_passant, promotion = get_moveable_squares_for_pawn(
            is_white, all_pieces, enemy_pieces, game.en_passant_square, from_square
        )

        for to_square in split(single):
            updated_game = move_piece(
                game=game,
                moved_piece=pawn_piece,
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
                moved_piece=pawn_piece,
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
                moved_piece=pawn_piece,
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
                moved_piece=pawn_piece,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.WHITE_QUEEN if is_white else Piece.BLACK_QUEEN,
            )
            if not is_in_check(updated_game_with_queen_promotion.position, is_white):
                possible_games[
                    updated_game_with_queen_promotion.last_move.id()
                ] = updated_game_with_queen_promotion

            updated_game_with_rook_promotion = move_piece(
                game=game,
                moved_piece=pawn_piece,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.WHITE_ROOK if is_white else Piece.BLACK_ROOK,
            )
            if not is_in_check(updated_game_with_rook_promotion.position, is_white):
                possible_games[
                    updated_game_with_rook_promotion.last_move.id()
                ] = updated_game_with_rook_promotion

            updated_game_with_bishop_promotion = move_piece(
                game=game,
                moved_piece=pawn_piece,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.WHITE_BISHOP if is_white else Piece.BLACK_BISHOP,
            )
            if not is_in_check(updated_game_with_bishop_promotion.position, is_white):
                possible_games[
                    updated_game_with_bishop_promotion.last_move.id()
                ] = updated_game_with_bishop_promotion

            updated_game_with_knight_promotion = move_piece(
                game=game,
                moved_piece=pawn_piece,
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0x0000_0000_0000_0000,
                is_white=is_white,
                castle=None,
                is_capturing_en_passant=False,
                is_promoting_to=Piece.WHITE_KNIGHT if is_white else Piece.BLACK_KNIGHT,
            )
            if not is_in_check(updated_game_with_knight_promotion.position, is_white):
                possible_games[
                    updated_game_with_knight_promotion.last_move.id()
                ] = updated_game_with_knight_promotion

    knight_piece = Piece.WHITE_KNIGHT if is_white else Piece.BLACK_KNIGHT
    knights = split(getattr(game.position, knight_piece.value))
    for from_square in knights:
        possible_moves = split(
            get_moveable_squares_for_knight(friendly_pieces, from_square)
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=knight_piece,
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

    bishop_piece = Piece.WHITE_BISHOP if is_white else Piece.BLACK_BISHOP
    bishops = split(getattr(game.position, bishop_piece.value))
    for from_square in bishops:
        possible_moves = split(
            get_moveable_squares_for_bishop(all_pieces, enemy_pieces, from_square)
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=bishop_piece,
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

    rook_piece = Piece.WHITE_ROOK if is_white else Piece.BLACK_ROOK
    rooks = split(getattr(game.position, rook_piece.value))
    for from_square in rooks:
        possible_moves = split(
            get_moveable_squares_for_rook(all_pieces, enemy_pieces, from_square)
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=rook_piece,
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

    queen_piece = Piece.WHITE_QUEEN if is_white else Piece.BLACK_QUEEN
    queens = split(getattr(game.position, queen_piece.value))
    for from_square in queens:
        possible_moves = split(
            get_moveable_squares_for_queen(all_pieces, enemy_pieces, from_square)
        )
        for to_square in possible_moves:
            updated_game = move_piece(
                game=game,
                moved_piece=queen_piece,
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

    king_piece = Piece.WHITE_KING if is_white else Piece.BLACK_KING
    king = getattr(game.position, king_piece.value)
    regular, kingsideCastles, queensideCastles = get_moveable_squares_for_king(
        all_pieces=all_pieces,
        friendly_pieces=friendly_pieces,
        capture_squares=get_capture_squares(game.position, is_white),
        king=king,
        enemy_king=game.position.k if is_white else game.position.K,
        is_white=is_white,
        possible_castles=game.possible_castles,
    )
    for to_square in split(regular):
        updated_game = move_piece(
            game=game,
            moved_piece=king_piece,
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
            moved_piece=king_piece,
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
            moved_piece=king_piece,
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
    white_queens = split(position.Q)
    white_rooks = split(position.R)
    white_bishops = split(position.B)
    white_knights = split(position.N)
    white_pawns = split(position.P)
    black_queens = split(position.q)
    black_rooks = split(position.r)
    black_bishops = split(position.b)
    black_knights = split(position.n)
    black_pawns = split(position.p)

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
