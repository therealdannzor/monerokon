use tari_template_lib::prelude::*;

/// The flat fee for each withdraw
const FEE: Amount = Amount(10);

#[template]
mod template {
    use super::*;

    /// Defines the component state
    pub struct Monerokon {
        fee_vault: Vault,
        nft_vault: Vault,
    }

    impl Monerokon {
        /// Construct the component with an initial supply of fungible and confidential tokens.
        pub fn new(initial_nfts: Vec<NonFungibleId>) -> Component<Self> {
            let initial_nfts = initial_nfts
                .into_iter()
                .map(|nft| (nft, (&(), &())))
                .collect::<Vec<_>>();

            let bucket = ResourceBuilder::non_fungible()
                .with_token_symbol("MNROKON-NFT")
                .mintable(AccessRule::AllowAll)
                .burnable(AccessRule::AllowAll)
                .initial_supply_with_data(initial_nfts);

            let state = Self {
                nft_vault: Vault::from_bucket(bucket),
                fee_vault: Vault::new_empty(XTR),
            };

            Component::new(state)
                .with_access_rules(
                    ComponentAccessRules::new().add_method_rule("withdraw", AccessRule::AllowAll),
                )
                .create()
        }

        pub fn withdraw(&mut self, fee: Bucket, _nft: NonFungibleId) -> Bucket {
            assert!(fee.clone().amount() >= FEE, "fee is too low");

            self.fee_vault.deposit(fee);
            self.nft_vault.withdraw_non_fungible(_nft)
        }

        pub fn mint_non_fungible(&self, _nft: NonFungibleId) {
            #[derive(serde::Serialize)]
            struct MyData {
                data: String,
            }

            let manager = ResourceManager::get(self.nft_vault.resource_address());

            let mut immutable = Metadata::new();
            immutable.insert("name", "Monerokon");

            let bucket = manager.mint_non_fungible(
                _nft.clone(),
                &immutable,
                &MyData {
                    data: "test".to_string(),
                },
            );

            self.nft_vault.deposit(bucket);
        }
    }
}
