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

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
near_sdk::setup_alloc!();


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

    pub fn get_owner(&self) -> AccountId {
        let admin = self.owner_id.clone();
        return admin.to_string()
    }

    pub fn register(&mut self, name: String, email: String) {
        let cust_id = self.customers.len() as i32;

        self.customers.push(Customer::new(
            cust_id,
            name,
            email,
        ));
        env::log("Registration submitted successfully!".as_bytes());
    }

    pub fn get_customer_by_id(&mut self, cust_id: usize) -> AccountId {
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let account = customer.near_account.clone();
        assert_eq!(signer.to_string(), owner);
        return account.to_string();
    }

    pub fn get_customers(&self) -> Vec<Customer> {
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        let customers = &self.customers;
        assert_eq!(signer.to_string(), owner);
        return customers.to_vec();
    }

    pub fn add_address(&mut self, cust_id: usize, customer_name: String, street_number: String,
        city: String, state: String, zip: String) {
            let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
            let address = Address::new(customer_name, street_number, city, state, zip);
            
            let signer = env::predecessor_account_id();
            let account = customer.near_account.clone();
            assert_eq!(signer.to_string(), account);

            customer.shipping_address.push(address);
            env::log("Address added successfully!".as_bytes());

        }

    pub fn customer_count(&mut self) -> usize {
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        assert_eq!(signer.to_string(), owner);
        return self.customers.len();
    }

    pub fn add_product(&mut self, title: String, description: String, price: u128) {
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        let prod_id = self.products.len() as i32;
        assert_eq!(signer.to_string(), owner);
        self.products.push(Product::new(
            prod_id, 
            title,
            description,
            price,

        ));
        env::log("Product added successfully!".as_bytes());
    }

    pub fn get_products(&self) -> Vec<Product> {
        let products = &self.products;
        return products.to_vec();

    }

    pub fn product_count(&mut self) -> usize {
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        assert_eq!(signer.to_string(), owner);
        return self.products.len();
    }

    pub fn get_orders_by_customer(&mut self, cust_id: usize) -> Vec<Order> {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        let orders = customer.orders.clone();
        assert_eq!(signer.to_string(), owner);
        return orders.to_vec();

    }

    pub fn create_order(&mut self, cust_id: usize) {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order_id = customer.orders.len() as usize;
        let mut order = Order::new(order_id.try_into().unwrap());

        let signer = env::predecessor_account_id();
        let account = customer.near_account.clone();
        assert_eq!(signer.to_string(), account);

        customer.orders.push(order);
        env::log("A new empty order was created successfully!".as_bytes());
    }

    pub fn edit_order(&mut self, cust_id: usize, order_id: usize, prod_id: usize) {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();
        let product = self.products.get(prod_id).unwrap();
        let price = product.price;

        let signer = env::predecessor_account_id();
        let account = customer.near_account.clone();
        assert_eq!(signer.to_string(), account);

        order.status = OrderStatus::ItemsBeingAdded;
       
        order.products.push(product.clone());
        order.total_cost += price;
        env::log("Product added - order was updated successfully!".as_bytes());
    }

    pub fn get_products_in_order(&mut self, cust_id: usize, order_id: usize) -> Vec<Product> {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();

        let signer = env::predecessor_account_id();
        let account = customer.near_account.clone();
        assert_eq!(signer.to_string(), account);

        return order.products.clone();
    }

    pub fn cust_review_order(&mut self, cust_id: usize, order_id: usize, approval_qty: i32) -> bool {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();
        let products = order.products.clone();

        let signer = env::predecessor_account_id();
        let account = customer.near_account.clone();
        assert_eq!(signer.to_string(), account);

        assert_eq!(products.len().to_string(), approval_qty.to_string());
        order.status = OrderStatus::ClientOrderSubmitted;
        env::log("Order approved successfully!".as_bytes());
        true
    }

    pub fn assemble_order(&mut self, cust_id: usize, order_id: usize) -> bool {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        assert_eq!(signer.to_string(), owner);
        order.status = OrderStatus::InAssembly;
        env::log("Assembly complete - order completed successfully!".as_bytes());
        true
    }

    pub fn verify_order_admin(&mut self, cust_id: usize, order_id: usize, verify_qty: i32) -> bool {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();
        let products = order.products.clone();
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        assert_eq!(signer.to_string(), owner);
        assert_eq!(products.len().to_string(), verify_qty.to_string());
        order.status = OrderStatus::Assembled;
        env::log("Order verified and approved successfully!".as_bytes());
        true
    }

    pub fn ready_to_ship(&mut self, cust_id: usize, order_id: usize, ready: bool) -> bool {
        let customer: &mut Customer = self.customers.get_mut(cust_id).unwrap();
        let order: &mut Order = customer.orders.get_mut(order_id).unwrap();
        let signer = env::predecessor_account_id();
        let owner = self.owner_id.clone();
        if order.status == OrderStatus::Assembled {
            assert_eq!(signer.to_string(), owner);
            assert_eq!(ready, true);
            order.status = OrderStatus::ReadyToShip;
        }
        env::log("Order is ready to ship - can notify customer!".as_bytes());
        true
    }
 
}


// PASTE ENTIRE TEST BLOCK HERE BELOW THESE COMMENTS.
// MAKE SURE IT IS OUTSIDE THE ABOVE CONTRACT BRACES.
// YOU CAN USE EITHER:
// bash ./test.sh
// OR
// cargo test
// AS LONG AS YOU ARE IN THE  "final_contract" DIRECTORY 
// IN YOUR TERMINAL TO RUN THE UNIT TESTS.
// THERE ARE 10 UNIT TESTS

