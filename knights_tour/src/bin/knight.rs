// Knights_Tour v1.1 - Solved from Multiple squares

use knights_tour::mxr::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }
}

fn main() -> Result<(), String> {
    let mut width = 1440;
    let mut height = 1080;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        width = args[1].parse::<u32>().unwrap();
        height = args[2].parse::<u32>().unwrap();
    }
    let mut mx = MXWindowBuilder::new()
        .create("Knights Tour", width, height)
        .set_icon("./data/knight.bmp")
        .build()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./data/font.ttf", 18)?;
    let tc = mx.can.texture_creator();
    let files = vec!["./data/knight.bmp"];
    let textures = mx.load_gfx(files, &tc, Some(sdl2::pixels::Color::RGB(255, 255, 255)))?;
    let tex = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            "Press Space to Move Knight",
        )
        .unwrap();
    let tex_over = mx
        .printtext_texture(
            &font,
            &tc,
            sdl2::pixels::Color::RGB(255, 255, 255),
            "Tour Complete",
        )
        .unwrap();
    let tex_s = tex_get_size(&tex);
    let tex_over_s = tex_get_size(&tex_over);
    let mut texture = tc
        .create_texture_target(tc.default_pixel_format(), 640, 480)
        .unwrap();

    let mut tour_over = false;
    let mut moves = 1;
    let startx = 100;
    let starty = 30;
    let mut board: [[i32; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
    let horizontal: [i32; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
    let vertical: [i32; 8] = [-1, -2, -2, -1, 1, 2, 2, 1];

    let mut rng = rand::thread_rng();
    let random_row = rng.gen_range(0..BOARD_SIZE as i32);
    let random_col = rng.gen_range(0..BOARD_SIZE as i32);
    let mut knight_pos = Position::new(random_row, random_col);
    let mut move_sequence = Vec::new();

    fn drawboard(
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        startx: i32,
        starty: i32,
        board: &[[i32; BOARD_SIZE]; BOARD_SIZE],
    ) {
        let mut dx = startx;
        let mut dy = starty;
        let mut ion = true;
        for row in board.iter() {
            for &cell in row.iter() {
                let color = if cell < 0 {
                    sdl2::pixels::Color::RGB(0, 0, 0)
                } else if ion {
                    sdl2::pixels::Color::RGB(255, 255, 255)
                } else {
                    sdl2::pixels::Color::RGB(255, 0, 0)
                };
                ion = !ion;
                canvas.set_draw_color(color);
                canvas
                    .fill_rect(sdl2::rect::Rect::new(dx, dy, 50, 50))
                    .expect("on drawing rectangle for grid");
                dx += 55;
                if dx >= startx + 8 * 55 {
                    dx = startx;
                    dy += 55;
                    ion = !ion;
                }
            }
        }
    }

    fn drawknight(
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        startx: i32,
        starty: i32,
        knight_pos: Position,
        texture: &sdl2::render::Texture,
    ) {
        let dx = startx + knight_pos.col * 55;
        let dy = starty + knight_pos.row * 55;
        canvas
            .copy(texture, None, sdl2::rect::Rect::new(dx + 5, dy + 5, 35, 35))
            .expect("on draw knight");
    }

    fn clearboard(board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE]) {
        for row in board.iter_mut().take(BOARD_SIZE) {
            for cell in row.iter_mut().take(BOARD_SIZE) {
                *cell = 0;
            }
        }
    }

    fn is_valid_move(board: &[[i32; BOARD_SIZE]; BOARD_SIZE], pos: Position) -> bool {
        pos.row >= 0
            && pos.row < BOARD_SIZE as i32
            && pos.col >= 0
            && pos.col < BOARD_SIZE as i32
            && board[pos.row as usize][pos.col as usize] == 0
    }

    fn get_degree(
        board: &[[i32; BOARD_SIZE]; BOARD_SIZE],
        pos: Position,
        horizontal: &[i32; 8],
        vertical: &[i32; 8],
    ) -> i32 {
        let mut count = 0;
        for i in 0..8 {
            let new_row = pos.row + horizontal[i];
            let new_col = pos.col + vertical[i];
            if (0..BOARD_SIZE as i32).contains(&new_row)
                && (0..BOARD_SIZE as i32).contains(&new_col)
                && board[new_row as usize][new_col as usize] == 0
            {
                count += 1;
            }
        }
        count
    }

    fn solve_knights_tour(
        board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE],
        pos: Position,
        move_count: i32,
        horizontal: &[i32; 8],
        vertical: &[i32; 8],
        move_sequence: &mut Vec<Position>,
    ) -> bool {
        if move_count == (BOARD_SIZE * BOARD_SIZE) as i32 + 1 {
            return true;
        }

        let mut min_degree_index: Option<usize> = None;
        let mut min_degree = 9;
        let mut next_pos = Position::new(-1, -1);

        for i in 0..8 {
            let new_pos = Position::new(pos.row + horizontal[i], pos.col + vertical[i]);
            if is_valid_move(board, new_pos) {
                let degree = get_degree(board, new_pos, horizontal, vertical);
                if degree < min_degree {
                    min_degree_index = Some(i);
                    min_degree = degree;
                    next_pos = new_pos;
                }
            }
        }

        if let Some(index) = min_degree_index {
            board[next_pos.row as usize][next_pos.col as usize] = move_count;
            move_sequence.push(next_pos);
            if solve_knights_tour(
                board,
                next_pos,
                move_count + 1,
                horizontal,
                vertical,
                move_sequence,
            ) {
                return true;
            } else {
                println!("Backtracking from: {:?}", next_pos);
                board[next_pos.row as usize][next_pos.col as usize] = 0;
                move_sequence.pop();
            }
        }

        false
    }

    fn nextmove(
        board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE],
        move_sequence: &mut Vec<Position>,
        knight_pos: &mut Position,
        moves: &mut i32,
        tour_over: &mut bool,
    ) {
        if !move_sequence.is_empty() {
            let next_pos = move_sequence.remove(0);
            board[knight_pos.row as usize][knight_pos.col as usize] = -1;
            *knight_pos = next_pos;
            board[knight_pos.row as usize][knight_pos.col as usize] = *moves;
            *moves += 1;
            if *moves == 65 {
                *tour_over = true;
            }
        }
    }

    fn reset_tour(
        board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE],
        move_sequence: &mut Vec<Position>,
        knight_pos: &mut Position,
        horizontal: &[i32; 8],
        vertical: &[i32; 8],
        rng: &mut rand::rngs::ThreadRng,
    ) -> bool {
        clearboard(board);
        let random_row = rng.gen_range(0..BOARD_SIZE as i32);
        let random_col = rng.gen_range(0..BOARD_SIZE as i32);
        *knight_pos = Position::new(random_row, random_col);
        move_sequence.clear();
        board[knight_pos.row as usize][knight_pos.col as usize] = 1;
        move_sequence.push(*knight_pos);
        solve_knights_tour(board, *knight_pos, 2, horizontal, vertical, move_sequence)
    }

    clearboard(&mut board);
    board[knight_pos.row as usize][knight_pos.col as usize] = 1;
    move_sequence.push(knight_pos);
    let solved = solve_knights_tour(
        &mut board,
        knight_pos,
        2,
        &horizontal,
        &vertical,
        &mut move_sequence,
    );

    if !solved {
        println!("Failed to find a solution for the initial position");
        return Ok(());
    }

    println!("Initial Position: {:?}", knight_pos);
    println!("Move Sequence Length: {}", move_sequence.len());

    'main: loop {
        for event in mx.event.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if !tour_over {
                        nextmove(
                            &mut board,
                            &mut move_sequence,
                            &mut knight_pos,
                            &mut moves,
                            &mut tour_over,
                        );
                        println!("Moved to: {:?}", knight_pos);
                        println!("Remaining moves: {}", move_sequence.len());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    tour_over = false;
                    moves = 1;
                    if !reset_tour(
                        &mut board,
                        &mut move_sequence,
                        &mut knight_pos,
                        &horizontal,
                        &vertical,
                        &mut rng,
                    ) {
                        println!("Failed to find a solution");
                    } else {
                        println!("New Initial Position: {:?}", knight_pos);
                        println!("New Move Sequence Length: {}", move_sequence.len());
                    }
                }
                _ => {}
            }
        }
        mx.can
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                texture_canvas.clear();
                drawboard(texture_canvas, startx, starty, &board);
                drawknight(texture_canvas, startx, starty, knight_pos, &textures[0]);
                if !tour_over {
                    texture_canvas
                        .copy(&tex, None, sdl2::rect::Rect::new(5, 5, tex_s.0, tex_s.1))
                        .expect("on copy");
                } else {
                    texture_canvas
                        .copy(
                            &tex_over,
                            None,
                            sdl2::rect::Rect::new(5, 5, tex_over_s.0, tex_over_s.1),
                        )
                        .expect("on copy");
                }
            })
            .map_err(|x| x.to_string())?;
        mx.can.clear();
        mx.can.copy(&texture, None, None)?;
        mx.can.present();
    }
    Ok(())
}
