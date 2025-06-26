pub mod calldata;
pub mod contracts;
pub mod error;
pub mod utils;
pub mod api;

pub use calldata::{CallDataGenerator, GameTokenFlow, CompleteFlowCallData, BurnFlowCallData};
pub use error::SdkError;
pub use utils::{parse_address, parse_u256, to_wei, format_units};



/// Game Token SDK for generating call data to interact with the game ecosystem contracts
/// 
/// This SDK allows you to generate the raw calldata needed to interact with the game ecosystem
/// smart contracts, following the flow: CREDA → XP → Deploy GameToken → Burn → XP
pub struct GameTokenSdk;

impl GameTokenSdk {
    /// Generate call data for the complete flow of creating a game token
    /// 
    /// # Arguments
    /// * `factory_address` - The address of the GameTokenFactory contract
    /// * `creda_amount` - The amount of CREDA tokens to lock
    /// * `game_name` - The name of the game token to create
    /// * `game_symbol` - The symbol of the game token to create
    /// * `decimals` - The number of decimals for the game token (max 18)
    pub fn create_game_token_flow(
        factory_address: &str,
        creda_amount: &str,
        game_name: &str,
        game_symbol: &str,
        decimals: u8,
    ) -> Result<CompleteFlowCallData, error::SdkError> {
        let factory_addr = utils::parse_address(factory_address)
            .map_err(|_| error::SdkError::InvalidAddress(factory_address.to_string()))?;
            
        let amount = utils::parse_u256(creda_amount)
            .map_err(|_| error::SdkError::InvalidNumber(creda_amount.to_string()))?;
            
        GameTokenFlow::complete_flow(
            factory_addr,
            amount,
            game_name.to_string(),
            game_symbol.to_string(),
            decimals,
        )
        .map_err(|e| error::SdkError::ContractError(e.to_string()))
    }
    
    /// Generate call data for burning game tokens to get XP back
    /// 
    /// # Arguments
    /// * `factory_address` - The address of the GameTokenFactory contract
    /// * `game_token_address` - The address of the game token contract
    /// * `game_id` - The ID of the game token
    /// * `burn_amount` - The amount of game tokens to burn
    pub fn burn_game_token_flow(
        factory_address: &str,
        game_token_address: &str,
        game_id: &str,
        burn_amount: &str,
    ) -> Result<BurnFlowCallData, error::SdkError> {
        let factory_addr = utils::parse_address(factory_address)
            .map_err(|_| error::SdkError::InvalidAddress(factory_address.to_string()))?;
            
        let token_addr = utils::parse_address(game_token_address)
            .map_err(|_| error::SdkError::InvalidAddress(game_token_address.to_string()))?;
            
        let id = utils::parse_u256(game_id)
            .map_err(|_| error::SdkError::InvalidNumber(game_id.to_string()))?;
            
        let amount = utils::parse_u256(burn_amount)
            .map_err(|_| error::SdkError::InvalidNumber(burn_amount.to_string()))?;
            
        GameTokenFlow::burn_flow(
            factory_addr,
            token_addr,
            id,
            amount,
        )
        .map_err(|e| error::SdkError::ContractError(e.to_string()))
    }
    
    /// Generate call data for approving CRIDA tokens to be spent by the factory
    /// 
    /// # Arguments
    /// * `factory_address` - The address of the GameTokenFactory contract
    /// * `amount` - The amount of CRIDA tokens to approve
    pub fn approve_crida(
        factory_address: &str,
        amount: &str,
    ) -> Result<String, error::SdkError> {
        let factory_addr = utils::parse_address(factory_address)
            .map_err(|_| error::SdkError::InvalidAddress(factory_address.to_string()))?;
            
        let amount_wei = utils::parse_u256(amount)
            .map_err(|_| error::SdkError::InvalidNumber(amount.to_string()))?;
            
        CallDataGenerator::crida_approve(factory_addr, amount_wei)
            .map_err(|e| error::SdkError::EncodingError(e.to_string()))
    }
    
    /// Generate call data for approving XP tokens to be spent by the factory
    /// 
    /// # Arguments
    /// * `factory_address` - The address of the GameTokenFactory contract
    /// * `amount` - The amount of XP tokens to approve
    pub fn approve_xp(
        factory_address: &str,
        amount: &str,
    ) -> Result<String, error::SdkError> {
        let factory_addr = utils::parse_address(factory_address)
            .map_err(|_| error::SdkError::InvalidAddress(factory_address.to_string()))?;
            
        let amount_wei = utils::parse_u256(amount)
            .map_err(|_| error::SdkError::InvalidNumber(amount.to_string()))?;
            
        CallDataGenerator::xp_approve(factory_addr, amount_wei)
            .map_err(|e| error::SdkError::EncodingError(e.to_string()))
    }
    
    /// Generate call data for locking CRIDA tokens to get XP
    /// 
    /// # Arguments
    /// * `amount` - The amount of CRIDA tokens to lock
    pub fn lock_crida(
        amount: &str,
    ) -> Result<String, error::SdkError> {
        let amount_wei = utils::parse_u256(amount)
            .map_err(|_| error::SdkError::InvalidNumber(amount.to_string()))?;
            
        CallDataGenerator::lock_creda(amount_wei)
            .map_err(|e| error::SdkError::EncodingError(e.to_string()))
    }
    
    /// Generate call data for creating a game token
    /// 
    /// # Arguments  
    /// * `xp_amount` - The amount of XP tokens to lock
    /// * `name` - The name of the game token
    /// * `symbol` - The symbol of the game token
    /// * `decimals` - The number of decimals for the game token
    pub fn create_game_token(
        xp_amount: &str,
        name: &str,
        symbol: &str,
        decimals: u8,
    ) -> Result<String, error::SdkError> {
        let amount = utils::parse_u256(xp_amount)
            .map_err(|_| error::SdkError::InvalidNumber(xp_amount.to_string()))?;
            
        CallDataGenerator::create_game_token(
            amount,
            name.to_string(),
            symbol.to_string(),
            decimals,
        )
        .map_err(|e| error::SdkError::EncodingError(e.to_string()))
    }
    
    /// Generate call data for burning game tokens
    /// 
    /// # Arguments
    /// * `game_id` - The ID of the game token
    /// * `amount` - The amount of game tokens to burn
    pub fn burn_game_token(
        game_id: &str,
        amount: &str,
    ) -> Result<String, error::SdkError> {
        let id = utils::parse_u256(game_id)
            .map_err(|_| error::SdkError::InvalidNumber(game_id.to_string()))?;
            
        let burn_amount = utils::parse_u256(amount)
            .map_err(|_| error::SdkError::InvalidNumber(amount.to_string()))?;
            
        CallDataGenerator::burn_game_token(id, burn_amount)
            .map_err(|e| error::SdkError::EncodingError(e.to_string()))
    }
} 