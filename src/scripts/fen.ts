import { Bitboard, bitwiseAnd, bitwiseOr, equals, isNull } from "./bitboard";
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

const squareForHumanNotation: Record<string, Bitboard> = {
  a8: [0x8000_0000, 0x0000_0000],
  b8: [0x4000_0000, 0x0000_0000],
  c8: [0x2000_0000, 0x0000_0000],
  d8: [0x1000_0000, 0x0000_0000],
  e8: [0x0800_0000, 0x0000_0000],
  f8: [0x0400_0000, 0x0000_0000],
  g8: [0x0200_0000, 0x0000_0000],
  h8: [0x0100_0000, 0x0000_0000],
  a7: [0x0080_0000, 0x0000_0000],
  b7: [0x0040_0000, 0x0000_0000],
  c7: [0x0020_0000, 0x0000_0000],
  d7: [0x0010_0000, 0x0000_0000],
  e7: [0x0008_0000, 0x0000_0000],
  f7: [0x0004_0000, 0x0000_0000],
  g7: [0x0002_0000, 0x0000_0000],
  h7: [0x0001_0000, 0x0000_0000],
  a6: [0x0000_8000, 0x0000_0000],
  b6: [0x0000_4000, 0x0000_0000],
  c6: [0x0000_2000, 0x0000_0000],
  d6: [0x0000_1000, 0x0000_0000],
  e6: [0x0000_0800, 0x0000_0000],
  f6: [0x0000_0400, 0x0000_0000],
  g6: [0x0000_0200, 0x0000_0000],
  h6: [0x0000_0100, 0x0000_0000],
  a5: [0x0000_0080, 0x0000_0000],
  b5: [0x0000_0040, 0x0000_0000],
  c5: [0x0000_0020, 0x0000_0000],
  d5: [0x0000_0010, 0x0000_0000],
  e5: [0x0000_0008, 0x0000_0000],
  f5: [0x0000_0004, 0x0000_0000],
  g5: [0x0000_0002, 0x0000_0000],
  h5: [0x0000_0001, 0x0000_0000],
  a4: [0x0000_0000, 0x8000_0000],
  b4: [0x0000_0000, 0x4000_0000],
  c4: [0x0000_0000, 0x2000_0000],
  d4: [0x0000_0000, 0x1000_0000],
  e4: [0x0000_0000, 0x0800_0000],
  f4: [0x0000_0000, 0x0400_0000],
  g4: [0x0000_0000, 0x0200_0000],
  h4: [0x0000_0000, 0x0100_0000],
  a3: [0x0000_0000, 0x0080_0000],
  b3: [0x0000_0000, 0x0040_0000],
  c3: [0x0000_0000, 0x0020_0000],
  d3: [0x0000_0000, 0x0010_0000],
  e3: [0x0000_0000, 0x0008_0000],
  f3: [0x0000_0000, 0x0004_0000],
  g3: [0x0000_0000, 0x0002_0000],
  h3: [0x0000_0000, 0x0001_0000],
  a2: [0x0000_0000, 0x0000_8000],
  b2: [0x0000_0000, 0x0000_4000],
  c2: [0x0000_0000, 0x0000_2000],
  d2: [0x0000_0000, 0x0000_1000],
  e2: [0x0000_0000, 0x0000_0800],
  f2: [0x0000_0000, 0x0000_0400],
  g2: [0x0000_0000, 0x0000_0200],
  h2: [0x0000_0000, 0x0000_0100],
  a1: [0x0000_0000, 0x0000_0080],
  b1: [0x0000_0000, 0x0000_0040],
  c1: [0x0000_0000, 0x0000_0020],
  d1: [0x0000_0000, 0x0000_0010],
  e1: [0x0000_0000, 0x0000_0008],
  f1: [0x0000_0000, 0x0000_0004],
  g1: [0x0000_0000, 0x0000_0002],
  h1: [0x0000_0000, 0x0000_0001],
};

export function gameToFen(game: Game) {
  let board = "";
  for (let rank = 0; rank < 8; rank++) {
    for (let file = 0; file < 8; file++) {
      const square: Bitboard = [
        rank < 4 ? 0 : 2 ** ((rank - 4) * 8 + file),
        rank < 4 ? 2 ** (rank * 8 + file) : 0,
      ];
      if (!isNull(bitwiseAnd([square, game.position.K]))) {
        board = "K" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.Q]))) {
        board = "Q" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.R]))) {
        board = "R" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.B]))) {
        board = "B" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.N]))) {
        board = "N" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.P]))) {
        board = "P" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.k]))) {
        board = "k" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.q]))) {
        board = "q" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.r]))) {
        board = "r" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.b]))) {
        board = "b" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.n]))) {
        board = "n" + board;
      } else if (!isNull(bitwiseAnd([square, game.position.p]))) {
        board = "p" + board;
      } else {
        board = " " + board;
      }
    }
    if (rank < 7) {
      board = "/" + board;
    }
  }
  board = board.replace(/[ ]+/g, (match) => match.length.toString());

  return [
    board,
    game.player === Player.WHITE ? "w" : "b",
    [
      game.possibleCastles.K ? "K" : "",
      game.possibleCastles.Q ? "Q" : "",
      game.possibleCastles.k ? "k" : "",
      game.possibleCastles.q ? "q" : "",
    ].join(""),
    Object.entries(squareForHumanNotation).find(([, square]) =>
      equals([game.enPassantSquare, square])
    )?.[0] || "-",
    game.fiftyMoveCounter,
    game.moveCounter,
  ].join(" ");
}
