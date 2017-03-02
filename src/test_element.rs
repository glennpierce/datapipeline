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
        let mut i = 0;
      
        while i<100 {
            //println!("Hello");
            let v = position.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
            i += 1;
        }
      
    }
}
