import arraymancer
import random
import os
import sequtils
import strutils

import policy_network
import value_network


randomize()


proc parseAsFloat(x: string): float32 =
    return x.parseFloat


echo "Loading training data"
var value_input: seq[seq[float32]] = @[]
var value_output: seq[seq[float32]] = @[]
var policy_input: seq[seq[float32]] = @[]
var policy_output: seq[seq[float32]] = @[]
for i in 1..paramCount():
    for line in readFile("value." & paramStr(i) & ".txt").strip().split("\n"):
        let splitted = line.strip().split(";")
        value_input.add(splitted[0].replace("@[", "").replace("]", "").split(", ").map(parseAsFloat))
        value_output.add(@[splitted[1].parseAsFloat])
    for line in readFile("policy." & paramStr(i) & ".txt").strip().split("\n"):
        let splitted = line.strip().split(";")
        policy_input.add(splitted[0].replace("@[", "").replace("]", "").split(", ").map(parseAsFloat))
        policy_output.add(splitted[1].replace("@[", "").replace("]", "").split(", ").map(parseAsFloat))

load_value_network()
train_value_network(
    value_network_ctx.variable(value_input.toTensor),
    value_output.toTensor
)
save_value_network()

load_policy_network()
train_policy_network(
    policy_network_ctx.variable(policy_input.toTensor),
    policy_output.toTensor
)
save_policy_network()
