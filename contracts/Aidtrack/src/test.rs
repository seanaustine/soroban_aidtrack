#![cfg(test)]
use super::*;
use soroban_sdk::{Env, testutils::Address as _};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;

fn setup_test() -> (Env, AidTrackContractClient, Address, Address, TokenClient) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    // Setup mock token
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_contract);
    let token_admin_client = StellarAssetClient::new(&env, &token_contract);

    let contract_id = env.register_contract(None, AidTrackContract);
    let client = AidTrackContractClient::new(&env, &contract_id);

    // Initialize contract
    client.init(&admin, &token_contract);
    
    // Fund the contract treasury
    token_admin_client.mint(&contract_id, &1000);

    (env, client, admin, beneficiary, token_client)
}

// Test 1 (Happy path): The MVP transaction executes successfully end-to-end
#[test]
fn test_happy_path_claim() {
    let (_env, client, admin, beneficiary, token) = setup_test();
    
    client.register_beneficiary(&admin, &beneficiary, &100);
    client.claim_aid(&beneficiary);
    
    assert_eq!(token.balance(&beneficiary), 100);
}

// Test 2 (Edge case): Unauthorized caller attempts to register a beneficiary
#[test]
#[should_panic(expected = "Unauthorized: Only admin can register beneficiaries")]
fn test_unauthorized_registration() {
    let (env, client, _admin, _beneficiary, _token) = setup_test();
    let fake_admin = Address::generate(&env);
    let new_beneficiary = Address::generate(&env);
    
    // fake_admin is calling, but they are not the initialized admin
    client.register_beneficiary(&fake_admin, &new_beneficiary, &100);
}

// Test 3 (Edge case): Beneficiary attempts to claim without being registered
#[test]
#[should_panic(expected = "No aid allocated or aid already claimed")]
fn test_unregistered_claim() {
    let (env, client, _admin, _beneficiary, _token) = setup_test();
    let unregistered_user = Address::generate(&env);
    
    client.claim_aid(&unregistered_user);
}

// Test 4 (Edge case): Beneficiary attempts to double-claim their aid
#[test]
#[should_panic(expected = "No aid allocated or aid already claimed")]
fn test_double_claim() {
    let (_env, client, admin, beneficiary, _token) = setup_test();
    
    client.register_beneficiary(&admin, &beneficiary, &100);
    client.claim_aid(&beneficiary); // First claim succeeds
    client.claim_aid(&beneficiary); // Second claim should panic
}

// Test 5 (State verification): Assert contract storage reflects correct state after MVP transaction
#[test]
fn test_state_verification_after_claim() {
    let (env, client, admin, beneficiary, _token) = setup_test();
    
    client.register_beneficiary(&admin, &beneficiary, &100);
    
    // Verify state before claim
    env.as_contract(&client.address, || {
        let amount: i128 = env.storage().persistent().get(&DataKey::Beneficiary(beneficiary.clone())).unwrap();
        assert_eq!(amount, 100);
    });

    client.claim_aid(&beneficiary);
    
    // Verify state after claim (allocation should be zeroed out)
    env.as_contract(&client.address, || {
        let amount: i128 = env.storage().persistent().get(&DataKey::Beneficiary(beneficiary.clone())).unwrap();
        assert_eq!(amount, 0);
    });
}