use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::{thread, time};
use std::collections::HashMap;

use element::*;


pub type PipelineResult<T> = Result<T, ElementError>;

pub type PipeLineStreamFormat = (String, String);

type ThreadSafeElement = Arc<Mutex<Element>>;
//type ThreadSafeRefElement = Arc<Mutex<&'a Element>>;

pub struct Pipeline {
    name : String,
    elements : HashMap<String, ThreadSafeElement>,
    connections : HashMap<String, ElementPadConnection>,
}

impl Pipeline {

    pub fn new(name: String) -> Self {
        Pipeline{
            name : name,
            elements : HashMap::new(),
            connections : HashMap::new(),
        }
    }

    fn find_element<'a>(&'a self, name : &str) -> PipelineResult<&'a ThreadSafeElement> {

        let option = self.elements.get(name);

        if !option.is_some()
        {
            warn!("Element with name {} not found in pipeline", name);
            return Err(ElementError::ELEMENT_NOT_FOUND)
        }; 
        
        Ok(&option.unwrap())
    }

    fn find_element_pad<'a>(&'a self, name : &str, pad_name : &str) -> PipelineResult<&ElementPad> {

        let element = try!(self.find_element(name)).clone().lock().unwrap();

        let pad = try!(element.get_pad(pad_name));
        
        Ok(pad)
    }

    pub fn add_element<T: Element + 'static>(&mut self, element: T) -> PipelineResult<()> {

        self.elements.insert(element.get_name(), Arc::new(Mutex::new(element)));

        Ok(()) 
    }

    //pub fn attach_output_pad_to_input_pad<T: Element + 'static>(&mut self, output : &T, input : &T) -> PipelineResult<()> {
    pub fn attach_output_pad_to_input_pad(&mut self, output_name : &str, output_pad_name : &str,
                                                     input_name : &str, input_pad_name : &str) -> PipelineResult<()> {

        //let output = try!(self.find_element(output_name));
        //let input = try!(self.find_element(input_name));
    
        let output_pad = try!(self.find_element_pad(output_name, output_pad_name));
        let input_pad = try!(self.find_element_pad(input_name, input_pad_name));

        //let output_pad = output.get_output_pad();
        //let input_pad = input.get_input_pad();

        let sender = output_pad.conn.0.clone();
        let receiver = input_pad.conn.1.clone();
         
        println!("connecting pads, sender : {} -> receiver {}", output_pad.name, input_pad.name);

        self.connections.insert(output_name.to_string(), (sender, receiver));

        Ok(())
    } 

    pub fn run(&self) -> Vec<thread::JoinHandle<()>>{

        let mut handles = Vec::with_capacity(self.elements.len());
        for e in self.elements.values() {
            let elem = e.clone();
            //let c = e.1.clone();
            handles.push(thread::spawn(move || {
                elem.lock().unwrap().run();
            }));
        }
        handles


        // let mut handles = Vec::with_capacity(self.elements.len());

        // for e in &self.elements {
        //     let element_clone = e.clone();

        //     // We unwrap() the return value to assert that we are not expecting
        //     // // threads to ever fail while holding the lock.
        //     let element = element_clone.lock().unwrap();
        //     let name = element.get_name();
        //     // let conn = self.connections.get(name).unwrap();
        //     println!("calling element run for {}", name);
            
        //     handles.push(thread::spawn(move || {
                
        //         //element.run(conn.0.clone(), conn.1.clone());
        //         element.run();

        //         //element.run();
        //     }));
        // }
        // handles
    }

    pub fn quick_test(&self) {

        println!("quick_test");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);

        //self.elements[0].1.fetch_add(100, Ordering::SeqCst);

    }

}
