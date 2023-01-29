use bracket_lib::prelude::{main_loop, BError, BTerm, BTermBuilder, GameState};
const Y_MAX: i32 = 50;
struct State {
    y: i32,
    dt: f32,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.dt += ctx.frame_time_ms;
        ctx.print_centered(self.y, "Falling Down");
        if self.dt > 300.0 || self.y == 0 {
            self.y += 1;
            self.dt = 0.0;
        }
        if self.y > Y_MAX {
            self.y = 0;
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("\u{1FFA}\u{0416}\u{040F}\u{013F}\u{04AC}")
        .build()?;
    let state = State { y: 0, dt: 0.0 };
    main_loop(context, state)
}
