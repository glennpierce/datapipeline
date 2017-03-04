use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use pipeline::PipeLineStreamFormat;

use element::*;

pub struct TestElement {
    name : String,
    output_pad : ElementPad,
    input_pad1 : ElementPad,
}

impl TestElement {

    pub fn new(name : &str) -> Self {
        TestElement{
            name : name.to_string(),
             output_pad : ElementPad::new("test_element_output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
             input_pad1 : ElementPad::new("test_element_input".to_string(), ElementPadType::INPUT, ElementPadDataType::STRING),
        }
    }
}


impl Element for TestElement{

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_type(&self) -> String {
        "TestElement".to_string()
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

    fn get_pad<'a>(&'a self, name : &str) -> ElementResult<&'a ElementPad> {
        if self.input_pad1.name == name {
            return Ok(&self.input_pad1);
        }
        else if self.output_pad.name == name {
            return Ok(&self.output_pad);
        }
        else {
            return Err(ElementError::PAD_DOES_NOT_EXIST);
        }
    }
    
    // fn get_input_pad<'a>(&'a self) -> &'a ElementPad {
    //     &self.input_pad1
    // }

    // fn get_output_pad<'a>(&'a self) -> &'a ElementPad {
    //     &self.output_pad
    // }
}