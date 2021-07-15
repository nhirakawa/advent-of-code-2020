use common::prelude::*;

use crate::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-13.txt");
    let part_one = part_one(input);
    Ok((part_one, PartAnswer::default()))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();
    let mut arcade_cabinet = ArcadeCabinet::new(program);

    arcade_cabinet.play();

    let num_blocks = arcade_cabinet.count_number_of_blocks();

    PartAnswer::new(num_blocks, start.elapsed().unwrap())
}

fn part_two(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut arcade_cabinet = ArcadeCabinet::new(program);

    arcade_cabinet.insert_quarters();
    arcade_cabinet.play();

    PartAnswer::new(0, start.elapsed().unwrap())
}

struct ArcadeCabinet {
    computer: Computer,
    last_score: i128,
    last_ball_position: Option<(i32, i32)>,
    last_paddle_position: Option<(i32, i32)>,
}

impl ArcadeCabinet {
    fn new(program: &str) -> ArcadeCabinet {
        let computer = Computer::from_program(program);

        ArcadeCabinet {
            computer,
            last_score: 0,
            last_ball_position: None,
            last_paddle_position: None,
        }
    }

    fn insert_quarters(&mut self) {
        self.computer.set(0, 2);
    }

    fn play(&mut self) {
        self.computer.step_until_halt();
    }

    fn count_number_of_blocks(&mut self) -> usize {
        let mut num_blocks = 0;

        for chunk in self.computer.get_outputs().chunks(3) {
            let x = chunk[0];
            let y = chunk[1];
            let tile_id = chunk[2];

            if (x, y) == (-1, 0) {
                self.last_score = tile_id;
            }

            if TileType::Block == tile_id.into() {
                num_blocks += 1;
            }
        }

        num_blocks
    }
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
