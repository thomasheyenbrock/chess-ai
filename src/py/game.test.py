import unittest

from fen import game_from_fen


class TestGame(unittest.TestCase):
    def test_position_1(self):
        cases = [
            (1, 20),
            (2, 400),
            (3, 8902),
            (4, 197281),
            (5, 4865609),
        ]
        for depth, moves in cases:
            game = game_from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            )
            self.assertEqual(game.count_legal_moves(depth), moves)

    def test_position_2(self):
        cases = [
            (1, 48),
            (2, 2039),
            (3, 97862),
            (4, 4085603),
        ]
        for depth, moves in cases:
            game = game_from_fen(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
            )
            self.assertEqual(game.count_legal_moves(depth), moves)

    def test_position_3(self):
        cases = [
            (1, 14),
            (2, 191),
            (3, 2812),
            (4, 43238),
            (5, 674624),
        ]
        for depth, moves in cases:
            game = game_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1")
            self.assertEqual(game.count_legal_moves(depth), moves)

    def test_position_4(self):
        cases = [
            (1, 6),
            (2, 264),
            (3, 9467),
            (4, 422333),
        ]
        for depth, moves in cases:
            game = game_from_fen(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"
            )
            self.assertEqual(game.count_legal_moves(depth), moves)

    def test_position_5(self):
        cases = [
            (1, 44),
            (2, 1486),
            (3, 62379),
            (4, 2103487),
        ]
        for depth, moves in cases:
            game = game_from_fen(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"
            )
            self.assertEqual(game.count_legal_moves(depth), moves)

    def test_position_6(self):
        cases = [
            (1, 46),
            (2, 2079),
            (3, 89890),
            (4, 3894594),
        ]
        for depth, moves in cases:
            game = game_from_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"
            )
            self.assertEqual(game.count_legal_moves(depth), moves)


if __name__ == "__main__":
    unittest.main()
