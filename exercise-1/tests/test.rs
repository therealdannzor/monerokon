use tari_template_lib::prelude::*;
use tari_template_test_tooling::TemplateTest;

#[test]
fn exercise_1_initial_states() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![], vec![]);

    let counter: u32 = test.extract_component_value(component_address, "$.counter");
    assert_eq!(counter, 0, "Counter was not initialize to 0");
}

#[test]
fn exercise_2_counter() {
    let mut test = TemplateTest::new(["."]);

    // Construct the component
    let component_address: ComponentAddress =
        test.call_function("Monerokon", "new", args![], vec![]);

    // Get proof of ownership for the component.
    let proof = test.get_test_proof();

    // Call `value` method on component
    let counter: u32 = test.call_method(component_address, "counter", args![], vec![proof.clone()]);
    assert_eq!(counter, 0);

    // Increase the counter (Mutate component state)
    test.call_method::<()>(component_address, "increase", args![], vec![proof.clone()]);
    test.call_method::<()>(component_address, "increase", args![], vec![proof.clone()]);

    // Assert the counter was increased
    let counter: u32 = test.call_method(component_address, "counter", args![], vec![proof]);
    assert_eq!(counter, 2);
}
