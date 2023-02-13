use crate::block::{BlockKind, BLOCKS};

pub const FIELD_WIDTH: usize = 11 + 2;
pub const FIELD_HEIGHT: usize = 20 + 1;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Self {
        Self { x: 4, y: 0 }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockKind,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [
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
            ],
            pos: Position::init(),
            block: rand::random::<BlockKind>(),
        }
    }
}

pub fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
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

pub fn draw(Game { field, pos, block }: &Game) {
    // 裏データの生成
    let mut field_buf = *field;

    // 裏データの更新
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
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

/// ブロックをフィールドに固定する
pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field[y + pos.y][x + pos.x] = 1;
            }
        }
    }
}

/// ラインが揃っているかチェックし、揃っている場合は削除する
pub fn erace_line(field: &mut Field) {
    for y in 1..FIELD_HEIGHT - 1 {
        // ラインが揃っているかチェック
        let mut can_erace = true;
        for x in 1..(FIELD_WIDTH - 1) {
            if field[y][x] == 0 {
                can_erace = false;
                break;
            }
        }

        // ラインを削除
        if can_erace {
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
}

/// ブロックを指定した座標へ移動できるなら移動する
pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, game.block) {
        game.pos = new_pos;
    }
}

/// ブロックを生成する
/// 生成に失敗した場合はエラーを返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.block = rand::random();
    if is_collision(&game.field, &game.pos, game.block) {
        Err(())
    } else {
        Ok(())
    }
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
