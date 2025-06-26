use ethers::{
    abi::Abi,
    contract::abigen,
};
use std::sync::Arc;
use once_cell::sync::Lazy;
use serde_json::{Value, from_str};

// Generate Rust bindings for contracts using the ethers-rs abigen macro
abigen!(
    CRIDAToken,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address from, address to, uint256 amount) external returns (bool)
        function mint(address to, uint256 amount) external
    ]"#
);

abigen!(
    XPToken,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address from, address to, uint256 amount) external returns (bool)
        function mint(address to, uint256 amount) external
        function burn(uint256 amount) external
        function burnFrom(address from, uint256 amount) external
        function totalInCirculation() external view returns (uint256)
        function getStats() external view returns (uint256 minted, uint256 burned, uint256 circulation)
        function pause() external
        function unpause() external
    ]"#
);

abigen!(
    GameTokenFactory,
    r#"[
        function lockCreda(uint256 amountCreda) external
        function createGameToken(uint256 xpAmount, string calldata name, string calldata symbol, uint8 decimals) external returns (uint256 gameId, address tokenAddress)
        function burnGameToken(uint256 gameId, uint256 burnAmount) external
        function setRate(uint256 newRate) external
        function pause() external
        function unpause() external
        function emergencyWithdraw(address token, address to, uint256 amount) external
        function credaToXpRate() external view returns (uint256)
        function userLockedCreda(address user) external view returns (uint256)
        function totalLockedCreda() external view returns (uint256)
        function gameTokens(uint256 gameId) external view returns (address tokenAddress, address creator, uint88 xpLocked, uint8 decimals, bool active)
        function xpReserves() external view returns (uint256)
        function nextGameId() external view returns (uint256)
    ]"#
);

abigen!(
    GameToken,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address from, address to, uint256 amount) external returns (bool)
        function burn(uint256 amount) external
        function burnFrom(address from, uint256 amount) external
        function factory() external view returns (address)
        function gameId() external view returns (uint256)
        function totalBurned() external view returns (uint256)
        function burnEnabled() external view returns (bool)
        function setBurnEnabled(bool enabled) external
        function emergencyRecover(address token, address to, uint256 amount) external
    ]"#
);

// Define contract JSON ABIs for call data encoding
pub static CRIDA_TOKEN_ABI: Lazy<Arc<Abi>> = Lazy::new(|| {
    let json = r#"
    [
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "owner",
                    "type": "address"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "spender",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "approve",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "to",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "mint",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "to",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "transfer",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "from",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "to",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "transferFrom",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "owner",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "spender",
                    "type": "address"
                }
            ],
            "name": "allowance",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "",
                    "type": "uint256"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "account",
                    "type": "address"
                }
            ],
            "name": "balanceOf",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "",
                    "type": "uint256"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;
    
    let value: Value = from_str(json).unwrap();
    let abi_json = serde_json::to_string(&value).unwrap();
    Arc::new(serde_json::from_str(&abi_json).unwrap())
});

pub static XP_TOKEN_ABI: Lazy<Arc<Abi>> = Lazy::new(|| {
    let json = r#"
    [
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "spender",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "approve",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "burn",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "from",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "burnFrom",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;
    
    let value: Value = from_str(json).unwrap();
    let abi_json = serde_json::to_string(&value).unwrap();
    Arc::new(serde_json::from_str(&abi_json).unwrap())
});

pub static GAME_TOKEN_FACTORY_ABI: Lazy<Arc<Abi>> = Lazy::new(|| {
    let json = r#"
    [
        {
            "inputs": [
                {
                    "internalType": "uint256",
                    "name": "gameId",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "burnAmount",
                    "type": "uint256"
                }
            ],
            "name": "burnGameToken",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "uint256",
                    "name": "xpAmount",
                    "type": "uint256"
                },
                {
                    "internalType": "string",
                    "name": "name",
                    "type": "string"
                },
                {
                    "internalType": "string",
                    "name": "symbol",
                    "type": "string"
                },
                {
                    "internalType": "uint8",
                    "name": "decimals",
                    "type": "uint8"
                }
            ],
            "name": "createGameToken",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "gameId",
                    "type": "uint256"
                },
                {
                    "internalType": "address",
                    "name": "tokenAddress",
                    "type": "address"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "to",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "emergencyWithdraw",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "uint256",
                    "name": "amountCreda",
                    "type": "uint256"
                }
            ],
            "name": "lockCreda",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;
    
    let value: Value = from_str(json).unwrap();
    let abi_json = serde_json::to_string(&value).unwrap();
    Arc::new(serde_json::from_str(&abi_json).unwrap())
});

pub static GAME_TOKEN_ABI: Lazy<Arc<Abi>> = Lazy::new(|| {
    let json = r#"
    [
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "spender",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "approve",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "burn",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "from",
                    "type": "address"
                },
                {
                    "internalType": "uint256",
                    "name": "amount",
                    "type": "uint256"
                }
            ],
            "name": "burnFrom",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;
    
    let value: Value = from_str(json).unwrap();
    let abi_json = serde_json::to_string(&value).unwrap();
    Arc::new(serde_json::from_str(&abi_json).unwrap())
}); 