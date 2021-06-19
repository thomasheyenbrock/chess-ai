import rdstdin
import sequtils

import game
import mcts
import policy_network
import value_network


load_value_network()
load_policy_network()

var roots: seq[Node] = @[newNode(
    treeId=0,
    state=empty_game(),
    prior=0,
)]

roots.expand(force=true)

let human = readLineFromStdin("Do you want to start? Type 'Y' or 'N': ") == "Y"

roots[0].state.position.to_string()
while not roots[0].is_terminal:
    if roots[0].state.player == human:
        var column: string
        let legal_moves = roots[0].state.legal_moves()
        while legal_moves.all(proc (move: Move): bool = return move.id != column):
            column = readLineFromStdin("Where do you want to place the stone? Type a number from 0 to 6: ")
        for child in roots[0].children:
            if child.state.last_move.id == column:
                roots = @[child]
                break
    else:
        roots = roots.find_best_moves(true)[0]
    roots[0].state.position.to_string()
    echo "\n\n"
echo roots[0].state.result(roots[0].state.legal_moves.len)
