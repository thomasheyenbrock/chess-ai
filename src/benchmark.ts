import { gameFromFen } from "./fen";
import { countLegalMoves } from "./move-generator";

const game = gameFromFen(
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
);

for (let i = 0; i < 100; i++) {
  console.time("benchmark");
  countLegalMoves(game, 3);
  console.timeEnd("benchmark");
}
