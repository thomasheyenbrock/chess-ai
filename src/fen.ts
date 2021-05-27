import { bitwiseOr } from "./bitboard";
import {
  Castle,
  Game,
  getLegalMoves,
  Piece,
  Player,
  setGameResult,
  squares,
} from "./move-generator";

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

export function gameFromFen(fen: string): Game {
  const [
    board,
    player,
    castles,
    enPassantSquare,
    fiftyMoveCounter,
    moveCounter,
  ] = fen.split(" ");
  const position: Game["position"] = {
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
  const game: Game = {
    position,
    player: player === "w" ? Player.WHITE : Player.BLACK,
    lastMove: null,
    possibleMoves: {},
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
    positionCounts: {},
    moveCounter: parseInt(moveCounter, 10),
    fiftyMoveCounter: parseInt(fiftyMoveCounter, 10),
    result: null,
  };

  game.possibleMoves = getLegalMoves(game);
  return setGameResult(game);
}

export function gameToString(game: Game) {
  return JSON.stringify({
    position: game.position,
    player: game.player,
    possibleCastles: game.possibleCastles,
    enPassantSquare: game.enPassantSquare,
  });
}
