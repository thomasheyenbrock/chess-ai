/**
 * INPUT LAYER
 * - 1 bit for the current player (1 = white, 0 = black)
 * - 12 Bitboards (one for each piece) with 64 bits each
 * - 4 bits for each castling (1 = still possible, 0 = no longer possible)
 * - One bitboard with 64 bits showing the en passant square (if any)
 *   => 1 + 12 * 64 + 4 + 64 = 837 nodes
 *
 * OUTPUT LAYER
 *   => 1972 nodes (one for each possible move)
 *
 * HIDDEN LAYERS
 * - one hidden layer
 * - number of nodes is the arithmetic mean of input and output
 *   => (837 + 1972) / 2 = 1405 nodes
 */

import { Bitboard, bitwiseAnd, isNull } from "../../bitboard.ts";
import { gameFromFen } from "../../fen.ts";
import {
  createRandomMatrix,
  createRandomVector,
  Matrix,
  step,
  Vector,
} from "../../matrix.ts";
import {
  Castle,
  Game,
  Piece,
  Player,
  Result,
  setGameResult,
} from "../../move-generator.ts";
import {
  findPossibleMove,
  moveForOutputIndex,
} from "./output-layer-mapping.ts";

function getActivationsFromBitboard(bitboard: Bitboard) {
  return [
    isNull(bitwiseAnd([bitboard, [0x80000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x40000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x20000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x10000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x08000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x04000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x02000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x01000000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00800000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00400000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00200000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00100000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00080000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00040000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00020000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00010000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00008000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00004000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00002000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00001000, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000800, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000400, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000200, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000100, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000080, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000040, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000020, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000010, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000008, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000004, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000002, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000001, 0x00000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x80000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x40000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x20000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x10000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x08000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x04000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x02000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x01000000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00800000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00400000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00200000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00100000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00080000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00040000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00020000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00010000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00008000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00004000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00002000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00001000]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000800]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000400]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000200]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000100]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000080]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000040]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000020]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000010]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000008]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000004]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000002]])) ? 0 : 1,
    isNull(bitwiseAnd([bitboard, [0x00000000, 0x00000001]])) ? 0 : 1,
  ];
}

const t = new TextEncoder();

function printProgress(description: string, progress: number) {
  Deno.stdout.writeSync(
    t.encode(
      "\r\x1b[K" +
        description +
        ": " +
        Math.round(progress * 1000) / 10 +
        "%" +
        (progress >= 1 ? "\n" : "")
    )
  );
}

type Network = { id: string; W1: Matrix; b1: Vector; W2: Matrix; b2: Vector };

function generateRandomNetwork(id: string): Network {
  return {
    id,
    W1: createRandomMatrix(1405, 837),
    b1: createRandomVector(1405),
    W2: createRandomMatrix(1972, 1405),
    b2: createRandomVector(1972),
  };
}

export function pickMove(game: Game, network: Network) {
  const input = [
    // player
    game.player === Player.WHITE ? 1 : 0,
    // position
    ...getActivationsFromBitboard(game.position[Piece.WHITE_KING]),
    ...getActivationsFromBitboard(game.position[Piece.WHITE_QUEEN]),
    ...getActivationsFromBitboard(game.position[Piece.WHITE_ROOK]),
    ...getActivationsFromBitboard(game.position[Piece.WHITE_BISHOP]),
    ...getActivationsFromBitboard(game.position[Piece.WHITE_KNIGHT]),
    ...getActivationsFromBitboard(game.position[Piece.WHITE_PAWN]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_KING]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_QUEEN]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_ROOK]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_BISHOP]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_KNIGHT]),
    ...getActivationsFromBitboard(game.position[Piece.BLACK_PAWN]),
    // castles
    game.possibleCastles[Castle.WHITE_KINGSIDE] ? 1 : 0,
    game.possibleCastles[Castle.WHITE_QUEENSIDE] ? 1 : 0,
    game.possibleCastles[Castle.BLACK_KINGSIDE] ? 1 : 0,
    game.possibleCastles[Castle.BLACK_QUEENSIDE] ? 1 : 0,
    // en passant
    ...getActivationsFromBitboard(game.enPassantSquare),
  ];

  const hiddenLayer = step(input, network.W1, network.b1);
  const output = step(hiddenLayer, network.W2, network.b2);

  let max = -Infinity;
  const validGames: { score: number; game: Game }[] = [];
  for (let i = 0; i < output.length; i++) {
    let possibleGame = findPossibleMove(game, moveForOutputIndex[i]);
    if (possibleGame) {
      const score = output[i];
      validGames.push({ score, game: possibleGame });
      max = Math.max(max, score);
    }
  }

  const topGames = validGames
    .filter((v) => v.score >= 0.9999 * max)
    .sort((g1, g2) => g1.score - g2.score)
    .slice(0, 3);

  return topGames[Math.floor(Math.random() * topGames.length)].game;
}

function playGame(white: Network, black: Network) {
  let game = gameFromFen(
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  );
  while (!game.result) {
    const p = pickMove(game, game.player === Player.WHITE ? white : black);
    game = setGameResult(p);
  }
  return game;
}

function playMatch(first: Network, second: Network) {
  const scores: { [networkId: string]: number } = {};
  scores[first.id] = 0;
  scores[second.id] = 0;

  for (let i = 0; i < 6; i++) {
    printProgress(`Match ${first.id} vs ${second.id}`, (2 * i) / 12);
    switch (playGame(first, second).result) {
      case Result.WHITE:
        scores[first.id] += 1;
        break;
      case Result.BLACK:
        scores[second.id] += 1;
        break;
      case null:
        throw new Error("game without result");
      default:
        scores[first.id] += 0.5;
        scores[second.id] += 0.5;
        break;
    }

    printProgress(`Match ${first.id} vs ${second.id}`, (2 * i + 1) / 12);
    switch (playGame(second, first).result) {
      case Result.WHITE:
        scores[second.id] += 1;
        break;
      case Result.BLACK:
        scores[first.id] += 1;
        break;
      case null:
        throw new Error("game without result");
      default:
        scores[first.id] += 0.5;
        scores[second.id] += 0.5;
        break;
    }
  }
  printProgress(`Match ${first.id} vs ${second.id}`, 1);

  return scores;
}

const NUMBER_OF_NETWORKS = 10;
const networks: Network[] = [];

for (let i = 0; i < NUMBER_OF_NETWORKS; i++) {
  printProgress("Generating random networks", i / NUMBER_OF_NETWORKS);
  networks.push(generateRandomNetwork(i.toString()));
}
printProgress("Generating random networks", 1);

const scores: { [networkId: string]: number } = {};
for (let i = 0; i < networks.length - 1; i++) {
  for (let j = i + 1; j < networks.length; j++) {
    const matchResult = playMatch(networks[i], networks[j]);
    Object.assign(scores, matchResult);
  }
}

console.log(scores);
