import {
  Bitboard,
  bitwiseAnd,
  bitwiseNot,
  bitwiseOr,
  getBottomSquare,
  equals,
  isNull,
  getLeftSquare,
  getRightSquare,
  getTopSquare,
  getSquaresOnRank,
  getSquaresOnFile,
  getSquaresOnDecreasingDiagonal,
  getSquaresOnIncreasingDiagonal,
} from "./bitboard";

const squares: Bitboard[][] = [
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

enum Piece {
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

const maxPieces: Record<Piece, number> = {
  [Piece.WHITE_KING]: 1,
  [Piece.WHITE_QUEEN]: 9,
  [Piece.WHITE_ROOK]: 10,
  [Piece.WHITE_BISHOP]: 10,
  [Piece.WHITE_KNIGHT]: 10,
  [Piece.WHITE_PAWN]: 8,
  [Piece.BLACK_KING]: 1,
  [Piece.BLACK_QUEEN]: 9,
  [Piece.BLACK_ROOK]: 10,
  [Piece.BLACK_BISHOP]: 10,
  [Piece.BLACK_KNIGHT]: 10,
  [Piece.BLACK_PAWN]: 8,
};

type Position = Record<Piece, Bitboard>;

enum Player {
  WHITE = "w",
  BLACK = "b",
}

enum Castle {
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
  isCapture: boolean;
};

type Game = {
  position: Position;
  player: Player;
  moves: Move[];
  possibleCastles: Record<Castle, boolean>;
  enPassantSquare: Bitboard | null;
};

const initialPosition: Position = {
  K: [0x00000000, 0x00000008],
  Q: [0x00000000, 0x00000010],
  R: [0x00000000, 0x00000081],
  B: [0x00000000, 0x00000024],
  N: [0x00000000, 0x00000042],
  P: [0x00000000, 0x0000ff00],
  k: [0x08000000, 0x00000000],
  q: [0x10000000, 0x00000000],
  r: [0x81000000, 0x00000000],
  b: [0x24000000, 0x00000000],
  n: [0x42000000, 0x00000000],
  p: [0x00ff0000, 0x00000000],
};

const initialPossibleCastles = {
  [Castle.WHITE_KINGSIDE]: true,
  [Castle.WHITE_QUEENSIDE]: true,
  [Castle.BLACK_KINGSIDE]: true,
  [Castle.BLACK_QUEENSIDE]: true,
};

const game: Game = {
  position: initialPosition,
  player: Player.WHITE,
  moves: [],
  possibleCastles: initialPossibleCastles,
  enPassantSquare: null,
};

let selectedPiece: { element: HTMLElement | null; square: Bitboard | null } = {
  element: null,
  square: null,
};

function hasPieceOnSquare(pieceBitboard: Bitboard, square: Bitboard) {
  return (
    (pieceBitboard[0] & square[0]) !== 0 || (pieceBitboard[1] & square[1]) !== 0
  );
}

function getPieceOnSquare(square: Bitboard): Piece | null {
  for (const [piece, bitboard] of Object.entries(game.position)) {
    if (hasPieceOnSquare(bitboard, square)) {
      return piece as Piece;
    }
  }
  return null;
}

function doesPieceBelongToPlayer(piece: Piece, player: Player) {
  return (
    (player === Player.WHITE && piece === piece.toUpperCase()) ||
    (player === Player.BLACK && piece === piece.toLowerCase())
  );
}

function areNoPiecesOnSquares(bitboard: Bitboard) {
  return isNull(
    bitwiseAnd([bitboard, bitwiseOr(Object.values(game.position))])
  );
}

type ValidationResult = {
  isValid: boolean;
  removeCastling?: Castle[];
  enPassantSquare?: Bitboard;
};

function validateKingMove(from: Bitboard, to: Bitboard): ValidationResult {
  const topNeighbour = getTopSquare(from);
  const rightNeighbour = getRightSquare(from);
  const bottomNeighbour = getBottomSquare(from);
  const leftNeighbour = getLeftSquare(from);
  const neighbours = bitwiseOr([
    topNeighbour,
    rightNeighbour,
    bottomNeighbour,
    leftNeighbour,
    getRightSquare(topNeighbour),
    getBottomSquare(rightNeighbour),
    getLeftSquare(bottomNeighbour),
    getTopSquare(leftNeighbour),
  ]);
  const isInRange = !isNull(bitwiseAnd([to, neighbours]));
  if (isInRange) {
    return {
      isValid: true,
      removeCastling:
        game.player === Player.WHITE
          ? [Castle.WHITE_KINGSIDE, Castle.WHITE_QUEENSIDE]
          : [Castle.BLACK_KINGSIDE, Castle.BLACK_QUEENSIDE],
    };
  }

  if (
    game.player === Player.WHITE &&
    game.possibleCastles[Castle.WHITE_KINGSIDE] &&
    equals([from, squares[7][4]]) &&
    equals([to, squares[7][6]])
  ) {
    return {
      isValid: true,
      removeCastling: [Castle.WHITE_KINGSIDE, Castle.WHITE_QUEENSIDE],
    };
  }
  if (
    game.player === Player.WHITE &&
    game.possibleCastles[Castle.WHITE_QUEENSIDE] &&
    equals([from, squares[7][4]]) &&
    equals([to, squares[7][2]])
  ) {
    return {
      isValid: true,
      removeCastling: [Castle.WHITE_KINGSIDE, Castle.WHITE_QUEENSIDE],
    };
  }

  if (
    game.player === Player.BLACK &&
    game.possibleCastles[Castle.BLACK_KINGSIDE] &&
    equals([from, squares[0][4]]) &&
    equals([to, squares[0][6]])
  ) {
    return {
      isValid: true,
      removeCastling: [Castle.BLACK_KINGSIDE, Castle.BLACK_QUEENSIDE],
    };
  }
  if (
    game.player === Player.BLACK &&
    game.possibleCastles[Castle.BLACK_QUEENSIDE] &&
    equals([from, squares[0][4]]) &&
    equals([to, squares[0][2]])
  ) {
    return {
      isValid: true,
      removeCastling: [Castle.BLACK_KINGSIDE, Castle.BLACK_QUEENSIDE],
    };
  }

  return { isValid: false };
}

function validateQueenMove(from: Bitboard, to: Bitboard): ValidationResult {
  const isValidRookMove = validateRookMove(from, to);
  if (isValidRookMove.isValid) {
    return { isValid: isValidRookMove.isValid };
  }

  const isValidBishopMove = validateBishopMove(from, to);
  if (isValidBishopMove.isValid) {
    return { isValid: isValidBishopMove.isValid };
  }

  return { isValid: false };
}

function validateRookMove(from: Bitboard, to: Bitboard): ValidationResult {
  const squaresOnRank = getSquaresOnRank(from, to);
  if (squaresOnRank) {
    const isValid = areNoPiecesOnSquares(squaresOnRank);
    if (isValid) {
      if (game.player === Player.WHITE && equals([from, squares[7][7]])) {
        return { isValid, removeCastling: [Castle.WHITE_KINGSIDE] };
      }
      if (game.player === Player.WHITE && equals([from, squares[7][0]])) {
        return { isValid, removeCastling: [Castle.WHITE_QUEENSIDE] };
      }
      if (game.player === Player.BLACK && equals([from, squares[0][7]])) {
        return { isValid, removeCastling: [Castle.BLACK_QUEENSIDE] };
      }
      if (game.player === Player.BLACK && equals([from, squares[0][0]])) {
        return { isValid, removeCastling: [Castle.BLACK_QUEENSIDE] };
      }
      return { isValid: true };
    }
    return { isValid: false };
  }

  const squaresOnFile = getSquaresOnFile(from, to);
  if (squaresOnFile) {
    const isValid = areNoPiecesOnSquares(squaresOnFile);
    if (isValid) {
      if (game.player === Player.WHITE && equals([from, squares[7][7]])) {
        return { isValid, removeCastling: [Castle.WHITE_KINGSIDE] };
      }
      if (game.player === Player.WHITE && equals([from, squares[7][0]])) {
        return { isValid, removeCastling: [Castle.WHITE_QUEENSIDE] };
      }
      if (game.player === Player.BLACK && equals([from, squares[0][7]])) {
        return { isValid, removeCastling: [Castle.BLACK_QUEENSIDE] };
      }
      if (game.player === Player.BLACK && equals([from, squares[0][0]])) {
        return { isValid, removeCastling: [Castle.BLACK_QUEENSIDE] };
      }
      return { isValid: true };
    }
    return { isValid: false };
  }

  return { isValid: false };
}

function validateBishopMove(from: Bitboard, to: Bitboard): ValidationResult {
  const squaresOnDecreasingDiagonal = getSquaresOnDecreasingDiagonal(from, to);
  if (squaresOnDecreasingDiagonal) {
    return { isValid: areNoPiecesOnSquares(squaresOnDecreasingDiagonal) };
  }

  const squaresOnIncreasingDiagonal = getSquaresOnIncreasingDiagonal(from, to);
  if (squaresOnIncreasingDiagonal) {
    return { isValid: areNoPiecesOnSquares(squaresOnIncreasingDiagonal) };
  }

  return { isValid: false };
}

function validateKnightMove(from: Bitboard, to: Bitboard): ValidationResult {
  const top = getTopSquare(getTopSquare(from));
  const right = getRightSquare(getRightSquare(from));
  const bottom = getBottomSquare(getBottomSquare(from));
  const left = getLeftSquare(getLeftSquare(from));
  const reachableSquares = bitwiseOr([
    getRightSquare(top),
    getLeftSquare(top),
    getTopSquare(right),
    getBottomSquare(right),
    getRightSquare(bottom),
    getLeftSquare(bottom),
    getTopSquare(left),
    getBottomSquare(left),
  ]);
  return { isValid: !isNull(bitwiseAnd([to, reachableSquares])) };
}

function validatePawnMove(from: Bitboard, to: Bitboard): ValidationResult {
  const isWhite = game.player === Player.WHITE;
  const getSquareInDirection = isWhite ? getTopSquare : getBottomSquare;
  const oneMoveForward = getSquareInDirection(from);
  if (equals([oneMoveForward, to]) && !getPieceOnSquare(to)) {
    return { isValid: true };
  }
  if (
    equals([getSquareInDirection(oneMoveForward), to]) &&
    !isNull(
      bitwiseAnd([
        from,
        isWhite ? [0x00000000, 0x0000ff00] : [0x00ff0000, 0x00000000],
      ])
    ) &&
    !getPieceOnSquare(oneMoveForward) &&
    !getPieceOnSquare(to)
  ) {
    return { isValid: true, enPassantSquare: oneMoveForward };
  }

  // TODO: take other pawns
  // TODO: taking en passant

  return { isValid: false };
}

function validateMove(piece: Piece, from: Bitboard, to: Bitboard) {
  switch (piece) {
    case Piece.WHITE_KING:
    case Piece.BLACK_KING:
      return validateKingMove(from, to);
    case Piece.WHITE_QUEEN:
    case Piece.BLACK_QUEEN:
      return validateQueenMove(from, to);
    case Piece.WHITE_ROOK:
    case Piece.BLACK_ROOK:
      return validateRookMove(from, to);
    case Piece.WHITE_BISHOP:
    case Piece.BLACK_BISHOP:
      return validateBishopMove(from, to);
    case Piece.WHITE_KNIGHT:
    case Piece.BLACK_KNIGHT:
      return validateKnightMove(from, to);
    case Piece.WHITE_PAWN:
    case Piece.BLACK_PAWN:
      return validatePawnMove(from, to);
  }
}

function move(from: Bitboard, to: Bitboard) {
  if (equals([from, to])) {
    return;
  }

  const movedPiece = getPieceOnSquare(from);
  if (!movedPiece) {
    return;
  }

  if (!doesPieceBelongToPlayer(movedPiece, game.player)) {
    return;
  }

  const capturedPiece = getPieceOnSquare(to);
  if (capturedPiece && doesPieceBelongToPlayer(capturedPiece, game.player)) {
    return;
  }

  const { isValid, removeCastling, enPassantSquare } = validateMove(
    movedPiece,
    from,
    to
  );
  if (!isValid) {
    return;
  }

  // TODO: a move is not valid if the player would be in chess afterwards

  game.moves.push({
    from,
    to,
    player: game.player,
    piece: movedPiece,
    isCapture: Boolean(capturedPiece),
    // TODO: take record of pawn promotions
  });
  game.player = game.player === Player.WHITE ? Player.BLACK : Player.WHITE;
  if (removeCastling) {
    removeCastling.forEach((castling) => {
      game.possibleCastles[castling] = false;
    });
  }
  game.enPassantSquare = enPassantSquare || null;

  game.position[movedPiece] = bitwiseOr([
    bitwiseAnd([game.position[movedPiece], bitwiseNot(from)]),
    to,
  ]);

  if (capturedPiece) {
    game.position[capturedPiece] = bitwiseAnd([
      game.position[capturedPiece],
      bitwiseNot(to),
    ]);
  }

  // TODO: check for checkmate
  // TODO: check for a draw

  if (selectedPiece.element) {
    selectedPiece.element.classList.remove("active");
    selectedPiece.element = null;
    selectedPiece.square = null;
  }
  drawBoard();
}

Object.values(Piece).forEach((pieceName) => {
  for (let count = 1; count <= maxPieces[pieceName]; count++) {
    const piece = document.getElementById(`${pieceName}${count}`)!;
    piece.addEventListener("click", () => {
      const [rankString, fileString] = piece.style.gridArea.split(" / ");
      const rankIndex = parseInt(rankString) - 1;
      const fileIndex = parseInt(fileString) - 1;

      if (doesPieceBelongToPlayer(pieceName, game.player)) {
        if (selectedPiece.element) {
          selectedPiece.element.classList.remove("active");
        }
        if (selectedPiece.element === piece) {
          selectedPiece.element = null;
          selectedPiece.square = null;
        } else {
          piece.classList.add("active");
          selectedPiece.element = piece;
          selectedPiece.square = squares[rankIndex][fileIndex];
        }
        return;
      }

      if (selectedPiece.square) {
        move(selectedPiece.square, squares[rankIndex][fileIndex]);
      }
    });
  }
});

for (const element of document.getElementsByClassName("square")) {
  const rank = parseInt(element.className.match(/rank(\d+)/)![1]) - 1;
  const file = parseInt(element.className.match(/file(\d+)/)![1]) - 1;
  const square = squares[rank][file];
  element.addEventListener("click", () => {
    if (!selectedPiece.square) {
      return;
    }
    move(selectedPiece.square, square);
  });
}

function drawBoard() {
  const counters: Record<Piece, number> = {
    [Piece.WHITE_KING]: 0,
    [Piece.WHITE_QUEEN]: 0,
    [Piece.WHITE_ROOK]: 0,
    [Piece.WHITE_BISHOP]: 0,
    [Piece.WHITE_KNIGHT]: 0,
    [Piece.WHITE_PAWN]: 0,
    [Piece.BLACK_KING]: 0,
    [Piece.BLACK_QUEEN]: 0,
    [Piece.BLACK_ROOK]: 0,
    [Piece.BLACK_BISHOP]: 0,
    [Piece.BLACK_KNIGHT]: 0,
    [Piece.BLACK_PAWN]: 0,
  };
  squares.forEach((rank, rankIndex) => {
    rank.forEach((bitboard, fileIndex) => {
      const id = Object.values(Piece).reduce<string | null>((acc, piece) => {
        if (acc) {
          return acc;
        }
        return hasPieceOnSquare(bitboard, game.position[piece])
          ? `${piece}${++counters[piece]}`
          : null;
      }, null);

      if (!id) {
        return;
      }

      const element = document.getElementById(id)!;
      element.style.gridArea = `${rankIndex + 1} / ${fileIndex + 1} / ${
        rankIndex + 2
      } / ${fileIndex + 2}`;
      element.style.display = "block";
    });
  });

  Object.entries(counters).forEach(([piece, count]) => {
    while (count < maxPieces[piece as Piece]) {
      document.getElementById(`${piece}${++count}`)!.style.display = "none";
    }
  });
}

drawBoard();
