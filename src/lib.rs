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

                let mut fake_src = FakeSourceElement::new("fake_source");
                let mut test_element = TestElement::new("test_element");
                let mut pipeline = Pipeline::new("example pipeline".to_string());

                pipeline.add_element(fake_src).unwrap();
                pipeline.add_element(test_element).unwrap();

                pipeline.attach_output_pad_to_input_pad("fake_source", "fake_source_output",
                                                        "test_element", "test_element_input").unwrap();

                let handles = pipeline.run();
                
                println!("Pipeline started - waiting for {} threads to finish", handles.len());

                pipeline.quick_test(); 

                io::stdout().flush().ok().expect("Could not flush stdout");
                for (i, h) in handles.into_iter().enumerate() {
                    h.join();
                    println!("Thread {} finished", i);
                }

                

                println!("Done");
            },
            Err(e) => {
                // fallback in case of failure.
                // you could log the error, panic, or do anything else.
                println!("{}", e);
            }
        };

        
    }
}
