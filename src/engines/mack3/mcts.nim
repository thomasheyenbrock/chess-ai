import math
import nimpy
import random

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


proc newNode(state: Game): Node {.exportpy.} =
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


proc find_best_move(fen: string): Move {.exportpy.} =
    var node = newNode(state=game_from_fen(fen))
    node.expand()
    iterate(node, 1000)

    var max = low(float)
    var best: Node
    for child in node.children:
        if child.value > max:
            max = child.value
            best = child
            # echo(child.state.last_move.to_string(), ": ", child.value, " (", child.visits, " visits)")

    return best.state.last_move


# var root = newNode(state=game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
# var best = root.find_best_move(1000)
# echo(best.state.last_move)
