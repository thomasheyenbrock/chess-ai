use std::fmt;
use std::hash;
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
pub struct Bitboard {
    n: u64,
}

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
            number_to_string(self.n & 0x8000_0000_0000_0000),
            number_to_string(self.n & 0x4000_0000_0000_0000),
            number_to_string(self.n & 0x2000_0000_0000_0000),
            number_to_string(self.n & 0x1000_0000_0000_0000),
            number_to_string(self.n & 0x0800_0000_0000_0000),
            number_to_string(self.n & 0x0400_0000_0000_0000),
            number_to_string(self.n & 0x0200_0000_0000_0000),
            number_to_string(self.n & 0x0100_0000_0000_0000),
            number_to_string(self.n & 0x0080_0000_0000_0000),
            number_to_string(self.n & 0x0040_0000_0000_0000),
            number_to_string(self.n & 0x0020_0000_0000_0000),
            number_to_string(self.n & 0x0010_0000_0000_0000),
            number_to_string(self.n & 0x0008_0000_0000_0000),
            number_to_string(self.n & 0x0004_0000_0000_0000),
            number_to_string(self.n & 0x0002_0000_0000_0000),
            number_to_string(self.n & 0x0001_0000_0000_0000),
            number_to_string(self.n & 0x0000_8000_0000_0000),
            number_to_string(self.n & 0x0000_4000_0000_0000),
            number_to_string(self.n & 0x0000_2000_0000_0000),
            number_to_string(self.n & 0x0000_1000_0000_0000),
            number_to_string(self.n & 0x0000_0800_0000_0000),
            number_to_string(self.n & 0x0000_0400_0000_0000),
            number_to_string(self.n & 0x0000_0200_0000_0000),
            number_to_string(self.n & 0x0000_0100_0000_0000),
            number_to_string(self.n & 0x0000_0080_0000_0000),
            number_to_string(self.n & 0x0000_0040_0000_0000),
            number_to_string(self.n & 0x0000_0020_0000_0000),
            number_to_string(self.n & 0x0000_0010_0000_0000),
            number_to_string(self.n & 0x0000_0008_0000_0000),
            number_to_string(self.n & 0x0000_0004_0000_0000),
            number_to_string(self.n & 0x0000_0002_0000_0000),
            number_to_string(self.n & 0x0000_0001_0000_0000),
            number_to_string(self.n & 0x0000_0000_8000_0000),
            number_to_string(self.n & 0x0000_0000_4000_0000),
            number_to_string(self.n & 0x0000_0000_2000_0000),
            number_to_string(self.n & 0x0000_0000_1000_0000),
            number_to_string(self.n & 0x0000_0000_0800_0000),
            number_to_string(self.n & 0x0000_0000_0400_0000),
            number_to_string(self.n & 0x0000_0000_0200_0000),
            number_to_string(self.n & 0x0000_0000_0100_0000),
            number_to_string(self.n & 0x0000_0000_0080_0000),
            number_to_string(self.n & 0x0000_0000_0040_0000),
            number_to_string(self.n & 0x0000_0000_0020_0000),
            number_to_string(self.n & 0x0000_0000_0010_0000),
            number_to_string(self.n & 0x0000_0000_0008_0000),
            number_to_string(self.n & 0x0000_0000_0004_0000),
            number_to_string(self.n & 0x0000_0000_0002_0000),
            number_to_string(self.n & 0x0000_0000_0001_0000),
            number_to_string(self.n & 0x0000_0000_0000_8000),
            number_to_string(self.n & 0x0000_0000_0000_4000),
            number_to_string(self.n & 0x0000_0000_0000_2000),
            number_to_string(self.n & 0x0000_0000_0000_1000),
            number_to_string(self.n & 0x0000_0000_0000_0800),
            number_to_string(self.n & 0x0000_0000_0000_0400),
            number_to_string(self.n & 0x0000_0000_0000_0200),
            number_to_string(self.n & 0x0000_0000_0000_0100),
            number_to_string(self.n & 0x0000_0000_0000_0080),
            number_to_string(self.n & 0x0000_0000_0000_0040),
            number_to_string(self.n & 0x0000_0000_0000_0020),
            number_to_string(self.n & 0x0000_0000_0000_0010),
            number_to_string(self.n & 0x0000_0000_0000_0008),
            number_to_string(self.n & 0x0000_0000_0000_0004),
            number_to_string(self.n & 0x0000_0000_0000_0002),
            number_to_string(self.n & 0x0000_0000_0000_0001),
        )
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Eq for Bitboard {}

impl hash::Hash for Bitboard {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.n.hash(state);
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.n &= rhs.n;
        self
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.n |= rhs.n;
        self
    }
}

impl ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.n ^= rhs.n;
        self
    }
}

impl ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.n |= rhs.n;
    }
}

impl ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.n ^= rhs.n;
    }
}

pub struct BitboardIter {
    n: u64,
    c: u32,
}

impl Iterator for BitboardIter {
    type Item = Bitboard;

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

        Some(Bitboard { n: pow_of_2 })
    }
}

impl Bitboard {
    pub fn new(n: u64) -> Bitboard {
        Bitboard { n }
    }

    pub fn str(self) -> String {
        self.n.to_string()
    }

    pub fn is_empty(self) -> bool {
        self.n == 0
    }

    pub fn into_iter(self) -> BitboardIter {
        BitboardIter { n: self.n, c: 0 }
    }

    pub fn get_left_square(self) -> Bitboard {
        Bitboard {
            n: (self.n & 0x7F7F_7F7F_7F7F_7F7F) << 1,
        }
    }

    pub fn get_right_square(self) -> Bitboard {
        Bitboard {
            n: (self.n & 0xFEFE_FEFE_FEFE_FEFE) >> 1,
        }
    }

    pub fn get_top_square(self) -> Bitboard {
        Bitboard {
            n: (self.n << 8) & 0xFFFF_FFFF_FFFF_FFFF,
        }
    }

    pub fn get_bottom_square(self) -> Bitboard {
        Bitboard { n: self.n >> 8 }
    }

    pub fn get_top_left_square(self) -> Bitboard {
        self.get_top_square().get_left_square()
    }

    pub fn get_top_right_square(self) -> Bitboard {
        self.get_top_square().get_right_square()
    }

    pub fn get_bottom_left_square(self) -> Bitboard {
        self.get_bottom_square().get_left_square()
    }

    pub fn get_bottom_right_square(self) -> Bitboard {
        self.get_bottom_square().get_right_square()
    }

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

    pub fn count_ones(self) -> u32 {
        self.n.count_ones()
    }

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
}
