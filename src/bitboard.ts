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

function isGreaterThan(first: Bitboard, second: Bitboard) {
  if (first[0] > second[0]) {
    return true;
  }
  if (first[0] < second[0]) {
    return false;
  }
  return first[1] > second[1];
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

function isOnSameRank(bitboards: Bitboard[]) {
  const or = bitwiseOr(bitboards);
  return (
    (isNull(bitwiseAnd([or, [0x00000000, 0x000000ff]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x0000ff00]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00ff0000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0xff000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x000000ff, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x0000ff00, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00ff0000, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0xff000000, 0x00000000]])) ? 0 : 1)
  );
}

export function getSquaresOnRank(
  from: Bitboard,
  to: Bitboard
): Bitboard | null {
  if (!isOnSameRank([from, to])) {
    return null;
  }
  const squares: Bitboard[] = [];
  const greaterThan = isGreaterThan(from, to);
  let current = getRightSquare(greaterThan ? from : to);
  const goal = greaterThan ? to : from;
  while (!equals([current, goal])) {
    squares.push(current);
    current = getRightSquare(current);
  }
  return bitwiseOr(squares);
}

function isOnSameFile(bitboards: Bitboard[]) {
  const or = bitwiseOr(bitboards);
  return (
    (isNull(bitwiseAnd([or, [0x01010101, 0x01010101]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x02020202, 0x02020202]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x04040404, 0x04040404]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x08080808, 0x08080808]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x10101010, 0x10101010]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x20202020, 0x20202020]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x40404040, 0x40404040]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x80808080, 0x80808080]])) ? 0 : 1)
  );
}

export function getSquaresOnFile(
  from: Bitboard,
  to: Bitboard
): Bitboard | null {
  if (!isOnSameFile([from, to])) {
    return null;
  }
  const squares: Bitboard[] = [];
  const greaterThan = isGreaterThan(from, to);
  let current = getBottomSquare(greaterThan ? from : to);
  const goal = greaterThan ? to : from;
  while (!equals([current, goal])) {
    squares.push(current);
    current = getBottomSquare(current);
  }
  return bitwiseOr(squares);
}

function isOnSameDecreasingDiagonal(bitboards: Bitboard[]) {
  const or = bitwiseOr(bitboards);
  return (
    (isNull(bitwiseAnd([or, [0x00000000, 0x00000080]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00008040]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00804020]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x80402010]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000080, 0x40201008]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00008040, 0x20100804]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00804020, 0x10080402]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x80402010, 0x08040201]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x40201008, 0x04020100]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x20100804, 0x02010000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x10080402, 0x01000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x08040201, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x04020100, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x02010000, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x01000000, 0x00000000]])) ? 0 : 1)
  );
}

export function getSquaresOnDecreasingDiagonal(
  from: Bitboard,
  to: Bitboard
): Bitboard | null {
  if (!isOnSameDecreasingDiagonal([from, to])) {
    return null;
  }
  const squares: Bitboard[] = [];
  const greaterThan = isGreaterThan(from, to);
  let current = getBottomSquare(getRightSquare(greaterThan ? from : to));
  const goal = greaterThan ? to : from;
  while (!equals([current, goal])) {
    squares.push(current);
    current = getBottomSquare(getRightSquare(current));
  }
  return bitwiseOr(squares);
}

function isOnSameIncreasingDiagonal(bitboards: Bitboard[]) {
  const or = bitwiseOr(bitboards);
  return (
    (isNull(bitwiseAnd([or, [0x80000000, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x40800000, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x20408000, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x10204080, 0x00000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x08102040, 0x80000000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x04081020, 0x40800000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x02040810, 0x20408000]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x01020408, 0x10204080]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00010204, 0x08102040]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000102, 0x04081020]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000001, 0x02040810]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x01020408]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00010204]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00000102]])) ? 0 : 1) ^
    (isNull(bitwiseAnd([or, [0x00000000, 0x00000001]])) ? 0 : 1)
  );
}

export function getSquaresOnIncreasingDiagonal(
  from: Bitboard,
  to: Bitboard
): Bitboard | null {
  if (!isOnSameIncreasingDiagonal([from, to])) {
    return null;
  }
  const squares: Bitboard[] = [];
  const greaterThan = isGreaterThan(from, to);
  let current = getBottomSquare(getLeftSquare(greaterThan ? from : to));
  const goal = greaterThan ? to : from;
  while (!equals([current, goal])) {
    squares.push(current);
    current = getBottomSquare(getLeftSquare(current));
  }
  return bitwiseOr(squares);
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
