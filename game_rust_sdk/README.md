# Game Token Rust SDK

A Rust SDK for generating call data to interact with Ethereum game token contracts. This SDK follows the enhanced game contract architecture, maintaining the original flow:

```
CREDA → XP → Deploy GameToken → Burn → XP
```

## Features

- Generate call data for the complete token creation flow
- Generate call data for token burning flow
- Individual function call data generation
- Command-line interface for easy testing
- Web API server for integration with other applications
- Beautiful web UI for interactive testing and documentation
- Proper error handling and type conversion
- No direct blockchain connection required (offline use)

## Installation

Add this library to your Rust project's `Cargo.toml`:

```toml
[dependencies]
game_rust_sdk = { git = "https://github.com/yourusername/game_rust_sdk" }
```

Or install the CLI tool:

```bash
cargo install --git https://github.com/yourusername/game_rust_sdk
```

## Usage

### As a Library

```rust
use game_rust_sdk::GameTokenSdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Generate calldata for the complete flow
    let flow = GameTokenSdk::create_game_token_flow(
        "0x1234567890123456789012345678901234567890", // factory address
        "1000000000000000000000",                     // 1000 CRIDA tokens
        "My Game Token",                              // token name
        "MGT",                                        // token symbol
        18                                            // decimals
    )?;
    
    println!("CRIDA Approval: {}", flow.creda_approve);
    println!("Lock CRIDA: {}", flow.lock_creda);
    println!("XP Approval: {}", flow.xp_approve);
    println!("Create Game Token: {}", flow.create_token);
    
    // Example: Generate calldata for burning tokens
    let burn_flow = GameTokenSdk::burn_game_token_flow(
        "0x1234567890123456789012345678901234567890", // factory address
        "0x0987654321098765432109876543210987654321", // game token address
        "1",                                          // game ID
        "500000000000000000000"                       // 500 game tokens
    )?;
    
    println!("Game Token Approval: {}", burn_flow.game_token_approve);
    println!("Burn Game Token: {}", burn_flow.burn_game_token);
    
    Ok(())
}
```

### As a CLI

The SDK includes a command-line interface for generating call data:

```bash
# Generate complete flow calldata
game-rust-sdk create-flow --factory 0x1234...5678 --creda-amount 1000000000000000000000 --name "My Game" --symbol "GAME"

# Generate burn flow calldata
game-rust-sdk burn-flow --factory 0x1234...5678 --token 0x8765...4321 --game-id 1 --amount 500000000000000000000

# Generate individual function calldata
game-rust-sdk approve-crida --factory 0x1234...5678 --amount 1000000000000000000000
game-rust-sdk lock-crida --amount 1000000000000000000000
```

### Using the API Server

The SDK includes an API server that can be used to generate call data via HTTP requests:

```bash
# Start the API server
cargo run --bin api_server

# Or specify a custom port
PORT=8000 cargo run --bin api_server
```

Once the server is running, you can make HTTP requests to generate call data:

```bash
# Example: Generate calldata for approving CRIDA tokens
curl -X POST http://localhost:3000/api/v1/calldata/creda-approve \
  -H "Content-Type: application/json" \
  -d '{"spender":"0x1234567890123456789012345678901234567890","amount":"1000000000000000000000"}'

# Example: Generate complete flow calldata
curl -X POST http://localhost:3000/api/v1/flow/create \
  -H "Content-Type: application/json" \
  -d '{"factory_address":"0x1234567890123456789012345678901234567890","creda_amount":"1000000000000000000000","game_name":"My Game Token","game_symbol":"MGT","decimals":18}'
```

### Using the Web UI

The SDK includes a beautiful web UI for interactive testing and documentation:

```bash
# Start both the API server and web UI
./start_servers.sh

# Then open your browser and navigate to:
# http://localhost:8080
```

The web UI provides:
- Interactive API documentation
- API playground for testing endpoints
- Visual representation of token flows
- Copy-paste functionality for generated call data

## API Reference

### GameTokenSdk

The main SDK interface:

```rust
// Complete flows
pub fn create_game_token_flow(
    factory_address: &str,
    creda_amount: &str,
    game_name: &str,
    game_symbol: &str,
    decimals: u8,
) -> Result<CompleteFlowCallData, SdkError>

pub fn burn_game_token_flow(
    factory_address: &str,
    game_token_address: &str,
    game_id: &str,
    burn_amount: &str,
) -> Result<BurnFlowCallData, SdkError>

// Individual function calldata
pub fn approve_crida(factory_address: &str, amount: &str) -> Result<String, SdkError>
pub fn lock_crida(amount: &str) -> Result<String, SdkError>
pub fn approve_xp(factory_address: &str, amount: &str) -> Result<String, SdkError>
pub fn create_game_token(
    xp_amount: &str,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> Result<String, SdkError>
pub fn burn_game_token(game_id: &str, amount: &str) -> Result<String, SdkError>
```

### Contract Architecture

The SDK supports the following contracts:

1. **CRIDAToken**: Base ERC-20 token that users lock to receive XP tokens
2. **XPToken**: Experience points token that can be locked to create game tokens
3. **GameTokenFactory**: Factory contract for creating and managing game tokens
4. **GameToken**: Individual game token contract deployable by users

## Contract Flow

1. **Lock CREDA → Get XP**:
   - Approve CREDA tokens to GameTokenFactory
   - Lock CREDA tokens in factory, receive XP tokens

2. **XP → Deploy GameToken**:
   - Approve XP tokens to GameTokenFactory
   - Create a new game token, locking XP tokens
   - Receive deployed GameToken contract

3. **Burn GameToken → Get XP**:
   - Approve game tokens to GameTokenFactory
   - Burn game tokens in factory
   - Receive XP tokens back based on burn amount

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 