#![feature(random)]
use alloy::primitives::{keccak256, B256, U256};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, Rect, StrokeOptions, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use std::io::Read;
use std::random::random;

fn main() -> Result<(), ggez::GameError> {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Game of (my) Life", "Ærvin").build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // let mut previous: B256 = Default::default();
    // loop {
    //     let i: i128 = random();
    //     let hash = keccak256(previous.to_string() + &i.to_string());

    //     if hash[..2] == [0; 2] {
    //         println!("Found a match! Hash: {:?} and {i}", hash);
    //         previous = hash;
    //     }
    // }

    // Run!
    event::run(ctx, event_loop, my_game);
}

#[derive(Default)]
struct MyGame {
    has_job: bool,
    has_house: bool,
    has_car: bool,
    money: f32,
    will_to_live: u8,
    happiness: u8,
    sleep: u8,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            will_to_live: 100,
            happiness: 100,
            sleep: 100,
            ..Default::default()
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(
            &Text::new(self.happiness.to_string()),
            DrawParam::default().scale([2.0, 2.0]).dest([25.0, 0.0]),
        );

        canvas.draw(
            &Text::new(self.money.to_string()),
            DrawParam::default().scale([2.0, 2.0]).dest([25.0, 30.0]),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(dir) = input.keycode {
            match dir {
                KeyCode::Right => self.happiness += 1,
                KeyCode::Left => self.happiness -= 1,
                _ => (),
            }
        }
        Ok(())
    }
}
