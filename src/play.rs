use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use getch_rs::{Getch, Key};
use rand::Rng;

use crate::game::{
    draw, gameover, hard_drop, hold, is_collision, landing, move_block, quit, rotate_left,
    rotate_right, Game, Position,
};

/// 通常プレイ
pub fn normal() -> ! {
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
                // 10ライン消すごとに、100ミリ秒速くすることにする
                let sleep_msec =
                    match 1000u64.saturating_sub((game.lock().unwrap().line as u64 / 10) * 100) {
                        0 => 100,
                        msec => msec,
                    };
                sleep(Duration::from_millis(sleep_msec));

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
            Ok(Key::Char(' ')) => {
                let mut game = game.lock().unwrap();
                hold(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => quit(),
            _ => {}
        }
    }
}

/// オートプレイ
pub fn auto() -> ! {
    // ゲームの初期化
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // 初回描画(フィールドの描画)
    draw(&game.lock().unwrap());

    // 自動操作処理
    thread::spawn(move || {
        loop {
            // 100ミリ秒毎に何かする
            thread::sleep(Duration::from_millis(100));

            // 必要な変数の取得
            let mut game = game.lock().unwrap();

            // 20%くらいの確立でホールド
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..5) == 0 {
                hold(&mut game);
            }

            // ランダムに回転
            for _ in 0..rng.gen_range(0..=3) {
                rotate_right(&mut game);
            }

            // ランダムに横移動
            let dx: isize = rng.gen_range(-4..=5);
            let new_pos = Position {
                x: (game.pos.x as isize + dx) as usize,
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);

            // ハードドロップ
            hard_drop(&mut game);
            if landing(&mut game).is_err() {
                // ブロックを生成できないならゲームオーバー
                gameover(&game);
            }
            draw(&game);
        }
    });

    // キー入力処理
    let g = Getch::new();
    loop {
        if let Ok(Key::Char('q')) = g.getch() {
            quit()
        }
    }
}
