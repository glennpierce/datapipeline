#![allow(warnings)]

extern crate time;
extern crate chrono;
extern crate crossbeam;

#[macro_use]
extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Weak;

mod element;
mod fake_source_element;
mod test_element;
mod pipeline;
mod file_source_element;

use element::*;

//use std::sync::deque::BufferPool;






// trait PipeLine {
//     fn create_element(name : &str, type : ElementType);
// }





// struct ConsoleEchoElement {
//     base : BaseElement,
// }

// impl ConsoleEchoElement {
//     fn new(name: String) -> Self {
//         let mut element = ConsoleEchoElement { base : BaseElement::new(name) };
//         element.initalise();
//         element
//     }
// }

// impl Element for ConsoleEchoElement {
    
    
//     fn initalise(&mut self) {
//         // TODO needs multiple types
//         self.base.add_input("data".to_string(), ElementPadType::INPUT, ElementPadDataType::FILE);
//     }

//     fn run(&self) {
//         //let (_, rx): (_, Receiver<String>) = mpsc::channel();
//         //println!("{}", rx.recv().unwrap());    

//         // let channels = &self.base.inputs[0].comm;
//         // let rx = &(channels.1);
//         // println!("{}", rx.recv().unwrap());    
//     }
// }



// struct ElementPipeline {
//     elements : Vec<Element>,
// }


// class Race {
// Frame createFrame() { return new Frame(); }
// Wheel createWheel() { return new Wheel(); }
// Bicycle createBicycle(Frame frame, Wheel front, Wheel rear) {
// return new Bicycle(frame, front, rear);
// }
// // return a complete bicycle without needing any arguments
// Bicycle completeBicycle() {
// Frame frame = createFrame();
// Wheel frontWheel = createWheel();
// Wheel rearWheel = createWheel();
// return createBicycle(frame, frontWheel, rearWheel);
// }
// Race createRace() {
// Bicycle bike1 = completeBicycle();
// Bicycle bike2 = completeBicycle();
// ...
// }
// }






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use pipeline::Pipeline;
        use element::Element;
        use test_element::TestElement;
        use fake_source_element::FakeSourceElement;

        //use file_source_element::FileSourceElement;
        //use ::ConsoleEchoElement;
        use std::fs::File;
        use std::sync::Arc;
        use std::sync::Mutex;

        use std::io::prelude::*;                                                           
        use std::io;   

        match File::open("data/test.txt") {
            Ok(file) => {

                let mut fake_src = FakeSourceElement::new();
                let mut test_element = TestElement::new();
                let mut pipeline = Pipeline::new("example pipeline".to_string());

                pipeline.add_element(&fake_src).unwrap();
                pipeline.add_element(&test_element).unwrap();
                
               // let mut fake_src_ref : &Element = &fake_src;
                pipeline.attach_output_pad_to_input_pad(&fake_src, &test_element).unwrap();


                let handles = pipeline.run();
                
                println!("Pipeline started - waiting for {} threads to finish", handles.len());

                pipeline.quick_test(); 

                io::stdout().flush().ok().expect("Could not flush stdout");
                for (i, h) in handles.into_iter().enumerate() {
                    h.join();
                    println!("Thread {} finished", i);
                }

                

                println!("Done");

                //pipeline.print_last_position();

                // let mut pipeline = Pipeline::new("example pipeline".to_string());

                // //let file_source_element = FileSourceElement::new("My Source".to_string(), file);
                // //pipeline.add_element(&file_source_element);

                // let e1 = TestElement::new();
                // pipeline.add_element(Arc::new(Mutex::new(e1))).unwrap();


                // pipeline.run();

                //let echo_element = ConsoleEchoElement::new("My Echo".to_string());

             // file_source_element.attach_output_pad(source_pad : &str, sink_element : BaseElement, sink : &str) {
              //echo_element.attach_pads(file_source_element, );

                //file_source_element.run();
                //echo_element.run();
            },
            Err(e) => {
                // fallback in case of failure.
                // you could log the error, panic, or do anything else.
                println!("{}", e);
            }
        };

        
    }
}
