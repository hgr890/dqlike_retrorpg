use macroquad::prelude::*;

// ─── 定数 ────────────────────────────────────────────────────────────────────
const TILE: f32 = 32.0;
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const MSG_SPEED: u32 = 2;

// ─── タイル種類 ──────────────────────────────────────────────────────────────
// 0=草, 1=木, 2=石壁, 3=床, 4=扉, 5=水
// 6=城壁, 7=城床, 8=城扉(内部用)
// 9=村建物(フィールド), 10=城建物(フィールド), 11=井戸(フィールド)

// ─── 家マップ (10×9) ─────────────────────────────────────────────────────────
const HOUSE_MAP: &[&[u8]] = &[
    &[2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 2, 2, 2, 4, 4, 2, 2, 2, 2],
];
const HOUSE_PLAYER_START: (i32, i32) = (4, 4);
const HOUSE_EXIT_TILES: &[(i32, i32)] = &[(4, 8), (5, 8)];

// ─── 井戸マップ (10×9) ───────────────────────────────────────────────────────
const WELL_MAP: &[&[u8]] = &[
    &[2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 5, 5, 3, 3, 3, 3, 3, 2],
    &[2, 3, 5, 5, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
    &[2, 2, 2, 2, 4, 4, 2, 2, 2, 2],
];
const WELL_PLAYER_START: (i32, i32) = (5, 7);
const WELL_EXIT_TILES: &[(i32, i32)] = &[(4, 8), (5, 8)];
const HOIMIN_POS: (i32, i32) = (7, 4);

// ─── 村マップ (20×15) ────────────────────────────────────────────────────────
const VILLAGE_MAP: &[&[u8]] = &[
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 2, 3, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 2, 3, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 2, 2, 4, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];
const VILLAGE_W: i32 = 20;
const VILLAGE_H: i32 = 15;
const VILLAGE_PLAYER_START: (i32, i32) = (5, 7);
const OLD_MAN_POS: (i32, i32) = (10, 8);
const VILLAGE_HOUSE_DOOR: (i32, i32) = (5, 6);

// ─── ワールドマップ (20×15) ──────────────────────────────────────────────────
// 20×15 × 32px = 640×480 → 画面にぴったり収まる・スクロール不要
//
// 配置:
//   村(タイル9) : (5,3)(6,3) … 2マス
//   井戸(タイル11): (15,4) … 1マス
//   城(タイル10): (12,9)(13,9)(12,10)(13,10) … 2×2マス
//   エンカウントゾーン: 行5〜8 の草地
//
const WORLD_MAP: &[&[u8]] = &[
    //0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //  0
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  1
    &[1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1], //  2
    &[1, 0, 0, 0, 0, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  3  ← 村(5,3)(6,3)
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 1], //  4  ← 井戸(15,4)
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  5  ← エンカウント
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  6  ← エンカウント
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  7  ← エンカウント
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], //  8  ← エンカウント
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 1], //  9  ← 城(12,9)(13,9)
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 1], // 10  ← 城(12,10)(13,10)
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], // 11
    &[1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1], // 12
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], // 13
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // 14
];
// 村・井戸から出た直後のフィールド座標
const WORLD_PLAYER_FROM_VILLAGE: (i32, i32) = (6, 4);
const WORLD_PLAYER_FROM_WELL: (i32, i32) = (15, 5);
// エンカウントゾーン
const ENCOUNTER_TILES: &[(i32, i32)] = &[
    (5, 5),
    (6, 5),
    (7, 5),
    (8, 5),
    (9, 5),
    (10, 5),
    (11, 5),
    (12, 5),
    (13, 5),
    (5, 6),
    (6, 6),
    (7, 6),
    (8, 6),
    (9, 6),
    (10, 6),
    (11, 6),
    (12, 6),
    (13, 6),
    (5, 7),
    (6, 7),
    (7, 7),
    (8, 7),
    (9, 7),
    (10, 7),
    (11, 7),
    (12, 7),
    (13, 7),
    (5, 8),
    (6, 8),
    (7, 8),
    (8, 8),
    (9, 8),
    (10, 8),
    (11, 8),
    (12, 8),
    (13, 8),
];

// ─── 型定義 ──────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
enum MapId {
    House,
    Village,
    World,
    Castle,
    Well,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum GamePhase {
    Title,
    Playing,
    Ending,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum BattlePhase {
    PlayerTurn,
    AllyTurn,
    EnemyTurn,
    PlayerWin,
    PlayerLose,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum EnemyKind {
    HagureMetaru,
    Maou,
    TrueMaou,
}

struct Enemy {
    kind: EnemyKind,
    hp: i32,
    max_hp: i32,
    atk: i32,
}
struct Player {
    x: i32,
    y: i32,
    hp: i32,
    max_hp: i32,
    atk: i32,
    move_cd: u32,
}

struct Battle {
    enemy: Enemy,
    phase: BattlePhase,
    log: Vec<String>,
    cursor: usize,
    anim_timer: u32,
    log_timer: u32,
    fled: bool,
    maou_defeated: bool,
    defended: bool,
    ally_joined: bool,
}

struct MsgBox {
    lines: Vec<String>,
    visible_chars: usize,
    char_timer: u32,
    done: bool,
}

struct GameState {
    phase: GamePhase,
    map_id: MapId,
    player: Player,
    battle: Option<Battle>,
    msg: Option<MsgBox>,
    camera_x: f32,
    camera_y: f32,
    maou_defeated: bool,
    hagure_defeated: bool,
    hoimin_joined: bool,
    ending_timer: u32,
}

// ─── フォントヘルパー ─────────────────────────────────────────────────────────
fn txt(text: &str, x: f32, y: f32, size: f32, color: Color, font: Option<&Font>) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size: size as u16,
            color,
            ..Default::default()
        },
    );
}

// ─── マップヘルパー ──────────────────────────────────────────────────────────
fn tile_at(map: &[&[u8]], x: i32, y: i32) -> u8 {
    if y < 0 || y >= map.len() as i32 {
        return 1;
    }
    let row = map[y as usize];
    if x < 0 || x >= row.len() as i32 {
        return 1;
    }
    row[x as usize]
}

fn is_walkable(tile: u8) -> bool {
    // 9=村建物, 10=城建物, 11=井戸 も歩行可能（乗ったら遷移）
    matches!(tile, 0 | 3 | 4 | 7 | 8 | 9 | 10 | 11)
}

fn get_current_map(map_id: MapId) -> &'static [&'static [u8]] {
    match map_id {
        MapId::House => HOUSE_MAP,
        MapId::Village => VILLAGE_MAP,
        MapId::Well => WELL_MAP,
        MapId::World | MapId::Castle => WORLD_MAP,
    }
}

// 村境界(木の外壁)に向かって歩く → 出口判定
fn is_village_boundary_exit(nx: i32, ny: i32) -> bool {
    nx <= 0 || ny <= 0 || nx >= VILLAGE_W - 1 || ny >= VILLAGE_H - 1
}

// ─── タイル色 ────────────────────────────────────────────────────────────────
fn tile_color(tile: u8) -> Color {
    match tile {
        0 => color_u8!(116, 190, 112, 255),  // 草
        1 => color_u8!(64, 134, 70, 255),    // 木
        2 => color_u8!(178, 156, 128, 255),  // 石壁
        3 => color_u8!(220, 198, 150, 255),  // 床
        4 => color_u8!(186, 126, 72, 255),   // 扉
        5 => color_u8!(82, 146, 210, 255),   // 水
        6 => color_u8!(124, 124, 148, 255),  // 城壁
        7 => color_u8!(186, 178, 170, 255),  // 城床
        8 => color_u8!(170, 110, 62, 255),   // 城扉
        9 => color_u8!(138, 208, 128, 255),  // 村建物(明るい草)
        10 => color_u8!(92, 92, 126, 255),   // 城建物(暗い石)
        11 => color_u8!(116, 190, 112, 255), // 井戸の地面
        _ => BLACK,
    }
}

// ─── タイルシンボル描画 ──────────────────────────────────────────────────────
fn draw_tile_symbol(tile: u8, px: f32, py: f32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    match tile {
        0 => {
            draw_circle(px + 8.0, py + 9.0, 1.7, color_u8!(255, 216, 132, 180));
            draw_circle(px + 22.0, py + 23.0, 1.5, color_u8!(255, 176, 190, 170));
        }
        1 => {
            // 木
            draw_rectangle(cx - 3.0, py + 18.0, 6.0, 10.0, color_u8!(120, 82, 48, 255));
            draw_triangle(
                Vec2::new(cx, py + 4.0),
                Vec2::new(px + 4.0, py + TILE - 6.0),
                Vec2::new(px + TILE - 4.0, py + TILE - 6.0),
                color_u8!(42, 116, 54, 255),
            );
            draw_circle(cx - 4.0, py + 14.0, 2.0, color_u8!(128, 212, 126, 220));
        }
        2 | 6 => {
            // 石壁 / 城壁
            draw_line(
                px,
                py + TILE * 0.5,
                px + TILE,
                py + TILE * 0.5,
                1.0,
                color_u8!(0, 0, 0, 60),
            );
            draw_line(
                px + TILE * 0.5,
                py,
                px + TILE * 0.5,
                py + TILE * 0.5,
                1.0,
                color_u8!(0, 0, 0, 60),
            );
        }
        4 | 8 => {
            // 扉
            draw_rectangle(
                cx - 5.0,
                py + 4.0,
                10.0,
                TILE - 8.0,
                color_u8!(120, 70, 20, 255),
            );
            draw_circle(cx + 3.0, py + TILE * 0.5, 2.5, color_u8!(220, 180, 50, 255));
        }
        5 => {
            // 水の波
            for i in 0..3 {
                let wy = py + 6.0 + i as f32 * 9.0;
                draw_line(
                    px + 4.0,
                    wy,
                    px + 12.0,
                    wy + 3.0,
                    1.5,
                    color_u8!(120, 180, 255, 180),
                );
                draw_line(
                    px + 12.0,
                    wy + 3.0,
                    px + 20.0,
                    wy,
                    1.5,
                    color_u8!(120, 180, 255, 180),
                );
                draw_line(
                    px + 20.0,
                    wy,
                    px + 28.0,
                    wy + 3.0,
                    1.5,
                    color_u8!(120, 180, 255, 180),
                );
            }
        }
        9 => {
            // 村建物 ─ 家アイコン
            // 屋根（三角）
            draw_triangle(
                Vec2::new(cx, py + 3.0),
                Vec2::new(px + 2.0, py + 16.0),
                Vec2::new(px + TILE - 2.0, py + 16.0),
                color_u8!(200, 80, 50, 255),
            );
            // 壁
            draw_rectangle(
                px + 5.0,
                py + 15.0,
                TILE - 10.0,
                TILE - 18.0,
                color_u8!(240, 220, 180, 255),
            );
            // 窓
            draw_rectangle(px + 7.0, py + 18.0, 6.0, 5.0, color_u8!(120, 180, 220, 255));
            // 煙突
            draw_rectangle(
                px + TILE - 10.0,
                py + 5.0,
                4.0,
                10.0,
                color_u8!(130, 120, 110, 255),
            );
        }
        10 => {
            // 城建物 ─ 塔アイコン
            // 塔の胴体
            draw_rectangle(
                px + 6.0,
                py + 12.0,
                TILE - 12.0,
                TILE - 14.0,
                color_u8!(130, 130, 160, 255),
            );
            // 城壁（ギザギザ）
            for i in 0..3u32 {
                let bx = px + 6.0 + i as f32 * 7.0;
                draw_rectangle(bx, py + 4.0, 5.0, 9.0, color_u8!(150, 150, 180, 255));
            }
            // 矢狭間（窓）
            draw_rectangle(cx - 2.0, py + 16.0, 4.0, 7.0, color_u8!(30, 20, 50, 255));
        }
        11 => {
            // 井戸 ─ 1マス表示
            draw_circle(cx, cy, 12.0, color_u8!(108, 98, 112, 255));
            draw_circle(cx, cy, 8.0, color_u8!(42, 82, 128, 255));
            draw_rectangle(px + 7.0, py + 6.0, 18.0, 5.0, color_u8!(188, 136, 82, 255));
            draw_line(
                px + 10.0,
                py + 8.0,
                px + 10.0,
                py + 18.0,
                2.0,
                color_u8!(124, 86, 58, 255),
            );
            draw_line(
                px + 22.0,
                py + 8.0,
                px + 22.0,
                py + 18.0,
                2.0,
                color_u8!(124, 86, 58, 255),
            );
            draw_circle(cx + 3.0, cy - 1.0, 2.0, color_u8!(150, 210, 255, 210));
        }
        _ => {}
    }
}

// ─── マップ描画 ──────────────────────────────────────────────────────────────
fn draw_map(map: &[&[u8]], cam_x: f32, cam_y: f32) {
    for (ry, row) in map.iter().enumerate() {
        for (rx, &tile) in row.iter().enumerate() {
            let px = rx as f32 * TILE - cam_x;
            let py = ry as f32 * TILE - cam_y;
            if px + TILE < 0.0 || px > SCREEN_W || py + TILE < 0.0 || py > SCREEN_H {
                continue;
            }
            draw_rectangle(px, py, TILE, TILE, tile_color(tile));
            draw_tile_symbol(tile, px, py);
            draw_rectangle_lines(px, py, TILE, TILE, 0.5, color_u8!(0, 0, 0, 30));
        }
    }
}

// フィールドのラベル（「村」「城」文字）
fn draw_world_labels(cam_x: f32, cam_y: f32, font: Option<&Font>) {
    // 「村」ラベル: 村建物(5,3)(6,3) の上
    let vx = 5.0 * TILE - cam_x + 6.0;
    let vy = 3.0 * TILE - cam_y - 4.0;
    draw_rectangle(vx - 2.0, vy - 20.0, 50.0, 22.0, color_u8!(0, 0, 0, 160));
    txt("村", vx, vy, 20.0, color_u8!(255, 240, 120, 255), font);

    // 「魔王の城」ラベル: 城建物(12,9)(13,10) の上
    let cx = 11.0 * TILE - cam_x + 2.0;
    let cy = 9.0 * TILE - cam_y - 4.0;
    draw_rectangle(cx - 2.0, cy - 20.0, 90.0, 22.0, color_u8!(0, 0, 0, 160));
    txt(
        "魔王の城",
        cx,
        cy,
        20.0,
        color_u8!(255, 100, 100, 255),
        font,
    );

    let wx = 14.0 * TILE - cam_x + 18.0;
    let wy = 4.0 * TILE - cam_y - 4.0;
    draw_rectangle(wx - 2.0, wy - 20.0, 56.0, 22.0, color_u8!(0, 0, 0, 150));
    txt("井戸", wx, wy, 20.0, color_u8!(150, 220, 255, 255), font);
}

// ─── キャラクター描画 ─────────────────────────────────────────────────────────
fn draw_player(px: f32, py: f32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    draw_circle(cx, cy + 8.0, 8.0, color_u8!(44, 92, 180, 255));
    draw_rectangle(cx - 7.0, cy - 3.0, 14.0, 13.0, color_u8!(64, 124, 220, 255));
    draw_circle(cx, cy - 8.0, 7.0, color_u8!(240, 200, 150, 255));
    draw_circle(cx - 3.0, cy - 9.0, 1.3, color_u8!(40, 50, 80, 255));
    draw_circle(cx + 3.0, cy - 9.0, 1.3, color_u8!(40, 50, 80, 255));
    draw_circle(cx - 5.0, cy - 6.0, 1.8, color_u8!(255, 150, 150, 160));
    draw_circle(cx + 5.0, cy - 6.0, 1.8, color_u8!(255, 150, 150, 160));
    draw_line(
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy + 8.0,
        2.5,
        color_u8!(200, 200, 220, 255),
    );
    draw_line(
        cx + 5.0,
        cy - 3.0,
        cx + 11.0,
        cy - 3.0,
        2.5,
        color_u8!(180, 140, 60, 255),
    );
}

fn draw_hoimin(px: f32, py: f32, timer: u32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0 + (timer as f32 * 0.08).sin() * 2.0;
    draw_circle(cx, cy - 5.0, 10.0, color_u8!(104, 212, 232, 255));
    draw_circle(cx - 3.5, cy - 7.0, 1.8, color_u8!(25, 42, 70, 255));
    draw_circle(cx + 3.5, cy - 7.0, 1.8, color_u8!(25, 42, 70, 255));
    draw_circle(cx - 5.5, cy - 3.0, 2.0, color_u8!(255, 160, 190, 170));
    draw_circle(cx + 5.5, cy - 3.0, 2.0, color_u8!(255, 160, 190, 170));
    draw_line(cx - 4.0, cy + 1.0, cx, cy + 3.0, 1.5, WHITE);
    draw_line(cx, cy + 3.0, cx + 4.0, cy + 1.0, 1.5, WHITE);
    for i in 0..4 {
        let tx = cx - 7.0 + i as f32 * 4.5;
        draw_line(
            tx,
            cy + 5.0,
            tx - 2.0,
            cy + 13.0,
            2.0,
            color_u8!(72, 172, 210, 255),
        );
    }
}

fn draw_old_man(px: f32, py: f32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    draw_rectangle(cx - 6.0, cy - 2.0, 12.0, 12.0, color_u8!(160, 100, 60, 255));
    draw_circle(cx, cy - 8.0, 6.0, color_u8!(230, 190, 140, 255));
    draw_rectangle(
        cx - 5.0,
        cy - 14.0,
        10.0,
        5.0,
        color_u8!(230, 230, 230, 255),
    );
    draw_line(
        cx + 7.0,
        cy - 8.0,
        cx + 7.0,
        cy + 10.0,
        2.0,
        color_u8!(120, 80, 40, 255),
    );
    draw_circle(cx + 7.0, cy - 9.0, 3.0, color_u8!(80, 200, 200, 255));
}

fn draw_hagure_metaru(px: f32, py: f32, timer: u32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    let bob = (timer as f32 * 0.1).sin() * 3.0;
    draw_circle(cx, cy + bob - 2.0, 14.0, color_u8!(200, 220, 240, 255));
    draw_circle(cx, cy + bob - 10.0, 8.0, color_u8!(210, 230, 250, 255));
    draw_circle(cx - 4.0, cy + bob - 4.0, 2.5, color_u8!(30, 30, 80, 255));
    draw_circle(cx + 4.0, cy + bob - 4.0, 2.5, color_u8!(30, 30, 80, 255));
    draw_circle(cx - 5.0, cy + bob - 6.0, 1.0, WHITE);
    draw_circle(cx + 5.0, cy + bob - 6.0, 1.0, WHITE);
}

fn draw_maou(px: f32, py: f32, timer: u32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    let pulse = (timer as f32 * 0.05).sin() * 2.0;
    draw_triangle(
        Vec2::new(cx, cy + 20.0),
        Vec2::new(cx - 20.0 - pulse, cy - 10.0),
        Vec2::new(cx + 20.0 + pulse, cy - 10.0),
        color_u8!(80, 0, 100, 255),
    );
    draw_rectangle(cx - 10.0, cy - 12.0, 20.0, 22.0, color_u8!(40, 0, 60, 255));
    draw_circle(cx, cy - 18.0, 10.0, color_u8!(30, 0, 40, 255));
    draw_triangle(
        Vec2::new(cx - 5.0, cy - 26.0),
        Vec2::new(cx - 12.0, cy - 38.0 - pulse),
        Vec2::new(cx - 1.0, cy - 26.0),
        color_u8!(180, 0, 0, 255),
    );
    draw_triangle(
        Vec2::new(cx + 5.0, cy - 26.0),
        Vec2::new(cx + 12.0, cy - 38.0 - pulse),
        Vec2::new(cx + 1.0, cy - 26.0),
        color_u8!(180, 0, 0, 255),
    );
    draw_circle(cx - 4.0, cy - 19.0, 2.5, color_u8!(255, 50, 0, 255));
    draw_circle(cx + 4.0, cy - 19.0, 2.5, color_u8!(255, 50, 0, 255));
}

fn draw_true_maou(px: f32, py: f32, timer: u32) {
    let cx = px + TILE / 2.0;
    let cy = py + TILE / 2.0;
    let pulse = (timer as f32 * 0.07).sin() * 4.0;
    draw_circle(cx, cy - 2.0, 25.0 + pulse, color_u8!(98, 14, 42, 255));
    draw_triangle(
        Vec2::new(cx - 8.0, cy - 20.0),
        Vec2::new(cx - 28.0, cy - 44.0),
        Vec2::new(cx - 2.0, cy - 25.0),
        color_u8!(245, 194, 78, 255),
    );
    draw_triangle(
        Vec2::new(cx + 8.0, cy - 20.0),
        Vec2::new(cx + 28.0, cy - 44.0),
        Vec2::new(cx + 2.0, cy - 25.0),
        color_u8!(245, 194, 78, 255),
    );
    draw_circle(cx - 8.0, cy - 8.0, 3.5, color_u8!(255, 236, 86, 255));
    draw_circle(cx + 8.0, cy - 8.0, 3.5, color_u8!(255, 236, 86, 255));
    draw_rectangle(cx - 10.0, cy + 8.0, 20.0, 5.0, color_u8!(42, 0, 16, 255));
    draw_circle(cx - 15.0, cy + 6.0, 3.0, color_u8!(210, 52, 92, 180));
    draw_circle(cx + 15.0, cy + 6.0, 3.0, color_u8!(210, 52, 92, 180));
}

// ─── メッセージボックス ──────────────────────────────────────────────────────
fn make_msg(lines: Vec<&str>) -> MsgBox {
    MsgBox {
        lines: lines.iter().map(|s| s.to_string()).collect(),
        visible_chars: 0,
        char_timer: 0,
        done: false,
    }
}

fn total_chars(msg: &MsgBox) -> usize {
    let ch: usize = msg.lines.iter().map(|l| l.chars().count()).sum();
    ch + msg.lines.len().saturating_sub(1)
}

fn update_msg(msg: &mut MsgBox) {
    if msg.done {
        return;
    }
    msg.char_timer += 1;
    if msg.char_timer >= MSG_SPEED {
        msg.char_timer = 0;
        msg.visible_chars += 1;
        let total = total_chars(msg);
        if msg.visible_chars >= total {
            msg.visible_chars = total;
            msg.done = true;
        }
    }
}

fn draw_msg_box(msg: &MsgBox, font: Option<&Font>) {
    let (bx, by, bw, bh) = (20.0, SCREEN_H - 120.0, SCREEN_W - 40.0, 110.0);
    draw_rectangle(bx, by, bw, bh, color_u8!(0, 0, 0, 210));
    draw_rectangle_lines(bx, by, bw, bh, 3.0, color_u8!(200, 180, 100, 255));

    let mut remaining = msg.visible_chars;
    for (i, line) in msg.lines.iter().enumerate() {
        if remaining == 0 {
            break;
        }
        let cc = line.chars().count();
        let show = remaining.min(cc);
        let visible: String = line.chars().take(show).collect();
        txt(
            &visible,
            bx + 16.0,
            by + 30.0 + i as f32 * 30.0,
            22.0,
            color_u8!(255, 240, 180, 255),
            font,
        );
        remaining = remaining.saturating_sub(cc + 1);
    }
    if msg.done {
        if (get_time() * 2.0) as u32 % 2 == 0 {
            txt(
                "▼",
                bx + bw - 30.0,
                by + bh - 10.0,
                22.0,
                color_u8!(255, 240, 100, 255),
                font,
            );
        }
    }
}

// ─── 戦闘 ────────────────────────────────────────────────────────────────────
fn start_battle(kind: EnemyKind, ally_joined: bool) -> Battle {
    let (hp, atk) = match kind {
        EnemyKind::HagureMetaru => (8, 3),
        EnemyKind::Maou => (60, 10),
        EnemyKind::TrueMaou => (95, 14),
    };
    Battle {
        enemy: Enemy {
            kind,
            hp,
            max_hp: hp,
            atk,
        },
        phase: BattlePhase::PlayerTurn,
        log: vec!["敵が現れた！".to_string()],
        cursor: 0,
        anim_timer: 0,
        log_timer: 0,
        fled: false,
        maou_defeated: false,
        defended: false,
        ally_joined,
    }
}

fn enemy_name(kind: EnemyKind) -> &'static str {
    match kind {
        EnemyKind::HagureMetaru => "はぐれメタル",
        EnemyKind::Maou => "大魔王ゾーマ",
        EnemyKind::TrueMaou => "真・大魔王ゾーマ",
    }
}

fn advance_after_player_action(battle: &mut Battle) {
    battle.phase = if battle.ally_joined {
        BattlePhase::AllyTurn
    } else {
        BattlePhase::EnemyTurn
    };
    battle.log_timer = 0;
}

fn handle_enemy_defeat(battle: &mut Battle) {
    if battle.enemy.kind == EnemyKind::Maou {
        battle.enemy = Enemy {
            kind: EnemyKind::TrueMaou,
            hp: 95,
            max_hp: 95,
            atk: 14,
        };
        battle.log.push("魔王は倒れた…かに見えた！".to_string());
        battle.log.push("真の姿を現した！".to_string());
        battle.phase = BattlePhase::EnemyTurn;
        battle.log_timer = 0;
        return;
    }

    battle.enemy.hp = 0;
    battle
        .log
        .push(format!("{}を倒した！", enemy_name(battle.enemy.kind)));
    battle.phase = BattlePhase::PlayerWin;
    battle.maou_defeated = battle.enemy.kind == EnemyKind::TrueMaou;
    battle.log_timer = 0;
}

fn update_battle(battle: &mut Battle, player: &mut Player, frame: u32) -> bool {
    battle.anim_timer = battle.anim_timer.wrapping_add(1);
    match battle.phase {
        BattlePhase::PlayerTurn => {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                if battle.cursor > 0 {
                    battle.cursor -= 1;
                }
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                if battle.cursor < 4 {
                    battle.cursor += 1;
                }
            }
            if is_key_pressed(KeyCode::Z)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
            {
                match battle.cursor {
                    0 => {
                        let dmg = (player.atk + (frame % 6) as i32 - 2).max(1);
                        battle.enemy.hp -= dmg;
                        battle.log.push(format!("勇者の攻撃！ {}のダメージ！", dmg));
                        if battle.enemy.hp <= 0 {
                            handle_enemy_defeat(battle);
                        } else {
                            advance_after_player_action(battle);
                        }
                    }
                    1 => {
                        let heal = 20 + (frame % 8) as i32;
                        let before = player.hp;
                        player.hp = (player.hp + heal).min(player.max_hp);
                        battle.log.push(format!(
                            "勇者はホイミを唱えた！ HPが{}回復！",
                            player.hp - before
                        ));
                        advance_after_player_action(battle);
                    }
                    2 => {
                        let dmg = 18 + (frame % 8) as i32;
                        battle.enemy.hp -= dmg;
                        battle
                            .log
                            .push(format!("勇者はメラを放った！ {}のダメージ！", dmg));
                        if battle.enemy.hp <= 0 {
                            handle_enemy_defeat(battle);
                        } else {
                            advance_after_player_action(battle);
                        }
                    }
                    3 => {
                        battle.defended = true;
                        battle.log.push("勇者は身を守っている！".to_string());
                        advance_after_player_action(battle);
                    }
                    4 => {
                        battle.log.push("逃げ出した！".to_string());
                        battle.fled = true;
                        battle.phase = BattlePhase::PlayerWin;
                        battle.log_timer = 0;
                    }
                    _ => {}
                }
            }
        }
        BattlePhase::AllyTurn => {
            battle.log_timer += 1;
            if battle.log_timer > 35 {
                if player.hp * 2 <= player.max_hp {
                    let before = player.hp;
                    player.hp = (player.hp + 18).min(player.max_hp);
                    battle.log.push(format!(
                        "ホイミンのホイミ！ HPが{}回復！",
                        player.hp - before
                    ));
                } else {
                    let roll = (frame / 17) % 3;
                    if roll == 0 {
                        battle
                            .log
                            .push("ホイミンはぷるぷる身を守った！".to_string());
                    } else {
                        let dmg = 5 + (frame % 4) as i32;
                        battle.enemy.hp -= dmg;
                        battle
                            .log
                            .push(format!("ホイミンの攻撃！ {}のダメージ！", dmg));
                        if battle.enemy.hp <= 0 {
                            handle_enemy_defeat(battle);
                            return false;
                        }
                    }
                }
                battle.phase = BattlePhase::EnemyTurn;
                battle.log_timer = 0;
            }
        }
        BattlePhase::EnemyTurn => {
            battle.log_timer += 1;
            if battle.log_timer > 60 {
                let mut dmg = (battle.enemy.atk + (frame % 4) as i32 - 1).max(1);
                if battle.defended {
                    dmg = ((dmg as f32) * 0.45).ceil() as i32;
                }
                player.hp -= dmg;
                battle.log.push(format!(
                    "{}の攻撃！ {}のダメージ！",
                    enemy_name(battle.enemy.kind),
                    dmg
                ));
                battle.defended = false;
                if player.hp <= 0 {
                    player.hp = 0;
                    battle.log.push("勇者は倒れた…".to_string());
                    battle.phase = BattlePhase::PlayerLose;
                } else {
                    battle.phase = BattlePhase::PlayerTurn;
                }
                battle.log_timer = 0;
            }
        }
        BattlePhase::PlayerWin => {
            battle.log_timer += 1;
            if battle.log_timer > 90 {
                if is_key_pressed(KeyCode::Z)
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Space)
                    || battle.log_timer > 200
                {
                    return true;
                }
            }
        }
        BattlePhase::PlayerLose => {
            battle.log_timer += 1;
            if battle.log_timer > 130 {
                player.hp = player.max_hp;
                return true;
            }
        }
    }
    false
}

fn draw_battle(battle: &Battle, player: &Player, frame: u32, font: Option<&Font>) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, color_u8!(10, 5, 30, 255));
    for i in 0..40u32 {
        let sx = ((i * 137 + 31) % 620) as f32 + 10.0;
        let sy = ((i * 97 + 17) % 200) as f32 + 10.0;
        let br = if (frame + i * 13) % 60 < 30 { 255 } else { 150 };
        draw_circle(sx, sy, 1.5, color_u8!(br, br, br, 255));
    }
    draw_line(0.0, 280.0, SCREEN_W, 280.0, 2.0, color_u8!(80, 80, 80, 255));
    draw_rectangle(
        0.0,
        280.0,
        SCREEN_W,
        SCREEN_H - 280.0,
        color_u8!(30, 30, 50, 255),
    );

    let ex = SCREEN_W * 0.65 - TILE;
    let ey = 140.0;
    match battle.enemy.kind {
        EnemyKind::HagureMetaru => draw_hagure_metaru(ex, ey, frame),
        EnemyKind::Maou => draw_maou(ex, ey, frame),
        EnemyKind::TrueMaou => draw_true_maou(ex, ey, frame),
    }

    let ename = enemy_name(battle.enemy.kind);
    txt(ename, 30.0, 40.0, 28.0, color_u8!(255, 220, 60, 255), font);
    txt(
        &format!("HP: {} / {}", battle.enemy.hp, battle.enemy.max_hp),
        30.0,
        72.0,
        22.0,
        color_u8!(200, 255, 200, 255),
        font,
    );
    let bw = 200.0;
    let ratio = (battle.enemy.hp as f32 / battle.enemy.max_hp as f32).max(0.0);
    draw_rectangle(30.0, 80.0, bw, 12.0, color_u8!(60, 60, 60, 255));
    draw_rectangle(30.0, 80.0, bw * ratio, 12.0, color_u8!(60, 220, 80, 255));
    draw_rectangle_lines(30.0, 80.0, bw, 12.0, 1.5, color_u8!(150, 150, 150, 255));

    txt(
        &format!("勇者 HP: {} / {}", player.hp, player.max_hp),
        SCREEN_W - 230.0,
        40.0,
        22.0,
        color_u8!(200, 200, 255, 255),
        font,
    );
    let pr = (player.hp as f32 / player.max_hp as f32).max(0.0);
    draw_rectangle(
        SCREEN_W - 230.0,
        50.0,
        190.0,
        12.0,
        color_u8!(60, 60, 60, 255),
    );
    draw_rectangle(
        SCREEN_W - 230.0,
        50.0,
        190.0 * pr,
        12.0,
        color_u8!(60, 120, 220, 255),
    );

    draw_player(SCREEN_W * 0.15, 220.0);
    if battle.ally_joined {
        draw_hoimin(SCREEN_W * 0.15 + 44.0, 232.0, frame);
        txt(
            "ホイミン",
            SCREEN_W * 0.15 + 30.0,
            218.0,
            16.0,
            color_u8!(160, 230, 255, 255),
            font,
        );
    }

    if battle.phase == BattlePhase::PlayerTurn {
        draw_rectangle(30.0, 292.0, 200.0, 156.0, color_u8!(0, 0, 0, 200));
        draw_rectangle_lines(
            30.0,
            292.0,
            200.0,
            156.0,
            2.0,
            color_u8!(200, 180, 100, 255),
        );
        for (i, cmd) in ["たたかう", "ホイミ", "メラ", "ぼうぎょ", "にげる"]
            .iter()
            .enumerate()
        {
            let col = if i == battle.cursor {
                color_u8!(255, 240, 80, 255)
            } else {
                color_u8!(200, 200, 200, 255)
            };
            let pre = if i == battle.cursor { "▶ " } else { "   " };
            txt(
                &format!("{}{}", pre, cmd),
                46.0,
                320.0 + i as f32 * 28.0,
                22.0,
                col,
                font,
            );
        }
    }

    let (lbx, lby, lbw, lbh) = (240.0, 295.0, SCREEN_W - 270.0, 100.0);
    draw_rectangle(lbx, lby, lbw, lbh, color_u8!(0, 0, 0, 190));
    draw_rectangle_lines(lbx, lby, lbw, lbh, 2.0, color_u8!(150, 150, 180, 255));
    let logs: Vec<&String> = battle
        .log
        .iter()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    for (i, line) in logs.iter().enumerate() {
        txt(
            line,
            lbx + 10.0,
            lby + 28.0 + i as f32 * 26.0,
            20.0,
            color_u8!(220, 220, 220, 255),
            font,
        );
    }

    if battle.phase == BattlePhase::PlayerLose {
        draw_rectangle(
            100.0,
            180.0,
            SCREEN_W - 200.0,
            60.0,
            color_u8!(0, 0, 0, 220),
        );
        txt(
            "勇者は倒れた…",
            SCREEN_W / 2.0 - 120.0,
            220.0,
            32.0,
            color_u8!(255, 80, 80, 255),
            font,
        );
    }
}

// ─── タイトル・エンディング ───────────────────────────────────────────────────
fn draw_title(frame: u32, font: Option<&Font>) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, color_u8!(5, 5, 40, 255));
    for i in 0..60u32 {
        let sx = ((i * 173 + 53) % 620) as f32 + 10.0;
        let sy = ((i * 89 + 23) % 440) as f32 + 10.0;
        let br = if (frame + i * 17) % 80 < 40 {
            255u8
        } else {
            120u8
        };
        draw_circle(sx, sy, 1.2, color_u8!(br, br, br, 255));
    }
    let t = frame as f32 * 0.03;
    let r = ((t.sin() * 0.5 + 0.5) * 255.0) as u8;
    let g = (((t + 2.0).sin() * 0.5 + 0.5) * 200.0) as u8;
    txt(
        "ドラクエ風",
        SCREEN_W / 2.0 - 155.0,
        140.0,
        34.0,
        color_u8!(r, g, 100, 255),
        font,
    );
    txt(
        "レトロRPG",
        SCREEN_W / 2.0 - 100.0,
        195.0,
        52.0,
        color_u8!(255, 220, 60, 255),
        font,
    );
    draw_line(
        60.0,
        215.0,
        SCREEN_W - 60.0,
        215.0,
        2.0,
        color_u8!(200, 160, 60, 180),
    );
    draw_player(SCREEN_W / 2.0 - 160.0, 255.0);
    draw_maou(SCREEN_W / 2.0 + 80.0, 245.0, frame);
    if (frame / 30) % 2 == 0 {
        txt(
            "Z / Enter キーで はじめる",
            SCREEN_W / 2.0 - 155.0,
            405.0,
            24.0,
            color_u8!(255, 240, 120, 255),
            font,
        );
    }
    txt(
        "操作: WASD / 矢印キー    決定: Z / Enter",
        SCREEN_W / 2.0 - 210.0,
        450.0,
        18.0,
        color_u8!(150, 150, 180, 255),
        font,
    );
}

fn draw_ending(timer: u32, font: Option<&Font>) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, color_u8!(5, 5, 40, 255));
    let sc = (timer / 3).min(80);
    for i in 0..sc {
        let sx = ((i * 173 + 53) % 620) as f32 + 10.0;
        let sy = ((i * 89 + 23) % 440) as f32 + 10.0;
        draw_circle(sx, sy, 1.5, color_u8!(255, 255, 200, 200));
    }
    let a = (timer * 4).min(255) as u8;
    txt(
        "魔王を倒した！",
        SCREEN_W / 2.0 - 120.0,
        150.0,
        40.0,
        color_u8!(255, 220, 60, a),
        font,
    );
    if timer > 50 {
        let a2 = ((timer - 50) * 4).min(255) as u8;
        txt(
            "世界に平和が訪れた…",
            SCREEN_W / 2.0 - 230.0,
            215.0,
            30.0,
            color_u8!(200, 220, 255, a2),
            font,
        );
    }
    if timer > 100 {
        draw_player(SCREEN_W / 2.0 - 16.0, 270.0);
    }
    if timer > 130 {
        let a3 = ((timer - 130) * 4).min(255) as u8;
        txt(
            "勇者の伝説は永遠に語り継がれる",
            SCREEN_W / 2.0 - 220.0,
            345.0,
            24.0,
            color_u8!(180, 200, 255, a3),
            font,
        );
    }
    if timer > 220 && (timer / 30) % 2 == 0 {
        txt(
            "Z / Enter でタイトルに戻る",
            SCREEN_W / 2.0 - 175.0,
            435.0,
            22.0,
            color_u8!(255, 240, 120, 255),
            font,
        );
    }
}

// ─── ゲームロジック ──────────────────────────────────────────────────────────
fn new_game() -> GameState {
    GameState {
        phase: GamePhase::Playing,
        map_id: MapId::House,
        player: Player {
            x: HOUSE_PLAYER_START.0,
            y: HOUSE_PLAYER_START.1,
            hp: 50,
            max_hp: 50,
            atk: 15,
            move_cd: 0,
        },
        battle: None,
        msg: Some(make_msg(vec![
            "目覚めよ、勇者よ！",
            "家の外に出て世界を救うのだ！",
        ])),
        camera_x: 0.0,
        camera_y: 0.0,
        maou_defeated: false,
        hagure_defeated: false,
        hoimin_joined: false,
        ending_timer: 0,
    }
}

// 家・村内の遷移（ワールドマップ遷移はupdate_game内で直接処理）
fn check_house_village_transition(gs: &mut GameState) {
    let (px, py) = (gs.player.x, gs.player.y);
    match gs.map_id {
        MapId::House => {
            if HOUSE_EXIT_TILES
                .iter()
                .any(|&(ex, ey)| ex == px && ey == py)
            {
                gs.map_id = MapId::Village;
                gs.player.x = VILLAGE_HOUSE_DOOR.0;
                gs.player.y = VILLAGE_HOUSE_DOOR.1 + 1;
                gs.msg = Some(make_msg(vec![
                    "村に出た。",
                    "老人に話しかけてみよう。(Zキー or Enterキー)",
                ]));
            }
        }
        MapId::Village => {
            // 家の扉に乗ったら家に戻る
            if px == VILLAGE_HOUSE_DOOR.0 && py == VILLAGE_HOUSE_DOOR.1 {
                gs.map_id = MapId::House;
                gs.player.x = HOUSE_PLAYER_START.0;
                gs.player.y = HOUSE_PLAYER_START.1;
            }
        }
        MapId::Well => {
            if WELL_EXIT_TILES.iter().any(|&(ex, ey)| ex == px && ey == py) {
                gs.map_id = MapId::World;
                gs.player.x = WORLD_PLAYER_FROM_WELL.0;
                gs.player.y = WORLD_PLAYER_FROM_WELL.1;
                gs.msg = Some(make_msg(vec!["井戸から出た。"]));
                gs.camera_x = 0.0;
                gs.camera_y = 0.0;
            }
        }
        _ => {}
    }
}

fn update_game(gs: &mut GameState, frame: u32) {
    if gs.player.move_cd > 0 {
        gs.player.move_cd -= 1;
    }

    // メッセージ表示中
    if let Some(ref mut msg) = gs.msg {
        update_msg(msg);
        if msg.done
            && (is_key_pressed(KeyCode::Z)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space))
        {
            gs.msg = None;
        }
        return;
    }

    // 戦闘中
    if let Some(ref mut battle) = gs.battle {
        let done = update_battle(battle, &mut gs.player, frame);
        if done {
            let maou_def = battle.maou_defeated;
            let fled = battle.fled;
            let is_lose = battle.phase == BattlePhase::PlayerLose;
            let kind = battle.enemy.kind;
            gs.battle = None;

            if maou_def {
                gs.maou_defeated = true;
                gs.phase = GamePhase::Ending;
            } else if is_lose {
                gs.player.hp = gs.player.max_hp;
                gs.map_id = MapId::Village;
                gs.player.x = VILLAGE_PLAYER_START.0;
                gs.player.y = VILLAGE_PLAYER_START.1;
                gs.msg = Some(make_msg(vec![
                    "倒れてしまった…",
                    "村で目が覚めた。回復した！",
                ]));
            } else if !fled && kind == EnemyKind::HagureMetaru {
                gs.hagure_defeated = true;
                gs.msg = Some(make_msg(vec![
                    "はぐれメタルを倒した！",
                    "そなたは十分に強い！ 城へ向かおう！",
                ]));
            }
        }
        return;
    }

    // 移動
    if gs.player.move_cd == 0 {
        let mut dx = 0i32;
        let mut dy = 0i32;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx = -1;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx = 1;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dy = -1;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dy = 1;
        }

        if dx != 0 || dy != 0 {
            let nx = gs.player.x + dx;
            let ny = gs.player.y + dy;
            let map = get_current_map(gs.map_id);
            let tile = tile_at(map, nx, ny);

            if is_walkable(tile) {
                gs.player.x = nx;
                gs.player.y = ny;
                gs.player.move_cd = 8;

                // ── ワールドマップ上の建物遷移 ─────────────────────────────
                if gs.map_id == MapId::World {
                    if tile == 9 {
                        // 村建物に乗った → 村へ
                        gs.map_id = MapId::Village;
                        gs.player.x = VILLAGE_PLAYER_START.0;
                        gs.player.y = VILLAGE_PLAYER_START.1;
                        gs.msg = Some(make_msg(vec![
                            "村に入った。",
                            "老人に話しかけてみよう。(Zキー)",
                        ]));
                        return;
                    }
                    if tile == 11 {
                        // 井戸に乗った → 井戸の中へ
                        gs.map_id = MapId::Well;
                        gs.player.x = WELL_PLAYER_START.0;
                        gs.player.y = WELL_PLAYER_START.1;
                        gs.msg = Some(make_msg(vec![
                            "井戸の中に入った。",
                            "奥で小さな声が聞こえる…",
                        ]));
                        return;
                    }
                    if tile == 10 {
                        // 城建物に乗った → 魔王戦
                        gs.map_id = MapId::Castle;
                        gs.msg = Some(make_msg(vec!["城に入った…", "魔王が現れた！"]));
                        gs.battle = Some(start_battle(EnemyKind::Maou, gs.hoimin_joined));
                        return;
                    }
                    // エンカウント
                    if !gs.hagure_defeated && gs.battle.is_none() {
                        if ENCOUNTER_TILES
                            .iter()
                            .any(|&(ex, ey)| ex == gs.player.x && ey == gs.player.y)
                        {
                            let seed = frame
                                .wrapping_mul(37)
                                .wrapping_add(gs.player.x as u32 * 13)
                                .wrapping_add(gs.player.y as u32 * 7);
                            if seed % 3 == 0 {
                                gs.battle =
                                    Some(start_battle(EnemyKind::HagureMetaru, gs.hoimin_joined));
                                return;
                            }
                        }
                    }
                }

                // ── 村境界 → フィールドへ ────────────────────────────────
                if gs.map_id == MapId::Village {
                    check_house_village_transition(gs);
                } else {
                    check_house_village_transition(gs);
                }
            } else if gs.map_id == MapId::Village && tile == 1 && is_village_boundary_exit(nx, ny) {
                // 村の境界の木に向かって歩く → フィールドへ
                gs.map_id = MapId::World;
                gs.player.x = WORLD_PLAYER_FROM_VILLAGE.0;
                gs.player.y = WORLD_PLAYER_FROM_VILLAGE.1;
                gs.player.move_cd = 8;
                gs.msg = Some(make_msg(vec!["村の外に出た！", "モンスターが現れるかも…"]));
                // フィールドのカメラをリセット
                gs.camera_x = 0.0;
                gs.camera_y = 0.0;
                return;
            }
        }
    }

    // 話しかける
    if is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Enter) {
        if gs.map_id == MapId::Village {
            let (ox, oy) = OLD_MAN_POS;
            if (gs.player.x - ox).abs() + (gs.player.y - oy).abs() == 1 {
                gs.msg = Some(make_msg(if gs.hagure_defeated {
                    vec!["老人「よくぞ帰った！", "城の奥に魔王が待っておるぞ！"]
                } else {
                    vec![
                        "老人「村の外は危険じゃ。",
                        "まずモンスターで腕試しするがよい！",
                    ]
                }));
            }
        } else if gs.map_id == MapId::Well {
            let (hx, hy) = HOIMIN_POS;
            if (gs.player.x - hx).abs() + (gs.player.y - hy).abs() == 1 {
                if gs.hoimin_joined {
                    gs.msg = Some(make_msg(vec!["ホイミン「ぼくも一緒に行くよ！」"]));
                } else {
                    gs.hoimin_joined = true;
                    gs.msg = Some(make_msg(vec![
                        "ホイミン「ひとりぼっちは寂しいな…」",
                        "ホイミンが仲間になった！",
                    ]));
                }
            }
        }
    }

    // カメラ
    // ワールドマップ: 20×15タイル = 640×480px → スクロール不要
    if gs.map_id == MapId::World || gs.map_id == MapId::Castle {
        gs.camera_x = 0.0;
        gs.camera_y = 0.0;
    } else {
        let tcx = gs.player.x as f32 * TILE - SCREEN_W / 2.0 + TILE / 2.0;
        let tcy = gs.player.y as f32 * TILE - SCREEN_H / 2.0 + TILE / 2.0;
        gs.camera_x += (tcx - gs.camera_x) * 0.15;
        gs.camera_y += (tcy - gs.camera_y) * 0.15;
    }
}

// ─── ウィンドウ設定 ──────────────────────────────────────────────────────────
fn window_conf() -> Conf {
    Conf {
        window_title: "レトロRPG".to_string(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

// ─── メインループ ────────────────────────────────────────────────────────────
#[macroquad::main(window_conf)]
async fn main() {
    // 日本語フォントをロード（Windows標準フォントを順番に試す）
    let font: Option<Font> = {
        let candidates = [
            "C:/Windows/Fonts/msgothic.ttc",
            "C:/Windows/Fonts/YuGothM.ttc",
            "C:/Windows/Fonts/meiryo.ttc",
            "C:/Windows/Fonts/yumin.ttf",
        ];
        let mut loaded = None;
        for path in &candidates {
            if let Ok(f) = load_ttf_font(path).await {
                loaded = Some(f);
                break;
            }
        }
        loaded
    };
    let font_ref = font.as_ref();

    let mut gs = GameState {
        phase: GamePhase::Title,
        map_id: MapId::House,
        player: Player {
            x: 0,
            y: 0,
            hp: 50,
            max_hp: 50,
            atk: 15,
            move_cd: 0,
        },
        battle: None,
        msg: None,
        camera_x: 0.0,
        camera_y: 0.0,
        maou_defeated: false,
        hagure_defeated: false,
        hoimin_joined: false,
        ending_timer: 0,
    };
    let mut frame: u32 = 0;

    loop {
        clear_background(BLACK);
        frame = frame.wrapping_add(1);

        match gs.phase {
            GamePhase::Title => {
                draw_title(frame, font_ref);
                if is_key_pressed(KeyCode::Z)
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Space)
                {
                    gs = new_game();
                }
            }

            GamePhase::Playing => {
                update_game(&mut gs, frame);

                let map = get_current_map(gs.map_id);
                draw_map(map, gs.camera_x, gs.camera_y);

                // フィールドのみ: 村・城ラベルを描画
                if gs.map_id == MapId::World || gs.map_id == MapId::Castle {
                    draw_world_labels(gs.camera_x, gs.camera_y, font_ref);
                }

                // 老人（村のみ）
                if gs.map_id == MapId::Village {
                    let (ox, oy) = OLD_MAN_POS;
                    draw_old_man(
                        ox as f32 * TILE - gs.camera_x,
                        oy as f32 * TILE - gs.camera_y,
                    );
                }

                // ホイミン（井戸のみ・仲間になる前）
                if gs.map_id == MapId::Well && !gs.hoimin_joined {
                    let (hx, hy) = HOIMIN_POS;
                    draw_hoimin(
                        hx as f32 * TILE - gs.camera_x,
                        hy as f32 * TILE - gs.camera_y,
                        frame,
                    );
                }

                // 勇者（戦闘中は描画しない）
                if gs.battle.is_none() {
                    draw_player(
                        gs.player.x as f32 * TILE - gs.camera_x,
                        gs.player.y as f32 * TILE - gs.camera_y,
                    );
                }

                // HUD
                draw_rectangle(0.0, 0.0, SCREEN_W, 30.0, color_u8!(0, 0, 0, 180));
                let map_name = match gs.map_id {
                    MapId::House => "家の中",
                    MapId::Village => "アレフガルド村",
                    MapId::World => "フィールド",
                    MapId::Castle => "魔王の城",
                    MapId::Well => "井戸の中",
                };
                txt(
                    &format!("【{}】", map_name),
                    10.0,
                    22.0,
                    22.0,
                    color_u8!(255, 220, 80, 255),
                    font_ref,
                );
                txt(
                    &format!("勇者 {}/{}", gs.player.hp, gs.player.max_hp),
                    SCREEN_W - 160.0,
                    22.0,
                    22.0,
                    color_u8!(200, 255, 200, 255),
                    font_ref,
                );
                if gs.hoimin_joined {
                    txt(
                        "仲間: ホイミン",
                        SCREEN_W - 330.0,
                        22.0,
                        20.0,
                        color_u8!(160, 230, 255, 255),
                        font_ref,
                    );
                }

                // フィールドミニヒント
                if (gs.map_id == MapId::World || gs.map_id == MapId::Castle)
                    && gs.battle.is_none()
                    && gs.msg.is_none()
                {
                    txt(
                        "村/城/井戸の上を歩くと中に入れます",
                        10.0,
                        SCREEN_H - 10.0,
                        16.0,
                        color_u8!(180, 180, 180, 200),
                        font_ref,
                    );
                }

                if let Some(ref battle) = gs.battle {
                    draw_battle(battle, &gs.player, frame, font_ref);
                }
                if let Some(ref msg) = gs.msg {
                    draw_msg_box(msg, font_ref);
                }
            }

            GamePhase::Ending => {
                gs.ending_timer += 1;
                draw_ending(gs.ending_timer, font_ref);
                if gs.ending_timer > 220 {
                    if is_key_pressed(KeyCode::Z)
                        || is_key_pressed(KeyCode::Enter)
                        || is_key_pressed(KeyCode::Space)
                    {
                        gs = GameState {
                            phase: GamePhase::Title,
                            map_id: MapId::House,
                            player: Player {
                                x: 0,
                                y: 0,
                                hp: 50,
                                max_hp: 50,
                                atk: 15,
                                move_cd: 0,
                            },
                            battle: None,
                            msg: None,
                            camera_x: 0.0,
                            camera_y: 0.0,
                            maou_defeated: false,
                            hagure_defeated: false,
                            hoimin_joined: false,
                            ending_timer: 0,
                        };
                        frame = 0;
                    }
                }
            }
        }

        next_frame().await;
    }
}
