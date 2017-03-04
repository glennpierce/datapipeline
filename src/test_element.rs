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
             output_pad : ElementPad::new("test_element_output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
             input_pad1 : ElementPad::new("test_element_input".to_string(), ElementPadType::INPUT, ElementPadDataType::STRING),
        }
    }
}


impl Element for TestElement{

    fn get_name(&self) -> &str {
        "TestElement"
    }

    // fn run(&self, output : SyncSender<PipeLineStreamFormat>, input : Arc<Mutex<Receiver<PipeLineStreamFormat>>>) {
    //     let mut i : usize = 0;
    //    // let max = position.load(Ordering::Relaxed);

    //     loop {

    //         let val = input.lock().unwrap().recv().unwrap();

    //         println!("TestElement: {:?}", val);

    //         thread::sleep(time::Duration::from_millis(1000));
    //     }
      
    // }

    fn run(&self) {
        let mut i : usize = 0;
    
        loop {

            println!("TestElement");

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