    #![no_std]
    use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

    #[contracttype]
    pub enum DataKey {
        Admin,
        Token,
        Beneficiary(Address), // Maps a beneficiary's address to their allocated claim amount
    }

    #[contract]
    pub struct AidTrackContract;

    #[contractimpl]
    impl AidTrackContract {
        /// Initializes the contract with an admin and the USDC/Asset token address.
        pub fn init(env: Env, admin: Address, token: Address) {
            admin.require_auth();
            env.storage().instance().set(&DataKey::Admin, &admin);
            env.storage().instance().set(&DataKey::Token, &token);
        }

        /// Admin registers a beneficiary with a specific token allocation.
        pub fn register_beneficiary(env: Env, admin: Address, beneficiary: Address, amount: i128) {
            admin.require_auth();
            
            // Verify caller is the registered admin
            let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
            assert!(admin == stored_admin, "Unauthorized: Only admin can register beneficiaries");
            
            // Store the allocation
            env.storage().persistent().set(&DataKey::Beneficiary(beneficiary), &amount);
        }

        /// Beneficiary claims their allocated aid. This is the MVP core transaction.
        pub fn claim_aid(env: Env, beneficiary: Address) {
            beneficiary.require_auth();
            
            // Check how much aid is allocated to this beneficiary
            let amount: i128 = env.storage().persistent().get(&DataKey::Beneficiary(beneficiary.clone())).unwrap_or(0);
            assert!(amount > 0, "No aid allocated or aid already claimed");
            
            let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
            let token_client = soroban_sdk::token::Client::new(&env, &token_addr);
            
            // Transfer the funds from the contract's balance to the beneficiary
            token_client.transfer(&env.current_contract_address(), &beneficiary, &amount);
            
            // Set allocation to 0 to prevent double-claiming
            env.storage().persistent().set(&DataKey::Beneficiary(beneficiary), &0_i128);
        }
    }