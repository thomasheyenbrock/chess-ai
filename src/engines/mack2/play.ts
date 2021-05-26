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
import { moveForOutputIndex } from "./output-layer-mapping";
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

type GameWithPlayers = {
  game: Game;
  whiteId: string;
  blackId: string;
};

type GamesPerNetwork = {
  [networkId: string]: GameWithPlayers[];
};

function pickMoves(games: GameWithPlayers[], network: tf.LayersModel) {
  const input = tf.tensor(
    games.map(({ game }) => [
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
    ])
  );

  const outputs = (network.predict(input) as tf.Tensor).dataSync();

  return games.map(({ game, whiteId, blackId }, i) => {
    const output = outputs.slice(i * 1972, (i + 1) * 1972);
    let max = -Infinity;
    const validGames: { score: number; game: Game }[] = [];
    for (let i = 0; i < output.length; i++) {
      let possibleGame = game.possibleMoves[moveForOutputIndex[i]];
      if (possibleGame) {
        const score = output[i];
        validGames.push({ score, game: possibleGame });
        max = Math.max(max, score);
      }
    }

    const topGames = validGames
      .filter((v) => v.score >= 0.95 * max)
      .sort((g1, g2) => g1.score - g2.score)
      .slice(0, 3);

    return {
      game: setGameResult(
        topGames[Math.floor(Math.random() * topGames.length)].game
      ),
      whiteId,
      blackId,
    };
  });
}

async function main() {
  const [, , generation, pairingBatch] = process.argv;
  if (!generation) {
    throw new Error("generation missing");
  }

  const pairingsFilename = path.join(
    __dirname,
    `generation${generation}`,
    `pairings_${pairingBatch}.json`
  );
  const pairings: Pairings = JSON.parse(
    await fs.promises.readFile(pairingsFilename, "utf8")
  );

  if (Object.values(pairings).every(Boolean)) {
    console.log("Already finished");
    return;
  }

  const networkIds = await fs.promises.readdir(
    path.join(__dirname, `generation${generation}`)
  );

  const networks = await networkIds
    .filter((id) => !id.match(/^pairings_\d+\.json$/))
    .reduce<Promise<{ [id: string]: tf.LayersModel }>>(
      async (accPromise, id) => {
        const acc = await accPromise;
        acc[id] = await tf.loadLayersModel(
          `file://${path.resolve(
            __dirname,
            `generation${generation}`,
            id,
            "model.json"
          )}`
        );
        return acc;
      },
      Promise.resolve({})
    );

  let gamesPerNetwork = Object.keys(pairings).reduce<GamesPerNetwork>(
    (acc, id) => {
      const [whiteId, blackId] = id.split("-");
      acc[whiteId] = acc[whiteId] || [];
      acc[whiteId].push({
        game: gameFromFen(
          "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        ),
        whiteId,
        blackId,
      });
      return acc;
    },
    {}
  );

  let move = 0;
  while (Object.keys(gamesPerNetwork).length > 0) {
    console.log(
      `Move ${++move}, ${
        Object.keys(gamesPerNetwork).length
      } networks still playing`
    );
    let newGamesPerNetwork: GamesPerNetwork = {};
    for (const networkId in gamesPerNetwork) {
      for (const gameWithPlayer of pickMoves(
        gamesPerNetwork[networkId],
        networks[networkId]
      )) {
        const result = gameWithPlayer.game.result;
        if (result) {
          pairings[`${gameWithPlayer.whiteId}-${gameWithPlayer.blackId}`] =
            result;
        } else {
          const newNetworkId =
            gameWithPlayer.game.player === Player.WHITE
              ? gameWithPlayer.whiteId
              : gameWithPlayer.blackId;
          newGamesPerNetwork[newNetworkId] =
            newGamesPerNetwork[newNetworkId] || [];
          newGamesPerNetwork[newNetworkId].push(gameWithPlayer);
        }
      }
    }
    gamesPerNetwork = newGamesPerNetwork;
  }

  await fs.promises.writeFile(pairingsFilename, JSON.stringify(pairings));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
