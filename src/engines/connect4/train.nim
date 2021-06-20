import arraymancer
import os
import sequtils
import strutils

import policy_network
import value_network


proc parseAsFloat(x: string): float32 =
    return x.parseFloat


echo "Loading training data"
var value_input: seq[seq[seq[seq[float32]]]] = @[]
var value_output: seq[seq[float32]] = @[]
var policy_input: seq[seq[seq[seq[float32]]]] = @[]
var policy_output: seq[seq[float32]] = @[]
for i in 1..paramCount():
    for line in readFile("value." & paramStr(i) & ".txt").strip().split("\n"):
        let splitted = line.strip().split(";")
        let inputs = splitted[0].replace("@[", "").replace("]", "").split(", ").map(parseAsFloat)
        value_input.add(@[
            @[
                inputs[0..5],
                inputs[6..11],
                inputs[12..17],
                inputs[18..23],
                inputs[24..29],
                inputs[30..35],
                inputs[36..41],
            ],
            @[
                inputs[42..47],
                inputs[48..53],
                inputs[54..59],
                inputs[60..65],
                inputs[66..71],
                inputs[72..77],
                inputs[78..83],
            ],
            @[
                inputs[84..89],
                inputs[90..95],
                inputs[96..101],
                inputs[102..107],
                inputs[108..113],
                inputs[114..119],
                inputs[120..125],
            ]
        ])
        value_output.add(@[splitted[1].parseAsFloat])
    for line in readFile("policy." & paramStr(i) & ".txt").strip().split("\n"):
        let splitted = line.strip().split(";")
        let inputs = splitted[0].replace("@[", "").replace("]", "").split(", ").map(parseAsFloat)
        policy_input.add(@[
            @[
                inputs[0..5],
                inputs[6..11],
                inputs[12..17],
                inputs[18..23],
                inputs[24..29],
                inputs[30..35],
                inputs[36..41],
            ],
            @[
                inputs[42..47],
                inputs[48..53],
                inputs[54..59],
                inputs[60..65],
                inputs[66..71],
                inputs[72..77],
                inputs[78..83],
            ],
            @[
                inputs[84..89],
                inputs[90..95],
                inputs[96..101],
                inputs[102..107],
                inputs[108..113],
                inputs[114..119],
                inputs[120..125],
            ]
        ])
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
