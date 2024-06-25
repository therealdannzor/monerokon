use tari_template_lib::prelude::*;
use tari_template_test_tooling::TemplateTest;
use tari_transaction::Transaction;

const INITIAL_SUPPLY: Amount = Amount(1_000_000);

#[test]
fn exercise_2_fungible_resource() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![INITIAL_SUPPLY], vec![]);

    let vault_id: VaultId = test.extract_component_value(component_address, "$.supply_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    assert_eq!(
        vault.resource_type(),
        ResourceType::Fungible,
        "Resource type in supply_vault is not fungible. Use `ResourceBuilder::fungible()`"
    );
    assert_ne!(*vault.resource_address(), XTR, "Resource address in supply_vault is XTR. Add the bucket of your new resource to the vault e.g `Vault::from_bucket(bucket)`");

    // Get proof of ownership for the component.
    let proof = test.get_test_proof();

    // Call `get_balance` method on component
    let value: Amount = test.call_method(
        component_address,
        "get_balance",
        args![],
        vec![proof.clone()],
    );
    assert_eq!(value, INITIAL_SUPPLY, "Balance is not equal to expected initial supply. Ensure the Resource initial supply is set correctly.");

    let vault_id: VaultId = test.extract_component_value(component_address, "$.fee_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    assert_eq!(
        *vault.resource_address(),
        XTR,
        "Resource address in fee_vault is not XTR. Create an empty bucket"
    );
}

#[test]
fn exercise_2_withdraw_and_fees() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![INITIAL_SUPPLY], vec![]);

    let (account, account_proof, account_secret) = test.create_funded_account();

    let vault_id: VaultId = test.extract_component_value(component_address, "$.supply_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    let fungible_resource = *vault.resource_address();

    // Check that the transaction rejects if the fee paid is too low
    test.execute_expect_failure(
        Transaction::builder()
            .call_method(account, "withdraw", args![XTR, Amount(9)])
            .put_last_instruction_output_on_workspace("fee")
            .call_method(
                component_address,
                "withdraw",
                args![Workspace("fee"), Amount(100)],
            )
            .put_last_instruction_output_on_workspace("bucket")
            .call_method(account, "deposit", args![Workspace("bucket")])
            .call_method(account, "balance", args![fungible_resource])
            .sign(&account_secret)
            .build(),
        vec![account_proof.clone()],
    );

    // Transfer some coins into the account
    let result = test.execute_expect_success(
        Transaction::builder()
            .call_method(account, "withdraw", args![XTR, Amount(10)])
            .put_last_instruction_output_on_workspace("fee")
            .call_method(
                component_address,
                "withdraw",
                args![Workspace("fee"), Amount(100)],
            )
            .put_last_instruction_output_on_workspace("bucket")
            .call_method(account, "deposit", args![Workspace("bucket")])
            .call_method(account, "balance", args![fungible_resource])
            .sign(&account_secret)
            .build(),
        vec![account_proof],
    );

    let balance = result.expect_return::<Amount>(5);
    assert_eq!(balance, 100i64, "`supply_vault` balance is not equal to expected value after withdrawing 100 coins. Ensure that `withdraw` withdraws the given amount and that balance returns the balance of supply_vault.");

    let vault_id: VaultId = test.extract_component_value(component_address, "$.fee_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    assert_eq!(vault.balance(), 10i64, "fee_vault balance is not equal to expected value after paying 10 XTR in fees. Ensure that the fee is deposited into the fee_vault.");
}
