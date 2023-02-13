use getch_rs::{Getch, Key};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{thread::sleep, time::Duration};

const FIELD_WIDTH: usize = 11 + 2;
const FIELD_HEIGHT: usize = 20 + 1;
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Debug, Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
    // I
    [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],
    // O
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    // S
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
    ],
    // Z
    [
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    // J
    [
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    // L
    [
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    // T
    [
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
];

struct Position {
    x: usize,
    y: usize,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            // ブロックの範囲外は無視
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            };

            // ブロックがすでにある場所への衝突、フィールドの外壁への衝突の場合はTrueを返す
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            };
        }
    }
    false
}

fn draw(field: &Field, pos: &Position, block: BlockKind) {
    // 裏データの生成
    let mut field_buf = field.clone();

    // 裏データの更新
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }

    // 裏データの描画
    println!("\x1b[H"); // カーソルを先頭へ移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn main() {
    let field = Arc::new(Mutex::new([
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // 初回描画(フィールドの描画)
    draw(
        &field.lock().unwrap(),
        &pos.lock().unwrap(),
        *block.lock().unwrap(),
    );

    // 自然落下処理
    {
        let field = Arc::clone(&field);
        let block = Arc::clone(&block);
        let pos = Arc::clone(&pos);

        thread::spawn(move || {
            loop {
                // 1秒待機
                sleep(Duration::from_millis(1000));

                // 必要な変数の取得
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut pos = pos.lock().unwrap();

                // 自然落下
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    // ブロックの移動
                    *pos = new_pos;
                } else {
                    // ブロックの固定
                    for y in 0..4 {
                        for x in 0..4 {
                            if BLOCKS[*block as usize][y][x] == 1 {
                                field[pos.y + y][pos.x + x] = 1;
                            }
                        }
                    }

                    // ブロックを初期座標へ移動
                    *pos = Position { x: 4, y: 0 };
                    *block = rand::random();
                }

                // 裏データの描画
                draw(&field, &pos, *block);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x.checked_sub(1).unwrap_or(pos.x),
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Right) => {
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Down) => {
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h"); // カーソルを再表示
                break;
            }
            _ => {}
        }
    }
}
