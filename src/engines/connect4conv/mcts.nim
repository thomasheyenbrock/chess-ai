import arraymancer
import math
import random
import sequtils
import times

import game
import policy_network
import value_network


randomize()


let now = getTime()
var r = initRand(now.toUnix * 1_000_000_000 + now.nanosecond)


type Node* = ref object
    treeId*: int
    state*: Game
    input*: Tensor[float32]
    children*: seq[Node]
    is_expanded*: bool
    has_checked_for_terminal*: bool
    is_terminal*: bool
    terminal_value*: float32
    prior*: float32
    visits*: float32
    total_value*: float32


proc newNode*(treeId: int, state: Game, prior: float32): Node =
    return Node(
        treeId: treeId,
        state: state,
        input: state.get_input(),
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
    var value = 0'f
    if self.visits > 0:
        value = (
            if self.state.player: 1 - self.total_value / self.visits
            else: self.total_value / self.visits
        )
    return value + 3 * self.prior * sqrt(parent_visits) / (self.visits + 1e-6)


proc choose_child(self: Node): Node =
    var max = low(float32)
    for child in self.children:
        let score = child.ucb_score(self.visits)
        if score > max:
            max = score
            result = child


proc expand*(nodes: seq[Node], force: bool = false) =
    let num_nodes = nodes.len

    var inputs = newTensor[float32](num_nodes, 3, 7, 6)
    for i in 0..<num_nodes:
        inputs[i, _, _, _] = nodes[i].input
    let all_priors = predict_policy_network(policy_network_ctx.variable(inputs)).value.softmax
    policy_network_ctx.nodes = @[]

    for i in 0..<num_nodes:
        var node = nodes[i]

        # If already expanded, then do nothing
        if node.is_expanded:
            continue
        # If already not visited or terminal, then we don't expand further,
        # except we force an expansion
        if (node.visits == 0 or node.is_terminal) and not force:
            continue

        let moves = node.state.legal_moves()

        var legal_priors = newTensor[float32](moves.len)
        for j in 0..<moves.len:
            legal_priors[j] = all_priors[i, moves[j].column]
        var priors_seq: seq[float32]
        if legal_priors.sum == 0:
            for j in 0..<moves.len:
                priors_seq.add(1 / moves.len)
        else:
            priors_seq = toSeq(legal_priors / legal_priors.sum)

        for j in 0..<moves.len:
            node.children.add(newNode(
                treeId=node.treeId,
                state=node.state.move(moves[j]),
                prior=priors_seq[j]
            ))
        node.is_expanded = true


proc result_to_float(game_result: string): float32 =
    if game_result == RESULT_RED:
        return 1.0
    if game_result == RESULT_YELLOW:
        return 0.0
    if game_result == "":
        raise newException(OSError, "There is no result")
    return 0.5


proc get_values(nodes: seq[Node]): Tensor[float32] =
    let num_nodes = nodes.len

    var inputs = newTensor[float32](nodes.len, 3, 7, 6)
    for i in 0..<num_nodes:
        inputs[i, _] = nodes[i].input
    var values = predict_value_network(value_network_ctx.variable(inputs)).value
    value_network_ctx.nodes = @[]

    for i in 0..<num_nodes:
        var node = nodes[i]

        if node.is_terminal:
            values[i, 0] = node.terminal_value
        elif not node.has_checked_for_terminal:
            node.has_checked_for_terminal = true
            let game = node.state
            let game_result = game.result(game.legal_moves().len)

            if game_result != "":
                node.is_terminal = true
                node.terminal_value = result_to_float(game_result)
                values[i, 0] = node.terminal_value
    return values


proc iteration(nodes: seq[Node]) =
    let num_nodes = nodes.len
    var search_paths: seq[seq[Node]] = @[]
    var evaluation_nodes: seq[Node] = @[]

    # Find child to explore
    for i in 0..<num_nodes:
        var node = nodes[i]
        var search_path = @[node]
        while node.is_expanded:
            node = node.choose_child()
            search_path.add(node)
        search_paths.add(search_path)
        evaluation_nodes.add(node)

    # Try to expand notes where possible and choose one of the children
    evaluation_nodes.expand()
    for i in 0..<num_nodes:
        var node = evaluation_nodes[i]
        if node.is_expanded:
            node = node.choose_child()
            search_paths[i].add(node)
            evaluation_nodes[i] = node

    # Find the values
    let values = evaluation_nodes.get_values().toSeq

    # Backpropagate
    for i in 0..<num_nodes:
        for j in 0..<search_paths[i].len:
            search_paths[i][j].total_value += values[i]
            search_paths[i][j].visits += 1


proc find_best_moves*(nodes: seq[Node], greedy: bool = false, runs: int = 1600): (seq[Node], seq[Tensor[float32]]) =
    for i in 1..runs:
        nodes.iteration()

    var new_nodes: seq[Node] = @[]
    var policies: seq[Tensor[float32]] = @[]
    for i in 0..<nodes.len:
        var node = nodes[i]
        var best = node.children[0]
        var policy = newTensor[float32](7)
        policy[node.children[0].state.last_move.column] = node.visits
        var cdf = @[best.visits]

        # echo best.state.last_move.id, "\t", best.prior, "\t", best.visits, "\t", best.total_value
        for i in 1..<node.children.len:
            let child = node.children[i]
            # echo child.state.last_move.id, "\t", child.prior, "\t", child.visits, "\t", child.total_value
            policy[child.state.last_move.column] = child.visits
            cdf.add(child.visits)
            if child.visits > best.visits:
                best = child

        if greedy:
            new_nodes.add(best)
        else:
            new_nodes.add(r.sample(node.children, cdf.cumsummed))
        policies.add(policy / policy.sum)
    return (new_nodes, policies)
