use std::collections::VecDeque;

use crate::block::{
    block_kind::{self, WALL as W},
    gen_block_7, BlockColor, BlockKind, BlockShape, BLOCKS, COLOR_TABLE,
};

pub const FIELD_WIDTH: usize = 11 + 2 + 2; //  フィールド + 壁 + 番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1; // フィールド + 底 + 番兵
pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];
pub const NEXT_LENGTH: usize = 3;
pub const SCORE_TABLE: [usize; 5] = [
    0,   // 0段消し
    1,   // 1段消し
    5,   // 2段消し
    25,  // 3段消し
    125, // 4段消し
];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Self {
        Self { x: 5, y: 0 }
    }
}

pub struct Game {
    pub field: Field,                   // フィールドデータ(裏データ)
    pub pos: Position,                  // 現在のブロックの位置
    pub block: BlockShape,              // 現在のブロック
    pub hold: Option<BlockShape>,       // ホールドしたブロック
    pub holded: bool,                   // ホールド済みか
    pub next: VecDeque<BlockShape>,     // 次のブロック(3つ)
    pub next_buf: VecDeque<BlockShape>, // 次のブロックのバッファ(1~7つ)
    pub score: usize,                   // 現在のスコア
    pub line: usize,                    // 消したライン数
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: [
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, W, W, W, W, W, W, W, W, W, W, W, W, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
            hold: None,
            holded: false,
            next: gen_block_7().into(),
            next_buf: gen_block_7().into(),
            score: 0,
            line: 0,
        };
        // 初期ブロックを供給
        spawn_block(&mut game).ok();
        game
    }
}

pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            // ブロックの範囲外は無視
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            };

            // ブロックがすでにある場所への衝突、フィールドの外壁への衝突の場合はTrueを返す
            if block[y][x] != block_kind::NONE && field[y + pos.y][x + pos.x] != block_kind::NONE {
                // ブロックとフィールドのどちらも何かしらのブロックがある場合は衝突してる
                return true;
            };
        }
    }
    false
}

fn get_ghost_pos(field: &Field, pos: &Position, block: &BlockShape) -> Position {
    let mut ghost_pos = *pos;
    loop {
        let new_pos = Position {
            x: ghost_pos.x,
            y: ghost_pos.y + 1,
        };
        if is_collision(field, &new_pos, block) {
            break;
        } else {
            ghost_pos = new_pos;
        }
    }
    ghost_pos
}

#[allow(clippy::needless_range_loop)]
pub fn draw(
    Game {
        field,
        pos,
        block,
        hold,
        next,
        score,
        ..
    }: &Game,
) {
    // 裏データの生成
    let mut field_buf = *field;

    // 裏データにゴーストブロックを書き込む
    let ghost_pos = get_ghost_pos(field, pos, block);
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + ghost_pos.y][x + ghost_pos.x] = block_kind::GHOST;
            }
        }
    }

    // 裏データにブロックを書き込む
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }

    // ホールドを描画
    println!("\x1b[2;28HHOLD");
    if let Some(hold) = hold {
        for y in 0..4 {
            print!("\x1b[{};28H", y + 3);
            for x in 0..4 {
                print!("{}", COLOR_TABLE[hold[y][x]]);
            }
            println!();
        }
    }

    // 3つのネクストブロックたちを描画
    println!("\x1b[8;28HNEXT"); // カーソルをネクスト位置に移動
    for (i, next) in next.iter().take(NEXT_LENGTH).enumerate() {
        for y in 0..4 {
            print!("\x1b[{};28H", i * 4 + y + 9); // カーソルを移動
            for x in 0..4 {
                print!("{}", COLOR_TABLE[next[y][x]]);
            }
            println!();
        }
    }

    // スコアを描画
    println!("\x1b[22;28H{score}");

    // 裏データの描画
    println!("\x1b[H"); // カーソルを先頭へ移動
    for y in 0..(FIELD_HEIGHT - 1) {
        for x in 1..(FIELD_WIDTH - 1) {
            print!("{}", COLOR_TABLE[field_buf[y][x]])
        }
        println!();
    }

    // 色情報をリセット
    println!("\x1b[0m");
}

/// ブロックをフィールドに固定する
pub fn fix_block(
    Game {
        field, pos, block, ..
    }: &mut Game,
) {
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }
}

/// ホールド処理
/// - 1回目のホールドは現在のブロックをホールド
/// - 2回目以降のホールドは現在のブロックとホールドを交換
/// - 現在のブロックに対して既にホールドしている場合は何もしない
pub fn hold(game: &mut Game) {
    if game.holded {
        // 現在のブロックに対して既にホールドしている場合は何もしない
        return;
    }
    if let Some(mut hold) = game.hold {
        // ホールドの交換
        std::mem::swap(&mut hold, &mut game.block);
        game.hold = Some(hold);
        game.pos = Position::init();
    } else {
        // ホールドして、新しいブロックを生成
        game.hold = Some(game.block);
        spawn_block(game).ok();
    }
    // ホールドしたのでフラグを立てる
    game.holded = true;
}

/// ラインが揃っているかチェックし、揃っている場合は削除する
/// return: 消したライン数
pub fn erace_line(field: &mut Field) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        // ラインが揃っているかチェック
        let mut can_erace = true;
        for x in 2..(FIELD_WIDTH - 2) {
            if field[y][x] == 0 {
                can_erace = false;
                break;
            }
        }

        // ラインを削除
        if can_erace {
            count += 1;
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
    count
}

/// ブロックを指定した座標へ移動できるなら移動する
pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.block) {
        game.pos = new_pos;
    }
}

/// 右に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.block[4 - 1 - x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.block = new_shape;
    }
}

/// 左に90度回転する
pub fn rotate_left(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[4 - 1 - x][y] = game.block[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.block = new_shape;
    }
}

/// スーパーローテーション
fn super_rotation(field: &Field, pos: &Position, block: &BlockShape) -> Result<Position, ()> {
    let diff_pos = [
        // 上へ1マスずらす
        Position {
            x: pos.x,
            y: pos.y.checked_sub(1).unwrap_or(pos.y),
        },
        // 右へ1マスずらす
        Position {
            x: pos.x.checked_add(1).unwrap_or(pos.x),
            y: pos.y,
        },
        // 下へ1マスずらす
        Position {
            x: pos.x,
            y: pos.y.checked_add(1).unwrap_or(pos.y),
        },
        // 左へ1マスずらす
        Position {
            x: pos.x.checked_sub(1).unwrap_or(pos.x),
            y: pos.y,
        },
        // 上へ2マスずらす
        Position {
            x: pos.x,
            y: pos.y.checked_sub(2).unwrap_or(pos.y),
        },
        // 右へ2マスずらす
        Position {
            x: pos.x.checked_add(2).unwrap_or(pos.x),
            y: pos.y,
        },
        // 下へ2マスずらす
        Position {
            x: pos.x,
            y: pos.y.checked_add(2).unwrap_or(pos.y),
        },
        // 左へ2マスずらす
        Position {
            x: pos.x.checked_sub(2).unwrap_or(pos.x),
            y: pos.y,
        },
    ];
    for pos in diff_pos {
        if !is_collision(field, &pos, block) {
            return Ok(pos);
        }
    }
    Err(())
}

/// ブロックを生成する
/// 生成に失敗した場合はエラーを返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.block = game.next.pop_front().unwrap();
    if let Some(next) = game.next_buf.pop_front() {
        // バフからネクストキューに追加
        game.next.push_back(next);
    } else {
        // バフを生成
        game.next_buf = gen_block_7().into();
        // バフからネクストキューに追加
        game.next.push_back(game.next_buf.pop_front().unwrap());
    }
    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}

pub fn hard_drop(game: &mut Game) {
    loop {
        let new_pos = Position {
            x: game.pos.x,
            y: game.pos.y + 1,
        };
        if is_collision(&game.field, &new_pos, &game.block) {
            break;
        } else {
            game.pos = new_pos;
        }
    }
    move_block(game, game.pos);
}

/// ブロックが着地したときの処理
pub fn landing(game: &mut Game) -> Result<(), ()> {
    fix_block(game);
    let line = erace_line(&mut game.field);
    game.score += SCORE_TABLE[line];
    game.line += line;
    spawn_block(game)?;
    game.holded = false;
    Ok(())
}

/// 盤面を描画し、ゲームオーバーを表示し、プログラムを終了する
pub fn gameover(game: &Game) -> ! {
    draw(game);
    println!("Game Over!");
    quit();
}

/// カーソルを表示し、プログラムを終了する
pub fn quit() -> ! {
    println!("\x1b[?25h"); // カーソルを表示
    std::process::exit(0);
}
