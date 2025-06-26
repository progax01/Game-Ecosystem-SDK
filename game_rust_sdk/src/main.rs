use game_rust_sdk::{GameTokenSdk, CompleteFlowCallData, BurnFlowCallData};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "game-sdk-cli")]
#[command(about = "CLI tool for generating game contract calldata", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate calldata for the complete game token creation flow
    CreateFlow {
        /// Address of the GameTokenFactory contract
        #[arg(long)]
        factory: String,
        
        /// Amount of CRIDA tokens to lock
        #[arg(long)]
        creda_amount: String,
        
        /// Name of the game token to create
        #[arg(long)]
        name: String,
        
        /// Symbol of the game token to create
        #[arg(long)]
        symbol: String,
        
        /// Number of decimals for the game token (max 18)
        #[arg(long, default_value_t = 18)]
        decimals: u8,
    },
    
    /// Generate calldata for burning game tokens to get XP back
    BurnFlow {
        /// Address of the GameTokenFactory contract
        #[arg(long)]
        factory: String,
        
        /// Address of the game token contract
        #[arg(long)]
        token: String,
        
        /// ID of the game token
        #[arg(long)]
        game_id: String,
        
        /// Amount of game tokens to burn
        #[arg(long)]
        amount: String,
    },
    
    /// Generate calldata for approving CRIDA tokens to the factory
    ApproveCrida {
        /// Address of the GameTokenFactory contract
        #[arg(long)]
        factory: String,
        
        /// Amount of CRIDA tokens to approve
        #[arg(long)]
        amount: String,
    },
    
    /// Generate calldata for locking CRIDA tokens to get XP
    LockCrida {
        /// Amount of CRIDA tokens to lock
        #[arg(long)]
        amount: String,
    },
    
    /// Generate calldata for approving XP tokens to the factory
    ApproveXp {
        /// Address of the GameTokenFactory contract
        #[arg(long)]
        factory: String,
        
        /// Amount of XP tokens to approve
        #[arg(long)]
        amount: String,
    },
    
    /// Generate calldata for creating a game token
    CreateToken {
        /// Amount of XP tokens to lock
        #[arg(long)]
        xp_amount: String,
        
        /// Name of the game token to create
        #[arg(long)]
        name: String,
        
        /// Symbol of the game token to create
        #[arg(long)]
        symbol: String,
        
        /// Number of decimals for the game token (max 18)
        #[arg(long, default_value_t = 18)]
        decimals: u8,
    },
    
    /// Generate calldata for burning game tokens
    BurnToken {
        /// ID of the game token
        #[arg(long)]
        game_id: String,
        
        /// Amount of game tokens to burn
        #[arg(long)]
        amount: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::CreateFlow { factory, creda_amount, name, symbol, decimals } => {
            match GameTokenSdk::create_game_token_flow(
                factory,
                creda_amount,
                name,
                symbol,
                *decimals,
            ) {
                Ok(flow) => print_create_flow(flow),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::BurnFlow { factory, token, game_id, amount } => {
            match GameTokenSdk::burn_game_token_flow(
                factory,
                token,
                game_id,
                amount,
            ) {
                Ok(flow) => print_burn_flow(flow),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::ApproveCrida { factory, amount } => {
            match GameTokenSdk::approve_crida(factory, amount) {
                Ok(calldata) => println!("CRIDA Approve Calldata: {}", calldata),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::LockCrida { amount } => {
            match GameTokenSdk::lock_crida(amount) {
                Ok(calldata) => println!("Lock CRIDA Calldata: {}", calldata),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::ApproveXp { factory, amount } => {
            match GameTokenSdk::approve_xp(factory, amount) {
                Ok(calldata) => println!("XP Approve Calldata: {}", calldata),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::CreateToken { xp_amount, name, symbol, decimals } => {
            match GameTokenSdk::create_game_token(xp_amount, name, symbol, *decimals) {
                Ok(calldata) => println!("Create Game Token Calldata: {}", calldata),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        
        Commands::BurnToken { game_id, amount } => {
            match GameTokenSdk::burn_game_token(game_id, amount) {
                Ok(calldata) => println!("Burn Game Token Calldata: {}", calldata),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
    }
}

fn print_create_flow(flow: CompleteFlowCallData) {
    println!("=== Complete Game Token Creation Flow ===");
    println!("1. CRIDA Approve Calldata:");
    println!("   {}", flow.creda_approve);
    println!();
    
    println!("2. Lock CRIDA Calldata:");
    println!("   {}", flow.lock_creda);
    println!();
    
    println!("3. XP Amount Received:");
    println!("   {} tokens", flow.xp_amount);
    println!();
    
    println!("4. XP Approve Calldata:");
    println!("   {}", flow.xp_approve);
    println!();
    
    println!("5. Create Game Token Calldata:");
    println!("   {}", flow.create_token);
}

fn print_burn_flow(flow: BurnFlowCallData) {
    println!("=== Game Token Burning Flow ===");
    println!("1. Game Token Approve Calldata:");
    println!("   {}", flow.game_token_approve);
    println!();
    
    println!("2. Burn Game Token Calldata:");
    println!("   {}", flow.burn_game_token);
} 