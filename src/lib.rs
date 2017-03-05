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

use element::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use pipeline::Pipeline;
        use element::Element;
        use test_element::TestElement;
        use fake_source_element::FakeSourceElement;

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


                pipeline.run();
                

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
