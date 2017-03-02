use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use element::{Element, ElementPad, ElementPadType, ElementPadDataType};
use std::{thread, time};

pub struct TestElement{
 
}

impl TestElement {
    pub fn new() -> Self {
        TestElement{
        }
    }
}


impl Element for TestElement {

    fn run(&mut self, position :  Arc<AtomicUsize>) {
        let mut i : usize = 0;
       // let max = position.load(Ordering::Relaxed);
      
        //println!("max {}", max);

        loop {

            let max = position.fetch_add(0, Ordering::SeqCst);

            println!("max {}", max);

            while i < max {
                //println!("Hello");

                //let v = position.fetch_add(1, Ordering::SeqCst);
                println!("{:?}", i);
                i += 1;

                thread::sleep(time::Duration::from_millis(100));
            }

            thread::sleep(time::Duration::from_millis(1000));
        }
      
    }
}
