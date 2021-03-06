use bracket_lib::prelude::*;
use legion::*;
use std::time::Instant;

#[derive(Clone)]
enum Tile {
    Floor,
    Wall,
}

struct State {
    tilemap: Vec<Vec<Tile>>,
    world: World,
    player: Entity,
    cipher_shard_connection_points: Vec<Point>,
    instant_time: Instant,
    delta_time: u128,
    animation: Animateable,
    player_is_positionable: bool,
    assigned_player_position: Positionable,
    player_path: Vec<Point>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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

struct Connectable {
    connection: Option<Identifyable>,
    connector: Option<Identifyable>,
}

#[derive(Clone, Copy, Debug)]
struct Activateable {
    state: bool,
    activator: Option<Identifyable>,
}

struct Activatorable {
    id: Identifyable,
}

struct Animateable {
    duration: u128,
    elapsed: u128,
}

const CIPHER_SHARD_GLYPH: FontCharType = 15;
const CIPHER_SHARD_CONNECTION_GLYPH: FontCharType = 7;
const FLOOR_TYLE_GLYPH: FontCharType = 250;

const MOUSE_BUTTON_LEFT: usize = 0;
const MOUSE_BUTTON_RIGHT: usize = 1;
const MOUSE_BUTTON_MIDDLE: usize = 2;

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
                        RGB::named(GRAY15),
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
            VirtualKeyCode::C => {
                connect(state);
            }
            VirtualKeyCode::A => {
                let id = get_player_activator_id(state);
                activate(state, id);
            }
            _ => {}
        },
    }
}

fn get_player_position(state: &mut State) -> Positionable {
    *state
        .world
        .entry(state.player)
        .unwrap()
        .get_component::<Positionable>()
        .unwrap()
}

fn get_player_collector_id(state: &mut State) -> Identifyable {
    state
        .world
        .entry(state.player)
        .unwrap()
        .get_component::<Collectorable>()
        .unwrap()
        .id
}

fn get_player_activator_id(state: &mut State) -> Identifyable {
    get_player_collector_id(state)
}

fn connect(state: &mut State) {
    let player_position = get_player_position(state);
    let player_collector_id = get_player_collector_id(state);
    for (collectable, collectable_position, connectable) in
        <(&mut Collectable, &Positionable, &mut Connectable)>::query().iter_mut(&mut state.world)
    {
        if *collectable_position == player_position {
            connectable.connector = Some(player_collector_id);
            console::log("Connected");
        }
    }
}

fn activate(state: &mut State, activator_id: Identifyable) {
    for (activateable, activateable_position, connectable) in
        <(&mut Activateable, &Positionable, &mut Connectable)>::query().iter_mut(&mut state.world)
    {
        if let Some(connector_id) = connectable.connector {
            if connector_id == activator_id {
                connectable.connector = None;
                activateable.state = true;
                console::log("Activated");
            }
        }
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

fn tick_for_activateables(state: &mut State, bterm: &mut BTerm) {
    let mut maybe_activateable_positions: Vec<(&mut Activateable, &Positionable)> =
        <(&mut Activateable, &Positionable)>::query()
            .iter_mut(&mut state.world)
            .collect();

    if maybe_activateable_positions.len() < 2 {
        return;
    }
    let mut activateable_positions: Vec<(&mut Activateable, &Positionable)> = Vec::new();
    for maybe_activateable_position in maybe_activateable_positions {
        if maybe_activateable_position.0.state {
            activateable_positions.push(maybe_activateable_position);
        }
        if activateable_positions.len() == 2 {
            break;
        }
    }
    if activateable_positions.len() < 2 {
        return;
    }

    let mut activateable_position_1 = activateable_positions.remove(0);
    let mut activateable_1 = activateable_position_1.0;
    let position_1 = activateable_position_1.1;
    let mut activateable_position_2 = activateable_positions.remove(0);
    let mut activateable_2 = activateable_position_2.0;
    let position_2 = activateable_position_2.1;

    if activateable_1.state && activateable_2.state {
        activateable_1.state = false;
        activateable_2.state = false;
        let line = line2d_vector(
            Point::new(position_1.x, position_1.y),
            Point::new(position_2.x, position_2.y),
        );
        for point in line {
            state.cipher_shard_connection_points.push(point);
        }
    }
}

fn tick_for_cipher_shard_connections(state: &mut State, bterm: &mut BTerm) {
    let elapsed_time_percentile = state.animation.elapsed as f64 / state.animation.duration as f64;
    let foreground_color = if elapsed_time_percentile < 0.125 {
        RED
    } else if elapsed_time_percentile <= 0.25 {
        RED1
    } else if elapsed_time_percentile <= 0.5 {
        RED2
    } else if elapsed_time_percentile <= 0.75 {
        RED3
    } else {
        RED4
    };

    for point in &state.cipher_shard_connection_points {
        bterm.set(
            point.x,
            point.y,
            RGB::named(foreground_color),
            RGB::named(BLACK),
            CIPHER_SHARD_CONNECTION_GLYPH,
        );
    }
}

fn tick_for_mouse(state: &mut State, bterm: &mut BTerm) {
    let (mouse_position_x, mouse_position_y) = bterm.mouse_pos();

    bterm.set(
        mouse_position_x,
        mouse_position_y,
        RGB::named(WHITE),
        RGB::named(BLACK),
        to_cp437('X'),
    );

    if !state.player_is_positionable && get_player_position(state) != state.assigned_player_position
    {
        console::log(format!("{:?}", state.player_path));
        if state.player_path.len() > 0 {
            let step = state.player_path.pop().unwrap();
            state.position_player(Some(step.x), Some(step.y), false);
        }
    } else {
        state.player_is_positionable = true;
    }

    let input = INPUT.lock();
    if input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
        if state.player_is_positionable {
            state.assigned_player_position.x = mouse_position_x;
            state.assigned_player_position.y = mouse_position_y;
            state.player_is_positionable = false;
            let mut tmp = line2d_vector(
                Point::new(get_player_position(state).x, get_player_position(state).y),
                Point::new(mouse_position_x, mouse_position_y),
            );
            tmp.reverse();
            state.player_path = tmp;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        self.write_delta_time();
        self.write_elapsed_animation_time();
        process_keyboard_input(self, bterm);
        bterm.cls();
        tick_for_tilemap(self, bterm);
        tick_for_activateables(self, bterm);
        tick_for_cipher_shard_connections(self, bterm);
        tick_for_render_positionables(self, bterm);
        tick_for_mouse(self, bterm);
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

    fn write_delta_time(&mut self) {
        self.delta_time = self.instant_time.elapsed().as_millis();
        self.instant_time = Instant::now();
    }

    fn write_elapsed_animation_time(&mut self) {
        self.animation.elapsed += self.delta_time;
        if self.animation.elapsed >= self.animation.duration {
            self.animation.elapsed = 0;
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
        Activatorable {
            id: Identifyable(1),
        },
    ));

    let tilemap = vec![vec![Tile::Floor; 50]; 80];

    let mut random_number_generator = RandomNumberGenerator::new();

    for _ in 0..5 {
        world.push((
            Collectable {
                name: "cipher_shard".to_string(),
                display_name: "????d??????S??????p".to_string(),
                collector: None,
            },
            Renderable {
                glyph: CIPHER_SHARD_GLYPH,
                foreground_color: RGB::named(YELLOW),
                background_color: RGB::named(BLACK),
            },
            Positionable {
                x: random_number_generator.range(0, 79),
                y: random_number_generator.range(0, 49),
            },
            Connectable {
                connection: None,
                connector: None,
            },
            Activateable {
                state: false,
                activator: None,
            },
        ));
    }

    let state = State {
        world: world,
        player: player,
        tilemap: tilemap,
        cipher_shard_connection_points: Vec::new(),
        instant_time: Instant::now(),
        delta_time: 0,
        animation: Animateable {
            duration: 1250,
            elapsed: 0,
        },
        player_is_positionable: true,
        assigned_player_position: Positionable { x: 0, y: 0 },
        player_path: Vec::new(),
    };

    main_loop(BTermBuilder::simple80x50().build()?, state)
}
