use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use std::collections::HashMap;

use crossbeam;

use element::ElementPadConnection;

#[derive(Clone, Debug)]
pub enum PipeLineError {
    ELEMENT_ALREADY_EXISTS,
    ELEMENT_CANNOT_CONNECT_TO_SELF,
}

pub type PipelineResult<T> = Result<T, PipeLineError>;

pub type PipeLineStreamFormat = (String, String);

pub struct Pipeline<'a> {
    name : String,
    elements : Vec<&'a Element>,
    connections : HashMap<String, ElementPadConnection>,
}

impl<'a> Pipeline<'a> {

    pub fn new(name: String) -> Self {
        Pipeline{
            name : name,
            elements : Vec::new(),
            connections : HashMap::new(),
        }
    }

    pub fn add_element(&mut self, element: &'a Element) -> PipelineResult<()> {
        self.elements.push(element);
        Ok(()) 
    }

    pub fn attach_output_pad_to_input_pad(&mut self, output : &'a Element, input : &'a Element) -> PipelineResult<()> {

        let output_pad = output.get_output_pad();
        let input_pad = input.get_input_pad();
        let sender = output_pad.conn.0.clone();
        let receiver = input_pad.conn.1.clone();
         
        println!("connecting pads, sender : {} -> receiver {}", output_pad.name, input_pad.name);

        self.connections.insert(output.get_name().to_string(), (sender, receiver));

        Ok(())
    } 

    pub fn run(&self) {
    
        
        crossbeam::scope(|scope| {
            for e in &self.elements {
                scope.spawn(move || {
                    println!("Running child thread in scope");
                        
                    let pad = self.connections.get(&e.get_name()).unwrap();
                    // let channel = sync_channel::<PipeLineStreamFormat>(1000);
                    // let sender = Arc::new(Mutex::new(channel.0)).clone(); //Arc::new( Mutex::new(channel.0));
                    // let receiver = Arc::new(Mutex::new(channel.1)).clone();
        
                    // /e.run(sender, receiver);
                });
            }
        });
    }

    pub fn quick_test(&self) {

        println!("quick_test");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);
    }

}
