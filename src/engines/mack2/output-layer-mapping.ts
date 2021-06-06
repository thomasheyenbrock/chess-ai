import {
  Bitboard,
  bitwiseAnd,
  bitwiseNot,
  bitwiseOr,
  getBottomSquare,
  getLeftSquare,
  getMoveableSqaresToBottom,
  getMoveableSqaresToBottomLeft,
  getMoveableSqaresToBottomRight,
  getMoveableSqaresToLeft,
  getMoveableSqaresToRight,
  getMoveableSqaresToTop,
  getMoveableSqaresToTopLeft,
  getMoveableSqaresToTopRight,
  getRightSquare,
  getTopSquare,
  split,
} from "../../scripts/bitboard";
import {
  Castle,
  generateMoveId,
  Piece,
  PromotionPiece,
} from "../../scripts/move-generator";

function getQueenMoves(from: Bitboard) {
  const moveableSquares = bitwiseAnd([
    bitwiseOr([
      getMoveableSqaresToLeft(from, [0, 0], from),
      getMoveableSqaresToRight(from, [0, 0], from),
      getMoveableSqaresToTop(from, [0, 0], from),
      getMoveableSqaresToBottom(from, [0, 0], from),
      getMoveableSqaresToTopLeft(from, [0, 0], from),
      getMoveableSqaresToTopRight(from, [0, 0], from),
      getMoveableSqaresToBottomLeft(from, [0, 0], from),
      getMoveableSqaresToBottomRight(from, [0, 0], from),
    ]),
    bitwiseNot(from),
  ]);
  return split(moveableSquares);
}

function getKnightMoves(from: Bitboard) {
  const top = getTopSquare(getTopSquare(from));
  const bottom = getBottomSquare(getBottomSquare(from));
  const left = getLeftSquare(getLeftSquare(from));
  const right = getRightSquare(getRightSquare(from));
  const moveableSquares = bitwiseOr([
    getLeftSquare(top),
    getRightSquare(top),
    getLeftSquare(bottom),
    getRightSquare(bottom),
    getTopSquare(left),
    getBottomSquare(left),
    getTopSquare(right),
    getBottomSquare(right),
  ]);
  return split(moveableSquares);
}

const whitePromotionMoves: { from: Bitboard; to: Bitboard }[] = [
  { from: [0x00010000, 0x00000000], to: [0x01000000, 0x00000000] },
  { from: [0x00010000, 0x00000000], to: [0x02000000, 0x00000000] },
  { from: [0x00020000, 0x00000000], to: [0x01000000, 0x00000000] },
  { from: [0x00020000, 0x00000000], to: [0x02000000, 0x00000000] },
  { from: [0x00020000, 0x00000000], to: [0x04000000, 0x00000000] },
  { from: [0x00040000, 0x00000000], to: [0x02000000, 0x00000000] },
  { from: [0x00040000, 0x00000000], to: [0x04000000, 0x00000000] },
  { from: [0x00040000, 0x00000000], to: [0x08000000, 0x00000000] },
  { from: [0x00080000, 0x00000000], to: [0x04000000, 0x00000000] },
  { from: [0x00080000, 0x00000000], to: [0x08000000, 0x00000000] },
  { from: [0x00080000, 0x00000000], to: [0x10000000, 0x00000000] },
  { from: [0x00100000, 0x00000000], to: [0x08000000, 0x00000000] },
  { from: [0x00100000, 0x00000000], to: [0x10000000, 0x00000000] },
  { from: [0x00100000, 0x00000000], to: [0x20000000, 0x00000000] },
  { from: [0x00200000, 0x00000000], to: [0x10000000, 0x00000000] },
  { from: [0x00200000, 0x00000000], to: [0x20000000, 0x00000000] },
  { from: [0x00200000, 0x00000000], to: [0x40000000, 0x00000000] },
  { from: [0x00400000, 0x00000000], to: [0x20000000, 0x00000000] },
  { from: [0x00400000, 0x00000000], to: [0x40000000, 0x00000000] },
  { from: [0x00400000, 0x00000000], to: [0x80000000, 0x00000000] },
  { from: [0x00800000, 0x00000000], to: [0x40000000, 0x00000000] },
  { from: [0x00800000, 0x00000000], to: [0x80000000, 0x00000000] },
];

const blackPromotionMoves: { from: Bitboard; to: Bitboard }[] = [
  { from: [0x00000000, 0x00000100], to: [0x00000000, 0x00000001] },
  { from: [0x00000000, 0x00000100], to: [0x00000000, 0x00000002] },
  { from: [0x00000000, 0x00000200], to: [0x00000000, 0x00000001] },
  { from: [0x00000000, 0x00000200], to: [0x00000000, 0x00000002] },
  { from: [0x00000000, 0x00000200], to: [0x00000000, 0x00000004] },
  { from: [0x00000000, 0x00000400], to: [0x00000000, 0x00000002] },
  { from: [0x00000000, 0x00000400], to: [0x00000000, 0x00000004] },
  { from: [0x00000000, 0x00000400], to: [0x00000000, 0x00000008] },
  { from: [0x00000000, 0x00000800], to: [0x00000000, 0x00000004] },
  { from: [0x00000000, 0x00000800], to: [0x00000000, 0x00000008] },
  { from: [0x00000000, 0x00000800], to: [0x00000000, 0x00000010] },
  { from: [0x00000000, 0x00001000], to: [0x00000000, 0x00000008] },
  { from: [0x00000000, 0x00001000], to: [0x00000000, 0x00000010] },
  { from: [0x00000000, 0x00001000], to: [0x00000000, 0x00000020] },
  { from: [0x00000000, 0x00002000], to: [0x00000000, 0x00000010] },
  { from: [0x00000000, 0x00002000], to: [0x00000000, 0x00000020] },
  { from: [0x00000000, 0x00002000], to: [0x00000000, 0x00000040] },
  { from: [0x00000000, 0x00004000], to: [0x00000000, 0x00000020] },
  { from: [0x00000000, 0x00004000], to: [0x00000000, 0x00000040] },
  { from: [0x00000000, 0x00004000], to: [0x00000000, 0x00000080] },
  { from: [0x00000000, 0x00008000], to: [0x00000000, 0x00000040] },
  { from: [0x00000000, 0x00008000], to: [0x00000000, 0x00000080] },
];

export const moveForOutputIndex = [
  ...split([0xffffffff, 0xffffffff]).flatMap((from) =>
    getQueenMoves(from).map((to) => generateMoveId(from, to, null, null))
  ),
  ...split([0xffffffff, 0xffffffff]).flatMap((from) =>
    getKnightMoves(from).map((to) => generateMoveId(from, to, null, null))
  ),
  ...whitePromotionMoves.flatMap(({ from, to }) => [
    generateMoveId(from, to, null, Piece.WHITE_QUEEN as PromotionPiece),
    generateMoveId(from, to, null, Piece.WHITE_ROOK as PromotionPiece),
    generateMoveId(from, to, null, Piece.WHITE_BISHOP as PromotionPiece),
    generateMoveId(from, to, null, Piece.WHITE_KNIGHT as PromotionPiece),
  ]),
  ...blackPromotionMoves.flatMap(({ from, to }) => [
    generateMoveId(from, to, null, Piece.BLACK_QUEEN as PromotionPiece),
    generateMoveId(from, to, null, Piece.BLACK_ROOK as PromotionPiece),
    generateMoveId(from, to, null, Piece.BLACK_BISHOP as PromotionPiece),
    generateMoveId(from, to, null, Piece.BLACK_KNIGHT as PromotionPiece),
  ]),
  generateMoveId(
    [0x00000000, 0x00000008],
    [0x00000000, 0x00000002],
    Castle.WHITE_KINGSIDE,
    null
  ),
  generateMoveId(
    [0x00000000, 0x00000008],
    [0x00000000, 0x00000020],
    Castle.WHITE_QUEENSIDE,
    null
  ),
  generateMoveId(
    [0x08000000, 0x00000000],
    [0x02000000, 0x00000000],
    Castle.BLACK_KINGSIDE,
    null
  ),
  generateMoveId(
    [0x08000000, 0x00000000],
    [0x20000000, 0x00000000],
    Castle.BLACK_QUEENSIDE,
    null
  ),
];
