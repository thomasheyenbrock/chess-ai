import arraymancer, strutils, sequtils

# ##################################################################
# Define the model.

# Create the autograd context that will hold the computational graph
let value_network_ctx* = newContext Tensor[float32]

network value_network_ctx, Value_Network:
    layers:
        fc1: Linear(837, 628)
        fc2: Linear(628, 419)
        fc3: Linear(419, 210)
        fc4: Linear(210, 1)
    forward x:
        x.fc1.relu.fc2.relu.fc3.relu.fc4.sigmoid

let value_network* = value_network_ctx.init(Value_Network)
let optim = value_network.optimizerSGD(learning_rate = 1e-4'f32)

# ##################################################################
# Training

proc train_value_network*(x: Variable, y: Tensor, epochs: int = 200) =
    echo "Training value network for ", epochs, " epochs"
    for e in 1..epochs:
        let y_pred = value_network.forward(x)
        let loss = y_pred.mse_loss(y)

        echo "  Epoch ", e, ": loss ", loss.value[0]

        loss.backprop()
        optim.update()


# ##################################################################
# Prediction

proc predict_value_network*(x: Variable): float32 =
    let y_pred = value_network.forward(x)
    return toSeq(y_pred.value)[0]


# ##################################################################
# Storing weights and biases

proc save_value_network*() =
    writeFile("value.fc1.weight", $toSeq(value_network.fc1.weight.value))
    writeFile("value.fc1.bias", $toSeq(value_network.fc1.bias.value))
    writeFile("value.fc2.weight", $toSeq(value_network.fc2.weight.value))
    writeFile("value.fc2.bias", $toSeq(value_network.fc2.bias.value))
    writeFile("value.fc3.weight", $toSeq(value_network.fc3.weight.value))
    writeFile("value.fc3.bias", $toSeq(value_network.fc3.bias.value))
    writeFile("value.fc4.weight", $toSeq(value_network.fc4.weight.value))
    writeFile("value.fc4.bias", $toSeq(value_network.fc4.bias.value))


# ##################################################################
# Restore weights and biases

proc parseAsFloat(x: string): float32 =
    return x.parseFloat

proc load_value_network*() =
    try:
        value_network.fc1.weight.value = readFile("value.fc1.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(628, 837)
        value_network.fc1.bias.value = readFile("value.fc1.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 628)
        value_network.fc2.weight.value = readFile("value.fc2.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(419, 628)
        value_network.fc2.bias.value = readFile("value.fc2.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 419)
        value_network.fc3.weight.value = readFile("value.fc3.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(210, 419)
        value_network.fc3.bias.value = readFile("value.fc3.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 210)
        value_network.fc4.weight.value = readFile("value.fc4.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 210)
        value_network.fc4.bias.value = readFile("value.fc4.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1)
        echo "Loaded value network"
    except:
        echo "No value network saved, saving the current one"
        save_value_network()
