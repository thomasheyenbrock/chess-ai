import { Bitboard, equals } from "./bitboard";
import { gameFromFen } from "./fen";
import { Piece, Player, setGameResult, squares } from "./move-generator";

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

let selectedPiece: { element: HTMLElement | null; square: Bitboard | null } = {
  element: null,
  square: null,
};

function hasPieceOnSquare(pieceBitboard: Bitboard, square: Bitboard) {
  return (
    (pieceBitboard[0] & square[0]) !== 0 || (pieceBitboard[1] & square[1]) !== 0
  );
}

function doesPieceBelongToPlayer(piece: Piece, player: Player) {
  return (
    (player === Player.WHITE && piece === piece.toUpperCase()) ||
    (player === Player.BLACK && piece === piece.toLowerCase())
  );
}

function move(from: Bitboard, to: Bitboard) {
  const legalMove = game.possibleMoves.find((possibleGame) => {
    const lastMove = possibleGame.pastMoves[possibleGame.pastMoves.length - 1];
    return equals([from, lastMove.from]) && equals([to, lastMove.to]);
  });
  if (!legalMove) {
    return;
  }
  game = setGameResult(legalMove);

  if (selectedPiece.element) {
    selectedPiece.element.classList.remove("active");
    selectedPiece.element = null;
    selectedPiece.square = null;
  }

  drawBoard();

  if (game.result) {
    alert("game is over, result: " + game.result);
  }
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

let game = gameFromFen(
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
);

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
