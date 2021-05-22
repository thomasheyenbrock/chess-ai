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

type PromotionPiece =
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
  isPromotingTo: PromotionPiece | null;
};

export type Game = {
  position: Position;
  player: Player;
  moves: Move[];
  possibleCastles: Record<Castle, boolean>;
  enPassantSquare: Bitboard;
};

const mapRankToRankIndex: Record<string, number> = {
  1: 7,
  2: 6,
  3: 5,
  4: 4,
  5: 3,
  6: 2,
  7: 1,
  8: 0,
};

const mapRankIndexToRank: Record<string, string> = {
  7: "1",
  6: "2",
  5: "3",
  4: "4",
  3: "5",
  2: "6",
  1: "7",
  0: "8",
};

const mapFileToFileIndex: Record<string, number> = {
  a: 0,
  b: 1,
  c: 2,
  d: 3,
  e: 4,
  f: 5,
  g: 6,
  h: 7,
};

const mapFileIndexToFile: Record<string, string> = {
  0: "a",
  1: "b",
  2: "c",
  3: "d",
  4: "e",
  5: "f",
  6: "g",
  7: "h",
};

function squareToHumanNotation(square: Bitboard) {
  for (const rankIndex in squares) {
    for (const fileIndex in squares[rankIndex]) {
      if (equals([square, squares[rankIndex][fileIndex]])) {
        return mapFileIndexToFile[fileIndex] + mapRankIndexToRank[rankIndex];
      }
    }
  }
  return "?";
}

export function gameFromFen(fen: string): Game {
  const [
    board,
    player,
    castles,
    enPassantSquare,
    fiftyMoveCounter,
    totalMoveCounter,
  ] = fen.split(" ");
  const position: Position = {
    K: [0x00000000, 0x00000000],
    Q: [0x00000000, 0x00000000],
    R: [0x00000000, 0x00000000],
    B: [0x00000000, 0x00000000],
    N: [0x00000000, 0x00000000],
    P: [0x00000000, 0x00000000],
    k: [0x00000000, 0x00000000],
    q: [0x00000000, 0x00000000],
    r: [0x00000000, 0x00000000],
    b: [0x00000000, 0x00000000],
    n: [0x00000000, 0x00000000],
    p: [0x00000000, 0x00000000],
  };
  board.split("/").forEach((rank, rankIndex) => {
    let fileIndex = 0;
    while (rank !== "") {
      const piece = rank[0] as Piece;
      const emptySquares = parseInt(piece, 10);
      if (Number.isFinite(emptySquares)) {
        fileIndex += emptySquares;
      } else {
        position[piece] = bitwiseOr([
          position[piece],
          squares[rankIndex][fileIndex],
        ]);
        fileIndex += 1;
      }
      rank = rank.substr(1);
    }
  });
  const [enPassantFile, enPassantRank] = enPassantSquare.split("");
  return {
    position,
    player: player === "w" ? Player.WHITE : Player.BLACK,
    moves: [],
    possibleCastles: {
      [Castle.WHITE_KINGSIDE]: castles.includes("K"),
      [Castle.WHITE_QUEENSIDE]: castles.includes("Q"),
      [Castle.BLACK_KINGSIDE]: castles.includes("k"),
      [Castle.BLACK_QUEENSIDE]: castles.includes("q"),
    },
    enPassantSquare:
      enPassantSquare === "-"
        ? [0x00000000, 0x00000000]
        : squares[mapRankToRankIndex[enPassantRank]][
            mapFileToFileIndex[enPassantFile]
          ],
  };
}

function getMoveableSquaresForKing(
  allPieces: Bitboard,
  friendlyPieces: Bitboard,
  captureSquares: Bitboard,
  king: Bitboard,
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
  return !isNull(
    bitwiseAnd([
      getCaptureSquares(position, isWhite),
      position[isWhite ? Piece.WHITE_KING : Piece.BLACK_KING],
    ])
  );
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
): Game {
  const newPosition = { ...game.position };

  switch (castle) {
    case Castle.WHITE_KINGSIDE:
      newPosition[Piece.WHITE_KING] = [0x00000000, 0x00000002];
      newPosition[Piece.WHITE_ROOK] = bitwiseXor([
        newPosition[Piece.WHITE_ROOK],
        [0x00000000, 0x00000005],
      ]);
      return {
        position: newPosition,
        player: Player.BLACK,
        moves: [
          ...game.moves,
          {
            piece: Piece.WHITE_KING,
            player: Player.WHITE,
            from: [0x00000000, 0x00000008],
            to: [0x00000000, 0x00000002],
            isCapturing: null,
            isPromotingTo: null,
          },
        ],
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: false,
          [Castle.WHITE_QUEENSIDE]: false,
          [Castle.BLACK_KINGSIDE]: game.possibleCastles[Castle.BLACK_KINGSIDE],
          [Castle.BLACK_QUEENSIDE]:
            game.possibleCastles[Castle.BLACK_QUEENSIDE],
        },
        enPassantSquare: [0x00000000, 0x00000000],
      };
    case Castle.WHITE_QUEENSIDE:
      newPosition[Piece.WHITE_KING] = [0x00000000, 0x00000020];
      newPosition[Piece.WHITE_ROOK] = bitwiseXor([
        newPosition[Piece.WHITE_ROOK],
        [0x00000000, 0x00000090],
      ]);
      return {
        position: newPosition,
        player: Player.BLACK,
        moves: [
          ...game.moves,
          {
            piece: Piece.WHITE_KING,
            player: Player.WHITE,
            from: [0x00000000, 0x00000008],
            to: [0x00000000, 0x00000020],
            isCapturing: null,
            isPromotingTo: null,
          },
        ],
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: false,
          [Castle.WHITE_QUEENSIDE]: false,
          [Castle.BLACK_KINGSIDE]: game.possibleCastles[Castle.BLACK_KINGSIDE],
          [Castle.BLACK_QUEENSIDE]:
            game.possibleCastles[Castle.BLACK_QUEENSIDE],
        },
        enPassantSquare: [0x00000000, 0x00000000],
      };
    case Castle.BLACK_KINGSIDE:
      newPosition[Piece.BLACK_KING] = [0x02000000, 0x00000000];
      newPosition[Piece.BLACK_ROOK] = bitwiseXor([
        newPosition[Piece.BLACK_ROOK],
        [0x05000000, 0x00000000],
      ]);
      return {
        position: newPosition,
        player: Player.WHITE,
        moves: [
          ...game.moves,
          {
            piece: Piece.BLACK_KING,
            player: Player.BLACK,
            from: [0x08000000, 0x00000000],
            to: [0x02000000, 0x00000000],
            isCapturing: null,
            isPromotingTo: null,
          },
        ],
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: game.possibleCastles[Castle.WHITE_KINGSIDE],
          [Castle.WHITE_QUEENSIDE]:
            game.possibleCastles[Castle.WHITE_QUEENSIDE],
          [Castle.BLACK_KINGSIDE]: false,
          [Castle.BLACK_QUEENSIDE]: false,
        },
        enPassantSquare: [0x00000000, 0x00000000],
      };
    case Castle.BLACK_QUEENSIDE:
      newPosition[Piece.BLACK_KING] = [0x20000000, 0x00000000];
      newPosition[Piece.BLACK_ROOK] = bitwiseXor([
        newPosition[Piece.BLACK_ROOK],
        [0x90000000, 0x00000000],
      ]);
      return {
        position: newPosition,
        player: Player.WHITE,
        moves: [
          ...game.moves,
          {
            piece: Piece.BLACK_KING,
            player: Player.BLACK,
            from: [0x08000000, 0x00000000],
            to: [0x20000000, 0x00000000],
            isCapturing: null,
            isPromotingTo: null,
          },
        ],
        possibleCastles: {
          [Castle.WHITE_KINGSIDE]: game.possibleCastles[Castle.WHITE_KINGSIDE],
          [Castle.WHITE_QUEENSIDE]:
            game.possibleCastles[Castle.WHITE_QUEENSIDE],
          [Castle.BLACK_KINGSIDE]: false,
          [Castle.BLACK_QUEENSIDE]: false,
        },
        enPassantSquare: [0x00000000, 0x00000000],
      };
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

  return {
    position: newPosition,
    player: isWhite ? Player.BLACK : Player.WHITE,
    moves: [
      ...game.moves,
      {
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
        piece: movedPiece,
        isPromotingTo,
      },
    ],
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
  };
}

export function getLegalMoves(game: Game, depth: number = 1): Game[] {
  if (depth === 0) {
    return [game];
  }

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

  const possibleGames: Game[] = [];

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
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
      }
    }
    for (const to of split(double)) {
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
      }
    }
    for (const to of split(enPassant)) {
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
      }
    }
    for (const to of split(promotion)) {
      const updatedGameWithQueenPromotion = movePiece(
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
        possibleGames.push(updatedGameWithQueenPromotion);
      }

      const updatedGameWithRookPromotion = movePiece(
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
        possibleGames.push(updatedGameWithRookPromotion);
      }
      const updatedGameWithBishopPromotion = movePiece(
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
        possibleGames.push(updatedGameWithBishopPromotion);
      }
      const updatedGameWithKnightPromotion = movePiece(
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
        possibleGames.push(updatedGameWithKnightPromotion);
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
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
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
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
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
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
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
      const updatedGame = movePiece(
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
        possibleGames.push(updatedGame);
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
      isWhite,
      game.possibleCastles
    );
  for (const to of split(regular)) {
    const updatedGame = movePiece(
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
      possibleGames.push(updatedGame);
    }
  }
  if (!isNull(kingsideCastles)) {
    const updatedGame = movePiece(
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
    if (!isInCheck(updatedGame.position, isWhite)) {
      possibleGames.push(updatedGame);
    }
  }
  if (!isNull(queensideCastles)) {
    const updatedGame = movePiece(
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
    if (!isInCheck(updatedGame.position, isWhite)) {
      possibleGames.push(updatedGame);
    }
  }

  return possibleGames;
}

export function countLegalMoves(game: Game, depth: number = 1) {
  const possibleGames = getLegalMoves(game);

  let sum = 0;
  for (let i = 0; i < possibleGames.length; i++) {
    const next = getLegalMoves(possibleGames[i], depth - 1);
    // if (depth === 2) {
    //   console.log(
    //     squareToHumanNotation(moves[i].moves[0].from) +
    //       squareToHumanNotation(moves[i].moves[0].to),
    //     next
    //   );
    // }
    sum += next.length;
  }
  return sum;
}

const game: Game = gameFromFen(
  // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q2/1PPBBPpP/R3K2R b KQkq -"
);

// console.log(countLegalMoves(game, 1));
// console.log(countLegalMoves(game, 2));
// console.log(countLegalMoves(game, 3));
// console.log(countLegalMoves(game, 4));
// console.log(countLegalMoves(game, 5));
