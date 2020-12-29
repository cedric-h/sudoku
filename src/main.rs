//! Draws a sudoku board you can interact with from primitives
//! like lines, text, and rectangles.
//!
//! If you've only ever made GUI with the web browser, then the
//! biggest difference to watch out for is that this is done in 
//! an immediate, rather than retained, style. What this means
//! is that rather than creating some line or rectangle object
//! and moving it around, I just draw a line or rectangle every
//! frame. Moving it around is accomplished via drawing the
//! line or rectangle in a different location each frame.
use macroquad::prelude::*;

const GRID_SIZE: f32 = 750.0;
const CELL_SIZE: f32 = GRID_SIZE / 9.0;

struct Cells {
    cells: [[u8; 9]; 9],
    editing_cell: Option<(usize, usize)>,
}

impl Cells {
    fn new() -> Self {
        Self {
            cells: [[0_u8; 9]; 9],
            editing_cell: None,
        }
    }

    fn draw_contents(&mut self, tick: u32, font: Font) {
        for (x, row) in self.cells.iter_mut().enumerate() {
            for (y, val) in row.iter_mut().enumerate() {
                let editing = self.editing_cell == Some((x, y));
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
    }

    fn update(&mut self, mouse: Vec2) {
        self.editing_cell = if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = (mouse / CELL_SIZE).floor().into();
            Some((x as usize, y as usize))
        } else if is_key_pressed(KeyCode::Escape) {
            None
        } else {
            self.editing_cell
        };
    }
}

#[macroquad::main("sudoku")]
async fn main() {
    let mut font = load_ttf_font("./Roboto-Regular.ttf").await;
    let mut cells = Cells::new();

    // This is the frame loop. Each iteration is one frame.
    let mut tick: u32 = 0;
    loop {
        tick = tick.wrapping_add(1);

        // This camera is the size of the screen, so all the units are in pixels.
        let cam = Camera2D {
            zoom: 1.0 / vec2(screen_width(), -screen_height()),
            // Horizontally, the grid is in the middle of the screen,
            // and things are pushed up some vertically for the button.
            offset: vec2(GRID_SIZE / screen_width() * -0.5, 0.65),
            ..Default::default()
        };
        set_camera(cam);
        
        clear_background(WHITE);

        // Drawing the grid afterward the contents
        // serves to make sure it appears on top
        cells.draw_contents(tick, font);
        draw_grid(3, 9.0);
        draw_grid(9, 4.0);

        let mouse = cam.screen_to_world(mouse_position().into());
        cells.update(mouse);
        if solve_button(mouse, font) {
            for x in 0..9 {
                for y in 0..9 {
                    cells.cells[x][y] = (tick % 9) as u8 + 1;
                }
            }
        }

        next_frame().await;
    }
}


fn draw_grid(cells: usize, thickness: f32) {
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
