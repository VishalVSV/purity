extern crate purity_lib;

use purity_lib::runtime::Program;

pub trait PureVM {
    fn run(program: Program) -> PureVMResults;
}

pub enum PureVMResults {
    Success,
    Error
}