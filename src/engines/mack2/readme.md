# mack2

## INPUT LAYER

- 1 bit for the current player (1 = white, 0 = black)
- 12 Bitboards (one for each piece) with 64 bits each
- 4 bits for each castling (1 = still possible, 0 = no longer possible)
- One bitboard with 64 bits showing the en passant square (if any)
  => 1 + 12 \* 64 + 4 + 64 = 837 nodes

## OUTPUT LAYER

- one for each possible move
  => 1972 nodes

## HIDDEN LAYERS

- one hidden layer
- number of nodes is the arithmetic mean of input and output
  => (837 + 1972) / 2 = 1405 nodes
