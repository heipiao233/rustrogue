/*
 * Rogue
 * Exploring the dungeons of doom
 * 
 * Rust-Rogue
 * Copyright (C) 2023 Hei Piao
 * MIT Licensed.
 *
 * Inspired by "Super-Rogue"
 * Copyright (C) 1984 Robert D. Kindelberger
 * All rights reserved.
 *
 * "Super-Rogue" Based on "Rogue: Exploring the Dungeons of Doom"
 * Copyright (C) 1980, 1981 Michael Toy, Ken Arnold and Glenn Wichman
 * All rights reserved.
 *
 * See the file LICENSE.TXT for full copyright and licensing information.
 */

use ncurses::{initscr, noecho, LINES, COLS, getch, endwin, mvwaddch, box_, WINDOW, newwin, wrefresh, curs_set, addstr, getnstr, wclear, erasechar, mvwhline, mvwvline, mvwprintw};
use rand::{random};
use rooms::Room;
mod rooms;
pub fn get_rand(x: i32, y: i32) -> i32 {
    let mut rx = x;
    let mut ry = y;
    if x > y {
        rx = y;
        ry = x;
    }
    random::<i32>() % (ry - rx + 1) + rx
}
pub struct Point {
    x: i32,
    y: i32
}
pub struct GameInfo {
    player_pos: Point,
    info_window: WINDOW,
    game_window: WINDOW,
    // item_window: WINDOW,
    rooms: Vec<Room>,
}
fn main() {
    initscr();
    let mut game: GameInfo = GameInfo {
        player_pos: Point {
            x: (COLS() - 1) / 2,
            y: LINES() / 2
        },
        info_window: newwin(1, COLS(), 0, 0),
        game_window: newwin(24, COLS(), 1, 0),
        // item_window: newwin(3, COLS(), 0, 0),
        rooms: Vec::new()
    };
    let mut name: String = "Rodney".to_string();
    addstr("Rogue's Name? ");
    getnstr(&mut name, 24);
    if name.trim().is_empty() {
        name = "Rodney".to_string();
    }
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    mvwaddch(game.game_window, game.player_pos.y, game.player_pos.x, '&' as u32);
    wclear(game.info_window);
    wrefresh(game.info_window);
    noecho();
    rooms::gen_rooms(&mut game);
    for room in game.rooms {
        let h = room.bottom_right.y - room.top_left.y + 1;
        let w = room.bottom_right.x - room.top_left.x + 1;
        mvwhline(game.game_window, room.top_left.y, room.top_left.x, 0, w);
        mvwhline(game.game_window, room.bottom_right.y, room.top_left.x, 0, w);
        mvwvline(game.game_window, room.top_left.y, room.top_left.x, 0, h);
        mvwvline(game.game_window, room.top_left.y, room.bottom_right.x, 0, h);
        mvwprintw(game.info_window, 0, 0, format!("{} {} {} {}", room.top_left.x, room.top_left.y, room.bottom_right.x, room.bottom_right.y).as_str());
        break;
    }
    wrefresh(game.game_window);
    wrefresh(game.info_window);
    getch();
    // loop {
    //     // mvwprintw(game.info_window, 0, 0, game.player_pos.x.to_string().as_str());
    //     // wrefresh(game.info_window);
    //     match char::from_u32(getch() as u32) {
    //         Some('h') => do_move(&mut game, -1, 0),
    //         Some('l') => do_move(&mut game, 1, 0),
    //         Some('k') => do_move(&mut game, 0, -1),
    //         Some('j') => do_move(&mut game, 0, 1),
    //         Some('Q') => break,
    //         _ => {},
    //     }
    //     wrefresh(game.game_window);
    // }
    endwin();
}

fn do_move(game: &mut GameInfo, delta_x: i32, delta_y: i32) {
    mvwaddch(game.game_window, game.player_pos.y, game.player_pos.x, ' ' as u32);
    game.player_pos.x += delta_x;
    game.player_pos.y += delta_y;
    mvwaddch(game.game_window, game.player_pos.y, game.player_pos.x, '&' as u32);
}
