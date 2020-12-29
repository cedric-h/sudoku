use macroquad::prelude::*;

const GRID_SIZE: f32 = 750.0;

#[macroquad::main("sudoku")]
async fn main() {
    let font = load_ttf_font("./Roboto-Regular.ttf").await;
    let mut cells = [[0_u8; 9]; 9];
    let mut editing_cell: Option<(usize, usize)> = None;

    let mut tick: u32 = 0;
    loop {
        tick = tick.wrapping_add(1);

        let cam = Camera2D {
            zoom: vec2(screen_width(), -screen_height()).recip(),
            offset: vec2(GRID_SIZE / screen_width() * -0.5, 0.65),
            ..Default::default()
        };
        set_camera(cam);
        
        clear_background(WHITE);
        const CELL_SIZE: f32 = GRID_SIZE / 9.0;
        for (x, row) in cells.iter_mut().enumerate() {
            for (y, val) in row.iter_mut().enumerate() {
                let editing = editing_cell == Some((x, y));
                if *val == 0 && !editing {
                    continue;
                }

                let text = val.to_string();
                let (x, y) = (
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                );

                if editing {
                    draw_rectangle(
                        x, y,
                        CELL_SIZE,
                        CELL_SIZE,
                        RED,
                    );
                    if let Some(num) = get_char_pressed().filter(|c| c.is_numeric()) {
                        *val = num.to_string().parse().unwrap();
                    }
                    if tick / 10 % 3 == 0 {
                        continue;
                    }
                }

                let (size, _) = measure_text(&text, Some(font), 60, 1.0);
                draw_text_ex(
                    &text,
                    x + CELL_SIZE / 2.0 - size / 2.0,
                    y + 5.0,
                    TextParams {
                        font_size: 60,
                        font,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        }

        grid(3, 9.0);
        grid(9, 4.0);

        let mouse = cam.screen_to_world(mouse_position().into());
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = (mouse / CELL_SIZE).floor().into();
            editing_cell = Some((x as usize, y as usize));
        }

        if is_key_pressed(KeyCode::Escape) {
            editing_cell = None;
        }

        if solve_button(mouse, font) {
            for x in 0..9 {
                for y in 0..9 {
                    cells[x][y] = (tick % 9) as u8 + 1;
                }
            }
        }

        next_frame().await;
    }
}

fn grid(cells: usize, thickness: f32) {
    for i in 0..=cells {
        let p = GRID_SIZE * (i as f32 / cells as f32);
        let t = thickness / 2.0;
        draw_line(p, -t, p, GRID_SIZE + t, thickness, BLACK);
        draw_line(0.0, p, GRID_SIZE, p, thickness, BLACK);
    }
}

fn solve_button(mouse: Vec2, font: Font) -> bool {
    let pos = vec2(GRID_SIZE / 2.0 - 100.0, GRID_SIZE + 40.0);
    let size = vec2(200.0, 80.0);
    draw_rectangle_lines(pos.x, pos.y, size.x, size.y, 12.0, BLACK);

    let (size_x, size_y) = measure_text(&"Solve", Some(font), 64, 1.0);
    draw_text_ex(
        "Solve",
        (GRID_SIZE - size_x) / 2.0,
        GRID_SIZE + size_y - 10.0,
        TextParams {
            font_size: 64,
            color: BLACK,
            font,
            ..Default::default()
        },
    );

    let d = mouse - pos;

    is_mouse_button_down(MouseButton::Left)
        && d.cmpgt(Vec2::zero()).all()
        && d.cmplt(size).all()
}
