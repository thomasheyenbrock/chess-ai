import arraymancer, strutils, sequtils

# ##################################################################
# Define the model.

# Create the autograd context that will hold the computational graph
let policy_network_ctx* = newContext Tensor[float32]

network policy_network_ctx, Policy_Network:
    layers:
        x: Input([3, 7, 6])
        cv1: Conv2D(x.out_shape, 32, 3, 3)
        cv2: Conv2D(cv1.out_shape, 16, 3, 3)
        fl: Flatten(cv2.out_shape)
        hidden: Linear(fl.out_shape, 48)
        output: Linear(48, 7)
    forward x:
        x.cv1.relu.cv2.relu.fl.hidden.relu.output

let policy_network* = policy_network_ctx.init(Policy_Network)
let optim = policy_network.optimizerSGD(learning_rate = 1e-1'f32)

# ##################################################################
# Training

proc train_policy_network*(x: Variable, y: Tensor, epochs: int = 10000) =
    echo "Training value network for ", epochs, " epochs"
    for e in 1..epochs:
        let y_pred = policy_network.forward(x)
        let loss = y_pred.softmax_cross_entropy(y)

        if e mod 100 == 0:
            echo "  Epoch ", e, ": loss ", loss.value[0]

        loss.backprop()
        optim.update()


# ##################################################################
# Prediction

proc predict_policy_network*(x: Variable): Variable[Tensor[float32]] =
    return policy_network.forward(x)


# ##################################################################
# Storing weights and biases

proc save_policy_network*() =
    writeFile("policy.cv1.weight", $toSeq(policy_network.cv1.weight.value))
    writeFile("policy.cv1.bias", $toSeq(policy_network.cv1.bias.value))
    writeFile("policy.cv2.weight", $toSeq(policy_network.cv2.weight.value))
    writeFile("policy.cv2.bias", $toSeq(policy_network.cv2.bias.value))
    writeFile("policy.hidden.weight", $toSeq(policy_network.hidden.weight.value))
    writeFile("policy.hidden.bias", $toSeq(policy_network.hidden.bias.value))
    writeFile("policy.output.weight", $toSeq(policy_network.output.weight.value))
    writeFile("policy.output.bias", $toSeq(policy_network.output.bias.value))


# ##################################################################
# Restore weights and biases

proc parseAsFloat(x: string): float32 =
    return x.parseFloat

proc load_policy_network*() =
    try:
        policy_network.cv1.weight.value = readFile("policy.cv1.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(32, 3, 3, 3)
        policy_network.cv1.bias.value = readFile("policy.cv1.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(32, 1, 1)
        policy_network.cv2.weight.value = readFile("policy.cv2.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(16, 32, 3, 3)
        policy_network.cv2.bias.value = readFile("policy.cv2.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(16, 1, 1)
        policy_network.hidden.weight.value = readFile("policy.hidden.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(48, 96)
        policy_network.hidden.bias.value = readFile("policy.hidden.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 48)
        policy_network.output.weight.value = readFile("policy.output.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(7, 48)
        policy_network.output.bias.value = readFile("policy.output.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 7)
        echo "Loaded policy network"
    except:
        echo "No policy network saved, saving the current one"
        save_policy_network()
