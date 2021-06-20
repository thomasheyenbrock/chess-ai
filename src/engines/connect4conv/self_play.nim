import arraymancer
import os
import sequtils
import strutils

import game
import mcts
import policy_network
import value_network


load_value_network()
load_policy_network()

var input_strings: seq[seq[string]] = @[]
var policy_strings: seq[seq[string]] = @[]
var terminal_values: seq[float32] = @[]
var logs: seq[string] = @[]
var roots: seq[Node] = @[]

for i in 0..<paramStr(2).parseInt:
    input_strings.add(@[])
    policy_strings.add(@[])
    terminal_values.add(-1)
    logs.add("")
    roots.add(newNode(
        treeId=i,
        state=empty_game(),
        prior=0,
    ))
roots.expand(force=true)

var counter = 0
while roots.len > 0:
    var (new_roots, policies) = roots.find_best_moves()
    for i in 0..<new_roots.len:
        let root = new_roots[i]
        input_strings[root.treeId].add($root.input.toSeq)
        policy_strings[root.treeId].add($policies[i].toSeq)
    roots = @[]
    for root in new_roots:
        if root.is_terminal:
            terminal_values[root.treeId] = root.terminal_value
            logs[root.treeId] = $root.terminal_value
        else:
            roots.add(root)
            logs[root.treeId] = root.state.last_move.id

    counter += 1
    echo counter, "\t", logs.join("\t")

let value_network_data_file = open("value." & paramStr(1) & ".txt", fmAppend)
let policy_network_data_file = open("policy." & paramStr(1) & ".txt", fmAppend)

for i in 0..<input_strings.len:
    for j in 0..<input_strings[i].len:
        value_network_data_file.write(input_strings[i][j])
        value_network_data_file.write(";")
        value_network_data_file.write(terminal_values[i])
        value_network_data_file.write("\n")
        policy_network_data_file.write(input_strings[i][j])
        policy_network_data_file.write(";")
        policy_network_data_file.write(policy_strings[i][j])
        policy_network_data_file.write("\n")

value_network_data_file.close()
policy_network_data_file.close()
