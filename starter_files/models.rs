// THIS FILE IS FOR APPLICATION-SPECIFIC DATA STRUCTURES:
// PRIMARILY STRUCTS, IMPLEMENTATION BLOCKS AND ENUMS FOR STATE.
// YOU CAN CUSTOMIZE AS YOU SEE FIT. 

// CRATE IMPORT STATEMENTS ALREADY SET UP FOR TUTORIAL::
  
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};


use crate::utils::{
    AccountId,
    Timestamp
};

// NEAR SDK STRUCTS, TRAITS ALREADY DECLARED HERE AT THE
// TOP OF EACH STRUCT OUTLINE
// FOLLOW THE STEPS BELOW AND IN THE ARTICLE TO 
// FILL IN THE DATA FIELDS/METHODS FOR THE STRUCTS / IMPLEMENTATION BLOCKS / ENUMS::

#[derive(PartialEq)]
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum OrderStatus {
  
    // PLACEHOLDER

}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {

        // PLACEHOLDER
}


impl Address {
    
      // PLACEHOLDER
    
}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Administrator {
 
        // PLACEHOLDER
}


impl Administrator{
    
    

    
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Customer {
 
        // PLACEHOLDER

}


impl Customer{
    
 
        // PLACEHOLDER

    
}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {

        // PLACEHOLDER

}


impl Product {
    
        // PLACEHOLDER

    
}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {

        // PLACEHOLDER
}


impl Order {
  
        // PLACEHOLDER


}
