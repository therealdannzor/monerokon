use tari_template_lib::prelude::*;
use tari_template_test_tooling::TemplateTest;
use tari_transaction::Transaction;

#[test]
fn exercise_3_nft_resource() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let component_address: ComponentAddress = test.call_function(
        "Monerokon",
        "new",
        args![vec![
            NonFungibleId::from_u64(1),
            NonFungibleId::from_string("monerokon")
        ]],
        vec![],
    );

    let vault_id: VaultId = test.extract_component_value(component_address, "$.nft_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    assert_eq!(vault.resource_type(), ResourceType::NonFungible, "Resource type for `nft_vault` is not NonFungible. Use `ResourceBuilder::non_fungible()` to create the resource and deposit the bucket into `nft_vault`.");
    assert_eq!(vault.balance(), 2i64);

    let (account, account_proof, account_secret) = test.create_funded_account();

    // Transfer some coins into the account
    test.execute_expect_success(
        Transaction::builder()
            .call_method(account, "withdraw", args![XTR2, Amount(10)])
            .put_last_instruction_output_on_workspace("fee")
            .call_method(
                component_address,
                "withdraw",
                args![Workspace("fee"), NonFungibleId::from_u64(1)],
            )
            .put_last_instruction_output_on_workspace("bucket")
            .call_method(account, "deposit", args![Workspace("bucket")])
            .sign(&account_secret)
            .build(),
        vec![account_proof],
    );

    let proof = test.get_test_proof();

    // Transfer some coins into the account
    test.execute_expect_success(
        Transaction::builder()
            .call_method(
                component_address,
                "mint_non_fungible",
                args![NonFungibleId::from_string("this-is-new")],
            )
            .sign(test.get_test_secret_key())
            .build(),
        vec![proof],
    );
}
