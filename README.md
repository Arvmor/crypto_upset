# Game of (My) Life

Ever wanted a game that accurately simulates the beautiful chaos of real life? No? Well, here it is anyway! 🤷‍♂️ This Rust-powered masterpiece lets you move around, get a job (or not), earn virtual money (that’s still worth nothing 💸), and interact with a blockchain-based smart contract—because what’s life without a little decentralization?

## Features
- **Move Around Aimlessly**: Navigate the world with no clear purpose, just like real life.
- **Get a Job (Maybe) 💼**: Stand on the magic work square and suddenly become employed.
- **Make Money (A Whole 0.01 Per Mining Success!) 💰**: Uses a blockchain-based mining mechanism that pays out just enough to keep you going.
- **Ethereum Smart Contract Integration**: Because even fictional lives need an immutable ledger.
- **Existential Crisis Mechanic**: Monitor your happiness, sleep, and will to live—just like in reality!

## Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/Arvmor/crypto_upset
   cd game-of-life
   ```
2. Install dependencies:
   ```sh
   cargo build
   ```
3. Create a `.env` file and add the following variables:
   ```sh
   CONTRACT_ADDRESS=53f4E4DF2c7B9f5B628eAf41Fd5621d08f833912
   RPC_URL=https://mainnet.base.org
   PRIVATE_KEY=<your_private_key>
   ```
4. Run the game:
   ```sh
   cargo run
   ```

## Usage
- Use **arrow keys** to move around the game world.
- Work by reaching the job location at `[80.0, 80.0]` / `+`.
- The mining mechanism runs asynchronously in the background.
- When a valid hash is found, it updates the contract and earns money.

## Contribution Guidelines
We welcome contributions! Follow these steps to contribute:
1. Fork the repository.
2. Create a new branch (`feature/your-feature` or `fix/your-fix`).
3. Commit your changes with clear commit messages.
4. Push your branch and open a pull request.

### Code Style
- Follow Rust best practices.
- Use `cargo fmt` and `cargo clippy` before submitting changes.

### Issues
If you encounter any issues, feel free to create a GitHub issue with details.

## License
This project is licensed under the MIT License. See `LICENSE` for details.

## Acknowledgments
- **ggez** for game development.
- **alloy-rs** for Ethereum integration.
