import { gameFromFen, getLegalMoves } from "./move-generator";

describe("getLegalMoves", () => {
  describe.each([
    [
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      [
        [1, 20],
        [2, 400],
        [3, 8902],
        [4, 197281],
        [5, 4865609],
      ],
    ],
    [
      "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
      [
        [1, 48],
        [2, 2039],
        [3, 97862],
        [4, 4085603],
      ],
    ],
  ])("%s", (fen, cases) => {
    it.each(cases)(
      "should calculate the correct number of moves for depth %s",
      (depth, expectedNumberOfMoves) => {
        const game = gameFromFen(fen);
        expect(getLegalMoves(game, depth)).toBe(expectedNumberOfMoves);
      }
    );
  });
});
