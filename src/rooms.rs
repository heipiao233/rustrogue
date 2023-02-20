use super::{get_rand, GameInfo, Point};

pub struct Room {
    pub top_left: Point,
    pub bottom_right: Point,
}

pub fn gen_rooms(game: &mut GameInfo) {
    let mut must_exist_1 = 0;
    let mut must_exist_2 = 0;
    let mut must_exist_3 = 0;
    let seed = get_rand(0, 5) as u8;
    match seed {
        0..=2 => {
            must_exist_1 = seed * 3;
            must_exist_2 = must_exist_1 + 1;
            must_exist_3 = must_exist_2 + 1;
        }
        3..=5 => {
            must_exist_1 = seed - 3;
            must_exist_2 = seed;
            must_exist_3 = seed + 3;
        }
        _ => {}
    }
    for i in 0..8 {
        if i != must_exist_1 && i != must_exist_2 && i != must_exist_3 && get_rand(1, 100) <= 40 {
            game.rooms.push(gen_room(i));
        }
    }
}

fn gen_room(id: u8) -> Room {
    let mut left_col = match id % 3 {
        0 => 0,
        1 => 27,
        2 => 53,
        _ => 0,
    };
    let mut top_row = match id / 3 {
        0 => 1,
        1 => 8,
        2 => 16,
        _ => 0,
    };
    let mut right_col = match id % 3 {
        0 => 25,
        1 => 51,
        2 => 79,
        _ => 0,
    };
    let mut bottom_row = match id / 3 {
        0 => 6,
        1 => 14,
        2 => 22,
        _ => 0,
    };
    let height = get_rand(4, bottom_row - top_row + 1);
    let width = get_rand(7, right_col - left_col - 2);
    let row_offset = get_rand(0, (bottom_row - top_row) - height + 1);
    let col_offset = get_rand(0, (right_col - left_col) - width + 1);
    top_row += row_offset;
    bottom_row = top_row + height - 1;
    left_col += col_offset;
    right_col = left_col + width - 1;
    return Room {
        top_left: Point {
            x: left_col,
            y: top_row
        },
        bottom_right: Point {
            x: right_col,
            y: bottom_row
        }
    }
}
