mod block;
mod game;

use game::{Game, Position};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{thread::sleep, time::Duration};

use crate::game::{
    draw, erace_line, fix_block, gameover, hard_drop, is_collision, landing, move_block, quit,
    rotate_left, rotate_right, spawn_block,
};

fn main() {
    // ゲームの初期化
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // 初回描画(フィールドの描画)
    draw(&game.lock().unwrap());

    // 自然落下処理
    {
        let game = Arc::clone(&game);

        thread::spawn(move || {
            loop {
                // 1秒待機
                sleep(Duration::from_millis(1000));

                // 必要な変数の取得
                let mut game = game.lock().unwrap();

                // 自然落下
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, &game.block) {
                    // ブロックの移動
                    game.pos = new_pos;
                } else if landing(&mut game).is_err() {
                    gameover(&game);
                }

                // 裏データの描画
                draw(&game);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Up) => {
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    gameover(&game);
                }
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => quit(),
            _ => {}
        }
    }
}
