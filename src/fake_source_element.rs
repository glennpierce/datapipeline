use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use element::{Element, ElementPad, ElementPadType, ElementPadDataType};
use pipeline::PipeLineStreamFormat;

pub struct FakeSourceElement {
    output_pad : ElementPad,
    input_pad1 : ElementPad,
}

impl FakeSourceElement {

    pub fn new() -> Self {
        FakeSourceElement{
             output_pad : ElementPad::new("output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
             input_pad1 : ElementPad::new("input".to_string(), ElementPadType::INPUT, ElementPadDataType::STRING),
        }
    }
}


impl Element for FakeSourceElement {

    fn get_name(&self) -> &str {
        "FakeSourceElement"
    }

    fn run(&mut self, position :  Arc<AtomicUsize>, output : SyncSender<PipeLineStreamFormat>, input : Receiver<PipeLineStreamFormat>) {
        let mut i : usize = 0;
       // let max = position.load(Ordering::Relaxed);
      
        //println!("max {}", max);

        let mut count : u64 = 0;

        loop {

            pub type PipeLineStreamFormat = (String, String);

            output.send((count.to_string(), count.to_string()));

            thread::sleep(time::Duration::from_millis(1000));

            count += 1;
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