from typing import Optional, Tuple

from bitboard import (
    get_top_square,
    get_bottom_square,
    split,
)
from constants import (
    EAST_ATTACKS,
    EAST_MOVES,
    EAST_RAY,
    KING_MOVES,
    KNIGHT_MOVES,
    NORTH_ATTACKS,
    NORTH_EAST_ATTACKS,
    NORTH_EAST_MOVES,
    NORTH_EAST_RAY,
    NORTH_MOVES,
    NORTH_RAY,
    NORTH_WEST_ATTACKS,
    NORTH_WEST_MOVES,
    NORTH_WEST_RAY,
    PAWN_ATTACK_MOVES,
    PAWN_ATTACK_MOVES_PROMOTION,
    PAWN_DOUBLE_MOVES,
    PAWN_EN_PASSANT_CAPTURES,
    PAWN_SINGLE_MOVES,
    PAWN_SINGLE_MOVES_PROMOTION,
    SOUTH_ATTACKS,
    SOUTH_EAST_ATTACKS,
    SOUTH_EAST_MOVES,
    SOUTH_EAST_RAY,
    SOUTH_MOVES,
    SOUTH_RAY,
    SOUTH_WEST_ATTACKS,
    SOUTH_WEST_MOVES,
    SOUTH_WEST_RAY,
    WEST_ATTACKS,
    WEST_MOVES,
    WEST_RAY,
)
from enums import Castle, Piece, Player, PromotionPiece, Result
from game import Game, Move, Position


def get_moveable_squares_for_king(
    position: Position,
    friendly_pieces: int,
    king: int,
    is_white: bool,
    possible_castles: dict[Castle, bool],
) -> Tuple[int, int, int]:
    regular_moves = KING_MOVES[king]
    regular_moves = regular_moves ^ (regular_moves & friendly_pieces)

    can_castle_kingside = (
        possible_castles.get(
            Castle.WHITE_KINGSIDE if is_white else Castle.BLACK_KINGSIDE
        )
        and (
            position.all_pieces
            & (0x0000_0000_0000_0006 if is_white else 0x0600_0000_0000_0000)
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0002 if is_white else 0x0200_0000_0000_0000,
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0004 if is_white else 0x0400_0000_0000_0000,
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0008 if is_white else 0x0800_0000_0000_0000,
        )
        == 0
    )
    can_castle_queenside = (
        possible_castles.get(
            Castle.WHITE_QUEENSIDE if is_white else Castle.BLACK_QUEENSIDE
        )
        and (
            position.all_pieces
            & (0x0000_0000_0000_0070 if is_white else 0x7000_0000_0000_0000)
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0008 if is_white else 0x0800_0000_0000_0000,
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0010 if is_white else 0x1000_0000_0000_0000,
        )
        == 0
        and position.attackers(
            Player.BLACK if is_white else Player.WHITE,
            0x0000_0000_0000_0020 if is_white else 0x2000_0000_0000_0000,
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


def get_rank_and_file_moves(all_pieces: int, enemy_pieces: int, square: int) -> int:
    north_pieces = NORTH_RAY[square] & all_pieces
    south_pieces = SOUTH_RAY[square] & all_pieces
    west_pieces = WEST_RAY[square] & all_pieces
    east_pieces = EAST_RAY[square] & all_pieces

    north_moves = NORTH_MOVES[square][north_pieces] ^ (
        NORTH_ATTACKS[square][north_pieces] & enemy_pieces
    )
    south_moves = SOUTH_MOVES[square][south_pieces] ^ (
        SOUTH_ATTACKS[square][south_pieces] & enemy_pieces
    )
    west_moves = WEST_MOVES[square][west_pieces] ^ (
        WEST_ATTACKS[square][west_pieces] & enemy_pieces
    )
    east_moves = EAST_MOVES[square][east_pieces] ^ (
        EAST_ATTACKS[square][east_pieces] & enemy_pieces
    )

    return north_moves | south_moves | west_moves | east_moves


def get_diagonal_moves(all_pieces: int, enemy_pieces: int, square: int) -> int:
    north_west_pieces = NORTH_WEST_RAY[square] & all_pieces
    south_west_pieces = SOUTH_WEST_RAY[square] & all_pieces
    north_east_pieces = NORTH_EAST_RAY[square] & all_pieces
    south_east_pieces = SOUTH_EAST_RAY[square] & all_pieces

    north_west_moves = NORTH_WEST_MOVES[square][north_west_pieces] ^ (
        NORTH_WEST_ATTACKS[square][north_west_pieces] & enemy_pieces
    )
    north_east_moves = NORTH_EAST_MOVES[square][north_east_pieces] ^ (
        NORTH_EAST_ATTACKS[square][north_east_pieces] & enemy_pieces
    )
    south_west_moves = SOUTH_WEST_MOVES[square][south_west_pieces] ^ (
        SOUTH_WEST_ATTACKS[square][south_west_pieces] & enemy_pieces
    )
    south_east_moves = SOUTH_EAST_MOVES[square][south_east_pieces] ^ (
        SOUTH_EAST_ATTACKS[square][south_east_pieces] & enemy_pieces
    )

    return north_west_moves | north_east_moves | south_west_moves | south_east_moves


def get_moveable_squares_for_knight(friendly_pieces: int, knight: int) -> int:
    moveable_squares = KNIGHT_MOVES[knight]
    return moveable_squares & (moveable_squares ^ friendly_pieces)


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
    empty_squares = 0xFFFF_FFFF_FFFF_FFFF ^ game.position.all_pieces

    possible_games: dict[str, Game] = {}

    pawns = split(getattr(game.position, Piece.PAWN.value)[game.player])
    for from_square in pawns:
        to_square = PAWN_SINGLE_MOVES[game.player][from_square] & empty_squares
        if to_square:
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

        attacks = [
            p & enemy_pieces for p in PAWN_ATTACK_MOVES[game.player][from_square]
        ]
        for to_square in attacks:
            if to_square == 0:
                continue
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

        to_square = (
            PAWN_DOUBLE_MOVES[game.player][from_square]
            & empty_squares
            & (
                get_top_square(empty_squares)
                if is_white
                else get_bottom_square(empty_squares)
            )
        )
        if to_square:
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

        to_square = (
            PAWN_EN_PASSANT_CAPTURES[game.player][from_square] & game.en_passant_square
        )
        if to_square:
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

        single_move_promotions = [
            p & empty_squares
            for p in PAWN_SINGLE_MOVES_PROMOTION[game.player][from_square]
        ]
        attack_promotions = [
            p & enemy_pieces
            for p in PAWN_ATTACK_MOVES_PROMOTION[game.player][from_square]
        ]
        for to_square in single_move_promotions + attack_promotions:
            if to_square == 0:
                continue
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
            if not updated_game_with_queen_promotion.position.is_check(game.player):
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
            if not updated_game_with_rook_promotion.position.is_check(game.player):
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
            if not updated_game_with_bishop_promotion.position.is_check(game.player):
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
            if not updated_game_with_knight_promotion.position.is_check(game.player):
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

    bishops = split(getattr(game.position, Piece.BISHOP.value)[game.player])
    for from_square in bishops:
        possible_moves = split(
            get_diagonal_moves(game.position.all_pieces, enemy_pieces, from_square)
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

    rooks = split(getattr(game.position, Piece.ROOK.value)[game.player])
    for from_square in rooks:
        possible_moves = split(
            get_rank_and_file_moves(game.position.all_pieces, enemy_pieces, from_square)
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

    queens = split(getattr(game.position, Piece.QUEEN.value)[game.player])
    for from_square in queens:
        possible_moves = split(
            get_rank_and_file_moves(game.position.all_pieces, enemy_pieces, from_square)
            | get_diagonal_moves(game.position.all_pieces, enemy_pieces, from_square)
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
            if not updated_game.position.is_check(game.player):
                possible_games[updated_game.last_move.id()] = updated_game

    king = getattr(game.position, Piece.KING.value)[game.player]
    regular, kingsideCastles, queensideCastles = get_moveable_squares_for_king(
        position=game.position,
        friendly_pieces=friendly_pieces,
        king=king,
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
        if not updated_game.position.is_check(game.player):
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


def get_game_result(game: Game, legal_moves: dict[str, Game]) -> Optional[Result]:
    if len(legal_moves) == 0:
        if game.position.is_check(game.player):
            return Result.BLACK if game.player == Player.WHITE else Result.WHITE
        return Result.STALEMATE

    if game.position.is_dead():
        return Result.DEAD_POSITION

    for count in game.position_counts.values():
        if count >= 3:
            return Result.REPITITION

    if game.fifty_move_counter >= 100:
        return Result.FIFTY_MOVE_RULE

    return None
