use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Bytes, BytesN, Env, Symbol, Map, Vec,
    IntoVal, TryFromVal, log
};

// Storage keys
const ADMIN: Symbol = symbol_short!("ADMIN");
const PROPOSAL_DATA: Symbol = symbol_short!("PROP_DATA");
const PROPOSAL_FAILED: Symbol = symbol_short!("PROP_FAIL");

// Event topics
const EVENT_NEW_PROPOSAL: Symbol = symbol_short!("NEW_PROP");
const EVENT_GOV_LOG: Symbol = symbol_short!("GOV_LOG");

#[derive(Clone)]
pub struct Proposal {
    proposal_data: Vec<Bytes>,
    failed: Vec<bool>,
}

#[contract]
pub struct C3Governor;

#[contractimpl]
impl C3Governor {
    // Initialize contract
    pub fn c3gov_init(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN, &admin);
    }

    // Helper function to check if caller is admin
    fn check_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
    }

    // Get proposals storage
    fn get_proposal(env: &Env, nonce: BytesN<32>) -> Option<Proposal> {
        let key = DataKey::Proposal(nonce);
        //env.storage().persistent().get(&key)
        None
    }

    // Save proposal
    fn save_proposal(env: &Env, nonce: BytesN<32>, proposal: &Proposal) {
        let key = DataKey::Proposal(nonce);
        // env.storage().persistent().set(&key, proposal);
    }

    // Get chain ID (in Stellar, this would be the network passphrase or identifier)
    fn chain_id(env: &Env) -> Bytes {
        env.ledger().network_id().into()
    }

    // Send single parameter
    pub fn send_params(env: Env, data: Bytes, nonce: BytesN<32>) {
        Self::check_admin(&env);
        
        if data.len() == 0 {
            panic!("No data to sendParams");
        }

        // Create or get existing proposal
        let mut proposal = Self::get_proposal(&env, nonce.clone())
            .unwrap_or(Proposal {
                proposal_data: Vec::new(&env),
                failed: Vec::new(&env),
            });

        // Add data
        proposal.proposal_data.push_back(data);
        proposal.failed.push_back(false);

        // Save proposal
        Self::save_proposal(&env, nonce.clone(), &proposal);

        // Emit event
        // env.events().publish(
        //     (EVENT_NEW_PROPOSAL, nonce.clone()),
        //     Vec::new(&env)
        // );

        // Execute governance action
        Self::c3_gov(&env, nonce, 0);
    }

    // Send multiple parameters
    pub fn send_multi_params(env: Env, data: Vec<Bytes>, nonce: BytesN<32>) {
        Self::check_admin(&env);
        
        if data.len() == 0 {
            panic!("No data to sendParams");
        }

        // Create or get existing proposal
        let mut proposal = Self::get_proposal(&env, nonce.clone())
            .unwrap_or(Proposal {
                proposal_data: Vec::new(&env),
                failed: Vec::new(&env),
            });

        // Process each data item
        for (index, item) in data.iter().enumerate() {
            if item.len() == 0 {
                panic!("No data passed to sendParams");
            }

            proposal.proposal_data.push_back(item.clone());
            proposal.failed.push_back(false);

            Self::c3_gov(&env, nonce.clone(), index as u32);
        }

        // Save proposal
        Self::save_proposal(&env, nonce.clone(), &proposal);

        // Emit event
        // env.events().publish(
        //     (EVENT_NEW_PROPOSAL, nonce),
        //     Vec::new(&env)
        // );
    }

    // Execute governance action
    pub fn do_gov(env: Env, nonce: BytesN<32>, offset: u32) {
        let proposal = Self::get_proposal(&env, nonce.clone())
            .expect("Proposal not found");

        if offset as u32 >= proposal.proposal_data.len() {
            panic!("Reading beyond the length of the offset array");
        }

        if proposal.failed.get(offset).unwrap() {
            panic!("Do not resend if it did not fail");
        }

        Self::c3_gov(&env, nonce, offset);
    }

    // Get proposal data
    pub fn get_proposal_data(env: Env, nonce: BytesN<32>, offset: u32) -> (Bytes, bool) {
        let proposal = Self::get_proposal(&env, nonce)
            .expect("Proposal not found");

        (
            proposal.proposal_data.get(offset).unwrap(),
            proposal.failed.get(offset).unwrap()
        )
    }

    // Internal governance execution
    fn c3_gov(env: &Env, nonce: BytesN<32>, offset: u32) {
        let proposal = Self::get_proposal(env, nonce.clone())
            .expect("Proposal not found");

        let raw_data = proposal.proposal_data.get(offset).unwrap();

        // TODO: Implement proper data decoding here
        // In Soroban, we'll need a different approach to decode the chain_id, target, and remote_data
        // This is a placeholder for the actual implementation
        
        // Emit governance log event
        env.events().publish(
            (EVENT_GOV_LOG, nonce),
            raw_data
        );
    }

    pub fn version() -> u32 {
        1
    }
}

#[derive(Clone)]
enum DataKey {
    Proposal(BytesN<32>),
}