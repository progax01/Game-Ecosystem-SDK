use game_rust_sdk::GameTokenSdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Game Token SDK Example");
    println!("=====================\n");
    
    // Example contract addresses
    let factory_address = "0x1234567890123456789012345678901234567890";
    let game_token_address = "0xabcdef0123456789abcdef0123456789abcdef01";
    
    // Example values
    let creda_amount = "1000000000000000000000"; // 1000 CREDA tokens (with 18 decimals)
    let game_id = "1";
    let burn_amount = "500000000000000000000"; // 500 tokens (with 18 decimals)
    
    // Example: Generate calldata for the complete flow
    println!("== Complete Flow Example ==");
    let flow = GameTokenSdk::create_game_token_flow(
        factory_address,
        creda_amount,
        "My Game Token",
        "MGT",
        18
    )?;
    
    println!("CRIDA Approval: {}", flow.creda_approve);
    println!("Lock CRIDA: {}", flow.lock_creda);
    println!("XP Approval: {}", flow.xp_approve);
    println!("Create Game Token: {}\n", flow.create_token);
    
    // Example: Generate calldata for burning tokens
    println!("== Burn Flow Example ==");
    let burn_flow = GameTokenSdk::burn_game_token_flow(
        factory_address,
        game_token_address,
        game_id,
        burn_amount
    )?;
    
    println!("Game Token Approval: {}", burn_flow.game_token_approve);
    println!("Burn Game Token: {}\n", burn_flow.burn_game_token);
    
    // Example: Individual function call data
    println!("== Individual Functions Example ==");
    let approve_calldata = GameTokenSdk::approve_crida(factory_address, creda_amount)?;
    println!("CRIDA Approve Calldata: {}", approve_calldata);
    
    let lock_calldata = GameTokenSdk::lock_crida(creda_amount)?;
    println!("Lock CRIDA Calldata: {}", lock_calldata);
    
    let burn_calldata = GameTokenSdk::burn_game_token(game_id, burn_amount)?;
    println!("Burn Game Token Calldata: {}", burn_calldata);
    
    Ok(())
} 