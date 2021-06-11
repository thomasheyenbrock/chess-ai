import arraymancer
import math
import random
import sequtils
import strutils
import tables

import game
import policy_network
import value_network


randomize()


type Node = ref object
    state: Game
    children: seq[Node]
    is_expanded: bool
    has_checked_for_terminal: bool
    is_terminal: bool
    terminal_value: float32
    prior: float32
    visits: float32
    total_value: float32
    value: float32


proc newNode(state: Game, prior: float32): Node =
    return Node(
        state: state,
        children: @[],
        is_expanded: false,
        has_checked_for_terminal: false,
        is_terminal: false,
        prior: prior,
        terminal_value: 0,
        visits: 0,
        total_value: 0
    )


proc ucb_score(self: Node, parent_visits: float32): float32 =
    return (
        (if self.state.player: 1 - self.value else: self.value) +
        sqrt(2.0) * self.prior * sqrt(parent_visits) / (self.visits + 1)
    )


proc choose_child(self: Node): Node =
    var max = low(float32)
    var node: Node
    for child in self.children:
        let score = child.ucb_score(self.visits)
        if score > max:
            max = score
            node = child
    return node


proc expand(self: var Node) =
    let moves = self.state.legal_moves()

    let input = policy_network_ctx.variable(self.state.get_input().toTensor.reshape(1, 837))
    let all_priors = predict_policy_network(input).value.softmax

    var legal_priors = newTensor[float32](@[1, moves.len])
    for i in 0..<moves.len:
        legal_priors[0, i] = all_priors[0, OUTPUT_LAYER_MAPPING[moves[i].id]]
    var priors_seq = toSeq(legal_priors.softmax)

    for i in 0..<moves.len:
        self.children.add(newNode(state=self.state.move(moves[i]), prior=priors_seq[i]))
    self.is_expanded = true


proc result_to_float(game_result: string): float32 =
    if game_result == RESULT_WHITE:
        return 1.0
    if game_result == RESULT_BLACK:
        return 0.0
    if game_result == "":
        raise newException(OSError, "There is no result")
    return 0.5


proc get_value(self: var Node): float32 =
    if self.is_terminal:
        return self.terminal_value

    if not self.has_checked_for_terminal:
        self.has_checked_for_terminal = true
        let game = self.state
        let game_result = game.result(game.legal_moves().len)

        if game_result != "":
            self.is_terminal = true
            self.terminal_value = result_to_float(game_result)
            return self.terminal_value

    let input = value_network_ctx.variable(self.state.get_input().toTensor.reshape(1, 837))
    return predict_value_network(input)


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
    for i in 0..<search_path.len:
        search_path[i].total_value += value
        search_path[i].visits += 1
        search_path[i].value = search_path[i].total_value / search_path[i].visits


proc find_best_move(node: var Node, runs: int = 1000): Node =
    if not node.is_expanded:
        node.expand()
    for i in 1 .. runs:
        node.iteration()

    var best = node.children[0]
    for i in 1..<node.children.len:
        let child = node.children[i]
        if (node.state.player and child.value > best.value) or (not node.state.player and child.value < best.value):
            best = child

    return best

proc traverse(node: Node, depth: int = 0) =
    echo " ".repeat(depth * 2), node.state.last_move.id, " (", node.total_value, " / ", node.visits, ")"
    for i in 0..<node.children.len:
        traverse(node.children[i], depth + 1)

proc size(node: Node): int =
    if not node.is_expanded:
        return 1
    result = 0
    for i in 0..<node.children.len:
        result += size(node.children[i])

var root = newNode(
    state=game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    prior=0
)
root.expand()
echo root.size()
while not root.is_terminal:
    root = root.find_best_move()
    echo root.state.last_move.id, " ", root.size()
echo root.terminal_value
