use crate::{
    block::block_kind,
    game::{
        fix_block, hard_drop, move_block, rotate_right, Field, Game, Position, FIELD_HEIGHT,
        FIELD_WIDTH,
    },
};

pub fn eval(game: &Game) -> Game {
    let mut elite = (game.clone(), 0, FIELD_HEIGHT);

    // 全回転
    for rotate_count in 0..=3 {
        let mut game = game.clone();
        for _ in 0..rotate_count {
            rotate_right(&mut game);
        }
        // 全横移動
        for dx in -4..5 {
            let mut game = game.clone();
            let new_pos = Position {
                x: match game.pos.x as isize + dx {
                    (..=0) => 0,
                    x => x as usize,
                },
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);
            hard_drop(&mut game);
            fix_block(&mut game);

            // インプット情報の取得
            let line = erase_line_count(&game.field);
            let height_max = field_height_max(&game.field);

            // インプット情報の評価
            if line >= elite.1 && height_max <= elite.2 {
                // 一番良い個体を記録
                elite = (game, line, height_max);
            }
        }
    }

    elite.0
}

/// 消去可能なラインの数を数える
#[allow(clippy::needless_range_loop)]
fn erase_line_count(field: &Field) -> usize {
    let mut count = 0;
    for y in 1..(FIELD_HEIGHT - 2) {
        let mut can_erase = true;
        for x in 2..(FIELD_WIDTH - 2) {
            if field[y][x] == block_kind::NONE {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
        }
    }
    count
}

/// フィールドの一番高いブロックの高さを数える
/// ブロックが何もない場合は0を返す
#[allow(clippy::needless_range_loop)]
fn field_height_max(field: &Field) -> usize {
    for y in 1..(FIELD_HEIGHT - 2) {
        for x in 2..(FIELD_WIDTH - 2) {
            if field[y][x] != block_kind::NONE {
                return FIELD_HEIGHT - y - 1;
            }
        }
    }
    0
}
