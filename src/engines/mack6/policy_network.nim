import arraymancer, strutils, sequtils

# ##################################################################
# Define the model.

# Create the autograd context that will hold the computational graph
let policy_network_ctx* = newContext Tensor[float32]

network policy_network_ctx, Policy_Network:
    layers:
        fc1: Linear(837, 1121)
        fc2: Linear(1121, 1405)
        fc3: Linear(1405, 1689)
        fc4: Linear(1689, 1972)
    forward x:
        x.fc1.relu.fc2.relu.fc3.relu.fc4

let policy_network* = policy_network_ctx.init(Policy_Network)
let optim = policy_network.optimizerSGD(learning_rate = 1e-1'f32)

# ##################################################################
# Training

proc train_policy_network*(x: Variable, y: Tensor, epochs: int = 200) =
    echo "Training value network for ", epochs, " epochs"
    for e in 1..epochs:
        let y_pred = policy_network.forward(x)
        let loss = y_pred.softmax_cross_entropy(y)

        echo "  Epoch ", e, ": loss ", loss.value[0]

        loss.backprop()
        optim.update()


# ##################################################################
# Prediction

proc predict_policy_network*(x: Variable): Variable[Tensor[float32]] =
    let y_pred = policy_network.forward(x)
    return y_pred


# ##################################################################
# Storing weights and biases

proc save_policy_network*() =
    writeFile("policy.fc1.weight", $toSeq(policy_network.fc1.weight.value))
    writeFile("policy.fc1.bias", $toSeq(policy_network.fc1.bias.value))
    writeFile("policy.fc2.weight", $toSeq(policy_network.fc2.weight.value))
    writeFile("policy.fc2.bias", $toSeq(policy_network.fc2.bias.value))
    writeFile("policy.fc3.weight", $toSeq(policy_network.fc3.weight.value))
    writeFile("policy.fc3.bias", $toSeq(policy_network.fc3.bias.value))
    writeFile("policy.fc4.weight", $toSeq(policy_network.fc4.weight.value))
    writeFile("policy.fc4.bias", $toSeq(policy_network.fc4.bias.value))


# ##################################################################
# Restore weights and biases

proc parseAsFloat(x: string): float32 =
    return x.parseFloat

proc load_policy_network*() =
    try:
        policy_network.fc1.weight.value = readFile("policy.fc1.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1121, 837)
        policy_network.fc1.bias.value = readFile("policy.fc1.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1121)
        policy_network.fc2.weight.value = readFile("policy.fc2.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1405, 1121)
        policy_network.fc2.bias.value = readFile("policy.fc2.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1405)
        policy_network.fc3.weight.value = readFile("policy.fc3.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1689, 1405)
        policy_network.fc3.bias.value = readFile("policy.fc3.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1689)
        policy_network.fc4.weight.value = readFile("policy.fc4.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1972, 1689)
        policy_network.fc4.bias.value = readFile("policy.fc4.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1972)
        echo "Loaded policy network"
    except:
        echo "No policy network saved, saving the current one"
        save_policy_network()
