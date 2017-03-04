//use element::*;
//use base_element::*;

use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use std::collections::HashMap;

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
    elements : Vec<Arc<Mutex<&'a Element>>>,
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

    //pub fn add_element<T: Element + 'static>(&mut self, element: &T) -> PipelineResult<()> {
    pub fn add_element(&mut self, element: &'a Element) -> PipelineResult<()> {
        //     // if let Some(found_element) = self.find_element(element.get_name()) {
    //     //         debug!("Element with that name already exits in pipeline");
    //     //         return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
    //     // }; // <-- immutable borrow ends here
    //     // now you can re-borrow mutably

    
   //     let safe_element = Arc::new(Mutex::new(element));
        self.elements.push(Arc::new(Mutex::new(element)));

        //self.elements.push(element);

        //Ok(safe_element) 
        Ok(()) 
    }

    //pub fn attach_output_pad_to_input_pad<T: Element + 'static>(&mut self, output : &T, input : &T) -> PipelineResult<()> {
    pub fn attach_output_pad_to_input_pad(&mut self, output : &'a Element, input : &'a Element) -> PipelineResult<()> {
        // Confirm pad name in correct

 //       let mut out = output;

        // Assert that output element and input element's are not the same.
   //     if output.get_name() == input.get_name() {
   //         return Err(PipeLineError::ELEMENT_CANNOT_CONNECT_TO_SELF);
   //     }

        let output_pad = output.get_output_pad();
        let input_pad = input.get_input_pad();
        let sender = output_pad.conn.0.clone();
        let receiver = input_pad.conn.1.clone();
         
        println!("connecting pads, sender : {} -> receiver {}", output_pad.name, input_pad.name);

        self.connections.insert(output.get_name().to_string(), (sender, receiver));

        Ok(())
    } 

    pub fn run(&self) -> Vec<thread::JoinHandle<()>>{
        let mut handles = Vec::with_capacity(self.elements.len());

        for e in &self.elements {
            let element_clone = e.clone();

            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let element = element_clone.lock().unwrap();
            let name = element.get_name();
            let conn = self.connections.get(name).unwrap();
            println!("calling element run for {}", name);
            
            handles.push(thread::spawn(move || {
                
                //element.run(conn.0.clone(), conn.1.clone());
                element_clone.lock().unwrap().run();

                //element.run();
            }));
        }
        handles
    }

    pub fn quick_test(&self) {

        println!("quick_test");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);

        //self.elements[0].1.fetch_add(100, Ordering::SeqCst);

    }

}
