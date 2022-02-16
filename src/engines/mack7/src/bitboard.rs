use std::cmp::Ordering;
use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug)]
pub struct Bitboard(u64);

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn number_to_string<'a>(n: u64) -> &'a str {
            if n > 0 {
                "X"
            } else {
                " "
            }
        }
        write!(
            f,
            "┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 8
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 7
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 6
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 5
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 4
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 3
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 2
┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ {} ┃ 1
┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛
  A   B   C   D   E   F   G   H",
            number_to_string(self.0 & 0x8000_0000_0000_0000),
            number_to_string(self.0 & 0x4000_0000_0000_0000),
            number_to_string(self.0 & 0x2000_0000_0000_0000),
            number_to_string(self.0 & 0x1000_0000_0000_0000),
            number_to_string(self.0 & 0x0800_0000_0000_0000),
            number_to_string(self.0 & 0x0400_0000_0000_0000),
            number_to_string(self.0 & 0x0200_0000_0000_0000),
            number_to_string(self.0 & 0x0100_0000_0000_0000),
            number_to_string(self.0 & 0x0080_0000_0000_0000),
            number_to_string(self.0 & 0x0040_0000_0000_0000),
            number_to_string(self.0 & 0x0020_0000_0000_0000),
            number_to_string(self.0 & 0x0010_0000_0000_0000),
            number_to_string(self.0 & 0x0008_0000_0000_0000),
            number_to_string(self.0 & 0x0004_0000_0000_0000),
            number_to_string(self.0 & 0x0002_0000_0000_0000),
            number_to_string(self.0 & 0x0001_0000_0000_0000),
            number_to_string(self.0 & 0x0000_8000_0000_0000),
            number_to_string(self.0 & 0x0000_4000_0000_0000),
            number_to_string(self.0 & 0x0000_2000_0000_0000),
            number_to_string(self.0 & 0x0000_1000_0000_0000),
            number_to_string(self.0 & 0x0000_0800_0000_0000),
            number_to_string(self.0 & 0x0000_0400_0000_0000),
            number_to_string(self.0 & 0x0000_0200_0000_0000),
            number_to_string(self.0 & 0x0000_0100_0000_0000),
            number_to_string(self.0 & 0x0000_0080_0000_0000),
            number_to_string(self.0 & 0x0000_0040_0000_0000),
            number_to_string(self.0 & 0x0000_0020_0000_0000),
            number_to_string(self.0 & 0x0000_0010_0000_0000),
            number_to_string(self.0 & 0x0000_0008_0000_0000),
            number_to_string(self.0 & 0x0000_0004_0000_0000),
            number_to_string(self.0 & 0x0000_0002_0000_0000),
            number_to_string(self.0 & 0x0000_0001_0000_0000),
            number_to_string(self.0 & 0x0000_0000_8000_0000),
            number_to_string(self.0 & 0x0000_0000_4000_0000),
            number_to_string(self.0 & 0x0000_0000_2000_0000),
            number_to_string(self.0 & 0x0000_0000_1000_0000),
            number_to_string(self.0 & 0x0000_0000_0800_0000),
            number_to_string(self.0 & 0x0000_0000_0400_0000),
            number_to_string(self.0 & 0x0000_0000_0200_0000),
            number_to_string(self.0 & 0x0000_0000_0100_0000),
            number_to_string(self.0 & 0x0000_0000_0080_0000),
            number_to_string(self.0 & 0x0000_0000_0040_0000),
            number_to_string(self.0 & 0x0000_0000_0020_0000),
            number_to_string(self.0 & 0x0000_0000_0010_0000),
            number_to_string(self.0 & 0x0000_0000_0008_0000),
            number_to_string(self.0 & 0x0000_0000_0004_0000),
            number_to_string(self.0 & 0x0000_0000_0002_0000),
            number_to_string(self.0 & 0x0000_0000_0001_0000),
            number_to_string(self.0 & 0x0000_0000_0000_8000),
            number_to_string(self.0 & 0x0000_0000_0000_4000),
            number_to_string(self.0 & 0x0000_0000_0000_2000),
            number_to_string(self.0 & 0x0000_0000_0000_1000),
            number_to_string(self.0 & 0x0000_0000_0000_0800),
            number_to_string(self.0 & 0x0000_0000_0000_0400),
            number_to_string(self.0 & 0x0000_0000_0000_0200),
            number_to_string(self.0 & 0x0000_0000_0000_0100),
            number_to_string(self.0 & 0x0000_0000_0000_0080),
            number_to_string(self.0 & 0x0000_0000_0000_0040),
            number_to_string(self.0 & 0x0000_0000_0000_0020),
            number_to_string(self.0 & 0x0000_0000_0000_0010),
            number_to_string(self.0 & 0x0000_0000_0000_0008),
            number_to_string(self.0 & 0x0000_0000_0000_0004),
            number_to_string(self.0 & 0x0000_0000_0000_0002),
            number_to_string(self.0 & 0x0000_0000_0000_0001),
        )
    }
}

impl PartialEq for Bitboard {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Bitboard {}

impl Ord for Bitboard {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Bitboard {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.0 &= rhs.0;
        self
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.0 |= rhs.0;
        self
    }
}

impl ops::BitXor for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.0 ^= rhs.0;
        self
    }
}

impl ops::BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ops::BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

pub struct BitboardIter {
    n: u64,
    c: u32,
}

impl Iterator for BitboardIter {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }

        let mut m = self.n >> self.c;
        while m & 1 == 0 {
            m >>= 1;
            self.c += 1;
        }
        let pow_of_2 = 1 << self.c;
        self.n ^= pow_of_2;

        Some(Bitboard(pow_of_2))
    }
}

impl Bitboard {
    #[inline]
    pub fn new(n: u64) -> Bitboard {
        Bitboard(n)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn into_iter(self) -> BitboardIter {
        BitboardIter { n: self.0, c: 0 }
    }

    #[inline]
    pub fn get_left_square(self) -> Bitboard {
        Bitboard((self.0 & 0x7F7F_7F7F_7F7F_7F7F) << 1)
    }

    #[inline]
    pub fn get_right_square(self) -> Bitboard {
        Bitboard((self.0 & 0xFEFE_FEFE_FEFE_FEFE) >> 1)
    }

    #[inline]
    pub fn get_top_square(self) -> Bitboard {
        Bitboard((self.0 << 8) & 0xFFFF_FFFF_FFFF_FFFF)
    }

    #[inline]
    pub fn get_bottom_square(self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    #[inline]
    pub fn get_top_left_square(self) -> Bitboard {
        self.get_top_square().get_left_square()
    }

    #[inline]
    pub fn get_top_right_square(self) -> Bitboard {
        self.get_top_square().get_right_square()
    }

    #[inline]
    pub fn get_bottom_left_square(self) -> Bitboard {
        self.get_bottom_square().get_left_square()
    }

    #[inline]
    pub fn get_bottom_right_square(self) -> Bitboard {
        self.get_bottom_square().get_right_square()
    }

    #[inline]
    pub fn get_square_in_direction(self, direction: Direction) -> Bitboard {
        match direction {
            Direction::Top => self.get_top_square(),
            Direction::Bottom => self.get_bottom_square(),
            Direction::Left => self.get_left_square(),
            Direction::Right => self.get_right_square(),
            Direction::TopLeft => self.get_top_left_square(),
            Direction::TopRight => self.get_top_right_square(),
            Direction::BottomLeft => self.get_bottom_left_square(),
            Direction::BottomRight => self.get_bottom_right_square(),
        }
    }

    #[inline]
    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn king_moves(self) -> Bitboard {
        let top = self.get_top_square();
        let bottom = self.get_bottom_square();
        let left = self.get_left_square();
        let right = self.get_right_square();
        top | top.get_left_square()
            | top.get_right_square()
            | bottom
            | bottom.get_left_square()
            | bottom.get_right_square()
            | left
            | right
    }

    #[inline]
    pub fn knight_moves(self) -> Bitboard {
        let top = self.get_top_square().get_top_square();
        let bottom = self.get_bottom_square().get_bottom_square();
        let left = self.get_left_square().get_left_square();
        let right = self.get_right_square().get_right_square();
        top.get_left_square()
            | top.get_right_square()
            | bottom.get_left_square()
            | bottom.get_right_square()
            | left.get_top_square()
            | left.get_bottom_square()
            | right.get_top_square()
            | right.get_bottom_square()
    }

    #[inline]
    pub fn to_human(self) -> String {
        match self.0 {
            0x8000_0000_0000_0000 => String::from("a8"),
            0x4000_0000_0000_0000 => String::from("b8"),
            0x2000_0000_0000_0000 => String::from("c8"),
            0x1000_0000_0000_0000 => String::from("d8"),
            0x0800_0000_0000_0000 => String::from("e8"),
            0x0400_0000_0000_0000 => String::from("f8"),
            0x0200_0000_0000_0000 => String::from("g8"),
            0x0100_0000_0000_0000 => String::from("h8"),
            0x0080_0000_0000_0000 => String::from("a7"),
            0x0040_0000_0000_0000 => String::from("b7"),
            0x0020_0000_0000_0000 => String::from("c7"),
            0x0010_0000_0000_0000 => String::from("d7"),
            0x0008_0000_0000_0000 => String::from("e7"),
            0x0004_0000_0000_0000 => String::from("f7"),
            0x0002_0000_0000_0000 => String::from("g7"),
            0x0001_0000_0000_0000 => String::from("h7"),
            0x0000_8000_0000_0000 => String::from("a6"),
            0x0000_4000_0000_0000 => String::from("b6"),
            0x0000_2000_0000_0000 => String::from("c6"),
            0x0000_1000_0000_0000 => String::from("d6"),
            0x0000_0800_0000_0000 => String::from("e6"),
            0x0000_0400_0000_0000 => String::from("f6"),
            0x0000_0200_0000_0000 => String::from("g6"),
            0x0000_0100_0000_0000 => String::from("h6"),
            0x0000_0080_0000_0000 => String::from("a5"),
            0x0000_0040_0000_0000 => String::from("b5"),
            0x0000_0020_0000_0000 => String::from("c5"),
            0x0000_0010_0000_0000 => String::from("d5"),
            0x0000_0008_0000_0000 => String::from("e5"),
            0x0000_0004_0000_0000 => String::from("f5"),
            0x0000_0002_0000_0000 => String::from("g5"),
            0x0000_0001_0000_0000 => String::from("h5"),
            0x0000_0000_8000_0000 => String::from("a4"),
            0x0000_0000_4000_0000 => String::from("b4"),
            0x0000_0000_2000_0000 => String::from("c4"),
            0x0000_0000_1000_0000 => String::from("d4"),
            0x0000_0000_0800_0000 => String::from("e4"),
            0x0000_0000_0400_0000 => String::from("f4"),
            0x0000_0000_0200_0000 => String::from("g4"),
            0x0000_0000_0100_0000 => String::from("h4"),
            0x0000_0000_0080_0000 => String::from("a3"),
            0x0000_0000_0040_0000 => String::from("b3"),
            0x0000_0000_0020_0000 => String::from("c3"),
            0x0000_0000_0010_0000 => String::from("d3"),
            0x0000_0000_0008_0000 => String::from("e3"),
            0x0000_0000_0004_0000 => String::from("f3"),
            0x0000_0000_0002_0000 => String::from("g3"),
            0x0000_0000_0001_0000 => String::from("h3"),
            0x0000_0000_0000_8000 => String::from("a2"),
            0x0000_0000_0000_4000 => String::from("b2"),
            0x0000_0000_0000_2000 => String::from("c2"),
            0x0000_0000_0000_1000 => String::from("d2"),
            0x0000_0000_0000_0800 => String::from("e2"),
            0x0000_0000_0000_0400 => String::from("f2"),
            0x0000_0000_0000_0200 => String::from("g2"),
            0x0000_0000_0000_0100 => String::from("h2"),
            0x0000_0000_0000_0080 => String::from("a1"),
            0x0000_0000_0000_0040 => String::from("b1"),
            0x0000_0000_0000_0020 => String::from("c1"),
            0x0000_0000_0000_0010 => String::from("d1"),
            0x0000_0000_0000_0008 => String::from("e1"),
            0x0000_0000_0000_0004 => String::from("f1"),
            0x0000_0000_0000_0002 => String::from("g1"),
            0x0000_0000_0000_0001 => String::from("h1"),
            _ => unreachable!("Bitboard contains multiple squares"),
        }
    }
}
