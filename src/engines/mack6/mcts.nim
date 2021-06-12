import arraymancer
import math
import os
import random
import sequtils
import tables
import times

import game
import policy_network
import value_network


randomize()


let now = getTime()
var r = initRand(now.toUnix * 1_000_000_000 + now.nanosecond)


type Node = ref object
    state: Game
    input: Tensor[float32]
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
        input: state.get_input().toTensor.reshape(1, 837),
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

    let input = policy_network_ctx.variable(self.input)
    let all_priors = predict_policy_network(input).value.softmax
    policy_network_ctx.nodes = @[]

    var legal_priors = newTensor[float32](moves.len)
    for i in 0..<moves.len:
        legal_priors[i] = all_priors[0, OUTPUT_LAYER_MAPPING[moves[i].id]]
    var priors_seq = toSeq(legal_priors / legal_priors.sum)

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

    result = predict_value_network(value_network_ctx.variable(self.input))
    value_network_ctx.nodes = @[]


proc iteration(self: var Node) =
    # Find child to explore
    var node = self
    var search_path = @[self]
    while node.is_expanded:
        node = node.choose_child()
        search_path.add(node)

    # If already visited and not terminal, then expand and choose one of the children
    if node.visits > 0 and not node.is_terminal:
        node.expand()
        node = node.choose_child()
        search_path.add(node)

    # Find the value
    let value = node.get_value()

    # Backpropagate
    for i in 0..<search_path.len:
        search_path[i].total_value += value
        search_path[i].visits += 1
        search_path[i].value = search_path[i].total_value / search_path[i].visits


proc find_best_move(node: var Node, greedy: bool = false, runs: int = 1600): (Node, Tensor[float32]) =
    if not node.is_expanded:
        node.expand()
    for i in 1 .. runs:
        node.iteration()

    var best = node.children[0]
    var policy = newTensor[float32](1972)
    policy[OUTPUT_LAYER_MAPPING[node.children[0].state.last_move.id]] = node.value
    var cdf = newTensor[float32]([1, node.children.len])
    cdf[_, _] = -999_999_999
    cdf[0, 0] = best.value

    for i in 1..<node.children.len:
        let child = node.children[i]
        policy[OUTPUT_LAYER_MAPPING[child.state.last_move.id]] = child.value
        cdf[0, i] = child.value
        if (
            (node.state.player and child.value > best.value) or
            (not node.state.player and child.value < best.value)
        ):
            best = child

    if greedy:
        return (best, policy / policy.sum)
    return (r.sample(node.children, cdf.softmax.toSeq.cumsummed), policy / policy.sum)


load_value_network()
load_policy_network()

var inputs: seq[string] = @[]
var policies: seq[string] = @[]
var root = newNode(
    state=game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    prior=0
)
root.expand()
while not root.is_terminal:
    var (new_root, policy) = root.find_best_move()
    root = new_root
    inputs.add($root.input.toSeq)
    policies.add($policy.toSeq)
    echo root.state.last_move.id
echo "Result: ", root.terminal_value

let value_network_data_file = open("value." & paramStr(1) & ".txt", fmAppend)
let policy_network_data_file = open("policy." & paramStr(1) & ".txt", fmAppend)
for i in 0..<inputs.len:
    value_network_data_file.write(inputs[i] & ";" & $root.terminal_value & "\n")
    policy_network_data_file.write(inputs[i] & ";" & policies[i] & "\n")
value_network_data_file.close()
policy_network_data_file.close()
