// THIS IS A TEMPLATE FOR THE MAIN NEAR SMART CONTRACT ("lib.rs") FILE.
// NOTE: YOU MUST INITIALIZE THE CONTRACT (AFTER DEPLOYMENT TO TESTNET) BEFORE USING.
// SEE INSTRUCTIONS IN THE MEDIUM ARTICLE FOR INITIALIZATION.

// CRATE IMPORT STATEMENTS ALREADY SET UP::

mod models;
mod utils;

use std::convert::TryInto;

use crate::{
    utils::{
        AccountId,
    },
    models::{
        Customer,
        Product,
        Order,
        OrderStatus,
        Address,

    }
};

// NEAR SDK STRUCTS, TRAITS ALREADY DECLARED HERE::

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
near_sdk::setup_alloc!();


// THE STARTER CONTRACT STRUCTURE IS DEFINED BELOW.
// FUNCTIONS SHOULD BE ADDED BY FOLLOWING THE STEPS OUTLINED 
// BELOW AND IN THE ARTICLE::

#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    owner_id: AccountId,
    orders: Vec<Order>,
    customers: Vec<Customer>,
    products: Vec<Product>,
}

impl Default for Contract {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}


#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(
        owner_id: AccountId,
    ) -> Self{
        let orders: Vec<Order> = Vec::new();
        let customers: Vec<Customer> = Vec::new();
        let products: Vec<Product> = Vec::new();
        assert!(env::state_read::<Self>().is_none(), "Already initialized");

        let mut contract = Self {
            owner_id,
            orders,
            customers,
            products,
        };

        contract

    }

    // CONTRACT FUNCTIONS GO BELOW THIS LINE.
    // FOLLOW STEP #'S FROM THE TUTORIAL / PLACEHOLDERS NUMBERED BELOW:





}

// PASTE ENTIRE TEST BLOCK HERE BELOW THESE COMMENTS.
// MAKE SURE IT IS OUTSIDE THE ABOVE CONTRACT BRACES.
// YOU CAN USE EITHER:
// bash ./test.sh
// OR
// cargo test
// AS LONG AS YOU ARE IN THE  "final_contract" DIRECTORY 
// IN YOUR TERMINAL TO RUN THE UNIT TESTS.