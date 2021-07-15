use core::num;

use common::prelude::*;

use crate::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-13.txt");
    let part_one = part_one(input);
    Ok((part_one, PartAnswer::default()))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();
    let mut computer = Computer::from_program(program);

    computer.step_until_halt();

    let mut num_blocks = 0;

    for chunk in computer.get_outputs().chunks(3) {
        let x = chunk[0];
        let y = chunk[1];
        let tile_type: TileType = chunk[2].into();

        if tile_type == TileType::Block {
            num_blocks += 1;
        }
    }

    PartAnswer::new(num_blocks, start.elapsed().unwrap())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i128> for TileType {
    fn from(i: i128) -> TileType {
        match i {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::HorizontalPaddle,
            4 => TileType::Ball,
            _ => panic!(format!("cannot convert {} to TileType", i)),
        }
    }
}
