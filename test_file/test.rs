
// THIS INITIALIZES THE TESTING ENVIRONMENT
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT
    #[test]
    fn initialize() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());
  
    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER
    #[test]
    fn register_user() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let bob_account = contract.get_customer_by_id(0);

        assert_eq!(bob.to_string(), bob_account.to_string());

    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER ADDS THEIR SHIPPING ADDRESS
    #[test]
    fn add_address_to_profile() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.add_address(0, "Bob Jones".to_string(),
                    "111 Maple Lane".to_string(),
                    "Detroit".to_string(),
                    "Michigan".to_string(),
                    "48313".to_string());


    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // THE OWNER ADDS ONE PRODUCT TO PRODUCTS
    #[test]
    fn add_a_product_by_owner() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    #[test]
    fn create_empty_order() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::Created, order_status);

    
    }
    
    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    // CUSTOMER ADDS A PRODUCT TO ORDER
    // ORDER STATUS ADVANCES TO "ITEMS BEING ADDED"
    #[test]
    fn add_product_to_order() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // Adds to the order

        contract.edit_order(0, 0, 0);

        let order_products = contract.get_products_in_order(0, 0).len() as usize;

        assert_eq!(order_products, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ItemsBeingAdded, order_status);


    
    }


    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    // CUSTOMER ADDS A PRODUCT TO ORDER
    // ORDER STATUS ADVANCES TO "ITEMS BEING ADDED"
    // CUSTOMER FINALIZES ORDER
    // ORDER STATUS ADVANCES TO "CLIENT ORDER SUBMITTED"

    #[test]
    fn customer_finalize_order() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // Adds to the order

        contract.edit_order(0, 0, 0);

        let order_products = contract.get_products_in_order(0, 0).len() as usize;

        assert_eq!(order_products, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ItemsBeingAdded, order_status);

        // SWITCH BACK TO BOB, APPROVE ORDER

        testing_env!(new_context.build());

        // CONFIRMS QTY OF PRODUCTS AS 1
        contract.cust_review_order(0, 0, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ClientOrderSubmitted, order_status);


    
    }


    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    // CUSTOMER ADDS A PRODUCT TO ORDER
    // ORDER STATUS ADVANCES TO "ITEMS BEING ADDED"
    // CUSTOMER FINALIZES ORDER
    // ORDER STATUS ADVANCES TO "CLIENT ORDER SUBMITTED"
    // OWNER STARTS ASSEMBLING ORDER
    // ORDER STATUS ADVANCES TO "IN ASSEMBLY"
    #[test]
    fn admin_assemble_order() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // Adds to the order

        contract.edit_order(0, 0, 0);

        let order_products = contract.get_products_in_order(0, 0).len() as usize;

        assert_eq!(order_products, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ItemsBeingAdded, order_status);

        // SWITCH BACK TO BOB, APPROVE ORDER

        testing_env!(new_context.build());

        // CONFIRMS QTY OF PRODUCTS AS 1
        contract.cust_review_order(0, 0, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ClientOrderSubmitted, order_status);

        contract.assemble_order(0, 0);

        // CHECK ORDER STATUS AGAIN = "IN ASSEMBLY"

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::InAssembly, order_status);


    
    }

    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    // CUSTOMER ADDS A PRODUCT TO ORDER
    // ORDER STATUS ADVANCES TO "ITEMS BEING ADDED"
    // CUSTOMER FINALIZES ORDER
    // ORDER STATUS ADVANCES TO "CLIENT ORDER SUBMITTED"
    // OWNER STARTS ASSEMBLING ORDER
    // ORDER STATUS ADVANCES TO "IN ASSEMBLY"
    // OWNER VERIFIES ORDER
    // ORDER STATUS ADVANCES TO "ASSEMBLED"

    #[test]
    fn admin_verify_order() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // Adds to the order

        contract.edit_order(0, 0, 0);

        let order_products = contract.get_products_in_order(0, 0).len() as usize;

        assert_eq!(order_products, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ItemsBeingAdded, order_status);

        // SWITCH BACK TO BOB, APPROVE ORDER

        testing_env!(new_context.build());

        // CONFIRMS QTY OF PRODUCTS AS 1
        contract.cust_review_order(0, 0, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ClientOrderSubmitted, order_status);

        contract.assemble_order(0, 0);

        // CHECK ORDER STATUS AGAIN = "IN ASSEMBLY"

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::InAssembly, order_status);

        // ADMIN VERIFY ORDER

        contract.verify_order_admin(0, 0, 1);


        // CHECK ORDER STATUS AGAIN = "ASSEMBLED"

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::Assembled, order_status);

    
    }


    //THIS TEST INITIALIZES CONTRACT AFTER DEPLOYMENT, AND
    // ALLOWS A CUSTOMER TO REGISTER,
    // CUSTOMER CREATES AN EMPTY ORDER
    // ORDER STATUS = "CREATED"
    // CUSTOMER ADDS A PRODUCT TO ORDER
    // ORDER STATUS ADVANCES TO "ITEMS BEING ADDED"
    // CUSTOMER FINALIZES ORDER
    // ORDER STATUS ADVANCES TO "CLIENT ORDER SUBMITTED"
    // OWNER STARTS ASSEMBLING ORDER
    // ORDER STATUS ADVANCES TO "IN ASSEMBLY"
    // OWNER VERIFIES ORDER
    // ORDER STATUS ADVANCES TO "ASSEMBLED"
    // OWNER FINALIZES ORDER - READY TO SHIP
    // ORDER STATUS ADVANCES TO "READY TO SHIP"

    #[test]
    fn admin_ready_to_ship() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        let owner = contract.get_owner();

        assert_eq!(owner, alice.to_string());

        contract.add_product("Nike Running Shoes".to_string(),
                "A great shoe for a good price".to_string(), 
                100);

        let count = contract.product_count();

        assert_eq!(count, 1);

        // make note second context is "NEAR" account not testnet

        let bob = AccountId::new_unchecked("bob.near".to_string());

        contract.register(bob.to_string(), "bobjones@gmail.com".to_string());

        let customer_count = contract.get_customers().len() as usize;

        assert_eq!(customer_count, 1);

        let new_context = get_context(bob.clone());

        testing_env!(new_context.build());

        contract.create_order(0);

        // Adds to the order

        contract.edit_order(0, 0, 0);

        let order_products = contract.get_products_in_order(0, 0).len() as usize;

        assert_eq!(order_products, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ItemsBeingAdded, order_status);

        // SWITCH BACK TO BOB, APPROVE ORDER

        testing_env!(new_context.build());

        // CONFIRMS QTY OF PRODUCTS AS 1
        contract.cust_review_order(0, 0, 1);

        // SWITCH BACK TO FIRST CONTEXT FOR "OWNER"

        testing_env!(context.build());

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::ClientOrderSubmitted, order_status);

        contract.assemble_order(0, 0);

        // CHECK ORDER STATUS AGAIN = "IN ASSEMBLY"

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::InAssembly, order_status);

        // ADMIN VERIFY ORDER

        contract.verify_order_admin(0, 0, 1);


        // CHECK ORDER STATUS AGAIN = "ASSEMBLED"

        let orders = contract.get_orders_by_customer(0);

        let first_order = &orders[0];

        let mut order_status = &first_order.status;

        assert_eq!(&OrderStatus::Assembled, order_status);

        // ADMIN "READY TO SHIP":

        contract.ready_to_ship(0, 0, true);

       // CHECK ORDER STATUS AGAIN = "READY TO SHIP"

       let orders = contract.get_orders_by_customer(0);

       let first_order = &orders[0];

       let mut order_status = &first_order.status;

       assert_eq!(&OrderStatus::ReadyToShip, order_status);

    
    }

    // TESTS COMPLETED - FOR ONE OWNER, ONE PRODUCT, ONE CUSTOMER, ONE ORDER.
    // FULL ORDER PROCESSING FROM "ORDER CREATED" TO "READY TO SHIP"


}

