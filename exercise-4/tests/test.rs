use tari_template_lib::prelude::*;
use tari_template_test_tooling::support::confidential::{
    generate_confidential_proof, generate_withdraw_proof_with_inputs, WithdrawProofOutput,
};
use tari_template_test_tooling::TemplateTest;
use tari_transaction::Transaction;

const INITIAL_SUPPLY: Amount = Amount(1_000_000);

#[test]
fn exercise_4_confidential_withdraw() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let (output, initial_supply_bf, _) = generate_confidential_proof(INITIAL_SUPPLY, None);
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![output], vec![]);

    let (account, account_proof, account_secret) = test.create_funded_account();

    let vault_id: VaultId = test.extract_component_value(component_address, "$.confidential_vault");
    let vault = test.read_only_state_store().get_vault(&vault_id).unwrap();
    assert_eq!(
        vault.resource_type(),
        ResourceType::Confidential,
      "Resource type for `confidential_vault` is not Confidential. Use `ResourceBuilder::confidential()` to create the resource and deposit the bucket into `confidential_vault`."
    );

    let WithdrawProofOutput {
        proof: withdraw_proof,
        ..
    } = generate_withdraw_proof_with_inputs(
        &[(initial_supply_bf, INITIAL_SUPPLY)],
        Amount::zero(),
        Amount(1000),
        Some(INITIAL_SUPPLY - Amount(1000)),
        Amount::zero(),
    );

    // Transfer some coins into the account
    test.execute_expect_success(
        Transaction::builder()
            // Withdraw revealed fee funds
            .call_method(account, "withdraw", args![XTR, Amount(10)])
            .put_last_instruction_output_on_workspace("fee")
            // Run withdraw_confidential on the component
            .call_method(
                component_address,
                "withdraw",
                args![Workspace("fee"), withdraw_proof],
            )
            .put_last_instruction_output_on_workspace("bucket")
            .call_method(account, "deposit", args![Workspace("bucket")])
            .sign(&account_secret)
            .build(),
        vec![account_proof],
    );
}

#[test]
fn exercise_4_confidential_mint() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let (output, _, _) = generate_confidential_proof(INITIAL_SUPPLY, None);
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![output], vec![]);

    // Get proof of ownership for the component.
    let proof = test.get_test_proof();

    let (output, _, _) = generate_confidential_proof(INITIAL_SUPPLY, None);

    // Transfer some coins into the account
    test.execute_expect_success(
        Transaction::builder()
            .call_method(component_address, "mint_confidential", args![output])
            .sign(test.get_test_secret_key())
            .build(),
        vec![proof],
    );

    let confidential_vault: VaultId =
        test.extract_component_value(component_address, "$.confidential_vault");
    let confidential_vault = test
        .read_only_state_store()
        .get_vault(&confidential_vault)
        .unwrap();

    let num_commitments = confidential_vault
        .get_confidential_commitments()
        .expect("Confidential vault does not have any commitments after minting. Ensure that the `mint_confidential` method calls `ResourceManager::mint_confidential()` with the `mint` arg.")
        .len();
    assert_eq!(
        num_commitments, 2,
        "Expected 2 commitments in the confidential vault, found {}",
        num_commitments
    );
}
