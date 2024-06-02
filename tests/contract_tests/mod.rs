use contract_tests_example::ContractTrait;

use super::*;

#[rstest]
fn product_should_be_100(implementation: impl ContractTrait) {
    assert_eq!(
        implementation.function_a() * implementation.function_b(),
        100
    );
}

#[rstest]
fn sum_should_be_52(implementation: impl ContractTrait) {
    assert_eq!(
        implementation.function_a() + implementation.function_b(),
        52
    );
}

#[rstest]
fn should_be_both_even(implementation: impl ContractTrait) {
    assert_eq!((implementation.function_a() % 2), 0);
    assert_eq!((implementation.function_b() % 2), 0);
}
