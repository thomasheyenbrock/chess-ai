import math
import random
import rdstdin

import game


randomize()


type Node = ref object
    state: Game
    children: seq[Node]
    is_expanded: bool
    is_terminal: bool
    terminal_value: float
    visits: float
    total_value: float
    value: float


proc newNode(state: Game): Node =
    return Node(
        state: state,
        children: @[],
        is_expanded: false,
        is_terminal: false,
        terminal_value: 0,
        visits: 0,
        total_value: 0
    )


proc ucb_score(self: Node, parent_visits: float): float =
    return self.value + 2 * sqrt(log(parent_visits, E) / self.visits)


proc choose_child(self: Node): Node =
    var not_visited: seq[Node] = @[]
    var max = low(float)
    var node: Node
    for child in self.children:
        if child.visits == 0:
            not_visited.add(child)
        else:
            let score = child.ucb_score(self.visits)
            if score > max:
                max = score
                node = child
    if not_visited.len > 0:
        return not_visited[rand(0 .. not_visited.len - 1)]
    return node


proc expand(self: var Node) =
    for move in self.state.legal_moves():
        self.children.add(
            newNode(state=self.state.move(move))
        )
    self.is_expanded = true


proc result_to_float(game_result: string): float =
    if game_result == RESULT_WHITE:
        return 1
    if game_result == RESULT_BLACK:
        return -1
    if game_result == "":
        raise newException(OSError, "There is no result")
    return 0


proc get_value(self: var Node): float =
    if self.is_terminal:
        return self.terminal_value

    var game = self.state
    var moves = game.legal_moves()
    var game_result = game.result(moves.len)

    if game_result != "":
        self.is_terminal = true
        self.terminal_value = result_to_float(game_result)
        return self.terminal_value

    while game_result == "":
        game = game.move(moves[rand(0 .. moves.len - 1)])
        moves = game.legal_moves()
        game_result = game.result(moves.len)
    return result_to_float(game_result)


proc iteration(self: var Node) =
    # Find child to explore
    var node = self
    var search_path = @[self]
    while node.is_expanded:
        node = node.choose_child()
        search_path.add(node)

    # If already visited and not terminal, then expand and choose a random child
    if node.visits > 0 and not node.is_terminal:
        node.expand()
        node = node.children[rand(0 .. node.children.len - 1)]
        search_path.add(node)

    # Find the value
    let value = node.get_value()

    # Backpropagate
    for n in search_path:
        n.total_value += value
        n.visits += 1
        n.value = n.total_value / n.visits


proc iterate(node: var Node, runs: int) =
    for i in 1 .. runs:
        # if i mod 50 == 0:
        #     echo(i, " / ", runs)
        node.iteration()


proc find_best_move(node: var Node): Node =
    iterate(node, 1000)

    var opt = if node.state.player: low(float) else: high(float)
    var best: Node
    for child in node.children:
        if (node.state.player and child.value > opt) or (not node.state.player and child.value < opt):
            opt = child.value
            best = child
            # echo(child.state.last_move.to_string(), ": ", child.value, " (", child.visits, " visits)")
    echo best.state.last_move.from_square
    echo best.state.last_move.to_square
    echo best.state.last_move.is_promoting_to

    return best


echo "ready"

var fen = readLineFromStdin("")
var current_game = game_from_fen(fen)
var node = newNode(state=current_game)
node.expand()
node = find_best_move(node)

while true:
    fen = readLineFromStdin("")

    if fen == "reset":
        # Fresh start
        fen = readLineFromStdin("")
        current_game = game_from_fen(fen)
        node = newNode(state=current_game)
        node.expand()
        node = find_best_move(node)
        continue

    current_game = game_from_fen(fen)

    # Check if the current node matches the game
    if node.state.equals(current_game):
        node = find_best_move(node)
        continue

    # Check if any child matches the game
    var done = false
    for child in node.children:
        if child.state.equals(current_game):
            node = child
            node = find_best_move(node)
            done = true
            break

    if not done:
        # Nothing matches...fuck
        raise newException(OSError, "No matching game found in tree")

