use tari_template_lib::prelude::*;

/// The flat fee for each withdraw
const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        // 🏋️ EXERCISE 3b: Add two vaults called `supply_vault` and `fee_vault`
        // TODO

        // 🏋️ EXERCISE 5b: Create a Non-Fungible resource with two NFTs in a new vault named 'nft_vault'
        // TODO

        // 🏋️ EXERCISE 6b: Add a confidential vault called 'confidential_vault'
        // TODO
    }

    impl Monerokon {
        /// Construct the component with an initial supply of fungible and confidential tokens.
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
                // 🏋️ EXERCISE 3c:
                // 1. Deposit the initial tokens into a supply vault and,
                // 2. create an empty XTR vault called `fee_vault`.
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
            todo!()
        }

        pub fn withdraw_confidential(
            &mut self,
            _fee: Bucket,
            _withdraw_proof: ConfidentialWithdrawProof,
        ) -> Bucket {
            // 🏋️ EXERCISE 6b: check fee amount and deposit then in the fee_vault. Withdraw requested amount from confidential vault and return the Bucket.
            todo!()
        }

        // 🏋️ EXERCISE 7a: Mint fungible tokens and deposit them in the supply_vault
        pub fn mint_fungible(&self, fungible_amount: Amount) {
            todo!()
        }

        // 🏋️ EXERCISE 7b: Mint a fungible token with data and deposit it in the nft_vault
        pub fn mint_non_fungible(&self, nft: NonFungibleId) {
            #[derive(serde::Serialize)]
            struct MyData {
                data: String,
            }

            let manager = ResourceManager::get(self.nft_vault.resource_address());
            todo!()
        }

        // 🏋️ EXERCISE 7c: Mint confidential tokens and deposit them in the confidential_vault
        pub fn mint_confidential(&self, confidential: ConfidentialOutputStatement) {
            let manager = ResourceManager::get(self.confidential_vault.resource_address());
            todo!()
        }
    }
}
