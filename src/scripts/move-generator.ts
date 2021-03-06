import {
  Bitboard,
  bitwiseAnd,
  bitwiseNot,
  bitwiseOr,
  bitwiseXor,
  equals,
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
  isNull,
  split,
} from "./bitboard";
import { gameToFen } from "./fen";

export const squares: Bitboard[][] = [
  [
    [0x80000000, 0x00000000],
    [0x40000000, 0x00000000],
    [0x20000000, 0x00000000],
    [0x10000000, 0x00000000],
    [0x08000000, 0x00000000],
    [0x04000000, 0x00000000],
    [0x02000000, 0x00000000],
    [0x01000000, 0x00000000],
  ],
  [
    [0x00800000, 0x00000000],
    [0x00400000, 0x00000000],
    [0x00200000, 0x00000000],
    [0x00100000, 0x00000000],
    [0x00080000, 0x00000000],
    [0x00040000, 0x00000000],
    [0x00020000, 0x00000000],
    [0x00010000, 0x00000000],
  ],
  [
    [0x00008000, 0x00000000],
    [0x00004000, 0x00000000],
    [0x00002000, 0x00000000],
    [0x00001000, 0x00000000],
    [0x00000800, 0x00000000],
    [0x00000400, 0x00000000],
    [0x00000200, 0x00000000],
    [0x00000100, 0x00000000],
  ],
  [
    [0x00000080, 0x00000000],
    [0x00000040, 0x00000000],
    [0x00000020, 0x00000000],
    [0x00000010, 0x00000000],
    [0x00000008, 0x00000000],
    [0x00000004, 0x00000000],
    [0x00000002, 0x00000000],
    [0x00000001, 0x00000000],
  ],
  [
    [0x00000000, 0x80000000],
    [0x00000000, 0x40000000],
    [0x00000000, 0x20000000],
    [0x00000000, 0x10000000],
    [0x00000000, 0x08000000],
    [0x00000000, 0x04000000],
    [0x00000000, 0x02000000],
    [0x00000000, 0x01000000],
  ],
  [
    [0x00000000, 0x00800000],
    [0x00000000, 0x00400000],
    [0x00000000, 0x00200000],
    [0x00000000, 0x00100000],
    [0x00000000, 0x00080000],
    [0x00000000, 0x00040000],
    [0x00000000, 0x00020000],
    [0x00000000, 0x00010000],
  ],
  [
    [0x00000000, 0x00008000],
    [0x00000000, 0x00004000],
    [0x00000000, 0x00002000],
    [0x00000000, 0x00001000],
    [0x00000000, 0x00000800],
    [0x00000000, 0x00000400],
    [0x00000000, 0x00000200],
    [0x00000000, 0x00000100],
  ],
  [
    [0x00000000, 0x00000080],
    [0x00000000, 0x00000040],
    [0x00000000, 0x00000020],
    [0x00000000, 0x00000010],
    [0x00000000, 0x00000008],
    [0x00000000, 0x00000004],
    [0x00000000, 0x00000002],
    [0x00000000, 0x00000001],
  ],
];

export enum Piece {
  WHITE_KING = "K",
  WHITE_QUEEN = "Q",
  WHITE_ROOK = "R",
  WHITE_BISHOP = "B",
  WHITE_KNIGHT = "N",
  WHITE_PAWN = "P",
  BLACK_KING = "k",
  BLACK_QUEEN = "q",
  BLACK_ROOK = "r",
  BLACK_BISHOP = "b",
  BLACK_KNIGHT = "n",
  BLACK_PAWN = "p",
}

export type PromotionPiece =
  | Piece.WHITE_QUEEN
  | Piece.WHITE_ROOK
  | Piece.WHITE_BISHOP
  | Piece.WHITE_KNIGHT
  | Piece.BLACK_QUEEN
  | Piece.BLACK_ROOK
  | Piece.BLACK_BISHOP
  | Piece.BLACK_KNIGHT;

type Position = Record<Piece, Bitboard>;

export enum Player {
  WHITE = "w",
  BLACK = "b",
}

export enum Result {
  WHITE = "White wins",
  BLACK = "Black wins",
  STALEMATE = "Stalemate",
  DEAD_POSITION = "Dead position",
  REPITITION = "Third repitition of position",
  FIFTY_MOVE_RULE = "Fifty moves without capture or pawn movement",
}

export enum Castle {
  WHITE_KINGSIDE = "K",
  WHITE_QUEENSIDE = "Q",
  BLACK_KINGSIDE = "k",
  BLACK_QUEENSIDE = "q",
}

type Move = {
  player: Player;
  piece: Piece;
  from: Bitboard;
  to: Bitboard;
  isCapturing: Piece | null;
  isCastling: Castle | null;
  isPromotingTo: PromotionPiece | null;
};

export type Game = {
  position: Position;
  player: Player;
  lastMove: Move | null;
  possibleMoves: { [moveId: string]: Game };
  possibleCastles: Record<Castle, boolean>;
  enPassantSquare: Bitboard;
  positionCounts: { [stringifiedGame: string]: number };
  moveCounter: number;
  fiftyMoveCounter: number;
  result: Result | null;
};

function incrementPositionCount(game: Game) {
  const positionCounts = Object.assign({}, game.positionCounts);
  const key = gameToFen(game).split(" ").slice(0, 4).join(" ");
  positionCounts[key] = (positionCounts[key] || 0) + 1;
  game.positionCounts = positionCounts;
  return game;
}

function getMoveableSquaresForKing(
  allPieces: Bitboard,
  friendlyPieces: Bitboard,
  captureSquares: Bitboard,
  king: Bitboard,
  enemyKing: Bitboard,
  isWhite: boolean,
  possibleCastles: Record<Castle, boolean>
): {
  regular: Bitboard;
  kingsideCastles: Bitboard;
  queensideCastles: Bitboard;
} {
  const top = getTopSquare(king);
  const bottom = getBottomSquare(king);
  const left = getLeftSquare(king);
  const right = getRightSquare(king);

  const enemyKingTop = getTopSquare(enemyKing);
  const enemyKingBottom = getBottomSquare(enemyKing);
  const enemyKingLeft = getLeftSquare(enemyKing);
  const enemyKingRight = getRightSquare(enemyKing);
  const enemyKingSquares = bitwiseOr([
    enemyKing,
    enemyKingTop,
    getRightSquare(enemyKingTop),
    enemyKingRight,
    getBottomSquare(enemyKingRight),
    enemyKingBottom,
    getLeftSquare(enemyKingBottom),
    enemyKingLeft,
    getTopSquare(enemyKingLeft),
  ]);

  return {
    regular: bitwiseAnd([
      bitwiseOr([
        top,
        getRightSquare(top),
        right,
        getBottomSquare(right),
        bottom,
        getLeftSquare(bottom),
        left,
        getTopSquare(left),
      ]),
      bitwiseNot(friendlyPieces),
      bitwiseNot(enemyKingSquares),
    ]),
    kingsideCastles:
      possibleCastles[
        isWhite ? Castle.WHITE_KINGSIDE : Castle.BLACK_KINGSIDE
      ] &&
      isNull(
        bitwiseAnd([
          allPieces,
          isWhite ? [0x00000000, 0x00000006] : [0x06000000, 0x00000000],
        ])
      ) &&
      isNull(
        bitwiseAnd([
          captureSquares,
          isWhite ? [0x00000000, 0x0000000e] : [0x0e000000, 0x00000000],
        ])
      )
        ? isWhite
          ? [0x00000000, 0x00000002]
          : [0x02000000, 0x00000000]
        : [0x00000000, 0x00000000],
    queensideCastles:
      possibleCastles[
        isWhite ? Castle.WHITE_QUEENSIDE : Castle.BLACK_QUEENSIDE
      ] &&
      isNull(
        bitwiseAnd([
          allPieces,
          isWhite ? [0x00000000, 0x00000070] : [0x70000000, 0x00000000],
        ])
      ) &&
      isNull(
        bitwiseAnd([
          captureSquares,
          isWhite ? [0x00000000, 0x00000038] : [0x38000000, 0x00000000],
        ])
      )
        ? isWhite
          ? [0x00000000, 0x00000020]
          : [0x20000000, 0x00000000]
        : [0x00000000, 0x00000000],
  };
}

function getMoveableSquaresForQueen(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  queen: Bitboard
) {
  return bitwiseAnd([
    bitwiseOr([
      getMoveableSqaresToLeft(allPieces, enemyPieces, queen),
      getMoveableSqaresToRight(allPieces, enemyPieces, queen),
      getMoveableSqaresToTop(allPieces, enemyPieces, queen),
      getMoveableSqaresToBottom(allPieces, enemyPieces, queen),
      getMoveableSqaresToTopLeft(allPieces, enemyPieces, queen),
      getMoveableSqaresToTopRight(allPieces, enemyPieces, queen),
      getMoveableSqaresToBottomLeft(allPieces, enemyPieces, queen),
      getMoveableSqaresToBottomRight(allPieces, enemyPieces, queen),
    ]),
    bitwiseNot(queen),
  ]);
}

function getMoveableSquaresForRook(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  rook: Bitboard
) {
  return bitwiseAnd([
    bitwiseOr([
      getMoveableSqaresToLeft(allPieces, enemyPieces, rook),
      getMoveableSqaresToRight(allPieces, enemyPieces, rook),
      getMoveableSqaresToTop(allPieces, enemyPieces, rook),
      getMoveableSqaresToBottom(allPieces, enemyPieces, rook),
    ]),
    bitwiseNot(rook),
  ]);
}

function getMoveableSquaresForBishop(
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  bishop: Bitboard
) {
  return bitwiseAnd([
    bitwiseOr([
      getMoveableSqaresToTopLeft(allPieces, enemyPieces, bishop),
      getMoveableSqaresToTopRight(allPieces, enemyPieces, bishop),
      getMoveableSqaresToBottomLeft(allPieces, enemyPieces, bishop),
      getMoveableSqaresToBottomRight(allPieces, enemyPieces, bishop),
    ]),
    bitwiseNot(bishop),
  ]);
}

function getMoveableSquaresForKnight(
  friendlyPieces: Bitboard,
  knight: Bitboard
) {
  const top = getTopSquare(getTopSquare(knight));
  const bottom = getBottomSquare(getBottomSquare(knight));
  const left = getLeftSquare(getLeftSquare(knight));
  const right = getRightSquare(getRightSquare(knight));
  return bitwiseAnd([
    bitwiseOr([
      getLeftSquare(top),
      getRightSquare(top),
      getLeftSquare(bottom),
      getRightSquare(bottom),
      getTopSquare(left),
      getBottomSquare(left),
      getTopSquare(right),
      getBottomSquare(right),
    ]),
    bitwiseOr([bitwiseNot(friendlyPieces), knight]),
  ]);
}

function getCaptureSquaresForPawn(
  isWhite: boolean,
  friendlyPieces: Bitboard,
  pawn: Bitboard
) {
  const forward = isWhite ? getBottomSquare(pawn) : getTopSquare(pawn);
  return bitwiseAnd([
    bitwiseOr([getLeftSquare(forward), getRightSquare(forward)]),
    bitwiseOr([bitwiseNot(friendlyPieces), pawn]),
  ]);
}

function getMoveableSquaresForPawn(
  isWhite: boolean,
  allPieces: Bitboard,
  enemyPieces: Bitboard,
  enPassantSquare: Bitboard,
  pawn: Bitboard
) {
  const forward = isWhite ? getTopSquare : getBottomSquare;
  const oneForward = forward(pawn);
  const twoForward = forward(oneForward);
  const freeSquares = bitwiseNot(allPieces);
  const single = bitwiseOr([
    // Move one square forward
    bitwiseAnd([oneForward, freeSquares]),
    // Taking to the left
    bitwiseAnd([getLeftSquare(oneForward), enemyPieces]),
    // Taking to the right
    bitwiseAnd([getRightSquare(oneForward), enemyPieces]),
  ]);
  return {
    single: bitwiseAnd([
      single,
      isWhite ? [0x00ffffff, 0xffffffff] : [0xffffffff, 0xffffff00],
    ]),
    // Move two squares forward (only when landing on the fourth / fifth rank)
    double: bitwiseAnd([
      twoForward,
      freeSquares,
      isWhite ? getTopSquare(freeSquares) : getBottomSquare(freeSquares),
      isWhite ? [0x00000000, 0xff000000] : [0x000000ff, 0x00000000],
    ]),
    enPassant: bitwiseOr([
      // Taking en passant to the left
      bitwiseAnd([getLeftSquare(oneForward), enPassantSquare]),
      // Taking en passant to the right
      bitwiseAnd([getRightSquare(oneForward), enPassantSquare]),
    ]),
    promotion: bitwiseAnd([
      single,
      isWhite ? [0xff000000, 0x00000000] : [0x00000000, 0x000000ff],
    ]),
  };
}

function getCaptureSquares(position: Position, isWhite: boolean) {
  const whitePieces = bitwiseOr([
    position[Piece.WHITE_KING],
    position[Piece.WHITE_QUEEN],
    position[Piece.WHITE_ROOK],
    position[Piece.WHITE_BISHOP],
    position[Piece.WHITE_KNIGHT],
    position[Piece.WHITE_PAWN],
  ]);
  const blackPieces = bitwiseOr([
    position[Piece.BLACK_KING],
    position[Piece.BLACK_QUEEN],
    position[Piece.BLACK_ROOK],
    position[Piece.BLACK_BISHOP],
    position[Piece.BLACK_KNIGHT],
    position[Piece.BLACK_PAWN],
  ]);
  const allPieces = bitwiseOr([whitePieces, blackPieces]);
  // If white is on turn, we check if black is checking white
  const friendlyPieces = isWhite ? blackPieces : whitePieces;
  const enemyPieces = isWhite ? whitePieces : blackPieces;
  return bitwiseOr([
    getMoveableSquaresForQueen(
      allPieces,
      enemyPieces,
      position[isWhite ? Piece.BLACK_QUEEN : Piece.WHITE_QUEEN]
    ),
    getMoveableSquaresForRook(
      allPieces,
      enemyPieces,
      position[isWhite ? Piece.BLACK_ROOK : Piece.WHITE_ROOK]
    ),
    getMoveableSquaresForBishop(
      allPieces,
      enemyPieces,
      position[isWhite ? Piece.BLACK_BISHOP : Piece.WHITE_BISHOP]
    ),
    getMoveableSquaresForKnight(
      friendlyPieces,
      position[isWhite ? Piece.BLACK_KNIGHT : Piece.WHITE_KNIGHT]
    ),
    getCaptureSquaresForPawn(
      isWhite,
      friendlyPieces,
      position[isWhite ? Piece.BLACK_PAWN : Piece.WHITE_PAWN]
    ),
  ]);
}

function isInCheck(position: Position, isWhite: boolean) {
  const king = position[isWhite ? Piece.WHITE_KING : Piece.BLACK_KING];

  const enemyKnight =
    position[isWhite ? Piece.BLACK_KNIGHT : Piece.WHITE_KNIGHT];
  const top = getTopSquare(getTopSquare(king));
  const bottom = getBottomSquare(getBottomSquare(king));
  const left = getLeftSquare(getLeftSquare(king));
  const right = getRightSquare(getRightSquare(king));
  if (
    !isNull(
      bitwiseAnd([
        bitwiseOr([
          getLeftSquare(top),
          getRightSquare(top),
          getLeftSquare(bottom),
          getRightSquare(bottom),
          getTopSquare(left),
          getBottomSquare(left),
          getTopSquare(right),
          getBottomSquare(right),
        ]),
        enemyKnight,
      ])
    )
  ) {
    // Checked by knight
    return true;
  }

  const enemyPawn = position[isWhite ? Piece.BLACK_PAWN : Piece.WHITE_PAWN];
  const inDirection = isWhite ? getTopSquare(king) : getBottomSquare(king);
  if (
    !isNull(
      bitwiseAnd([
        bitwiseOr([getLeftSquare(inDirection), getRightSquare(inDirection)]),
        enemyPawn,
      ])
    )
  ) {
    // Checked by pawn
    return true;
  }

  const allPieces = bitwiseOr([
    position[Piece.WHITE_KING],
    position[Piece.WHITE_QUEEN],
    position[Piece.WHITE_ROOK],
    position[Piece.WHITE_BISHOP],
    position[Piece.WHITE_KNIGHT],
    position[Piece.WHITE_PAWN],
    position[Piece.BLACK_KING],
    position[Piece.BLACK_QUEEN],
    position[Piece.BLACK_ROOK],
    position[Piece.BLACK_BISHOP],
    position[Piece.BLACK_KNIGHT],
    position[Piece.BLACK_PAWN],
  ]);

  const enemyQueenAndRook = isWhite
    ? bitwiseOr([position[Piece.BLACK_QUEEN], position[Piece.BLACK_ROOK]])
    : bitwiseOr([position[Piece.WHITE_QUEEN], position[Piece.WHITE_ROOK]]);

  let currentTop = getTopSquare(king);
  while (!isNull(currentTop)) {
    if (!isNull(bitwiseAnd([currentTop, enemyQueenAndRook]))) {
      // Checked on file by queen or rook
      return true;
    }
    if (!isNull(bitwiseAnd([currentTop, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentTop = getTopSquare(currentTop);
  }

  let currentBottom = getBottomSquare(king);
  while (!isNull(currentBottom)) {
    if (!isNull(bitwiseAnd([currentBottom, enemyQueenAndRook]))) {
      // Checked on file by queen or rook
      return true;
    }
    if (!isNull(bitwiseAnd([currentBottom, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentBottom = getBottomSquare(currentBottom);
  }

  let currentLeft = getLeftSquare(king);
  while (!isNull(currentLeft)) {
    if (!isNull(bitwiseAnd([currentLeft, enemyQueenAndRook]))) {
      // Checked on rank by queen or rook
      return true;
    }
    if (!isNull(bitwiseAnd([currentLeft, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentLeft = getLeftSquare(currentLeft);
  }

  let currentRight = getRightSquare(king);
  while (!isNull(currentRight)) {
    if (!isNull(bitwiseAnd([currentRight, enemyQueenAndRook]))) {
      // Checked on rank by queen or rook
      return true;
    }
    if (!isNull(bitwiseAnd([currentRight, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentRight = getRightSquare(currentRight);
  }

  const enemyQueenAndBishop = isWhite
    ? bitwiseOr([position[Piece.BLACK_QUEEN], position[Piece.BLACK_BISHOP]])
    : bitwiseOr([position[Piece.WHITE_QUEEN], position[Piece.WHITE_BISHOP]]);

  let currentTopLeft = getLeftSquare(getTopSquare(king));
  while (!isNull(currentTopLeft)) {
    if (!isNull(bitwiseAnd([currentTopLeft, enemyQueenAndBishop]))) {
      // Checked on diagonal by queen or bishop
      return true;
    }
    if (!isNull(bitwiseAnd([currentTopLeft, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentTopLeft = getLeftSquare(getTopSquare(currentTopLeft));
  }

  let currentTopRight = getRightSquare(getTopSquare(king));
  while (!isNull(currentTopRight)) {
    if (!isNull(bitwiseAnd([currentTopRight, enemyQueenAndBishop]))) {
      // Checked on diagonal by queen or bishop
      return true;
    }
    if (!isNull(bitwiseAnd([currentTopRight, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentTopRight = getRightSquare(getTopSquare(currentTopRight));
  }

  let currentBottomLeft = getLeftSquare(getBottomSquare(king));
  while (!isNull(currentBottomLeft)) {
    if (!isNull(bitwiseAnd([currentBottomLeft, enemyQueenAndBishop]))) {
      // Checked on diagonal by queen or bishop
      return true;
    }
    if (!isNull(bitwiseAnd([currentBottomLeft, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentBottomLeft = getLeftSquare(getBottomSquare(currentBottomLeft));
  }

  let currentBottomRight = getRightSquare(getBottomSquare(king));
  while (!isNull(currentBottomRight)) {
    if (!isNull(bitwiseAnd([currentBottomRight, enemyQueenAndBishop]))) {
      // Checked on diagonal by queen or bishop
      return true;
    }
    if (!isNull(bitwiseAnd([currentBottomRight, allPieces]))) {
      // Some other piece, so break the loop
      break;
    }

    currentBottomRight = getRightSquare(getBottomSquare(currentBottomRight));
  }

  return false;
}

export function generateMoveId(
  from: Bitboard,
  to: Bitboard,
  castle: Castle | null,
  isPromotingTo: PromotionPiece | null
) {
  return [from[0], from[1], to[0], to[1], castle, isPromotingTo].join("-");
}

function movePiece(
  game: Game,
  movedPiece: Piece,
  from: Bitboard,
  to: Bitboard,
  enPassantSquare: Bitboard,
  isWhite: boolean,
  castle: Castle | null,
  isCapturingEnPassant: boolean,
  isPromotingTo: PromotionPiece | null
): { moveId: string; game: Game } {
  const newPosition = { ...game.position };

  switch (castle) {
    case Castle.WHITE_KINGSIDE: {
      newPosition[Piece.WHITE_KING] = [0x00000000, 0x00000002];
      newPosition[Piece.WHITE_ROOK] = bitwiseXor([
        newPosition[Piece.WHITE_ROOK],
        [0x00000000, 0x00000005],
      ]);
      const newGame: Game = {
        position: newPosition,
        player: Player.BLACK,
        lastMove: {
          piece: Piece.WHITE_KING,
          player: Player.WHITE,
          from: [0x00000000, 0x00000008],
          to: [0x00000000, 0x00000002],
          isCapturing: null,
          isCastling: Castle.WHITE_KINGSIDE,
          isPromotingTo: null,
        },
        possibleMoves: {},
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: false,
          [Castle.WHITE_QUEENSIDE]: false,
          [Castle.BLACK_KINGSIDE]: game.possibleCastles[Castle.BLACK_KINGSIDE],
          [Castle.BLACK_QUEENSIDE]:
            game.possibleCastles[Castle.BLACK_QUEENSIDE],
        },
        enPassantSquare: [0x00000000, 0x00000000],
        moveCounter: game.moveCounter,
        fiftyMoveCounter: game.fiftyMoveCounter + 1,
        positionCounts: game.positionCounts,
        result: null,
      };
      return {
        moveId: generateMoveId(
          [0x00000000, 0x00000008],
          [0x00000000, 0x00000002],
          Castle.WHITE_KINGSIDE,
          null
        ),
        game: incrementPositionCount(newGame),
      };
    }
    case Castle.WHITE_QUEENSIDE: {
      newPosition[Piece.WHITE_KING] = [0x00000000, 0x00000020];
      newPosition[Piece.WHITE_ROOK] = bitwiseXor([
        newPosition[Piece.WHITE_ROOK],
        [0x00000000, 0x00000090],
      ]);
      const newGame: Game = {
        position: newPosition,
        player: Player.BLACK,
        lastMove: {
          piece: Piece.WHITE_KING,
          player: Player.WHITE,
          from: [0x00000000, 0x00000008],
          to: [0x00000000, 0x00000020],
          isCapturing: null,
          isCastling: Castle.WHITE_QUEENSIDE,
          isPromotingTo: null,
        },
        possibleMoves: {},
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: false,
          [Castle.WHITE_QUEENSIDE]: false,
          [Castle.BLACK_KINGSIDE]: game.possibleCastles[Castle.BLACK_KINGSIDE],
          [Castle.BLACK_QUEENSIDE]:
            game.possibleCastles[Castle.BLACK_QUEENSIDE],
        },
        enPassantSquare: [0x00000000, 0x00000000],
        moveCounter: game.moveCounter,
        fiftyMoveCounter: game.fiftyMoveCounter + 1,
        positionCounts: game.positionCounts,
        result: null,
      };
      return {
        moveId: generateMoveId(
          [0x00000000, 0x00000008],
          [0x00000000, 0x00000020],
          Castle.WHITE_QUEENSIDE,
          null
        ),
        game: incrementPositionCount(newGame),
      };
    }
    case Castle.BLACK_KINGSIDE: {
      newPosition[Piece.BLACK_KING] = [0x02000000, 0x00000000];
      newPosition[Piece.BLACK_ROOK] = bitwiseXor([
        newPosition[Piece.BLACK_ROOK],
        [0x05000000, 0x00000000],
      ]);
      const newGame: Game = {
        position: newPosition,
        player: Player.WHITE,
        lastMove: {
          piece: Piece.BLACK_KING,
          player: Player.BLACK,
          from: [0x08000000, 0x00000000],
          to: [0x02000000, 0x00000000],
          isCapturing: null,
          isCastling: Castle.BLACK_KINGSIDE,
          isPromotingTo: null,
        },
        possibleMoves: {},
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: game.possibleCastles[Castle.WHITE_KINGSIDE],
          [Castle.WHITE_QUEENSIDE]:
            game.possibleCastles[Castle.WHITE_QUEENSIDE],
          [Castle.BLACK_KINGSIDE]: false,
          [Castle.BLACK_QUEENSIDE]: false,
        },
        enPassantSquare: [0x00000000, 0x00000000],
        moveCounter: game.moveCounter + 1,
        fiftyMoveCounter: game.fiftyMoveCounter + 1,
        positionCounts: game.positionCounts,
        result: null,
      };
      return {
        moveId: generateMoveId(
          [0x08000000, 0x00000000],
          [0x02000000, 0x00000000],
          Castle.BLACK_KINGSIDE,
          null
        ),
        game: incrementPositionCount(newGame),
      };
    }
    case Castle.BLACK_QUEENSIDE: {
      newPosition[Piece.BLACK_KING] = [0x20000000, 0x00000000];
      newPosition[Piece.BLACK_ROOK] = bitwiseXor([
        newPosition[Piece.BLACK_ROOK],
        [0x90000000, 0x00000000],
      ]);
      const newGame: Game = {
        position: newPosition,
        player: Player.WHITE,
        lastMove: {
          piece: Piece.BLACK_KING,
          player: Player.BLACK,
          from: [0x08000000, 0x00000000],
          to: [0x20000000, 0x00000000],
          isCapturing: null,
          isCastling: Castle.BLACK_QUEENSIDE,
          isPromotingTo: null,
        },
        possibleMoves: {},
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: game.possibleCastles[Castle.WHITE_KINGSIDE],
          [Castle.WHITE_QUEENSIDE]:
            game.possibleCastles[Castle.WHITE_QUEENSIDE],
          [Castle.BLACK_KINGSIDE]: false,
          [Castle.BLACK_QUEENSIDE]: false,
        },
        enPassantSquare: [0x00000000, 0x00000000],
        moveCounter: game.moveCounter + 1,
        fiftyMoveCounter: game.fiftyMoveCounter + 1,
        positionCounts: game.positionCounts,
        result: null,
      };
      return {
        moveId: generateMoveId(
          [0x08000000, 0x00000000],
          [0x20000000, 0x00000000],
          Castle.BLACK_QUEENSIDE,
          null
        ),
        game: incrementPositionCount(newGame),
      };
    }
  }

  const capturedPiece = isNull(
    bitwiseAnd([to, game.position[Piece.BLACK_PAWN]])
  )
    ? isNull(bitwiseAnd([to, game.position[Piece.WHITE_PAWN]]))
      ? isNull(bitwiseAnd([to, game.position[Piece.BLACK_KNIGHT]]))
        ? isNull(bitwiseAnd([to, game.position[Piece.WHITE_KNIGHT]]))
          ? isNull(bitwiseAnd([to, game.position[Piece.BLACK_BISHOP]]))
            ? isNull(bitwiseAnd([to, game.position[Piece.WHITE_BISHOP]]))
              ? isNull(bitwiseAnd([to, game.position[Piece.BLACK_ROOK]]))
                ? isNull(bitwiseAnd([to, game.position[Piece.WHITE_ROOK]]))
                  ? isNull(bitwiseAnd([to, game.position[Piece.BLACK_QUEEN]]))
                    ? isNull(bitwiseAnd([to, game.position[Piece.WHITE_QUEEN]]))
                      ? null
                      : Piece.WHITE_QUEEN
                    : Piece.BLACK_QUEEN
                  : Piece.WHITE_ROOK
                : Piece.BLACK_ROOK
              : Piece.WHITE_BISHOP
            : Piece.BLACK_BISHOP
          : Piece.WHITE_KNIGHT
        : Piece.BLACK_KNIGHT
      : Piece.WHITE_PAWN
    : Piece.BLACK_PAWN;

  newPosition[movedPiece] = bitwiseOr([
    bitwiseAnd([newPosition[movedPiece], bitwiseNot(from)]),
    to,
  ]);

  if (capturedPiece) {
    newPosition[capturedPiece] = bitwiseAnd([
      newPosition[capturedPiece],
      bitwiseNot(to),
    ]);
  }
  if (isCapturingEnPassant) {
    const capturedPawnPiece = isWhite ? Piece.BLACK_PAWN : Piece.WHITE_PAWN;
    const getSquareInDirection = isWhite ? getBottomSquare : getTopSquare;
    newPosition[capturedPawnPiece] = bitwiseAnd([
      newPosition[capturedPawnPiece],
      bitwiseNot(getSquareInDirection(to)),
    ]);
  }
  if (isPromotingTo) {
    const pawnPiece = isWhite ? Piece.WHITE_PAWN : Piece.BLACK_PAWN;
    newPosition[isPromotingTo] = bitwiseOr([newPosition[isPromotingTo], to]);
    newPosition[pawnPiece] = bitwiseXor([newPosition[pawnPiece], to]);
  }

  const newGame: Game = {
    position: newPosition,
    player: isWhite ? Player.BLACK : Player.WHITE,
    lastMove: {
      piece: movedPiece,
      from,
      to,
      player: isWhite ? Player.WHITE : Player.BLACK,
      isCapturing:
        capturedPiece ||
        (isCapturingEnPassant
          ? isWhite
            ? Piece.BLACK_PAWN
            : Piece.WHITE_PAWN
          : null),
      isCastling: null,
      isPromotingTo,
    },
    possibleMoves: {},
    possibleCastles: {
      [Castle.WHITE_KINGSIDE]:
        game.possibleCastles[Castle.WHITE_KINGSIDE] &&
        movedPiece !== Piece.WHITE_KING &&
        !(
          movedPiece === Piece.WHITE_ROOK &&
          equals([from, [0x00000000, 0x00000001]])
        ) &&
        !(
          capturedPiece === Piece.WHITE_ROOK &&
          equals([to, [0x00000000, 0x00000001]])
        ),
      [Castle.WHITE_QUEENSIDE]:
        game.possibleCastles[Castle.WHITE_QUEENSIDE] &&
        movedPiece !== Piece.WHITE_KING &&
        !(
          movedPiece === Piece.WHITE_ROOK &&
          equals([from, [0x00000000, 0x00000080]])
        ) &&
        !(
          capturedPiece === Piece.WHITE_ROOK &&
          equals([to, [0x00000000, 0x00000080]])
        ),
      [Castle.BLACK_KINGSIDE]:
        game.possibleCastles[Castle.BLACK_KINGSIDE] &&
        movedPiece !== Piece.BLACK_KING &&
        !(
          movedPiece === Piece.BLACK_ROOK &&
          equals([from, [0x01000000, 0x00000000]])
        ) &&
        !(
          capturedPiece === Piece.BLACK_ROOK &&
          equals([to, [0x01000000, 0x00000000]])
        ),
      [Castle.BLACK_QUEENSIDE]:
        game.possibleCastles[Castle.BLACK_QUEENSIDE] &&
        movedPiece !== Piece.BLACK_KING &&
        !(
          movedPiece === Piece.BLACK_ROOK &&
          equals([from, [0x80000000, 0x00000000]])
        ) &&
        !(
          capturedPiece === Piece.BLACK_ROOK &&
          equals([to, [0x80000000, 0x00000000]])
        ),
    },
    enPassantSquare,
    moveCounter: game.moveCounter + (isWhite ? 0 : 1),
    fiftyMoveCounter:
      movedPiece === Piece.WHITE_PAWN ||
      movedPiece === Piece.BLACK_PAWN ||
      capturedPiece ||
      isCapturingEnPassant
        ? 0
        : game.fiftyMoveCounter + 1,
    positionCounts: game.positionCounts,
    result: null,
  };
  return {
    moveId: generateMoveId(from, to, null, isPromotingTo),
    game: incrementPositionCount(newGame),
  };
}

export function getLegalMoves(game: Game): { [moveId: string]: Game } {
  const isWhite = game.player === Player.WHITE;

  const whitePieces = bitwiseOr([
    game.position[Piece.WHITE_KING],
    game.position[Piece.WHITE_QUEEN],
    game.position[Piece.WHITE_ROOK],
    game.position[Piece.WHITE_BISHOP],
    game.position[Piece.WHITE_KNIGHT],
    game.position[Piece.WHITE_PAWN],
  ]);
  const blackPieces = bitwiseOr([
    game.position[Piece.BLACK_KING],
    game.position[Piece.BLACK_QUEEN],
    game.position[Piece.BLACK_ROOK],
    game.position[Piece.BLACK_BISHOP],
    game.position[Piece.BLACK_KNIGHT],
    game.position[Piece.BLACK_PAWN],
  ]);
  const allPieces = bitwiseOr([whitePieces, blackPieces]);
  const friendlyPieces = isWhite ? whitePieces : blackPieces;
  const enemyPieces = isWhite ? blackPieces : whitePieces;

  const possibleGames: { [moveId: string]: Game } = {};

  const pawnPiece = isWhite ? Piece.WHITE_PAWN : Piece.BLACK_PAWN;
  const pawns = split(game.position[pawnPiece]);
  for (const from of pawns) {
    const { single, double, enPassant, promotion } = getMoveableSquaresForPawn(
      isWhite,
      allPieces,
      enemyPieces,
      game.enPassantSquare,
      from
    );
    for (const to of split(single)) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
    for (const to of split(double)) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        isWhite ? getBottomSquare(to) : getTopSquare(to),
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
    for (const to of split(enPassant)) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        true,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
    for (const to of split(promotion)) {
      const {
        game: updatedGameWithQueenPromotion,
        moveId: moveIdWithQueenPromotion,
      } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        isWhite ? Piece.WHITE_QUEEN : Piece.BLACK_QUEEN
      );
      if (!isInCheck(updatedGameWithQueenPromotion.position, isWhite)) {
        possibleGames[moveIdWithQueenPromotion] = updatedGameWithQueenPromotion;
      }

      const {
        game: updatedGameWithRookPromotion,
        moveId: moveIdWithRookPromotion,
      } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        isWhite ? Piece.WHITE_ROOK : Piece.BLACK_ROOK
      );
      if (!isInCheck(updatedGameWithRookPromotion.position, isWhite)) {
        possibleGames[moveIdWithRookPromotion] = updatedGameWithRookPromotion;
      }

      const {
        game: updatedGameWithBishopPromotion,
        moveId: moveIdWithBishopPromotion,
      } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        isWhite ? Piece.WHITE_BISHOP : Piece.BLACK_BISHOP
      );
      if (!isInCheck(updatedGameWithBishopPromotion.position, isWhite)) {
        possibleGames[moveIdWithBishopPromotion] =
          updatedGameWithBishopPromotion;
      }

      const {
        game: updatedGameWithKnightPromotion,
        moveId: moveIdWithKnightPromotion,
      } = movePiece(
        game,
        pawnPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        isWhite ? Piece.WHITE_KNIGHT : Piece.BLACK_KNIGHT
      );
      if (!isInCheck(updatedGameWithKnightPromotion.position, isWhite)) {
        possibleGames[moveIdWithKnightPromotion] =
          updatedGameWithKnightPromotion;
      }
    }
  }

  const knightPiece = isWhite ? Piece.WHITE_KNIGHT : Piece.BLACK_KNIGHT;
  const knights = split(game.position[knightPiece]);
  for (const from of knights) {
    const possibleMoves = split(
      getMoveableSquaresForKnight(friendlyPieces, from)
    );
    for (const to of possibleMoves) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        knightPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
  }

  const bishopPiece = isWhite ? Piece.WHITE_BISHOP : Piece.BLACK_BISHOP;
  const bishops = split(game.position[bishopPiece]);
  for (const from of bishops) {
    const possibleMoves = split(
      getMoveableSquaresForBishop(allPieces, enemyPieces, from)
    );
    for (const to of possibleMoves) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        bishopPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
  }

  const rookPiece = isWhite ? Piece.WHITE_ROOK : Piece.BLACK_ROOK;
  const rooks = split(game.position[rookPiece]);
  for (const from of rooks) {
    const possibleMoves = split(
      getMoveableSquaresForRook(allPieces, enemyPieces, from)
    );
    for (const to of possibleMoves) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        rookPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
  }

  const queenPiece = isWhite ? Piece.WHITE_QUEEN : Piece.BLACK_QUEEN;
  const queens = split(game.position[queenPiece]);
  for (const from of queens) {
    const possibleMoves = split(
      getMoveableSquaresForQueen(allPieces, enemyPieces, from)
    );
    for (const to of possibleMoves) {
      const { game: updatedGame, moveId } = movePiece(
        game,
        queenPiece,
        from,
        to,
        [0x00000000, 0x00000000],
        isWhite,
        null,
        false,
        null
      );
      if (!isInCheck(updatedGame.position, isWhite)) {
        possibleGames[moveId] = updatedGame;
      }
    }
  }

  const kingPiece = isWhite ? Piece.WHITE_KING : Piece.BLACK_KING;
  const king = game.position[kingPiece];
  const { regular, kingsideCastles, queensideCastles } =
    getMoveableSquaresForKing(
      allPieces,
      friendlyPieces,
      getCaptureSquares(game.position, isWhite),
      king,
      game.position[isWhite ? Piece.BLACK_KING : Piece.WHITE_KING],
      isWhite,
      game.possibleCastles
    );
  for (const to of split(regular)) {
    const { game: updatedGame, moveId } = movePiece(
      game,
      kingPiece,
      king,
      to,
      [0x00000000, 0x00000000],
      isWhite,
      null,
      false,
      null
    );
    if (!isInCheck(updatedGame.position, isWhite)) {
      possibleGames[moveId] = updatedGame;
    }
  }
  if (!isNull(kingsideCastles)) {
    const { game: updatedGame, moveId } = movePiece(
      game,
      kingPiece,
      king,
      kingsideCastles,
      [0x00000000, 0x00000000],
      isWhite,
      isWhite ? Castle.WHITE_KINGSIDE : Castle.BLACK_KINGSIDE,
      false,
      null
    );
    // We already checked that it's not check
    possibleGames[moveId] = updatedGame;
  }
  if (!isNull(queensideCastles)) {
    const { game: updatedGame, moveId } = movePiece(
      game,
      kingPiece,
      king,
      queensideCastles,
      [0x00000000, 0x00000000],
      isWhite,
      isWhite ? Castle.WHITE_QUEENSIDE : Castle.BLACK_QUEENSIDE,
      false,
      null
    );
    // We already checked that it's not check
    possibleGames[moveId] = updatedGame;
  }

  return possibleGames;
}

function isDeadPosition(position: Position) {
  const whiteQueens = split(position[Piece.WHITE_QUEEN]);
  const whiteRooks = split(position[Piece.WHITE_ROOK]);
  const whiteBishops = split(position[Piece.WHITE_BISHOP]);
  const whiteKnights = split(position[Piece.WHITE_KNIGHT]);
  const whitePawns = split(position[Piece.WHITE_PAWN]);
  const blackQueens = split(position[Piece.BLACK_QUEEN]);
  const blackRooks = split(position[Piece.BLACK_ROOK]);
  const blackBishops = split(position[Piece.BLACK_BISHOP]);
  const blackKnights = split(position[Piece.BLACK_KNIGHT]);
  const blackPawns = split(position[Piece.BLACK_PAWN]);

  const numberOfWhitePieces =
    whiteQueens.length +
    whiteRooks.length +
    whiteBishops.length +
    whiteKnights.length +
    whitePawns.length;
  const numberOfBlackPieces =
    blackQueens.length +
    blackRooks.length +
    blackBishops.length +
    blackKnights.length +
    blackPawns.length;

  // king against king
  if (numberOfWhitePieces + numberOfBlackPieces === 0) {
    return true;
  }

  // king against king and bishop
  if (
    numberOfWhitePieces === 0 &&
    numberOfBlackPieces === 1 &&
    blackBishops.length === 1
  ) {
    return true;
  }
  if (
    numberOfBlackPieces === 0 &&
    numberOfWhitePieces === 1 &&
    whiteBishops.length === 1
  ) {
    return true;
  }

  // king against king and knight
  if (
    numberOfWhitePieces === 0 &&
    numberOfBlackPieces === 1 &&
    blackKnights.length === 1
  ) {
    return true;
  }
  if (
    numberOfBlackPieces === 0 &&
    numberOfWhitePieces === 1 &&
    whiteKnights.length === 1
  ) {
    return true;
  }

  // king and bishop against king and bishop, with both bishops on squares of the same color
  if (
    numberOfWhitePieces === 1 &&
    numberOfBlackPieces === 1 &&
    whiteBishops.length === 1 &&
    blackBishops.length === 1
  ) {
    const isWhiteBishopOnWhiteSquare = isNull(
      bitwiseAnd([whiteBishops[0], [0xaa55aa55, 0xaa55aa55]])
    );
    const isBlackBishopOnWhiteSquare = isNull(
      bitwiseAnd([blackBishops[0], [0xaa55aa55, 0xaa55aa55]])
    );
    return isWhiteBishopOnWhiteSquare === isBlackBishopOnWhiteSquare;
  }

  return false;
}

export function setGameResult(game: Game) {
  game.possibleMoves = getLegalMoves(game);

  if (Object.keys(game.possibleMoves).length === 0) {
    const isWhite = game.player === Player.WHITE;
    if (isInCheck(game.position, isWhite)) {
      game.result = isWhite ? Result.BLACK : Result.WHITE;
    } else {
      game.result = Result.STALEMATE;
    }
    return game;
  }

  if (isDeadPosition(game.position)) {
    game.result = Result.DEAD_POSITION;
    return game;
  }

  for (const key in game.positionCounts) {
    if (game.positionCounts[key] >= 3) {
      game.result = Result.REPITITION;
      return game;
    }
  }

  if (game.fiftyMoveCounter >= 100) {
    game.result = Result.FIFTY_MOVE_RULE;
    return game;
  }

  return game;
}
