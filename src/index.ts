import { Bitboard, bitwiseAnd, equals, isNull } from "./bitboard";
import * as engines from "./engines";
import { PickMove } from "./engines/type";
import { gameFromFen } from "./fen";
import {
  Piece,
  Player,
  PromotionPiece,
  setGameResult,
  squares,
} from "./move-generator";

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

const squareElements = [
  [
    document.getElementById("a8")!,
    document.getElementById("b8")!,
    document.getElementById("c8")!,
    document.getElementById("d8")!,
    document.getElementById("e8")!,
    document.getElementById("f8")!,
    document.getElementById("g8")!,
    document.getElementById("h8")!,
  ],
  [
    document.getElementById("a7")!,
    document.getElementById("b7")!,
    document.getElementById("c7")!,
    document.getElementById("d7")!,
    document.getElementById("e7")!,
    document.getElementById("f7")!,
    document.getElementById("g7")!,
    document.getElementById("h7")!,
  ],
  [
    document.getElementById("a6")!,
    document.getElementById("b6")!,
    document.getElementById("c6")!,
    document.getElementById("d6")!,
    document.getElementById("e6")!,
    document.getElementById("f6")!,
    document.getElementById("g6")!,
    document.getElementById("h6")!,
  ],
  [
    document.getElementById("a5")!,
    document.getElementById("b5")!,
    document.getElementById("c5")!,
    document.getElementById("d5")!,
    document.getElementById("e5")!,
    document.getElementById("f5")!,
    document.getElementById("g5")!,
    document.getElementById("h5")!,
  ],
  [
    document.getElementById("a4")!,
    document.getElementById("b4")!,
    document.getElementById("c4")!,
    document.getElementById("d4")!,
    document.getElementById("e4")!,
    document.getElementById("f4")!,
    document.getElementById("g4")!,
    document.getElementById("h4")!,
  ],
  [
    document.getElementById("a3")!,
    document.getElementById("b3")!,
    document.getElementById("c3")!,
    document.getElementById("d3")!,
    document.getElementById("e3")!,
    document.getElementById("f3")!,
    document.getElementById("g3")!,
    document.getElementById("h3")!,
  ],
  [
    document.getElementById("a2")!,
    document.getElementById("b2")!,
    document.getElementById("c2")!,
    document.getElementById("d2")!,
    document.getElementById("e2")!,
    document.getElementById("f2")!,
    document.getElementById("g2")!,
    document.getElementById("h2")!,
  ],
  [
    document.getElementById("a1")!,
    document.getElementById("b1")!,
    document.getElementById("c1")!,
    document.getElementById("d1")!,
    document.getElementById("e1")!,
    document.getElementById("f1")!,
    document.getElementById("g1")!,
    document.getElementById("h1")!,
  ],
];

const result = document.getElementById("result")!;

let selectedPiece: { element: HTMLElement | null; square: Bitboard | null } = {
  element: null,
  square: null,
};

const startingPositionFen =
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
let game = gameFromFen(startingPositionFen);

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
        return !isNull(bitwiseAnd([bitboard, game.position[piece]]))
          ? `${piece}-${++counters[piece]}`
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
      document.getElementById(`${piece}-${++count}`)!.style.display = "none";
    }
  });
}

function move(
  from: Bitboard,
  to: Bitboard,
  isPromotingTo: PromotionPiece | null
) {
  const legalMove = Object.values(game.possibleMoves).find(
    (possibleGame) =>
      possibleGame.lastMove &&
      equals([from, possibleGame.lastMove.from]) &&
      equals([to, possibleGame.lastMove.to]) &&
      isPromotingTo === possibleGame.lastMove.isPromotingTo
  );
  if (!legalMove) {
    return;
  }
  game = setGameResult(legalMove);

  if (selectedPiece.element) {
    selectedPiece.element.classList.remove("active");
    selectedPiece.element = null;
    selectedPiece.square = null;
  }

  squares.forEach((rank, rankIndex) => {
    rank.forEach((square, fileIndex) => {
      if (
        game.lastMove &&
        (equals([square, game.lastMove.from]) ||
          equals([square, game.lastMove.to]))
      ) {
        squareElements[rankIndex][fileIndex].classList.add("last-move");
      } else {
        squareElements[rankIndex][fileIndex].classList.remove("last-move");
      }
    });
  });

  drawBoard();

  if (game.result) {
    result.innerText = "The game is over: " + game.result;
  }
}

drawBoard();

// Click handlers for pieces
Object.values(Piece).forEach((pieceName) => {
  for (let count = 1; count <= maxPieces[pieceName]; count++) {
    const piece = document.getElementById(`${pieceName}-${count}`)!;
    piece.addEventListener("click", () => {
      if (game.result) {
        return;
      }

      const [rankString, fileString] = piece.style.gridArea.split(" / ");
      const rankIndex = parseInt(rankString) - 1;
      const fileIndex = parseInt(fileString) - 1;

      const doesPieceBelongToPlayer =
        (game.player === Player.WHITE &&
          pieceName === pieceName.toUpperCase()) ||
        (game.player === Player.BLACK && pieceName === pieceName.toLowerCase());
      if (doesPieceBelongToPlayer) {
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
        move(selectedPiece.square, squares[rankIndex][fileIndex], null);
      }
    });
  }
});

const fileIndexForFile: Record<string, number> = {
  a: 0,
  b: 1,
  c: 2,
  d: 3,
  e: 4,
  f: 5,
  g: 6,
  h: 7,
};

const rankIndexForRank: Record<string, number> = {
  1: 7,
  2: 6,
  3: 5,
  4: 4,
  5: 3,
  6: 2,
  7: 1,
  8: 0,
};

// Click handlers for empty squares
for (const element of document.getElementsByClassName("square")) {
  const [, file, rank] = element.id.match(/^([a-h])([1-8])$/)!;
  const fileIndex = fileIndexForFile[file];
  const rankIndex = rankIndexForRank[rank];
  const square = squares[rankIndex][fileIndex];
  element.addEventListener("click", () => {
    if (game.result || !selectedPiece.square) {
      return;
    }
    move(selectedPiece.square, square, null);
  });
}

function makeMove(pickMove: PickMove, humanPlayer?: Player) {
  if (game.result || Object.keys(game.possibleMoves).length === 0) {
    return;
  }

  if (game.player !== humanPlayer) {
    const nextGame = pickMove(game);
    if (nextGame.lastMove) {
      move(
        nextGame.lastMove.from,
        nextGame.lastMove.to,
        nextGame.lastMove.isPromotingTo
      );
    }
  }

  setTimeout(() => {
    makeMove(pickMove, humanPlayer);
  }, 100);
}

document.getElementById("button-reset")!.addEventListener("click", () => {
  game = gameFromFen(startingPositionFen);
  result.innerText = "";
  drawBoard();
});

document.getElementById("button-mack1")!.addEventListener("click", () => {
  makeMove(engines.mack1);
});

document.getElementById("button-mack1-white")!.addEventListener("click", () => {
  makeMove(engines.mack1, Player.WHITE);
});

document.getElementById("button-mack1-black")!.addEventListener("click", () => {
  makeMove(engines.mack1, Player.BLACK);
});

document.getElementById("button-mack2")!.addEventListener("click", () => {
  makeMove(engines.mack2);
});

document.getElementById("button-mack2-white")!.addEventListener("click", () => {
  makeMove(engines.mack2, Player.WHITE);
});

document.getElementById("button-mack2-black")!.addEventListener("click", () => {
  makeMove(engines.mack2, Player.BLACK);
});
