  
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};
use std::cmp::{PartialEq};


use crate::utils::{
    AccountId,
    Timestamp,
};

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Admin,
    User,
}

#[derive(PartialEq)]
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum OrderStatus {
    Created,
    ItemsBeingAdded,
    ClientOrderSubmitted,
    InAssembly,
    Assembled,
    ReadyToShip,

}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {
    pub customer_name: String,
    pub street_number: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

impl Address {
    
    pub fn new(customer_name: String, street_number: String,
        city: String, state: String, zip: String) -> Self {

        Address {
            customer_name,
            street_number,
            city,
            state,
            zip,

        }
    }

    
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Administrator {
    id: i32,
    pub name: String,
    pub added_by: AccountId,
    pub near_account: AccountId,
    pub email: String,
    pub role: Role,

}

impl Administrator{
    
    pub fn new(id:i32, name: String, near_account: AccountId,
        email: String) -> Self {

        Administrator {
            id,
            added_by: env::signer_account_id().to_string(),
            name,
            near_account,
            email,
            role: Role::Admin,

        }
    }

    
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Customer {
    cust_id: i32,
    pub name: String,
    pub email: String,
    pub near_account: AccountId,
    pub shipping_address: Vec<Address>,
    pub joined: Timestamp,
    pub role: Role,
    pub orders: Vec<Order>,

}

impl Customer{
    
    pub fn new(cust_id:i32, name: String, email: String) -> Self {

        Customer {
            cust_id,
            name,
            email,
            near_account: env::signer_account_id().to_string(),
            shipping_address: vec![],
            joined: env::block_timestamp(),
            role: Role::User,
            orders: vec![],

        }
    }

    
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    pub prod_id: i32,
    pub title: String,
    pub description: String,
    pub price: u128,
    added_by: AccountId,
}

impl Product {
    pub fn new(prod_id:i32, title: String, description: String, price: u128) -> Self {
        
        Product {
            prod_id,
            title,
            description,
            price,
            added_by: env::signer_account_id().to_string(),

        }
    }

    
}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
    order_id: i32,
  pub creator: AccountId,
    created_at: Timestamp,
    pub products: Vec<Product>,
    pub status: OrderStatus,
    pub total_cost: u128,

}


impl Order {
    pub fn new(order_id:i32) -> Self {
        
        Order {
            order_id,
            creator: env::signer_account_id().to_string(),
            created_at: env::block_timestamp(),
            products: vec![],
            status: OrderStatus::Created,
            total_cost: 0,
        
        }
    }


}

