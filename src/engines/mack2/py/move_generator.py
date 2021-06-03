from typing import Iterable, Optional

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
from enums import Castle, Piece, Player, Result
from game import Game, Move


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


def get_legal_moves(game: Game) -> Iterable[Move]:
    is_white = game.player == Player["WHITE"]
    opposite_player = Player["BLACK"] if is_white else Player["WHITE"]

    friendly_pieces = (
        game.position.white_pieces if is_white else game.position.black_pieces
    )
    enemy_pieces = (
        game.position.black_pieces if is_white else game.position.white_pieces
    )
    empty_squares = 0xFFFF_FFFF_FFFF_FFFF ^ game.position.all_pieces
    attacked_squares = game.position.attacked_squares(
        player=opposite_player, exclude_king=True
    )
    non_attacked_squares = 0xFFFF_FFFF_FFFF_FFFF ^ attacked_squares

    king = game.position.pieces[Piece["KING"]][game.player]
    king_moves = KING_MOVES[king] & non_attacked_squares
    king_moves ^= king_moves & friendly_pieces
    for to_square in split(king_moves):
        yield Move(
            player=game.player,
            piece=Piece["KING"],
            from_square=king,
            to_square=to_square,
            en_passant_square=0,
            is_capturing_en_passant=False,
            is_castling=None,
            is_promoting_to=None,
        )

    attackers = list(game.position.checkers(player=opposite_player, king=king))
    number_of_attackers = len(attackers)
    if number_of_attackers > 1:
        # Multiple pieces are giving check, so the king has to move
        return

    capture_mask = 0xFFFF_FFFF_FFFF_FFFF
    push_mask = 0xFFFF_FFFF_FFFF_FFFF
    if number_of_attackers == 1:
        attacker = attackers[0]
        capture_mask = attacker
        if (attacker & game.position.pieces[Piece["KNIGHT"]][opposite_player] != 0) or (
            attacker & game.position.pieces[Piece["PAWN"]][opposite_player] != 0
        ):
            # checked by knight or pawn, this can't be blocked
            push_mask = 0
        else:
            # checked by slider, this can be blocked
            push_mask = (
                NORTH_MOVES[king].get(attacker, 0)
                | SOUTH_MOVES[king].get(attacker, 0)
                | WEST_MOVES[king].get(attacker, 0)
                | EAST_MOVES[king].get(attacker, 0)
                | NORTH_WEST_MOVES[king].get(attacker, 0)
                | NORTH_EAST_MOVES[king].get(attacker, 0)
                | SOUTH_WEST_MOVES[king].get(attacker, 0)
                | SOUTH_EAST_MOVES[king].get(attacker, 0)
            )

    capture_or_push_mask = capture_mask | push_mask

    enemy_queens = game.position.pieces[Piece["QUEEN"]][opposite_player]
    enemy_queens_and_rooks = (
        enemy_queens | game.position.pieces[Piece["ROOK"]][opposite_player]
    )
    enemy_queens_and_bishops = (
        enemy_queens | game.position.pieces[Piece["BISHOP"]][opposite_player]
    )

    for from_square in split(game.position.pieces[Piece["QUEEN"]][game.player]):
        moveable_squares = (
            capture_or_push_mask
            & (
                get_rank_and_file_moves(
                    game.position.all_pieces, enemy_pieces, from_square
                )
                | get_diagonal_moves(
                    game.position.all_pieces, enemy_pieces, from_square
                )
            )
            & game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            yield Move(
                player=game.player,
                piece=Piece["QUEEN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

    for from_square in split(game.position.pieces[Piece["ROOK"]][game.player]):
        moveable_squares = (
            capture_or_push_mask
            & get_rank_and_file_moves(
                game.position.all_pieces, enemy_pieces, from_square
            )
            & game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            yield Move(
                player=game.player,
                piece=Piece["ROOK"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

    for from_square in split(game.position.pieces[Piece["BISHOP"]][game.player]):
        moveable_squares = (
            capture_or_push_mask
            & get_diagonal_moves(game.position.all_pieces, enemy_pieces, from_square)
            & game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            yield Move(
                player=game.player,
                piece=Piece["BISHOP"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

    for from_square in split(game.position.pieces[Piece["KNIGHT"]][game.player]):
        moveable_squares = (
            capture_or_push_mask
            & KNIGHT_MOVES[from_square]
            & (KNIGHT_MOVES[from_square] ^ friendly_pieces)
            & game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            yield Move(
                player=game.player,
                piece=Piece["KNIGHT"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

    for from_square in split(game.position.pieces[Piece["PAWN"]][game.player]):
        pinned_movement = game.position.pinned_movement(
            square=from_square,
            king=king,
            enemy_queens_and_rooks=enemy_queens_and_rooks,
            enemy_queens_and_bishops=enemy_queens_and_bishops,
        )
        to_square = (
            PAWN_SINGLE_MOVES[game.player][from_square]
            & empty_squares
            & pinned_movement
            & push_mask
        )
        if to_square != 0:
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

        attacks = [
            p & enemy_pieces & pinned_movement & capture_mask
            for p in PAWN_ATTACK_MOVES[game.player][from_square]
        ]
        for to_square in attacks:
            if to_square == 0:
                continue
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

        to_square = (
            PAWN_DOUBLE_MOVES[game.player][from_square]
            & empty_squares
            & (
                get_top_square(empty_squares)
                if is_white
                else get_bottom_square(empty_squares)
            )
            & pinned_movement
            & push_mask
        )
        if to_square:
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=get_bottom_square(to_square)
                if is_white
                else get_top_square(to_square),
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=None,
            )

        to_square = (
            PAWN_EN_PASSANT_CAPTURES[game.player][from_square]
            & game.en_passant_square
            & pinned_movement
            & (
                get_top_square(capture_mask)
                if is_white
                else get_bottom_square(capture_mask)
            )
        )
        if to_square:
            move = Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=True,
                is_castling=None,
                is_promoting_to=None,
            )
            position = game.position.move(move)[0]
            if not position.is_check(game.player):
                yield move

        single_move_promotions = [
            p & empty_squares & pinned_movement & push_mask
            for p in PAWN_SINGLE_MOVES_PROMOTION[game.player][from_square]
        ]
        attack_promotions = [
            p & enemy_pieces & pinned_movement & capture_mask
            for p in PAWN_ATTACK_MOVES_PROMOTION[game.player][from_square]
        ]
        for to_square in single_move_promotions + attack_promotions:
            if to_square == 0:
                continue
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=Piece["QUEEN"],
            )
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=Piece["ROOK"],
            )
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=Piece["BISHOP"],
            )
            yield Move(
                player=game.player,
                piece=Piece["PAWN"],
                from_square=from_square,
                to_square=to_square,
                en_passant_square=0,
                is_capturing_en_passant=False,
                is_castling=None,
                is_promoting_to=Piece["KNIGHT"],
            )

    can_castle_kingside = (
        game.possible_castles.get(
            Castle["WHITE_KINGSIDE"] if is_white else Castle["BLACK_KINGSIDE"]
        )
        and (
            game.position.all_pieces
            & (0x0000_0000_0000_0006 if is_white else 0x0600_0000_0000_0000)
        )
        == 0
        and (
            attacked_squares
            & (0x0000_0000_0000_000E if is_white else 0x0E00_0000_0000_0000)
            == 0
        )
    )

    if can_castle_kingside:
        yield Move(
            player=game.player,
            piece=Piece["KING"],
            from_square=0x0000_0000_0000_0008 if is_white else 0x0800_0000_0000_0000,
            to_square=0x0000_0000_0000_0002 if is_white else 0x0200_0000_0000_0000,
            en_passant_square=0,
            is_capturing_en_passant=False,
            is_castling=Castle["WHITE_KINGSIDE"]
            if is_white
            else Castle["BLACK_KINGSIDE"],
            is_promoting_to=None,
        )

    can_castle_queenside = (
        game.possible_castles.get(
            Castle["WHITE_QUEENSIDE"] if is_white else Castle["BLACK_QUEENSIDE"]
        )
        and (
            game.position.all_pieces
            & (0x0000_0000_0000_0070 if is_white else 0x7000_0000_0000_0000)
        )
        == 0
        and (
            attacked_squares
            & (0x0000_0000_0000_0038 if is_white else 0x3800_0000_0000_0000)
            == 0
        )
    )

    if can_castle_queenside:
        yield Move(
            player=game.player,
            piece=Piece["KING"],
            from_square=0x0000_0000_0000_0008 if is_white else 0x0800_0000_0000_0000,
            to_square=0x0000_0000_0000_0020 if is_white else 0x2000_0000_0000_0000,
            en_passant_square=0,
            is_capturing_en_passant=False,
            is_castling=Castle["WHITE_QUEENSIDE"]
            if is_white
            else Castle["BLACK_QUEENSIDE"],
            is_promoting_to=None,
        )


def count_legal_moves(game: Game, depth: int = 1):
    if depth == 0:
        return 1

    sum = 0
    for move in get_legal_moves(game):
        next_game = game.move(move)
        add = count_legal_moves(next_game, depth - 1)
        # if depth == 1:
        #     print(next_game.last_move.__str__() + ":", add)
        sum += add

    return sum


def get_game_result(game: Game, legal_moves: dict[str, Game]) -> Optional[str]:
    if len(legal_moves) == 0:
        if game.position.is_check(game.player):
            return (
                Result["BLACK"] if game.player == Player["WHITE"] else Result["WHITE"]
            )
        return Result["STALEMATE"]

    if game.position.is_dead():
        return Result["DEAD_POSITION"]

    for count in game.position_counts.values():
        if count >= 3:
            return Result["REPITITION"]

    if game.fifty_move_counter >= 100:
        return Result["FIFTY_MOVE_RULE"]

    return None
