use tari_template_lib::prelude::*;

/// The flat fee for each withdraw
const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        fee_vault: Vault,
        confidential_vault: Vault,
    }

    impl Monerokon {
        /// Construct the component with an initial supply of fungible and confidential tokens.
        pub fn new(initial_supply: ConfidentialOutputStatement) -> Component<Self> {
            let bucket = ResourceBuilder::confidential().initial_supply(initial_supply);

            let state = Self {
                fee_vault: Vault::new_empty(XTR),
                confidential_vault: Vault::from_bucket(bucket),
            };

            Component::new(state)
                .with_access_rules(
                    ComponentAccessRules::new().add_method_rule("withdraw", AccessRule::AllowAll),
                )
                .create()
        }

        pub fn withdraw(&mut self, fee: Bucket, withdraw: ConfidentialWithdrawProof) -> Bucket {
            assert!(fee.amount() >= FEE, "fee is too low");

            self.fee_vault.deposit(fee);
            self.confidential_vault.withdraw_confidential(withdraw)
        }

        pub fn mint_confidential(&self, mint: ConfidentialOutputStatement) {
            let manager = ResourceManager::get(self.confidential_vault.resource_address());
            let bucket = manager.mint_confidential(mint);
            self.confidential_vault.deposit(bucket);
        }
    }
}
