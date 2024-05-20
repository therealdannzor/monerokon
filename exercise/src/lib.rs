use tari_template_lib::prelude::*;

const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    pub struct Monerokon {
        counter: u32,
        // 🏋️ EXERCISE 3b: Add two vault called 'supply_vault' and 'fee_vault'
        // TODO

        // 🏋️ EXERCISE 5b: Create a Non-Fungible resource with two NFTs in a new vault named 'nft_vault'
        // TODO

        // 🏋️ EXERCISE 6b: Add a confidential vault called 'confidential_vault'
        // TODO
    }

    // 🏋️ EXERCISE 1: Initialize component with a zero counter value
    impl Monerokon {
        pub fn new() -> Component<Self> {
            let state = Self {
                // TODO set counter to 0
            };

            Component::new(state).create()
        }

        // 🏋️ EXERCISE 2a: Implement method to return the counter value
        pub fn counter(&self) -> u32 {
            todo!()
        }

        // 🏋️ EXERCISE 2b: Mutate some state! Increase the counter value by 1
        pub fn increase(&mut self) {
            todo!()
        }
    }

    // Uncomment the following code block after completing exercise 2
    /*
    impl Monerokon {
        pub fn new(
            initial_supply: Amount,
            confidential_initial_supply: ConfidentialOutputStatement,
        ) -> Component<Self> {
            // 🏋️ EXERCISE 3a: Create a fungible resource with an initial supply
            // TODO

            // 🏋️ EXERCISE 5: Create a Non-Fungible resource with two NFTs in a new vault named 'nft_vault'
            // TODO

            // 🏋️ EXERCISE 6a: Create a confidential resource with an initial supply and add a vault to the component called 'confidential_vault'
            // TODO

            let state = Self {
                counter: 0,
                // 🏋️ EXERCISE 3c: Deposit the initial tokens into a supply vault and create an empty fee vault
                // TODO
            };

            Component::new(state)
                // 🏋️ EXERCISE 4b: allow anyone to call the "withdraw" method
                // .with_access_rules(
                //     ComponentAccessRules::new()
                // TODO
                // )
                .create()
        }



        pub fn get_balance(&self) -> Amount {
            // 🏋️ EXERCISE 3c: Return the supply vault balance
            todo!()
        }

        pub fn withdraw(&mut self, _fee: Bucket, _amount: Amount) -> Bucket {
            // 🏋️ EXERCISE 4a: check fee amount and deposit in the fee_vault. Withdraw requested amount from supply vault and return the Bucket.
            //                Increment the counter.
            todo!()
        }

        pub fn withdraw_confidential(
            &mut self,
            _fee: Bucket,
            _withdraw_proof: ConfidentialWithdrawProof,
        ) -> Bucket {
            // 🏋️ EXERCISE 6b: check fee amount and deposit then in the fee_vault. Withdraw requested amount from confidential vault and return the Bucket.
            //                Increment the counter.
            todo!()
        }

        pub fn counter(&self) -> u32 {
            self.counter
        }

        pub fn increase(&mut self) {
            self.counter += 1;
        }
    }
     */
}
