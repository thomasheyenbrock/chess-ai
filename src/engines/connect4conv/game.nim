import arraymancer


const RESULT_RED* = "0 wins"
const RESULT_YELLOW* = "X wins"
const RESULT_DRAW* = "Draw"


type Move* = object
    id*: string
    player*: bool
    column*: int


proc newMove(
    player: bool,
    column: int,
): Move =
    return Move(id: $column, player: player, column: column)


type Position* = object
    red_pieces*: array[7, array[6, float32]]
    yellow_pieces*: array[7, array[6, float32]]
    first_empty_row*: array[7, int]


proc piece_string(position: Position, column: int, row: int): string =
    if position.red_pieces[column][row] != 0:
        return " O "
    if position.yellow_pieces[column][row] != 0:
        return " X "
    return "   "


proc to_string*(position: Position) =
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 5), "|", position.piece_string(1, 5), "|", position.piece_string(2, 5), "|", position.piece_string(3, 5), "|", position.piece_string(4, 5), "|", position.piece_string(5, 5), "|", position.piece_string(6, 5), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 4), "|", position.piece_string(1, 4), "|", position.piece_string(2, 4), "|", position.piece_string(3, 4), "|", position.piece_string(4, 4), "|", position.piece_string(5, 4), "|", position.piece_string(6, 4), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 3), "|", position.piece_string(1, 3), "|", position.piece_string(2, 3), "|", position.piece_string(3, 3), "|", position.piece_string(4, 3), "|", position.piece_string(5, 3), "|", position.piece_string(6, 3), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 2), "|", position.piece_string(1, 2), "|", position.piece_string(2, 2), "|", position.piece_string(3, 2), "|", position.piece_string(4, 2), "|", position.piece_string(5, 2), "|", position.piece_string(6, 2), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 1), "|", position.piece_string(1, 1), "|", position.piece_string(2, 1), "|", position.piece_string(3, 1), "|", position.piece_string(4, 1), "|", position.piece_string(5, 1), "|", position.piece_string(6, 1), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "|", position.piece_string(0, 0), "|", position.piece_string(1, 0), "|", position.piece_string(2, 0), "|", position.piece_string(3, 0), "|", position.piece_string(4, 0), "|", position.piece_string(5, 0), "|", position.piece_string(6, 0), "|"
    echo "+---+---+---+---+---+---+---+"
    echo "  0   1   2   3   4   5   6  "


proc move(position: Position, move: Move): Position =
    result = position
    if move.player:
        result.red_pieces[move.column][result.first_empty_row[move.column]] = 1
    else:
        result.yellow_pieces[move.column][result.first_empty_row[move.column]] = 1
    result.first_empty_row[move.column] += 1


type Game* = object
    position*: Position
    player*: bool
    last_move*: Move


proc newGame(
    position: Position,
    player: bool,
    last_move: Move,
): Game =
    return Game(
        position: position,
        player: player,
        last_move: last_move,
    )


proc move*(game: Game, move: Move): Game =
    result = newGame(
        position=game.position.move(move),
        player=not game.player,
        last_move=move,
    )


proc legal_moves*(game: Game): seq[Move] =
    for column in 0..6:
        if game.position.first_empty_row[column] < 6:
            result.add(newMove(game.player, column))


proc result*(game: Game, legal_moves: int): string =
    for column in 0..6:
        for row in 0..2:
            if (
                game.position.red_pieces[column][row] == 1 and
                game.position.red_pieces[column][row + 1] == 1 and
                game.position.red_pieces[column][row + 2] == 1 and
                game.position.red_pieces[column][row + 3] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[column][row] == 1 and
                game.position.yellow_pieces[column][row + 1] == 1 and
                game.position.yellow_pieces[column][row + 2] == 1 and
                game.position.yellow_pieces[column][row + 3] == 1
            ):
                return RESULT_YELLOW
    for row in 0..5:
        for column in 0..3:
            if (
                game.position.red_pieces[column][row] == 1 and
                game.position.red_pieces[column + 1][row] == 1 and
                game.position.red_pieces[column + 2][row] == 1 and
                game.position.red_pieces[column + 3][row] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[column][row] == 1 and
                game.position.yellow_pieces[column + 1][row] == 1 and
                game.position.yellow_pieces[column + 2][row] == 1 and
                game.position.yellow_pieces[column + 3][row] == 1
            ):
                return RESULT_YELLOW
    for diag in 0..2:
        for i in 0..diag:
            if (
                game.position.red_pieces[i][2 - diag + i] == 1 and
                game.position.red_pieces[i + 1][3 - diag + i] == 1 and
                game.position.red_pieces[i + 2][4 - diag + i] == 1 and
                game.position.red_pieces[i + 3][5 - diag + i] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[i][2 - diag + i] == 1 and
                game.position.yellow_pieces[i + 1][3 - diag + i] == 1 and
                game.position.yellow_pieces[i + 2][4 - diag + i] == 1 and
                game.position.yellow_pieces[i + 3][5 - diag + i] == 1
            ):
                return RESULT_YELLOW
    for diag in 0..2:
        for i in 0..diag:
            if (
                game.position.red_pieces[3 + i - diag][i] == 1 and
                game.position.red_pieces[4 + i - diag][i + 1] == 1 and
                game.position.red_pieces[5 + i - diag][i + 2] == 1 and
                game.position.red_pieces[6 + i - diag][i + 3] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[3 + i - diag][i] == 1 and
                game.position.yellow_pieces[4 + i - diag][i + 1] == 1 and
                game.position.yellow_pieces[5 + i - diag][i + 2] == 1 and
                game.position.yellow_pieces[6 + i - diag][i + 3] == 1
            ):
                return RESULT_YELLOW
    for diag in 0..2:
        for i in 0..diag:
            if (
                game.position.red_pieces[6 - i][2 - diag + i] == 1 and
                game.position.red_pieces[5 - i][3 - diag + i] == 1 and
                game.position.red_pieces[4 - i][4 - diag + i] == 1 and
                game.position.red_pieces[3 - i][5 - diag + i] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[6 - i][2 - diag + i] == 1 and
                game.position.yellow_pieces[5 - i][3 - diag + i] == 1 and
                game.position.yellow_pieces[4 - i][4 - diag + i] == 1 and
                game.position.yellow_pieces[3 - i][5 - diag + i] == 1
            ):
                return RESULT_YELLOW
    for diag in 0..2:
        for i in 0..diag:
            if (
                game.position.red_pieces[diag - i + 3][i] == 1 and
                game.position.red_pieces[diag - i + 2][i + 1] == 1 and
                game.position.red_pieces[diag - i + 1][i + 2] == 1 and
                game.position.red_pieces[diag - i][i + 3] == 1
            ):
                return RESULT_RED
            if (
                game.position.yellow_pieces[diag - i + 3][i] == 1 and
                game.position.yellow_pieces[diag - i + 2][i + 1] == 1 and
                game.position.yellow_pieces[diag - i + 1][i + 2] == 1 and
                game.position.yellow_pieces[diag - i][i + 3] == 1
            ):
                return RESULT_YELLOW
    if legal_moves == 0:
        return RESULT_DRAW
    return ""


proc get_input*(game: Game): Tensor[float32] =
    result = newTensor[float32](1, 3, 7, 6)
    result[0, 0, _, _] = game.position.red_pieces.toTensor.reshape(1, 1, 7, 6)
    result[0, 1, _, _] = game.position.yellow_pieces.toTensor.reshape(1, 1, 7, 6)
    result[0, 2, _, _] = if game.player: 1 else: 0


proc empty_game*(): Game =
    return newGame(
        Position(
            red_pieces: [
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
            ],
            yellow_pieces: [
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
                [0'f, 0'f, 0'f, 0'f, 0'f, 0'f],
            ],
            first_empty_row: [0, 0, 0, 0, 0, 0, 0]
        ),
        true,
        newMove(false, -1)
    )
