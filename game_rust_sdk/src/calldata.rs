use ethers::{
    types::{Address, U256},
    abi::{Abi, Token, Function},
};
use std::sync::Arc;
use anyhow::{Result, anyhow};
use crate::contracts::{CRIDA_TOKEN_ABI, XP_TOKEN_ABI, GAME_TOKEN_FACTORY_ABI, GAME_TOKEN_ABI};
use hex;

/// CallDataGenerator is responsible for generating call data for all contract interactions
pub struct CallDataGenerator;

impl CallDataGenerator {
    /// Generate call data for CRIDA token approval to GameTokenFactory
    pub fn crida_approve(spender: Address, amount: U256) -> Result<String> {
        let function = Self::get_function(&CRIDA_TOKEN_ABI, "approve")?;
        let tokens = vec![
            Token::Address(spender),
            Token::Uint(amount),
        ];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for XP token approval to GameTokenFactory
    pub fn xp_approve(spender: Address, amount: U256) -> Result<String> {
        let function = Self::get_function(&XP_TOKEN_ABI, "approve")?;
        let tokens = vec![
            Token::Address(spender),
            Token::Uint(amount),
        ];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for locking CRIDA tokens in the factory to get XP
    pub fn lock_creda(amount_creda: U256) -> Result<String> {
        let function = Self::get_function(&GAME_TOKEN_FACTORY_ABI, "lockCreda")?;
        let tokens = vec![Token::Uint(amount_creda)];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for creating a new game token
    pub fn create_game_token(
        xp_amount: U256,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<String> {
        let function = Self::get_function(&GAME_TOKEN_FACTORY_ABI, "createGameToken")?;
        let tokens = vec![
            Token::Uint(xp_amount),
            Token::String(name),
            Token::String(symbol),
            Token::Uint(U256::from(decimals)),
        ];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for burning game tokens to get XP back
    pub fn burn_game_token(game_id: U256, burn_amount: U256) -> Result<String> {
        let function = Self::get_function(&GAME_TOKEN_FACTORY_ABI, "burnGameToken")?;
        let tokens = vec![
            Token::Uint(game_id),
            Token::Uint(burn_amount),
        ];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for direct game token burning (user-initiated)
    pub fn game_token_burn(amount: U256) -> Result<String> {
        let function = Self::get_function(&GAME_TOKEN_ABI, "burn")?;
        let tokens = vec![Token::Uint(amount)];
        
        Self::encode_function_data(&function, tokens)
    }
    
    /// Generate call data for game token approval to factory for burning
    pub fn game_token_approve(factory: Address, amount: U256) -> Result<String> {
        let function = Self::get_function(&GAME_TOKEN_ABI, "approve")?;
        let tokens = vec![
            Token::Address(factory),
            Token::Uint(amount),
        ];
        
        Self::encode_function_data(&function, tokens)
    }
    
    // Helper function to get a function from an ABI
    fn get_function(abi: &Arc<Abi>, name: &str) -> Result<Function> {
        abi.functions
            .get(name)
            .and_then(|functions| functions.first())
            .cloned()
            .ok_or_else(|| anyhow!("Function '{}' not found in ABI", name))
    }
    
    // Helper function to encode function call data
    fn encode_function_data(function: &Function, tokens: Vec<Token>) -> Result<String> {
        let encoded = function.encode_input(&tokens)?;
        Ok(format!("0x{}", hex::encode(encoded)))
    }
}

/// Represents the complete flow for game token creation and management
pub struct GameTokenFlow;

impl GameTokenFlow {
    /// Generate all call data for the complete flow of creating and using a game token
    pub fn complete_flow(
        factory_address: Address,
        creda_amount: U256,
        game_name: String,
        game_symbol: String,
        decimals: u8,
    ) -> Result<CompleteFlowCallData> {
        // Step 1: CREDA approval to factory
        let creda_approve = CallDataGenerator::crida_approve(factory_address, creda_amount)?;
        
        // Step 2: Lock CREDA to get XP
        let lock_creda = CallDataGenerator::lock_creda(creda_amount)?;
        
        // Step 3: Estimate XP received (this is a simplification, actual rate would come from contract)
        let xp_amount = creda_amount;
        
        // Step 4: XP approval to factory
        let xp_approve = CallDataGenerator::xp_approve(factory_address, xp_amount)?;
        
        // Step 5: Create game token
        let create_token = CallDataGenerator::create_game_token(
            xp_amount,
            game_name,
            game_symbol,
            decimals,
        )?;
        
        Ok(CompleteFlowCallData {
            creda_approve,
            lock_creda,
            xp_amount,
            xp_approve,
            create_token,
        })
    }
    
    /// Generate all call data for burning game tokens to get XP back
    pub fn burn_flow(
        factory_address: Address,
        _game_token_address: Address,
        game_id: U256,
        burn_amount: U256,
    ) -> Result<BurnFlowCallData> {
        // Step 1: Game token approval to factory
        let game_token_approve = CallDataGenerator::game_token_approve(factory_address, burn_amount)?;
        
        // Step 2: Burn game tokens
        let burn_game_token = CallDataGenerator::burn_game_token(game_id, burn_amount)?;
        
        Ok(BurnFlowCallData {
            game_token_approve,
            burn_game_token,
        })
    }
}

/// Container for all call data in the complete game token creation flow
#[derive(Debug)]
pub struct CompleteFlowCallData {
    pub creda_approve: String,
    pub lock_creda: String,
    pub xp_amount: U256,
    pub xp_approve: String,
    pub create_token: String,
}

/// Container for all call data in the game token burning flow
#[derive(Debug)]
pub struct BurnFlowCallData {
    pub game_token_approve: String,
    pub burn_game_token: String,
} 