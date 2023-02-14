use crate::block::{
    block_kind::{self, WALL as W},
    BlockColor, BlockKind, BlockShape, BLOCKS, COLOR_TABLE,
};

pub const FIELD_WIDTH: usize = 11 + 2 + 2; //  フィールド + 壁 + 番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1; // フィールド + 底 + 番兵
pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

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
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
}

impl Game {
    pub fn new() -> Game {
        Game {
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
        }
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

#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, block }: &Game) {
    // 裏データの生成
    let mut field_buf = *field;

    // 裏データの更新
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }

    // 裏データの描画
    println!("\x1b[H"); // カーソルを先頭へ移動
    for y in 0..(FIELD_HEIGHT - 1) {
        for x in 0..(FIELD_WIDTH - 1) {
            print!("{}", COLOR_TABLE[field_buf[y][x]])
        }
        println!();
    }
}

/// ブロックをフィールドに固定する
pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }
}

/// ラインが揃っているかチェックし、揃っている場合は削除する
pub fn erace_line(field: &mut Field) {
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
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
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
    }
}

/// ブロックを生成する
/// 生成に失敗した場合はエラーを返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.block = BLOCKS[rand::random::<BlockKind>() as usize];
    if is_collision(&game.field, &game.pos, &game.block) {
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
