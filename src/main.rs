use macroquad::prelude::*;

#[macroquad::main("\u{1FFA}\u{0416}\u{040F}\u{013F}\u{04AC}")]
async fn main() {
    const COLS: f32 = 40.0;
    const ROWS: f32 = 35.0;

    const PLAYER_SYMBOL: &str = "@";

    const GRID_LINE_THICKNESS: f32 = 1.0;
    let grid_line_color: Color = Color::from_rgba(0, 255, 0, 30);

    loop {
        clear_background(BLANK);

        let offset_left = screen_width() * 0.05;
        let offset_top = screen_height() * 0.05;
        let playfield_width = screen_width() - screen_width() * 0.05;
        let playfield_height = screen_height() - screen_height() * 0.05;
        let cell_width = playfield_width / COLS;
        let cell_height = playfield_height / ROWS;

        let lines_x: Vec<f32> = ((offset_left as usize)..=(playfield_width as usize))
            .step_by(cell_width as usize)
            .map(|l| l as f32)
            .collect();
        let lines_y: Vec<f32> = ((offset_top as usize)..=(playfield_height as usize))
            .step_by(cell_height as usize)
            .map(|l| l as f32)
            .collect();
        for x in &lines_x {
            draw_line(
                *x,
                offset_top,
                *x,
                *lines_y.last().unwrap(),
                GRID_LINE_THICKNESS,
                grid_line_color,
            );
        }
        for y in &lines_y {
            draw_line(
                offset_left,
                *y,
                *lines_x.last().unwrap(),
                *y,
                GRID_LINE_THICKNESS,
                grid_line_color,
            );
        }

        let (mouse_x, mouse_y) = mouse_position();
        if let Some(line_x_gt_mouse_x) = lines_x.iter().position(|x| x > &mouse_x) {
            if let Some(line_y_gt_mouse_y) = lines_y.iter().position(|y| y > &mouse_y) {
                if !(line_x_gt_mouse_x == 0 || line_y_gt_mouse_y == 0) {
                    let player_x = *lines_x.get(line_x_gt_mouse_x - 1).unwrap() as f32;
                    let player_y = *lines_y.get(line_y_gt_mouse_y - 1).unwrap() as f32;
                    draw_text(
                        PLAYER_SYMBOL,
                        player_x + cell_width * 0.32,
                        player_y + cell_height * 0.8,
                        cell_height * 1.2,
                        WHITE,
                    );
                }
            }
        }

        next_frame().await;
    }
}
