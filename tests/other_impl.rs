use contract_tests_example::ContractTrait;
use rstest::{fixture, rstest};

#[fixture]
fn implementation() -> impl ContractTrait {
    struct B;
    impl ContractTrait for B {
        fn function_a(&self) -> u32 {
            2
        }
        fn function_b(&self) -> u32 {
            50
        }
    }
    B
}

mod contract_tests;
