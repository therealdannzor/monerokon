use tari_template_lib::prelude::*;

/// The flat fee for each withdraw
const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        // 🏋️ EXERCISE 2a: Add two vaults called `supply_vault` and `fee_vault`
        // TODO
    }

    impl Monerokon {
        /// Construct the component with an initial supply of fungible and confidential tokens.
        pub fn new(
            initial_supply: Amount,
            confidential_initial_supply: ConfidentialOutputStatement,
        ) -> Component<Self> {
            // 🏋️ EXERCISE 2b: Create a fungible resource with an initial supply
            // TODO


            let state = Self {
                // 🏋️ EXERCISE 2c:
                // 1. Deposit the initial tokens into a supply vault and,
                // 2. create an empty XTR vault called `fee_vault`.
                // TODO
            };

            Component::new(state)
                // 🏋️ EXERCISE 2d: allow anyone to call the "withdraw" method
                // .with_access_rules(
                //     ComponentAccessRules::new()
                //  // TODO
                // )
                .create()
        }

        pub fn get_balance(&self) -> Amount {
            // 🏋️ EXERCISE 2e: Return the supply vault balance
            todo!()
        }

        pub fn withdraw(&mut self, _fee: Bucket, _amount: Amount) -> Bucket {
            // 🏋️ EXERCISE 2f: check fee amount and deposit in the fee_vault. Withdraw requested amount from supply vault and return the Bucket.
            todo!()
        }
    }
}
