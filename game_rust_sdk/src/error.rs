use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid numeric value: {0}")]
    InvalidNumber(String),
    
    #[error("Contract interaction error: {0}")]
    ContractError(String),
    
    #[error("Ethereum provider error: {0}")]
    ProviderError(String),
    
    #[error("ABI encoding error: {0}")]
    EncodingError(String),
    
    #[error("Contract deployment failed: {0}")]
    DeploymentError(String),
    
    #[error("Insufficient balance: required {0}, available {1}")]
    InsufficientBalance(String, String),
    
    #[error("Insufficient allowance: required {0}, available {1}")]
    InsufficientAllowance(String, String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>> 
    for SdkError {
    fn from(err: ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>) -> Self {
        SdkError::ContractError(format!("{}", err))
    }
}

impl From<ethers::abi::Error> for SdkError {
    fn from(err: ethers::abi::Error) -> Self {
        SdkError::EncodingError(format!("{}", err))
    }
}

impl From<std::io::Error> for SdkError {
    fn from(err: std::io::Error) -> Self {
        SdkError::Unknown(format!("IO error: {}", err))
    }
}

impl From<serde_json::Error> for SdkError {
    fn from(err: serde_json::Error) -> Self {
        SdkError::Unknown(format!("JSON error: {}", err))
    }
} 