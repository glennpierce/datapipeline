use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use element::{Element, ElementPad, ElementPadType, ElementPadDataType};
use pipeline::PipeLineStreamFormat;

pub struct TestElement {
    output_pad : ElementPad,
    input_pad1 : ElementPad,
}

impl TestElement {

    pub fn new() -> Self {
        TestElement{
             output_pad : ElementPad::new("output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
             input_pad1 : ElementPad::new("input".to_string(), ElementPadType::INPUT, ElementPadDataType::STRING),
        }
    }
}


impl Element for TestElement{

    fn get_name(&self) -> &str {
        "TestElement"
    }

    fn run(&mut self, output : SyncSender<PipeLineStreamFormat>, input : Arc<Mutex<Receiver<PipeLineStreamFormat>>>) {
        let mut i : usize = 0;
       // let max = position.load(Ordering::Relaxed);
      
        //println!("max {}", max);

        loop {

            // let max = position.fetch_add(0, Ordering::SeqCst);

            // println!("max {}", max);

            // while i < max {
            //     //println!("Hello");

            //     //let v = position.fetch_add(1, Ordering::SeqCst);
            //     println!("{:?}", i);
            //     i += 1;

            //     thread::sleep(time::Duration::from_millis(100));
            // }

            thread::sleep(time::Duration::from_millis(1000));
        }
      
    }

    // fn get_input_pads<'a>(&'a mut self) -> &'a[&'a ElementPad]
    // {
    //     &[&self.input_pad1]
    // }

    fn get_input_pad<'a>(&'a self) -> &'a ElementPad {
        &self.input_pad1
    }

    fn get_output_pad<'a>(&'a self) -> &'a ElementPad {
        &self.output_pad
    }
}