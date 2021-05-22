export type Bitboard = [number, number];

export function bitwiseNot(bitboard: Bitboard): Bitboard {
  return [~bitboard[0], ~bitboard[1]];
}

export function bitwiseAnd(bitboards: Bitboard[]): Bitboard {
  let and = bitboards[0];
  for (let i = 1; i < bitboards.length; i++) {
    and = [and[0] & bitboards[i][0], and[1] & bitboards[i][1]];
  }
  return [and[0] >>> 0, and[1] >>> 0];
}

export function bitwiseOr(bitboards: Bitboard[]): Bitboard {
  let or = bitboards[0];
  for (let i = 1; i < bitboards.length; i++) {
    or = [or[0] | bitboards[i][0], or[1] | bitboards[i][1]];
  }
  return [or[0] >>> 0, or[1] >>> 0];
}

export function bitwiseXor(bitboards: Bitboard[]): Bitboard {
  let xor = bitboards[0];
  for (let i = 1; i < bitboards.length; i++) {
    xor = [xor[0] ^ bitboards[i][0], xor[1] ^ bitboards[i][1]];
  }
  return [xor[0] >>> 0, xor[1] >>> 0];
}

export function isNull(bitboard: Bitboard) {
  return bitboard[0] === 0 && bitboard[1] === 0;
}

export function equals(bitboards: Bitboard[]) {
  const firstBitboard = bitboards[0];
  const first = firstBitboard[0];
  const second = firstBitboard[1];
  for (let i = 1; i < bitboards.length; i++) {
    if (bitboards[i][0] !== first) {
      return false;
    }
    if (bitboards[i][1] !== second) {
      return false;
    }
  }
  return true;
}

export function split(bitboard: Bitboard) {
  const singleBits: Bitboard[] = [];
  let first = bitboard[0];
  let second = bitboard[1];
  let c = 0;
  while (second !== 0) {
    const bit = second & 0x00000001;
    if (bit) {
      singleBits.push([0x00000000, (bit << c) >>> 0]);
    }
    c++;
    second = second >>> 1;
  }

  c = 0;
  while (first !== 0) {
    const bit = first & 0x00000001;
    if (bit) {
      singleBits.push([(bit << c) >>> 0, 0x00000000]);
    }
    c++;
    first = first >>> 1;
  }

  return singleBits;
}

export function getLeftSquare(bitboard: Bitboard): Bitboard {
  return [
    ((bitboard[0] & 0x7f7f7f7f) << 1) >>> 0,
    ((bitboard[1] & 0x7f7f7f7f) << 1) >>> 0,
  ];
}

export function getRightSquare(bitboard: Bitboard): Bitboard {
  return [(bitboard[0] & 0xfefefefe) >>> 1, (bitboard[1] & 0xfefefefe) >>> 1];
}

export function getTopSquare(bitboard: Bitboard): Bitboard {
  return [
    ((bitboard[0] << 8) | (bitboard[1] >>> 24)) >>> 0,
    (bitboard[1] << 8) >>> 0,
  ];
}

export function getBottomSquare(bitboard: Bitboard): Bitboard {
  return [
    (bitboard[0] >>> 8) >>> 0,
    ((bitboard[1] >>> 8) | (bitboard[0] << 24)) >>> 0,
  ];
}

export function getMoveableSqaresToLeft(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toLeft = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toLeft) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toLeft = getLeftSquare(toLeft);
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toLeft, allPieces]), toLeft]),
      getLeftSquare(changeMask),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getLeftSquare(bitwiseAnd([getRightSquare(enemyPieces), moveableSquares])),
  ]);
}

export function getMoveableSqaresToRight(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toRight = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toRight) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toRight = getRightSquare(toRight);
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toRight, allPieces]), toRight]),
      getRightSquare(changeMask),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getRightSquare(bitwiseAnd([getLeftSquare(enemyPieces), moveableSquares])),
  ]);
}

export function getMoveableSqaresToTop(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toTop = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toTop) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toTop = getTopSquare(toTop);
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toTop, allPieces]), toTop]),
      getTopSquare(changeMask),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getTopSquare(bitwiseAnd([getBottomSquare(enemyPieces), moveableSquares])),
  ]);
}

export function getMoveableSqaresToBottom(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toBottom = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toBottom) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toBottom = getBottomSquare(toBottom);
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toBottom, allPieces]), toBottom]),
      getBottomSquare(changeMask),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getBottomSquare(bitwiseAnd([getTopSquare(enemyPieces), moveableSquares])),
  ]);
}

export function getMoveableSqaresToTopLeft(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toTopLeft = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toTopLeft) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toTopLeft = getTopSquare(getLeftSquare(toTopLeft));
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toTopLeft, allPieces]), toTopLeft]),
      getTopSquare(getLeftSquare(changeMask)),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getTopSquare(
      getLeftSquare(
        bitwiseAnd([
          getBottomSquare(getRightSquare(enemyPieces)),
          moveableSquares,
        ])
      )
    ),
  ]);
}

export function getMoveableSqaresToTopRight(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toTopLeft = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toTopLeft) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toTopLeft = getTopSquare(getRightSquare(toTopLeft));
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toTopLeft, allPieces]), toTopLeft]),
      getTopSquare(getRightSquare(changeMask)),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getTopSquare(
      getRightSquare(
        bitwiseAnd([
          getBottomSquare(getLeftSquare(enemyPieces)),
          moveableSquares,
        ])
      )
    ),
  ]);
}

export function getMoveableSqaresToBottomLeft(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toTopLeft = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toTopLeft) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toTopLeft = getBottomSquare(getLeftSquare(toTopLeft));
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toTopLeft, allPieces]), toTopLeft]),
      getBottomSquare(getLeftSquare(changeMask)),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getBottomSquare(
      getLeftSquare(
        bitwiseAnd([getTopSquare(getRightSquare(enemyPieces)), moveableSquares])
      )
    ),
  ]);
}

export function getMoveableSqaresToBottomRight(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  observingPieces: Bitboard
) {
  let changeMask: Bitboard = [0xffffffff, 0xffffffff];
  let toTopLeft = observingPieces;
  let moveableSquares = observingPieces;
  let c = 0;
  while (!isNull(toTopLeft) && !isNull(changeMask)) {
    if (c++ > 100) {
      throw new Error("infinite loop");
    }

    toTopLeft = getBottomSquare(getRightSquare(toTopLeft));
    changeMask = bitwiseAnd([
      bitwiseXor([bitwiseAnd([toTopLeft, allPieces]), toTopLeft]),
      getBottomSquare(getRightSquare(changeMask)),
    ]);
    moveableSquares = bitwiseXor([moveableSquares, changeMask]);
  }
  return bitwiseOr([
    observingPieces,
    moveableSquares,
    getBottomSquare(
      getRightSquare(
        bitwiseAnd([getTopSquare(getLeftSquare(enemyPieces)), moveableSquares])
      )
    ),
  ]);
}

export function printBitboard(bitboard: Bitboard) {
  const first = bitboard[0].toString(2).padStart(32, "0");
  const second = bitboard[1].toString(2).padStart(32, "0");
  console.log(
    [
      first.slice(0, 8),
      first.slice(8, 16),
      first.slice(16, 24),
      first.slice(24, 32),
      second.slice(0, 8),
      second.slice(8, 16),
      second.slice(16, 24),
      second.slice(24, 32),
    ].join("\n")
  );
}
