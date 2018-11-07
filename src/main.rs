//! The smallest running game.

extern crate ggez;
extern crate rand;

use std::env;
use std::path;
use std::time;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use rand::Rng;

const WINDOW_WIDTH : u32 = 190;
const WINDOW_HEIGHT : u32 = 95;

const CHANGE_FREQUENCY : u32 = 2;
const CHANGE_PROPABILITY : f32 = 0.7;

const COUNTDOWN_FROM : u32 = 3;

const GOAL_POSITION : u32 = 180;
const PLAYER_START : u32 = 20;
const SCREEN_PADDING : u32 = 5;

const LUNATIC_RUNNING : [[[u32; 5]; 5]; 6] = [
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 1, 0, 1, 0],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 0, 0],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 1, 0],
    ],
];

const ALPHABET : [[[u32; 5]; 5]; 35] = [
    [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [0, 1, 0, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 1, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 1, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 1],
    ],
    [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ],
    [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 1, 1],
    ],
    [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [0, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [1, 0, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ],
    [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
];

const ALPHABET_STR : [char; 35]= [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

fn get_random_char() -> char {
    return rand::thread_rng().gen_range(b'A', b'Z') as char;
}

fn is_countdown_finished(countdown: u32) -> bool {
    return countdown > COUNTDOWN_FROM - 1;
}

fn convert_countdown(countdown: u32) -> u32 {
    return COUNTDOWN_FROM - countdown;
}

pub struct Timer {
    started: time::Duration,
    value: f32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            started: time::Duration::new(0, 0),
            value: 0f32,
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value as u32
    }

    pub fn start(&mut self, ctx: &ggez::Context) {
        self.started = timer::get_time_since_start(ctx);
        self.value = 0f32;
    }

    pub fn update(&mut self, ctx: &ggez::Context) {
        let time_passed = timer::duration_to_f64(timer::get_time_since_start(ctx)) - timer::duration_to_f64(self.started);
        self.value = time_passed as f32;
    }
}

pub struct Player {
    key: char,
    pos: usize,
}

impl Player {
    pub fn new() -> ggez::GameResult<Player> {
        Ok(Player {
            key: get_random_char(),
            pos: 0,
        })
    }

    pub fn get_position(&self) -> usize {
        return self.pos;
    }

    pub fn get_key(&self) -> String {
        return self.key.to_string();
    }

    pub fn has_reached_goal(&self) -> bool {
        return self.get_position() > (GOAL_POSITION - PLAYER_START) as usize;
    }

    pub fn has_key_pressed(&self, keycode: event::Keycode) -> bool {
        return self.get_key() == keycode.name();
    }

    pub fn run(&mut self) {
        self.pos += 1;
    }

    pub fn assign_random_key(&mut self) {
        self.key = get_random_char();
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.assign_random_key();
    }
}

fn draw_bitfield<'a, T, U>(
    ctx: &mut ggez::Context,
    bitfield: &'a T,
    pos_x: f32,
    pos_y: f32,
) -> ggez::GameResult<()>
where
    &'a T: std::iter::IntoIterator<Item = U>,
    U: std::iter::IntoIterator<Item = &'a u32>,
{
    let mut points: Vec<graphics::Point2> = vec![];

    for (y, row) in bitfield.into_iter().enumerate() {
        for (x, &value) in row.into_iter().enumerate() {
            if value == 1 {
                let point_x = x as f32 + pos_x;
                let point_y = y as f32 + pos_y;

                points.push(graphics::Point2::new(point_x, point_y));
            }
        }
    }

    graphics::points(ctx, &points, 1.0)?;

    Ok(())
}

fn draw_text(
    ctx: &mut ggez::Context,
    text: &str,
    position: graphics::Point2,
) -> ggez::GameResult<()> {
    for (i, character) in text.to_uppercase().chars().enumerate() {
        let pos_x = position.x + (i as f32 * 8.0);
        let pos_y = position.y;

        match ALPHABET_STR.iter().position(|&x| x == character) {
            Some(character_index) => {
                draw_bitfield(ctx, &ALPHABET[character_index], pos_x, pos_y)?;
            }
            None => ()
        }
    }

    Ok(())
}

fn draw_player(
    ctx: &mut ggez::Context,
    player: &Player,
    offset_y: f32,
) -> ggez::GameResult<()> {
    let position = player.get_position();
    let frame = position % LUNATIC_RUNNING.len();
    let offset_x: f32 = position as f32 + PLAYER_START as f32;

    draw_bitfield(ctx, &LUNATIC_RUNNING[frame], offset_x, offset_y)?;

    draw_text(
        ctx,
        &player.get_key(),
        graphics::Point2::new(SCREEN_PADDING as f32, offset_y)
    )?;

    Ok(())
}

pub fn draw_select_screen(
    ctx: &mut ggez::Context,
) -> ggez::GameResult<()> {
    draw_text(
        ctx,
        "Press number of players",
        graphics::Point2::new(5.0, 40.0)
    )?;

    Ok(())
}

pub fn draw_countdown_screen(
    ctx: &mut ggez::Context,
    countdown: u32,
) -> ggez::GameResult<()> {
    draw_text(
        ctx,
        &countdown.to_string(),
        graphics::Point2::new(95.0, 40.0)
    )?;

    Ok(())
}

pub fn draw_players_screen(
    ctx: &mut ggez::Context,
    players: &mut Vec<Player>
) -> ggez::GameResult<()> {
    for (i, player) in players.iter().enumerate() {
        let offset = (i as f32 * 10.0 as f32) + SCREEN_PADDING as f32;

        draw_player(ctx, player, offset)?;
    }

    let points = &[
        graphics::Point2::new(GOAL_POSITION as f32, 0.0),
        graphics::Point2::new(GOAL_POSITION as f32, WINDOW_HEIGHT as f32)];

    graphics::line(ctx, points, 1.0).unwrap();

    Ok(())
}

pub fn draw_winner_screen(
    ctx: &mut ggez::Context,
    winner_no: u32,
) -> ggez::GameResult<()> {
    draw_text(
        ctx,
        &format!("player {} won", winner_no),
        graphics::Point2::new(50.0, 40.0)
    )?;

    Ok(())
}

fn init_players(state: &mut MainState, num: u32) {
    for _ in 0..num {
        state.players.push(Player::new().unwrap());
    }
}

fn start_game(state: &mut MainState, ctx: &mut ggez::Context) {
    state.timer.start(ctx);
    state.ui = UIState::Play;
}

fn move_players(state: &mut MainState, keycode: event::Keycode) {
    if !is_countdown_finished(state.timer.get_value()) {
        return;
    }

    for player in &mut state.players {
        if player.has_key_pressed(keycode) {
            player.run();
        }
    }
}

fn restart_game(state: &mut MainState, ctx: &mut ggez::Context) {
    for player in &mut state.players {
        player.reset();
    }

    state.timer.start(ctx);

    state.ui = UIState::Play;
}

enum UIState {
    Select,
    Play,
    Win,
}

pub struct MainState {
    change_mode: bool,
    players: Vec<Player>,
    rng: rand::ThreadRng,
    timer: Timer,
    ui: UIState,
    winner: u32,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        graphics::set_background_color(ctx, (0, 0, 255, 255).into());

        let rng = rand::thread_rng();

        let s = MainState {
            change_mode: false,
            players: Vec::new(),
            rng,
            timer: Timer::new(),
            ui: UIState::Select,
            winner: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.timer.update(ctx);

        match self.ui {
            UIState::Play => {
                for (i, player) in self.players.iter().enumerate() {
                    if player.has_reached_goal() {
                        self.winner = (i + 1) as u32;
                        self.ui = UIState::Win;
                    }
                }

                let timer_value = self.timer.get_value();

                if is_countdown_finished(timer_value) && timer_value % CHANGE_FREQUENCY == 0 {
                    if !self.change_mode {
                        self.change_mode = true;

                        if rand::random::<f32>() < CHANGE_PROPABILITY {
                            let random_index = self.rng.gen_range(0, self.players.len());
                            self.players[random_index].assign_random_key();
                        }
                    }
                } else {
                    self.change_mode = false;
                }
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx);

        match self.ui {
            UIState::Select => {
                draw_select_screen(ctx)?;
            }
            UIState::Play => {
                let time = self.timer.get_value();
                if !is_countdown_finished(time) {
                    draw_countdown_screen(ctx, convert_countdown(time))?;
                }
                draw_players_screen(ctx, &mut self.players)?;
            }
            UIState::Win => {
                draw_winner_screen(ctx, self.winner)?;
            }
        }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        repeat: bool
    ) {
        match self.ui {
            UIState::Select => {
                match keycode.name().parse::<u32>() {
                    Ok(num) => {
                        init_players(self, num);
                        start_game(self, ctx);
                    }
                    Err(_) => {}
                }
            }
            UIState::Play => {
                if repeat {
                    return;
                }

                move_players(self, keycode);
            }
            UIState::Win => {
                if keycode != event::Keycode::Space &&
                    keycode != event::Keycode::Return {
                    return;
                }

                restart_game(self, ctx);
            }
        }
    }
}

pub fn main() {
    let window_setup = conf::WindowSetup::default()
        .icon("/icon.png")
        .title("Lunatics");

    let window_mode = conf::WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut cb = ggez::ContextBuilder::new("luncatics", "adzialocha")
        .window_setup(window_setup)
        .window_mode(window_mode);

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
