export type Bitboard = [number, number];

export function bitwiseNot(bitboard: Bitboard): Bitboard {
  return [~bitboard[0], ~bitboard[1]];
}

export function bitwiseAnd(bitboards: Bitboard[]): Bitboard {
  return bitboards.reduce(
    (first, second) => [first[0] & second[0], first[1] & second[1]],
    [0xffffffff, 0xffffffff]
  );
}

export function bitwiseOr(bitboards: Bitboard[]): Bitboard {
  return bitboards.reduce(
    (first, second) => [first[0] | second[0], first[1] | second[1]],
    [0x00000000, 0x00000000]
  );
}

export function isNull(bitboard: Bitboard) {
  return bitboard[0] === 0 && bitboard[1] === 0;
}

export function equals(bitboards: Bitboard[]) {
  const [first, ...rest] = bitboards;
  return rest.every(
    (bitboard) => bitboard[0] === first[0] && bitboard[1] === first[1]
  );
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
