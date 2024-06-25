use tari_template_lib::prelude::*;

/// The flat fee for each withdraw
const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        supply_vault: Vault,
        fee_vault: Vault,
    }

    impl Monerokon {
        /// Construct the component with an initial supply of fungible and confidential tokens.
        pub fn new(initial_supply: Amount) -> Component<Self> {
            let fungible = ResourceBuilder::fungible().initial_supply(initial_supply);
            let sv = Vault::from_bucket(fungible);

            let fv = Vault::new_empty(XTR);

            let state = Self {
                supply_vault: sv,
                fee_vault: fv,
            };

            Component::new(state)
                .with_access_rules(ComponentAccessRules::allow_all())
                .create()
        }

        pub fn get_balance(&self) -> Amount {
            self.supply_vault.balance()
        }

        pub fn withdraw(&mut self, _fee: Bucket, _amount: Amount) -> Bucket {
            assert!(_fee.amount() >= FEE);

            self.fee_vault.deposit(_fee);
            self.supply_vault.withdraw(_amount)
        }
    }
}
