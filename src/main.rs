#![feature(random)]
use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Address, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use dotenv::dotenv;
use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameResult,
};
use std::{
    env::var,
    random::random,
    str::FromStr,
    sync::{Arc, RwLock},
};

// Life Contract
sol!(
    #[sol(rpc)]
    "./contract/life.sol"
);

#[tokio::main]
async fn main() -> Result<(), ggez::GameError> {
    dotenv().ok();

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Game of (my) Life", "Ærvin")
        .window_mode(WindowMode::default().maximized(true))
        .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    let cloned_game = my_game.clone();
    tokio::spawn(async move {
        let rpc_url = var("RPC_URL").unwrap().parse().unwrap();
        let wallet =
            EthereumWallet::from(PrivateKeySigner::from_str(&var("PRIVATE_KEY").unwrap()).unwrap());
        let contract_address = Address::from_str(&var("CONTRACT_ADDRESS").unwrap()).unwrap();

        // Instance
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let life_contract = Life::new(contract_address, provider);
        *cloned_game.genesis.write().unwrap() = life_contract
            .LATEST_HASH()
            .call()
            .await
            .unwrap()
            .LATEST_HASH;

        loop {
            if *cloned_game.has_job.read().unwrap() {
                if let Some(number) = mine(&cloned_game) {
                    life_contract
                        .setNewHash(U256::from(number))
                        .send()
                        .await
                        .ok();
                }
            }
        }
    });

    // Run!
    event::run(ctx, event_loop, my_game);
}

#[derive(Default, Clone)]
struct MyGame {
    has_job: Arc<RwLock<bool>>,
    has_house: bool,
    has_car: bool,
    money: Arc<RwLock<f64>>,
    will_to_live: u8,
    happiness: u8,
    sleep: u8,
    location: [f32; 2],
    genesis: Arc<RwLock<B256>>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        Self {
            will_to_live: 100,
            happiness: 100,
            sleep: 100,
            location: [50.0, 50.0],
            ..Default::default()
        }
    }
}

fn mine(game: &MyGame) -> Option<u128> {
    let i = random::<u128>();
    let packed = SolValue::abi_encode_packed(&(*game.genesis.read().unwrap(), i));
    let hash = keccak256(packed);

    if hash[..2] == [0; 2] {
        println!("Found a match! Hash: {} and {:?}", hash, i);
        *game.genesis.write().unwrap() = hash;
        *game.money.write().unwrap() += 0.01;
        return Some(i);
    }

    None
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        *self.has_job.write().unwrap() = self.location == [80.0, 80.0];

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(
            &Text::new(format!("Is working: {}", self.has_job.read().unwrap())),
            DrawParam::default().scale([2.0, 2.0]).dest([25.0, 0.0]),
        );

        canvas.draw(
            &Text::new(format!("Money: ${}", self.money.read().unwrap())),
            DrawParam::default().scale([2.0, 2.0]).dest([25.0, 30.0]),
        );

        canvas.draw(
            &Text::new("+"),
            DrawParam::default().scale([3.0, 3.0]).dest(self.location),
        );

        canvas.draw(
            &Text::new("+"),
            DrawParam::default().scale([3.0, 3.0]).dest([80.0, 80.0]),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(dir) = input.keycode {
            match dir {
                KeyCode::Right => self.location[0] += 10.0,
                KeyCode::Left => self.location[0] -= 10.0,
                KeyCode::Down => self.location[1] += 10.0,
                KeyCode::Up => self.location[1] -= 10.0,
                _ => (),
            }
        }
        Ok(())
    }
}
