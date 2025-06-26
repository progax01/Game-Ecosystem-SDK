use game_rust_sdk::GameTokenSdk;

/// This test verifies that the SDK can generate valid call data for the complete game token flow
#[test]
fn test_complete_game_token_flow() {
    // Sample addresses and values
    let factory_address = "0x1234567890123456789012345678901234567890";
    let creda_amount = "1000000000000000000000";  // 1000 CREDA tokens
    let game_name = "Test Game";
    let game_symbol = "TEST";
    let decimals = 18;

    // Generate call data for the complete flow
    let flow = GameTokenSdk::create_game_token_flow(
        factory_address, 
        creda_amount,
        game_name,
        game_symbol,
        decimals
    ).expect("Failed to generate complete flow call data");

    // Verify that all call data was generated correctly
    assert!(flow.creda_approve.starts_with("0x095ea7b3"), "Invalid CRIDA approve call data");
    assert!(flow.lock_creda.starts_with("0xbec697db"), "Invalid lock CRIDA call data");
    assert!(flow.xp_approve.starts_with("0x095ea7b3"), "Invalid XP approve call data");
    assert!(flow.create_token.starts_with("0xfd44d274"), "Invalid create token call data");
}

/// This test verifies that the SDK can generate valid call data for the burn flow
#[test]
fn test_burn_game_token_flow() {
    // Sample addresses and values
    let factory_address = "0x1234567890123456789012345678901234567890";
    let game_token_address = "0xabcdef0123456789abcdef0123456789abcdef01";
    let game_id = "1";
    let burn_amount = "500000000000000000000";  // 500 game tokens

    // Generate call data for the burn flow
    let flow = GameTokenSdk::burn_game_token_flow(
        factory_address,
        game_token_address,
        game_id,
        burn_amount
    ).expect("Failed to generate burn flow call data");

    // Verify that all call data was generated correctly
    assert!(flow.game_token_approve.starts_with("0x095ea7b3"), "Invalid game token approve call data");
    assert!(flow.burn_game_token.starts_with("0x5ed6c1db"), "Invalid burn game token call data");
}

/// This test verifies that the SDK handles invalid inputs correctly
#[test]
fn test_error_handling() {
    // Invalid address format
    let result = GameTokenSdk::approve_crida("invalid_address", "1000");
    assert!(result.is_err(), "Should fail with invalid address");

    // Invalid amount format
    let result = GameTokenSdk::lock_crida("not_a_number");
    assert!(result.is_err(), "Should fail with invalid amount");
}

/// This test verifies individual function call data generation
#[test]
fn test_individual_functions() {
    // Test approve CRIDA
    let factory_address = "0x1234567890123456789012345678901234567890";
    let creda_amount = "1000000000000000000000";

    let approve_calldata = GameTokenSdk::approve_crida(factory_address, creda_amount)
        .expect("Failed to generate approve calldata");
    assert!(approve_calldata.starts_with("0x095ea7b3"), "Invalid approve call data");

    // Test lock CRIDA
    let lock_calldata = GameTokenSdk::lock_crida(creda_amount)
        .expect("Failed to generate lock calldata");
    assert!(lock_calldata.starts_with("0xbec697db"), "Invalid lock call data");

    // Test burn game token
    let game_id = "1";
    let burn_amount = "500000000000000000000";
    
    let burn_calldata = GameTokenSdk::burn_game_token(game_id, burn_amount)
        .expect("Failed to generate burn calldata");
    assert!(burn_calldata.starts_with("0x5ed6c1db"), "Invalid burn call data");
} 