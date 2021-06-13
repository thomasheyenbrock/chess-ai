import time
import tensorflow as tf

# Define Sequential model with 3 layers
model = tf.keras.Sequential(
    [
        tf.keras.layers.Dense(1121, activation="relu", name="layer1"),
        tf.keras.layers.Dense(1405, activation="relu", name="layer2"),
        tf.keras.layers.Dense(1689, activation="relu", name="layer3"),
        tf.keras.layers.Dense(1972, activation="sigmoid", name="layer4"),
    ]
)

t = 0
for i in range(1000):
    x = tf.random.uniform((1, 837))
    s = time.perf_counter_ns()
    y = model(x)
    t += time.perf_counter_ns() - s
print(t / 1_000_000_000)
