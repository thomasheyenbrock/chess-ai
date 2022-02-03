use crate::bitboard::{Bitboard, Direction};
use std::collections::HashMap;

fn generate_possibilities(current: Bitboard, direction: Direction) -> Vec<Bitboard> {
    let mut forward = current.get_square_in_direction(direction);
    let mut possibilities: Vec<Bitboard> = vec![Bitboard::new(0)];
    if !forward.is_empty() {
        possibilities.push(forward);
    }

    while !forward.is_empty() {
        forward = forward.get_square_in_direction(direction);
        let mut new_possibilities: Vec<Bitboard> = vec![];
        for p in possibilities.iter() {
            new_possibilities.push(*p);
            if !forward.is_empty() {
                new_possibilities.push(*p | forward);
            }
        }
        possibilities = new_possibilities;
    }
    possibilities
}

#[derive(Debug)]
pub struct BitboardMap<V> {
    pub m: HashMap<Bitboard, V>, // TODO: make private
}

impl<V> BitboardMap<V> {
    fn new() -> BitboardMap<V> {
        BitboardMap { m: HashMap::new() }
    }

    pub fn get(&self, key: Bitboard) -> &V {
        self.m.get(&key).unwrap()
    }

    pub fn get_or_default<'a>(&'a self, key: Bitboard, default: &'a V) -> &'a V {
        match self.m.get(&key) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn set(&mut self, key: Bitboard, value: V) {
        self.m.insert(key, value);
    }
}

#[derive(Debug)]
pub struct Constants {
    pub squares: Vec<Bitboard>,
    pub human_to_squares: HashMap<String, Bitboard>,

    pub north_ray: BitboardMap<Bitboard>,
    pub south_ray: BitboardMap<Bitboard>,
    pub west_ray: BitboardMap<Bitboard>,
    pub east_ray: BitboardMap<Bitboard>,
    pub north_west_ray: BitboardMap<Bitboard>,
    pub north_east_ray: BitboardMap<Bitboard>,
    pub south_west_ray: BitboardMap<Bitboard>,
    pub south_east_ray: BitboardMap<Bitboard>,

    pub north_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub south_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub west_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub east_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub north_west_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub north_east_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub south_west_moves: BitboardMap<BitboardMap<Bitboard>>,
    pub south_east_moves: BitboardMap<BitboardMap<Bitboard>>,

    pub north_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub south_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub west_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub east_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub north_west_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub north_east_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub south_west_attacks: BitboardMap<BitboardMap<Bitboard>>,
    pub south_east_attacks: BitboardMap<BitboardMap<Bitboard>>,

    pub king_moves: BitboardMap<Bitboard>,
    pub knight_moves: BitboardMap<Bitboard>,
    pub pawn_attacks: HashMap<bool, BitboardMap<Bitboard>>,
    pub pawn_single_moves: HashMap<bool, BitboardMap<Bitboard>>,
    pub pawn_double_moves: HashMap<bool, BitboardMap<Bitboard>>,
    pub pawn_attack_moves: HashMap<bool, BitboardMap<Vec<Bitboard>>>,
    pub pawn_en_passant_captures: HashMap<bool, BitboardMap<Bitboard>>,
}

pub fn get() -> Constants {
    let squares = vec![
        Bitboard::new(0x8000_0000_0000_0000),
        Bitboard::new(0x4000_0000_0000_0000),
        Bitboard::new(0x2000_0000_0000_0000),
        Bitboard::new(0x1000_0000_0000_0000),
        Bitboard::new(0x0800_0000_0000_0000),
        Bitboard::new(0x0400_0000_0000_0000),
        Bitboard::new(0x0200_0000_0000_0000),
        Bitboard::new(0x0100_0000_0000_0000),
        Bitboard::new(0x0080_0000_0000_0000),
        Bitboard::new(0x0040_0000_0000_0000),
        Bitboard::new(0x0020_0000_0000_0000),
        Bitboard::new(0x0010_0000_0000_0000),
        Bitboard::new(0x0008_0000_0000_0000),
        Bitboard::new(0x0004_0000_0000_0000),
        Bitboard::new(0x0002_0000_0000_0000),
        Bitboard::new(0x0001_0000_0000_0000),
        Bitboard::new(0x0000_8000_0000_0000),
        Bitboard::new(0x0000_4000_0000_0000),
        Bitboard::new(0x0000_2000_0000_0000),
        Bitboard::new(0x0000_1000_0000_0000),
        Bitboard::new(0x0000_0800_0000_0000),
        Bitboard::new(0x0000_0400_0000_0000),
        Bitboard::new(0x0000_0200_0000_0000),
        Bitboard::new(0x0000_0100_0000_0000),
        Bitboard::new(0x0000_0080_0000_0000),
        Bitboard::new(0x0000_0040_0000_0000),
        Bitboard::new(0x0000_0020_0000_0000),
        Bitboard::new(0x0000_0010_0000_0000),
        Bitboard::new(0x0000_0008_0000_0000),
        Bitboard::new(0x0000_0004_0000_0000),
        Bitboard::new(0x0000_0002_0000_0000),
        Bitboard::new(0x0000_0001_0000_0000),
        Bitboard::new(0x0000_0000_8000_0000),
        Bitboard::new(0x0000_0000_4000_0000),
        Bitboard::new(0x0000_0000_2000_0000),
        Bitboard::new(0x0000_0000_1000_0000),
        Bitboard::new(0x0000_0000_0800_0000),
        Bitboard::new(0x0000_0000_0400_0000),
        Bitboard::new(0x0000_0000_0200_0000),
        Bitboard::new(0x0000_0000_0100_0000),
        Bitboard::new(0x0000_0000_0080_0000),
        Bitboard::new(0x0000_0000_0040_0000),
        Bitboard::new(0x0000_0000_0020_0000),
        Bitboard::new(0x0000_0000_0010_0000),
        Bitboard::new(0x0000_0000_0008_0000),
        Bitboard::new(0x0000_0000_0004_0000),
        Bitboard::new(0x0000_0000_0002_0000),
        Bitboard::new(0x0000_0000_0001_0000),
        Bitboard::new(0x0000_0000_0000_8000),
        Bitboard::new(0x0000_0000_0000_4000),
        Bitboard::new(0x0000_0000_0000_2000),
        Bitboard::new(0x0000_0000_0000_1000),
        Bitboard::new(0x0000_0000_0000_0800),
        Bitboard::new(0x0000_0000_0000_0400),
        Bitboard::new(0x0000_0000_0000_0200),
        Bitboard::new(0x0000_0000_0000_0100),
        Bitboard::new(0x0000_0000_0000_0080),
        Bitboard::new(0x0000_0000_0000_0040),
        Bitboard::new(0x0000_0000_0000_0020),
        Bitboard::new(0x0000_0000_0000_0010),
        Bitboard::new(0x0000_0000_0000_0008),
        Bitboard::new(0x0000_0000_0000_0004),
        Bitboard::new(0x0000_0000_0000_0002),
        Bitboard::new(0x0000_0000_0000_0001),
    ];

    let human_to_squares = HashMap::from([
        ("a8".to_owned(), Bitboard::new(0x8000_0000_0000_0000)),
        ("b8".to_owned(), Bitboard::new(0x4000_0000_0000_0000)),
        ("c8".to_owned(), Bitboard::new(0x2000_0000_0000_0000)),
        ("d8".to_owned(), Bitboard::new(0x1000_0000_0000_0000)),
        ("e8".to_owned(), Bitboard::new(0x0800_0000_0000_0000)),
        ("f8".to_owned(), Bitboard::new(0x0400_0000_0000_0000)),
        ("g8".to_owned(), Bitboard::new(0x0200_0000_0000_0000)),
        ("h8".to_owned(), Bitboard::new(0x0100_0000_0000_0000)),
        ("a7".to_owned(), Bitboard::new(0x0080_0000_0000_0000)),
        ("b7".to_owned(), Bitboard::new(0x0040_0000_0000_0000)),
        ("c7".to_owned(), Bitboard::new(0x0020_0000_0000_0000)),
        ("d7".to_owned(), Bitboard::new(0x0010_0000_0000_0000)),
        ("e7".to_owned(), Bitboard::new(0x0008_0000_0000_0000)),
        ("f7".to_owned(), Bitboard::new(0x0004_0000_0000_0000)),
        ("g7".to_owned(), Bitboard::new(0x0002_0000_0000_0000)),
        ("h7".to_owned(), Bitboard::new(0x0001_0000_0000_0000)),
        ("a6".to_owned(), Bitboard::new(0x0000_8000_0000_0000)),
        ("b6".to_owned(), Bitboard::new(0x0000_4000_0000_0000)),
        ("c6".to_owned(), Bitboard::new(0x0000_2000_0000_0000)),
        ("d6".to_owned(), Bitboard::new(0x0000_1000_0000_0000)),
        ("e6".to_owned(), Bitboard::new(0x0000_0800_0000_0000)),
        ("f6".to_owned(), Bitboard::new(0x0000_0400_0000_0000)),
        ("g6".to_owned(), Bitboard::new(0x0000_0200_0000_0000)),
        ("h6".to_owned(), Bitboard::new(0x0000_0100_0000_0000)),
        ("a5".to_owned(), Bitboard::new(0x0000_0080_0000_0000)),
        ("b5".to_owned(), Bitboard::new(0x0000_0040_0000_0000)),
        ("c5".to_owned(), Bitboard::new(0x0000_0020_0000_0000)),
        ("d5".to_owned(), Bitboard::new(0x0000_0010_0000_0000)),
        ("e5".to_owned(), Bitboard::new(0x0000_0008_0000_0000)),
        ("f5".to_owned(), Bitboard::new(0x0000_0004_0000_0000)),
        ("g5".to_owned(), Bitboard::new(0x0000_0002_0000_0000)),
        ("h5".to_owned(), Bitboard::new(0x0000_0001_0000_0000)),
        ("a4".to_owned(), Bitboard::new(0x0000_0000_8000_0000)),
        ("b4".to_owned(), Bitboard::new(0x0000_0000_4000_0000)),
        ("c4".to_owned(), Bitboard::new(0x0000_0000_2000_0000)),
        ("d4".to_owned(), Bitboard::new(0x0000_0000_1000_0000)),
        ("e4".to_owned(), Bitboard::new(0x0000_0000_0800_0000)),
        ("f4".to_owned(), Bitboard::new(0x0000_0000_0400_0000)),
        ("g4".to_owned(), Bitboard::new(0x0000_0000_0200_0000)),
        ("h4".to_owned(), Bitboard::new(0x0000_0000_0100_0000)),
        ("a3".to_owned(), Bitboard::new(0x0000_0000_0080_0000)),
        ("b3".to_owned(), Bitboard::new(0x0000_0000_0040_0000)),
        ("c3".to_owned(), Bitboard::new(0x0000_0000_0020_0000)),
        ("d3".to_owned(), Bitboard::new(0x0000_0000_0010_0000)),
        ("e3".to_owned(), Bitboard::new(0x0000_0000_0008_0000)),
        ("f3".to_owned(), Bitboard::new(0x0000_0000_0004_0000)),
        ("g3".to_owned(), Bitboard::new(0x0000_0000_0002_0000)),
        ("h3".to_owned(), Bitboard::new(0x0000_0000_0001_0000)),
        ("a2".to_owned(), Bitboard::new(0x0000_0000_0000_8000)),
        ("b2".to_owned(), Bitboard::new(0x0000_0000_0000_4000)),
        ("c2".to_owned(), Bitboard::new(0x0000_0000_0000_2000)),
        ("d2".to_owned(), Bitboard::new(0x0000_0000_0000_1000)),
        ("e2".to_owned(), Bitboard::new(0x0000_0000_0000_0800)),
        ("f2".to_owned(), Bitboard::new(0x0000_0000_0000_0400)),
        ("g2".to_owned(), Bitboard::new(0x0000_0000_0000_0200)),
        ("h2".to_owned(), Bitboard::new(0x0000_0000_0000_0100)),
        ("a1".to_owned(), Bitboard::new(0x0000_0000_0000_0080)),
        ("b1".to_owned(), Bitboard::new(0x0000_0000_0000_0040)),
        ("c1".to_owned(), Bitboard::new(0x0000_0000_0000_0020)),
        ("d1".to_owned(), Bitboard::new(0x0000_0000_0000_0010)),
        ("e1".to_owned(), Bitboard::new(0x0000_0000_0000_0008)),
        ("f1".to_owned(), Bitboard::new(0x0000_0000_0000_0004)),
        ("g1".to_owned(), Bitboard::new(0x0000_0000_0000_0002)),
        ("h1".to_owned(), Bitboard::new(0x0000_0000_0000_0001)),
    ]);

    let mut north_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut south_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut west_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut east_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut north_west_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut north_east_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut south_west_ray: BitboardMap<Bitboard> = BitboardMap::new();
    let mut south_east_ray: BitboardMap<Bitboard> = BitboardMap::new();

    let mut north_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut west_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut east_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut north_west_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut north_east_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_west_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_east_moves: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();

    let mut north_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut west_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut east_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut north_west_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut north_east_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_west_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();
    let mut south_east_attacks: BitboardMap<BitboardMap<Bitboard>> = BitboardMap::new();

    let mut king_moves: BitboardMap<Bitboard> = BitboardMap::new();
    let mut knight_moves: BitboardMap<Bitboard> = BitboardMap::new();

    let mut pawn_attacks_white: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_attacks_black: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_single_moves_white: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_single_moves_black: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_double_moves_white: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_double_moves_black: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_attack_moves_white: BitboardMap<Vec<Bitboard>> = BitboardMap::new();
    let mut pawn_attack_moves_black: BitboardMap<Vec<Bitboard>> = BitboardMap::new();
    let mut pawn_en_passant_captures_white: BitboardMap<Bitboard> = BitboardMap::new();
    let mut pawn_en_passant_captures_black: BitboardMap<Bitboard> = BitboardMap::new();

    for rank in 0..8 {
        for file in 0..8 {
            let square = Bitboard::new(2_u64.pow(8 * rank + file));
            let mut current: Bitboard;
            let mut carry: Bitboard;
            let mut ray: Bitboard;
            let mut moves: BitboardMap<Bitboard>;
            let mut attacks: BitboardMap<Bitboard>;

            let top = square.get_top_square();
            let bottom = square.get_bottom_square();
            let left = square.get_left_square();
            let right = square.get_right_square();
            let top_left = top.get_left_square();
            let top_right = top.get_right_square();
            let bottom_left = bottom.get_left_square();
            let bottom_right = bottom.get_right_square();

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = top;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::Top).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_top_square();
            }
            north_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            north_moves.set(square, moves);
            north_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = bottom;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::Bottom).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_bottom_square();
            }
            south_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            south_moves.set(square, moves);
            south_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = left;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::Left).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_left_square();
            }
            west_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            west_moves.set(square, moves);
            west_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = right;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::Right).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_right_square();
            }
            east_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            east_moves.set(square, moves);
            east_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = top_left;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::TopLeft).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_top_square().get_left_square();
            }
            north_west_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            north_west_moves.set(square, moves);
            north_west_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = top_right;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::TopRight).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_top_square().get_right_square();
            }
            north_east_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            north_east_moves.set(square, moves);
            north_east_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = bottom_left;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::BottomLeft).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_bottom_square().get_left_square();
            }
            south_west_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            south_west_moves.set(square, moves);
            south_west_attacks.set(square, attacks);

            ray = Bitboard::new(0);
            moves = BitboardMap::new();
            attacks = BitboardMap::new();
            attacks.set(Bitboard::new(0), Bitboard::new(0));
            current = bottom_right;
            carry = Bitboard::new(0);
            while !current.is_empty() {
                ray = ray | current;
                for p in generate_possibilities(current, Direction::BottomRight).iter() {
                    moves.set(*p | current, carry);
                    attacks.set(*p | current, current);
                }
                carry = carry | current;
                current = current.get_bottom_square().get_right_square();
            }
            south_east_ray.set(square, ray);
            moves.set(Bitboard::new(0), carry);
            south_east_moves.set(square, moves);
            south_east_attacks.set(square, attacks);

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

            pawn_attacks_white.set(square, bottom_left | bottom_right);
            pawn_attacks_black.set(square, top_left | top_right);

            pawn_single_moves_white.set(square, top);
            pawn_single_moves_black.set(square, bottom);

            if (square & Bitboard::new(0x0000_0000_0000_FF00)).is_empty() {
                pawn_double_moves_white.set(square, Bitboard::new(0));
            } else {
                pawn_double_moves_white.set(square, top2);
            }

            if (square & Bitboard::new(0x00FF_0000_0000_0000)).is_empty() {
                pawn_double_moves_black.set(square, Bitboard::new(0));
            } else {
                pawn_double_moves_black.set(square, bottom2);
            }

            let mut white_pawn_attacks: Vec<Bitboard> = vec![];
            if !top_left.is_empty() {
                white_pawn_attacks.push(top_left);
            }
            if !top_right.is_empty() {
                white_pawn_attacks.push(top_right);
            }
            pawn_attack_moves_white.set(square, white_pawn_attacks);

            let mut black_pawn_attacks: Vec<Bitboard> = vec![];
            if !bottom_left.is_empty() {
                black_pawn_attacks.push(bottom_left);
            }
            if !bottom_right.is_empty() {
                black_pawn_attacks.push(bottom_right);
            }
            pawn_attack_moves_black.set(square, black_pawn_attacks);

            if (square & Bitboard::new(0x0000_00FF_0000_0000)).is_empty() {
                pawn_en_passant_captures_white.set(square, Bitboard::new(0));
            } else {
                pawn_en_passant_captures_white.set(square, top_left | top_right);
            }

            if (square & Bitboard::new(0x0000_0000_FF00_0000)).is_empty() {
                pawn_en_passant_captures_black.set(square, Bitboard::new(0));
            } else {
                pawn_en_passant_captures_black.set(square, bottom_left | bottom_right);
            }
        }
    }

    let mut pawn_attacks: HashMap<bool, BitboardMap<Bitboard>> = HashMap::new();
    let mut pawn_single_moves: HashMap<bool, BitboardMap<Bitboard>> = HashMap::new();
    let mut pawn_double_moves: HashMap<bool, BitboardMap<Bitboard>> = HashMap::new();
    let mut pawn_attack_moves: HashMap<bool, BitboardMap<Vec<Bitboard>>> = HashMap::new();
    let mut pawn_en_passant_captures: HashMap<bool, BitboardMap<Bitboard>> = HashMap::new();

    pawn_attacks.insert(true, pawn_attacks_white);
    pawn_attacks.insert(false, pawn_attacks_black);
    pawn_single_moves.insert(true, pawn_single_moves_white);
    pawn_single_moves.insert(false, pawn_single_moves_black);
    pawn_double_moves.insert(true, pawn_double_moves_white);
    pawn_double_moves.insert(false, pawn_double_moves_black);
    pawn_attack_moves.insert(true, pawn_attack_moves_white);
    pawn_attack_moves.insert(false, pawn_attack_moves_black);
    pawn_en_passant_captures.insert(true, pawn_en_passant_captures_white);
    pawn_en_passant_captures.insert(false, pawn_en_passant_captures_black);

    Constants {
        squares,
        human_to_squares,

        north_ray,
        south_ray,
        west_ray,
        east_ray,
        north_west_ray,
        north_east_ray,
        south_west_ray,
        south_east_ray,

        north_moves,
        south_moves,
        west_moves,
        east_moves,
        north_west_moves,
        north_east_moves,
        south_west_moves,
        south_east_moves,

        north_attacks,
        south_attacks,
        west_attacks,
        east_attacks,
        north_west_attacks,
        north_east_attacks,
        south_west_attacks,
        south_east_attacks,

        king_moves,
        knight_moves,
        pawn_attacks,
        pawn_single_moves,
        pawn_double_moves,
        pawn_attack_moves,
        pawn_en_passant_captures,
    }
}
