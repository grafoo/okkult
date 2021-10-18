use bracket_lib::prelude::*;
use legion::*;

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
}

struct State {
    tilemap: Vec<Vec<Tile>>,
    world: World,
    player: Entity,
}

#[derive(Clone, Copy, PartialEq)]
struct Positionable {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Renderable {
    glyph: FontCharType,
    foreground_color: RGB,
    background_color: RGB,
}

#[derive(Clone, Copy, PartialEq)]
struct Identifyable(u8);

#[derive(Clone, Copy)]
struct Collectorable {
    id: Identifyable,
}

struct Collectable {
    name: String,
    display_name: String,
    collector: Option<Collectorable>,
}

const CIPHER_SHARD: FontCharType = 15;
const FLOOR_TYLE_GLYPH: FontCharType = 250;

fn render_positionable(render: Renderable, position: &mut Positionable, bterm: &mut BTerm) {
    if position.x > 79 {
        position.x = 79;
    }
    if position.x < 0 {
        position.x = 0;
    }
    if position.y > 49 {
        position.y = 49;
    }
    if position.y < 0 {
        position.y = 0;
    }
    bterm.set(
        position.x,
        position.y,
        render.foreground_color,
        render.background_color,
        render.glyph,
    )
}

fn tick_for_tilemap(state: &mut State, bterm: &mut BTerm) {
    for x in 0..80 {
        for y in 0..50 {
            match state.tilemap[x][y] {
                Tile::Floor => {
                    bterm.set(
                        x,
                        y,
                        RGB::named(GRAY25),
                        RGB::named(BLACK),
                        FLOOR_TYLE_GLYPH,
                    );
                }
                Tile::Wall => {}
            }
        }
    }
}

fn process_keyboard_input(state: &mut State, bterm: &mut BTerm) {
    match bterm.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => {
                state.position_player(Some(-1), None, true);
            }
            VirtualKeyCode::J | VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => {
                state.position_player(None, Some(1), true);
            }
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => {
                state.position_player(None, Some(-1), true);
            }
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => {
                state.position_player(Some(1), None, true);
            }
            VirtualKeyCode::Numpad7 => {
                state.position_player(Some(-1), Some(-1), true);
            }
            VirtualKeyCode::Numpad1 => {
                state.position_player(Some(-1), Some(1), true);
            }
            VirtualKeyCode::Numpad3 => {
                state.position_player(Some(1), Some(1), true);
            }
            VirtualKeyCode::Numpad9 => {
                state.position_player(Some(1), Some(-1), true);
            }
            VirtualKeyCode::G => {
                grab_collectable(state);
            }
            VirtualKeyCode::D => {
                drop_collectable(state);
            }
            _ => {}
        },
    }
}

fn grab_collectable(state: &mut State) {
    if let Some(player) = state.world.entry(state.player) {
        let collector = *player.get_component::<Collectorable>().unwrap();
        let player_position = *player.get_component::<Positionable>().unwrap();
        for (collectable, collectable_position) in
            <(&mut Collectable, &Positionable)>::query().iter_mut(&mut state.world)
        {
            if *collectable_position == player_position {
                collectable.collector = Some(collector);
                console::log(format!("Collected {}", collectable.display_name));
            }
        }
    }
}

fn drop_collectable(state: &mut State) {
    if let Some(player) = state.world.entry(state.player) {
        let collector = *player.get_component::<Collectorable>().unwrap();
        let player_position = *player.get_component::<Positionable>().unwrap();
        for (collectable, collectable_position) in
            <(&mut Collectable, &mut Positionable)>::query().iter_mut(&mut state.world)
        {
            if let Some(collectable_collector) = collectable.collector {
                if collectable_collector.id == collector.id {
                    collectable_position.x = player_position.x;
                    collectable_position.y = player_position.y;
                    collectable.collector = None;
                    console::log(format!("Droped {}", collectable.display_name));
                    break;
                }
            }
        }
    }
}

fn tick_for_player(state: &mut State, bterm: &mut BTerm) {
    if let Some(mut player) = state.world.entry(state.player) {
        let render = *player.get_component::<Renderable>().unwrap();
        let position = player.get_component_mut::<Positionable>().unwrap();
        render_positionable(render, position, bterm);
    }
}

fn tick_for_render_positionables(state: &mut State, bterm: &mut BTerm) {
    for (render, position, collectable) in
        <(&Renderable, &mut Positionable, &Collectable)>::query().iter_mut(&mut state.world)
    {
        if let None = collectable.collector {
            render_positionable(*render, position, bterm);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        process_keyboard_input(self, bterm);
        bterm.cls();
        tick_for_tilemap(self, bterm);
        tick_for_render_positionables(self, bterm);
        tick_for_player(self, bterm);
    }
}

impl State {
    fn position_player(&mut self, x: Option<i32>, y: Option<i32>, relative: bool) {
        if let Some(mut player) = self.world.entry(self.player) {
            let mut position = player.get_component_mut::<Positionable>().unwrap();
            if relative {
                if let Some(x) = x {
                    position.x += x;
                }
                if let Some(y) = y {
                    position.y += y;
                }
            } else {
                if let Some(x) = x {
                    position.x = x;
                }
                if let Some(y) = y {
                    position.y = y;
                }
            }
        }
    }
}

fn main() -> BError {
    let mut world = World::default();

    let player = world.push((
        Positionable { x: 39, y: 24 },
        Renderable {
            glyph: to_cp437('@'),
            foreground_color: RGB::named(WHITE),
            background_color: RGB::named(BLACK),
        },
        Collectorable {
            id: Identifyable(1),
        },
    ));

    let tilemap = vec![vec![Tile::Floor; 50]; 80];

    let mut random_number_generator = RandomNumberGenerator::new();

    for _ in 0..5 {
        world.push((
            Collectable {
                name: "cipher_shard".to_string(),
                display_name: "ƆıdɥǝɹSɥɐɹp".to_string(),
                collector: None,
            },
            Renderable {
                glyph: CIPHER_SHARD,
                foreground_color: RGB::named(WHITE),
                background_color: RGB::named(BLACK),
            },
            Positionable {
                x: random_number_generator.range(0, 79),
                y: random_number_generator.range(0, 49),
            },
        ));
    }

    let state = State {
        world: world,
        player: player,
        tilemap: tilemap,
    };

    main_loop(BTermBuilder::simple80x50().build()?, state)
}
