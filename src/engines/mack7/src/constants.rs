use crate::bitboard::Bitboard;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BitboardMap<V> {
    m: HashMap<Bitboard, V>,
}

impl<V> BitboardMap<V> {
    fn new() -> BitboardMap<V> {
        BitboardMap { m: HashMap::new() }
    }

    pub fn get(&self, key: Bitboard) -> &V {
        self.m.get(&key).unwrap()
    }

    pub fn set(&mut self, key: Bitboard, value: V) {
        self.m.insert(key, value);
    }
}

#[derive(Debug)]
pub struct Constants {
    pub king_moves: BitboardMap<Bitboard>,
    pub knight_moves: BitboardMap<Bitboard>,
}

pub fn get() -> Constants {
    let mut king_moves: BitboardMap<Bitboard> = BitboardMap::new();
    let mut knight_moves: BitboardMap<Bitboard> = BitboardMap::new();

    for rank in 0..8 {
        for file in 0..8 {
            let square = Bitboard::new(2_u64.pow(8 * rank + file));
            let top = square.get_top_square();
            let bottom = square.get_bottom_square();
            let left = square.get_left_square();
            let right = square.get_right_square();
            let top_left = top.get_left_square();
            let top_right = top.get_right_square();
            let bottom_left = bottom.get_left_square();
            let bottom_right = bottom.get_right_square();

            king_moves.set(
                square,
                top | bottom | left | right | top_left | top_right | bottom_left | bottom_right,
            );

            let top2 = top.get_top_square();
            let bottom2 = bottom.get_bottom_square();
            let left2 = left.get_left_square();
            let right2 = right.get_right_square();
            knight_moves.set(
                square,
                top2.get_left_square()
                    | top2.get_right_square()
                    | bottom2.get_left_square()
                    | bottom2.get_right_square()
                    | left2.get_top_square()
                    | left2.get_bottom_square()
                    | right2.get_top_square()
                    | right2.get_bottom_square(),
            );
        }
    }

    Constants {
        king_moves,
        knight_moves,
    }
}
