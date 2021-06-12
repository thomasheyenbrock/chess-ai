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

echo "Shuffling"
for i in 1..value_input.len*4:
    let index = rand(0..value_input.len-1)

    let value_input_item = value_input[index]
    value_input.del(index)
    value_input.add(value_input_item)

    let value_output_item = value_output[index]
    value_output.del(index)
    value_output.add(value_output_item)

    let policy_input_item = policy_input[index]
    policy_input.del(index)
    policy_input.add(policy_input_item)

    let policy_output_item = policy_output[index]
    policy_output.del(index)
    policy_output.add(policy_output_item)

let data_points = if value_input.len < 1024: value_input.len else: 1024

load_value_network()
train_value_network(
    value_network_ctx.variable(value_input[0..data_points].toTensor),
    value_output[0..data_points].toTensor
)
save_value_network()

load_policy_network()
train_policy_network(
    policy_network_ctx.variable(policy_input[0..data_points].toTensor),
    policy_output[0..data_points].toTensor
)
save_policy_network()
