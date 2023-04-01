use ndarray::{Array1, Array2, Axis, ShapeError};
use rand::Rng;
use std::fs::File;
use std::io::Write;

use crate::{
    game::{Game, GameResult},
    policy_network::{self, PolicyNetwork},
    value_network::{self, ValueNetwork},
};

impl Game {
    pub fn get_input(&self) -> Array1<f32> {
        fn t(bitboard: Bitboard) -> f32 {
            if bitboard.is_empty() {
                0. as f32
            } else {
                1.
            }
        }
        array![
            if self.player { 1. as f32 } else { 0. as f32 },
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.king & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.queen & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.rook & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.bishop & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.knight & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.white.pawn & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.king & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.queen & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.rook & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.bishop & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.knight & Bitboard::new(0x8000_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.position.black.pawn & Bitboard::new(0x8000_0000_0000_0000)),
            if self.possible_castles.white_kingside {
                1.
            } else {
                0.
            },
            if self.possible_castles.white_queenside {
                1.
            } else {
                0.
            },
            if self.possible_castles.black_kingside {
                1.
            } else {
                0.
            },
            if self.possible_castles.black_queenside {
                1.
            } else {
                0.
            },
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0001)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0002)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0004)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0008)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0010)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0020)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0040)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0080)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0100)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0200)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0400)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_0800)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_1000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_2000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_4000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0000_8000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0001_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0002_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0004_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0008_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0010_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0020_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0040_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0080_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0100_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0200_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0400_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_0800_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_1000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_2000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_4000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0000_8000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0001_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0002_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0004_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0008_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0010_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0020_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0040_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0080_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0100_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0200_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0400_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_0800_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_1000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_2000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_4000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0000_8000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0001_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0002_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0004_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0008_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0010_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0020_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0040_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0080_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0100_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0200_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0400_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x0800_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x1000_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x2000_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x4000_0000_0000_0000)),
            t(self.en_passant_square & Bitboard::new(0x8000_0000_0000_0000)),
        ]
    }
}

#[derive(Debug)]
struct Node {
    tree_id: usize,
    state: Game,
    input: Array1<f32>,
    children: Vec<Node>,
    is_expanded: bool,
    has_checked_for_terminal: bool,
    is_terminal: bool,
    terminal_value: f32,
    prior: f32,
    visits: f32,
    total_value: f32,
}

impl Node {
    fn new(tree_id: usize, state: Game, prior: f32) -> Node {
        let input = state.get_input();
        return Node {
            tree_id,
            state,
            input,
            children: vec![],
            is_expanded: false,
            has_checked_for_terminal: false,
            is_terminal: false,
            prior,
            terminal_value: 0.,
            visits: 0.,
            total_value: 0.,
        };
    }

    fn from_node(node: &Node) -> Node {
        let state = Game::from_game(&node.state);
        let input = state.get_input();
        let children = node
            .children
            .iter()
            .map(|child| Node::from_node(child))
            .collect();
        return Node {
            tree_id: node.tree_id,
            state,
            input,
            children,
            is_expanded: node.is_expanded,
            has_checked_for_terminal: node.has_checked_for_terminal,
            is_terminal: node.is_terminal,
            terminal_value: node.terminal_value,
            prior: node.prior,
            visits: node.visits,
            total_value: node.total_value,
        };
    }

    fn ucb_score(&self, parent_visits: f32) -> f32 {
        let mut value = 0.;
        if self.visits > 0. {
            value = if self.state.player {
                1. - self.total_value / self.visits
            } else {
                self.total_value / self.visits
            };
        }
        value + 5. * self.prior * parent_visits.sqrt() / (self.visits + 1e-6)
    }

    fn choose_child(&mut self) -> (&mut Node, usize) {
        let mut result: Option<&mut Node> = None;
        let mut index = 0;
        let mut max = f32::MIN;

        for (i, child) in self.children.iter_mut().enumerate() {
            let score = child.ucb_score(self.visits);
            if score > max {
                max = score;
                result = Some(child);
                index = i;
            }
        }
        (result.unwrap(), index)
    }
}

fn expand(
    nodes: &mut Vec<&mut Node>,
    force: bool,
    policy_nn: &PolicyNetwork,
) -> Result<(), ShapeError> {
    let mut inputs = Array2::<f32>::zeros((0, 837));
    for node in nodes.iter() {
        inputs.push(Axis(0), node.input.view())?;
    }

    let all_priors_diff = policy_nn.forward(neuronika::from_ndarray(inputs));
    all_priors_diff.forward();
    let all_priors = all_priors_diff.data();

    for (i, node) in nodes.iter_mut().enumerate() {
        // If already expanded, then do nothing
        if node.is_expanded {
            continue;
        }

        // If already not visited or terminal, then we don't expand further,
        // except we force an expansion
        if (node.visits == 0. || node.is_terminal) && !force {
            continue;
        }

        let moves = node.state.legal_moves();
        let num_moves = moves.len() as f32;

        let mut legal_priors: Vec<f32> = vec![];
        let mut legal_priors_sum = 0.;
        for m in moves.iter() {
            let v = all_priors[[i, m.index().0]];
            legal_priors.push(v);
            legal_priors_sum += v;
        }

        for (j, m) in moves.iter().enumerate() {
            let normalized_prior = if legal_priors_sum > 0. {
                legal_priors[j] / legal_priors_sum
            } else {
                1. / num_moves
            };

            node.children.push(Node::new(
                node.tree_id,
                node.state.make_move(m, true),
                normalized_prior,
            ))
        }

        node.is_expanded = true
    }

    Ok(())
}

fn iteration(
    nodes: &mut Vec<Node>,
    value_nn: &ValueNetwork,
    policy_nn: &PolicyNetwork,
) -> Result<(), ShapeError> {
    let mut search_paths: Vec<Vec<usize>> = vec![];
    let mut leaf_nodes: Vec<&mut Node> = vec![];

    // Find child to explore
    for node in nodes.iter_mut() {
        let mut n = node;
        let mut search_path = vec![];
        while n.is_expanded {
            let (child, index) = n.choose_child();
            n = child;
            search_path.push(index);
        }
        search_paths.push(search_path);
        leaf_nodes.push(n);
    }

    // Try to expand notes where possible and choose one of the children
    expand(&mut leaf_nodes, false, policy_nn)?;

    let mut evaluation_nodes: Vec<&mut Node> = vec![];
    for (i, node) in leaf_nodes.iter_mut().enumerate() {
        if node.is_expanded {
            let (child, index) = node.choose_child();
            search_paths[i].push(index);
            evaluation_nodes.push(child);
        } else {
            evaluation_nodes.push(node);
        }
    }

    let mut inputs = Array2::<f32>::zeros((0, 837));
    for node in evaluation_nodes.iter() {
        inputs.push(Axis(0), node.input.view())?;
    }

    let values_diff = value_nn.forward(neuronika::from_ndarray(inputs));
    values_diff.forward();
    let predicted_values = values_diff.data();

    let mut values: Vec<f32> = vec![];
    for (i, node) in evaluation_nodes.iter_mut().enumerate() {
        if node.is_terminal {
            values.push(node.terminal_value);
        } else if !node.has_checked_for_terminal {
            node.has_checked_for_terminal = true;
            let game = &mut node.state;
            match game.result() {
                Some(result) => {
                    node.is_terminal = true;
                    node.terminal_value = match result {
                        // TODO: Is it right to use 1&0, or should it be 1&-1 ???
                        GameResult::White => 1.,
                        GameResult::Black => 0.,
                        _ => 0.5,
                    };
                    values.push(node.terminal_value);
                }
                None => values.push(predicted_values[[i, 0]]),
            }
        } else {
            values.push(predicted_values[[i, 0]])
        }
    }

    // Backpropagate
    for (i, path) in search_paths.iter().enumerate() {
        let mut node = nodes.get_mut(i).unwrap();
        node.total_value += values[i];
        node.visits += 1.;
        for j in path {
            node = node.children.get_mut(*j).unwrap();
            node.total_value += values[i];
            node.visits += 1.;
        }
    }

    Ok(())
}

fn choose_random(cdf: Vec<f32>) -> usize {
    let mut rng = rand::thread_rng();
    let sum = cdf.iter().sum::<f32>();
    let random = rng.gen_range(0.0..sum);

    let mut cum = 0.;
    for (i, num) in cdf.iter().enumerate() {
        cum += num;
        if cum > random {
            return i;
        }
    }

    unreachable!()
}

const RUNS: u32 = 1600;

fn find_best_moves<'a>(
    nodes: &'a mut Vec<Node>,
    greedy: bool,
    value_nn: &'a ValueNetwork,
    policy_nn: &'a PolicyNetwork,
) -> Result<(Vec<&'a Node>, Array2<f32>), ShapeError> {
    for _ in 0..RUNS {
        iteration(nodes, value_nn, policy_nn)?;
    }

    let mut new_nodes: Vec<&Node> = vec![];
    let mut policies = Array2::<f32>::zeros((0, 1972));
    for node in nodes.iter() {
        let mut policy = Array1::<f32>::zeros(1972);

        let mut best: Option<&Node> = None;
        let mut cdf: Vec<f32> = vec![];

        for child in &node.children {
            // println!(
            //     "{}\t{}\t{}\t{}",
            //     child.state.last_move.as_ref().unwrap(),
            //     child.prior,
            //     child.visits,
            //     child.total_value
            // );
            policy[child.state.last_move.as_ref().unwrap().0] = child.visits;
            cdf.push(child.visits);
            if best.is_none() || child.visits > best.unwrap().visits {
                best = Some(child);
            }
        }

        if greedy {
            new_nodes.push(best.unwrap());
        } else {
            new_nodes.push(&node.children[choose_random(cdf)]);
        }

        let policy_sum = policy.sum();
        policies.push(Axis(0), (policy / policy_sum).view())?;
    }
    Ok((new_nodes, policies))
}

fn save(
    run_index: String,
    input_strings: Vec<Vec<String>>,
    terminal_values: Vec<f32>,
    policy_strings: Vec<Vec<String>>,
) -> std::io::Result<()> {
    let mut value_network_data_file = File::create(format!("value.{}.csv", run_index))?;
    let mut policy_network_data_file = File::create(format!("policy.{}.csv", run_index))?;
    for (i, strings) in input_strings.iter().enumerate() {
        for (j, string) in strings.iter().enumerate() {
            value_network_data_file
                .write(format!("{},{}\n", string, terminal_values[i]).as_bytes())?;
            policy_network_data_file
                .write(format!("{},{}\n", string, policy_strings[i][j]).as_bytes())?;
        }
    }
    Ok(())
}

pub fn run(run_index: String, parallel_games: usize) -> Result<(), ShapeError> {
    let value_nn = match value_network::load() {
        Ok(nn) => nn,
        Err(err) => panic!("Error loading the value network: {:?}", err),
    };
    let policy_nn = match policy_network::load() {
        Ok(nn) => nn,
        Err(err) => panic!("Error loading the policy network: {:?}", err),
    };

    value_nn.eval();
    policy_nn.eval();

    let mut input_strings: Vec<Vec<String>> = vec![];
    let mut policy_strings: Vec<Vec<String>> = vec![];
    let mut terminal_values: Vec<f32> = vec![];
    let mut logs: Vec<String> = vec![];
    let mut roots: Vec<Node> = vec![];

    for i in 0..parallel_games {
        input_strings.push(vec![]);
        policy_strings.push(vec![]);
        terminal_values.push(-999.);
        logs.push(String::from(""));
        roots.push(Node::new(
            i,
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            0.,
        ))
    }

    expand(&mut roots.iter_mut().collect(), true, &policy_nn)?;

    let mut counter = 0;
    while roots.len() > 0 {
        let (new_roots, policies) = find_best_moves(&mut roots, false, &value_nn, &policy_nn)?;

        for (i, root) in new_roots.iter().enumerate() {
            input_strings[root.tree_id].push(
                root.input
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
            policy_strings[root.tree_id].push(
                policies
                    .row(i)
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }

        let mut roots_to_continue: Vec<Node> = vec![];
        for root in new_roots {
            if root.is_terminal {
                terminal_values[root.tree_id] = root.terminal_value;
                logs[root.tree_id] = root.terminal_value.to_string();
            } else {
                roots_to_continue.push(Node::from_node(root));
                logs[root.tree_id] = format!("{}", root.state.last_move.as_ref().unwrap());
            }
        }

        roots = roots_to_continue;
        counter += 1;
        println!("{}\t{}", counter, logs.join("\t"));
    }

    match save(run_index, input_strings, terminal_values, policy_strings) {
        Ok(_) => Ok(()),
        Err(_) => panic!("Error saving training data"),
    }
}
