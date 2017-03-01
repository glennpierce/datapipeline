use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

pub struct TestElement{
 
}

impl TestElement {
    pub fn new() -> Self {
        TestElement{
        }
    }
}


impl Element for TestElement {

    fn run(&self, position :  Arc<AtomicUsize>) {
        loop {
            println!("Hello");
        }
    }
}
