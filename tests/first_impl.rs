use contract_tests_example::ContractTrait;
use rstest::{fixture, rstest};

#[fixture]
fn implementation() -> impl ContractTrait {
    struct A;
    impl ContractTrait for A {
        fn function_a(&self) -> u32 {
            50
        }
        fn function_b(&self) -> u32 {
            2
        }
    }
    A
}

mod contract_tests;
