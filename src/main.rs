use bracket_lib::prelude::{
    main_loop, BError, BTerm, BTermBuilder, GameState, Point, VirtualKeyCode,
};
use std::collections::HashMap;
const X_MAX: i32 = 79;
const Y_MAX: i32 = 49;
const PLAYER: &str = "@";
const LEFT_ONE: (i32, i32) = (-1, 0);
const DOWN_ONE: (i32, i32) = (0, 1);
const UP_ONE: (i32, i32) = (0, -1);
const RIGHT_ONE: (i32, i32) = (1, 0);
const TILE_FLOOR: &str = ".";
const TILE_WALL: &str = "#";
const TILE_DOOR_CLOSED: &str = "-";
const TILE_DOOR_OPEN: &str = "/";
struct State {
    map: HashMap<Point, TileKind>,
    player: Player,
}
struct Player {
    position: Point,
}
enum TileKind {
    Floor,
    Wall,
    DoorClosed,
    DoorOpen,
}
enum Level {
    Zero,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        for x in 0..=79 {
            for y in 0..=49 {
                match self.map.get(&Point { x, y }) {
                    Some(TileKind::Floor) => {
                        ctx.print(x, y, TILE_FLOOR);
                    }
                    Some(TileKind::Wall) => {
                        ctx.print(x, y, TILE_WALL);
                    }
                    Some(TileKind::DoorClosed) => {
                        ctx.print(x, y, TILE_DOOR_CLOSED);
                    }
                    Some(TileKind::DoorOpen) => {
                        ctx.print(x, y, TILE_DOOR_OPEN);
                    }
                    None => {}
                }
            }
        }
        ctx.print(self.player.position.x, self.player.position.y, PLAYER);
        self.handle_keyboard_input(ctx);
    }
}
impl State {
    fn new(level: Level) -> State {
        match level {
            Level::Zero => {
                let mut map: HashMap<Point, TileKind> = HashMap::new();
                for x in 0..=X_MAX {
                    for y in 0..=Y_MAX {
                        map.insert(Point { x, y }, TileKind::Floor);
                    }
                }
                for x in 0..=X_MAX {
                    map.insert(Point { x, y: 0 }, TileKind::Wall);
                    map.insert(Point { x, y: 30 }, TileKind::Wall);
                }
                for y in 1..=15 {
                    map.insert(Point { x: 40, y }, TileKind::Wall);
                    map.insert(Point { x: 50, y }, TileKind::Wall);
                }
                for x in 51..=X_MAX {
                    map.insert(Point { x, y: 15 }, TileKind::Wall);
                }
                for y in 0..=Y_MAX {
                    map.insert(Point { x: 0, y }, TileKind::Wall);
                }
                for y in 30..=Y_MAX {
                    map.insert(Point { x: 25, y }, TileKind::Wall);
                    map.insert(Point { x: 55, y }, TileKind::Wall);
                }
                for x in 0..=X_MAX {
                    map.insert(Point { x, y: Y_MAX }, TileKind::Wall);
                }
                for x in 0..=40 {
                    map.insert(Point { x, y: 15 }, TileKind::Wall);
                }
                for y in 0..=Y_MAX {
                    map.insert(Point { x: X_MAX, y }, TileKind::Wall);
                }
                for y in 15..=30 {
                    map.insert(Point { x: 60, y }, TileKind::Wall);
                }
                map.insert(Point { x: 10, y: 15 }, TileKind::DoorOpen);
                map.insert(Point { x: 55, y: 15 }, TileKind::DoorOpen);
                map.insert(Point { x: 60, y: 20 }, TileKind::DoorOpen);
                map.insert(Point { x: 20, y: 30 }, TileKind::DoorOpen);
                map.insert(Point { x: 35, y: 30 }, TileKind::DoorOpen);
                map.insert(Point { x: 70, y: 30 }, TileKind::DoorOpen);
                map.insert(Point { x: 45, y: 0 }, TileKind::DoorClosed);
                let player = Player {
                    position: Point { x: 45, y: 0 },
                };
                State { map, player }
            }
        }
    }
    fn handle_keyboard_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::H => self.move_player(LEFT_ONE),
                VirtualKeyCode::J => self.move_player(DOWN_ONE),
                VirtualKeyCode::K => self.move_player(UP_ONE),
                VirtualKeyCode::L => self.move_player(RIGHT_ONE),
                _ => {}
            }
        }
    }
    fn move_player(&mut self, (x, y): (i32, i32)) {
        let new_position = Point {
            x: self.player.position.x + x,
            y: self.player.position.y + y,
        };
        if let Some(TileKind::Wall) = self.map.get(&new_position) {
            return;
        }
        if let Some(TileKind::DoorClosed) = self.map.get(&new_position) {
            return;
        }
        self.player.position.x += x;
        self.player.position.y += y;
        if self.player.position.x > X_MAX {
            self.player.position.x = X_MAX;
        }
        if self.player.position.y > Y_MAX {
            self.player.position.y = Y_MAX;
        }

        if self.player.position.x < 0 {
            self.player.position.x = 0;
        }
        if self.player.position.y < 0 {
            self.player.position.y = 0;
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("\u{1FFA}\u{0416}\u{040F}\u{013F}\u{04AC}")
        .build()?;
    let state = State::new(Level::Zero);
    main_loop(context, state)
}
