import arraymancer, strutils, sequtils

# ##################################################################
# Define the model.

# Create the autograd context that will hold the computational graph
let value_network_ctx* = newContext Tensor[float32]

network value_network_ctx, Value_Network:
    layers:
        x: Input([3, 7, 6])
        cv1: Conv2D(x.out_shape, 32, 3, 3)
        cv2: Conv2D(cv1.out_shape, 16, 3, 3)
        fl: Flatten(cv2.out_shape)
        hidden: Linear(fl.out_shape, 48)
        output: Linear(48, 1)
    forward x:
        x.cv1.relu.cv2.relu.fl.hidden.relu.output.sigmoid

let value_network* = value_network_ctx.init(Value_Network)
let optim = value_network.optimizerSGD(learning_rate = 1e-4'f32)

# ##################################################################
# Training

proc train_value_network*(x: Variable, y: Tensor, epochs: int = 10000) =
    echo "Training value network for ", epochs, " epochs"
    for e in 1..epochs:
        let y_pred = value_network.forward(x)
        let loss = y_pred.mse_loss(y)

        if e mod 100 == 0:
            echo "  Epoch ", e, ": loss ", loss.value[0]

        loss.backprop()
        optim.update()


# ##################################################################
# Prediction

proc predict_value_network*(x: Variable): Variable[Tensor[float32]] =
    return value_network.forward(x)


# ##################################################################
# Storing weights and biases

proc save_value_network*() =
    writeFile("value.cv1.weight", $toSeq(value_network.cv1.weight.value))
    writeFile("value.cv1.bias", $toSeq(value_network.cv1.bias.value))
    writeFile("value.cv2.weight", $toSeq(value_network.cv2.weight.value))
    writeFile("value.cv2.bias", $toSeq(value_network.cv2.bias.value))
    writeFile("value.hidden.weight", $toSeq(value_network.hidden.weight.value))
    writeFile("value.hidden.bias", $toSeq(value_network.hidden.bias.value))
    writeFile("value.output.weight", $toSeq(value_network.output.weight.value))
    writeFile("value.output.bias", $toSeq(value_network.output.bias.value))


# ##################################################################
# Restore weights and biases

proc parseAsFloat(x: string): float32 =
    return x.parseFloat

proc load_value_network*() =
    try:
        value_network.cv1.weight.value = readFile("value.cv1.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(32, 3, 3, 3)
        value_network.cv1.bias.value = readFile("value.cv1.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(32, 1, 1)
        value_network.cv2.weight.value = readFile("value.cv2.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(16, 32, 3, 3)
        value_network.cv2.bias.value = readFile("value.cv2.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(16, 1, 1)
        value_network.hidden.weight.value = readFile("value.hidden.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(48, 96)
        value_network.hidden.bias.value = readFile("value.hidden.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 48)
        value_network.output.weight.value = readFile("value.output.weight")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 48)
        value_network.output.bias.value = readFile("value.output.bias")
            .replace("@[", "").replace("]", "")
            .split(", ").map(parseAsFloat).toTensor.reshape(1, 1)
        echo "Loaded value network"
    except:
        echo "No value network saved, saving the current one"
        save_value_network()
