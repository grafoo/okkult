use macroquad::prelude::*;
#[macroquad::main("\u{1FFA}\u{0416}\u{040F}\u{013F}\u{04AC}")]
async fn main() {
    loop {
        clear_background(BLANK);
        next_frame().await;
    }
}
