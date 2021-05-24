import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";
import { Bitboard, bitwiseAnd, isNull } from "../../bitboard";
import { gameFromFen } from "../../fen";
import {
  Castle,
  Game,
  Piece,
  Player,
  setGameResult,
} from "../../move-generator";
import { CONCURRENT_GAMES, PAIRINGS_FILENAME } from "./constants";
import { findPossibleMove, moveForOutputIndex } from "./output-layer-mapping";
import { Pairings } from "./types";

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

async function pickMove(game: Game, network: tf.LayersModel) {
  const input = tf.tensor(
    [
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
    ],
    [1, 837]
  );

  const output = await (network.predict(input) as tf.Tensor).data();

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

async function playGame(white: tf.LayersModel, black: tf.LayersModel) {
  let game = gameFromFen(
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  );
  while (!game.result) {
    game = setGameResult(
      await pickMove(game, game.player === Player.WHITE ? white : black)
    );
  }
  return game.result;
}

async function main() {
  const pairings: Pairings = JSON.parse(
    await fs.promises.readFile(PAIRINGS_FILENAME, "utf8")
  );
  const networkIds = await fs.promises.readdir(
    path.join(__dirname, "networks")
  );

  const networks = await networkIds
    .filter((id) => id !== "pairings.json")
    .reduce<Promise<{ [id: string]: tf.LayersModel }>>(
      async (accPromise, id) => {
        const acc = await accPromise;
        acc[id] = await tf.loadLayersModel(
          `file://${path.resolve(__dirname, "networks", id, "model.json")}`
        );
        return acc;
      },
      Promise.resolve({})
    );

  let missing = Object.entries(pairings).filter(([, result]) => !result);

  while (missing.length > 0) {
    const batch = missing.slice(0, CONCURRENT_GAMES);

    await Promise.all(
      batch.map(async ([id]) => {
        const [whiteId, blackId] = id.split("-");
        const result = await playGame(networks[whiteId], networks[blackId]);
        console.log(whiteId, "vs", blackId, ":", result);
        pairings[id] = result;
      })
    );

    await fs.promises.writeFile(PAIRINGS_FILENAME, JSON.stringify(pairings));
    missing = Object.entries(pairings).filter(([, result]) => !result);
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
