//use element::*;
//use base_element::*;

use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

//use base_element::BaseElement;
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

    //base : BaseElement<'a>,
    name : String,
    //elements : Vec<Arc<Mutex<Element>>>,
    elements : Vec<&'a Element>,
    connections : HashMap<String, ElementPadConnection>,
    //data_queue : Vec<PipeLineStreamFormat>,

}

impl<'a> Pipeline<'a> {

    pub fn new(name: String) -> Self {
        Pipeline{
            name : name,
            //next : None,
            //pipeline : None,
            elements : Vec::new(),
            connections : HashMap::new(),
            //data_queue : Vec::new(),

        }
    }

    // pub fn print_last_position(&self) {
    //     if self.elements.is_empty() {
    //         println!("0");
    //     }

    //     println!("{:?}", self.elements.last().unwrap().1);
    // }

    //pub fn add_element<T: Element + 'static>(&mut self, element: &T) -> PipelineResult<()> {
    pub fn add_element(&mut self, element: &'a Element) -> PipelineResult<()> {
        //     // if let Some(found_element) = self.find_element(element.get_name()) {
    //     //         debug!("Element with that name already exits in pipeline");
    //     //         return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
    //     // }; // <-- immutable borrow ends here
    //     // now you can re-borrow mutably

    
    //    let safe_element = Arc::new(Mutex::new(element));
    //    self.elements.push(safe_element);

        self.elements.push(element);

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

        let sender = output.get_output_pad().conn.0.clone();
        let receiver = output.get_output_pad().conn.1.clone();
         
         //element_clone.lock().unwrap()

//pub type ElementPadConnection = (SyncSender<PipeLineStreamFormat>, Receiver<PipeLineStreamFormat>);

        self.connections.insert(output.get_name().to_string(), (sender, receiver));

//element_clone.lock().unwrap()


        Ok(())
    } 

    // pub fn get_first_element(&self) -> &Element {
    //     return self.elements[0];
    // }

    // pub fn find_element(&self, name : &str) -> Option<&&Element> {
    //     return self.elements.iter().find(|&&e| e.get_name() == name);
    // }

    // pub fn run(&self) {

    //     loop {

    //         let e = &self.elements[0];

    //         thread::spawn(move || {
    //             e.0.run(e.1);
    //         });
    //     }
    // }


    pub fn run(&self) -> Vec<thread::JoinHandle<()>>{
        let mut handles = Vec::with_capacity(self.elements.len());

        //let mut last_element;

        for (i, e) in self.elements.iter().enumerate() {
            //let element_clone = e.clone();
            //last_element = &elem;
            //let c = e.1.clone();

            let conn = self.connections.get(e.get_name()).unwrap();

            let element_clone = Arc::new(Mutex::new(e)).clone();
           // let c = e.1.clone();

            //elf.connections.insert(output.get_name().to_string(), (sender, receiver));
            //map.insert(1, "a");
        //assert_eq!(map.get(&1), Some(&"a"));
        //assert_eq!(map.get(&2), None);

            

        //    let sender = output.get_output_pad().conn.0.clone();
      //  let receiver = output.get_output_pad().conn.1.clone();
         

            handles.push(thread::spawn(move || {
                //element_clone.lock().unwrap().run(c);
                element_clone.lock().unwrap().run(conn.0, conn.1);
                //  fn run(&mut self, position : Arc<AtomicUsize>, output : SyncSender<PipeLineStreamFormat>, input : Receiver<PipeLineStreamFormat>);
   
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


// impl Element for Pipeline {

//     fn run(&self, position : usize) {

//     }


//     // fn next(&self) -> &Element {
//     //     return self.next;
//     // }

//     // fn pipeline(&self) -> &Element {
//     //     return &self;
//     // }
    
//     // fn get_name(&self) -> &str {
//     //     return "FileSourceElement";
//     // }

//     // fn initalise(&mut self) {
//     //     //self.base.add_input("input_pad1_filesource".to_string(), ElementPadDataType::FILE);
//     //    // self.base.add_output("data".to_string(), ElementPadType::OUTPUT, ElementPadDataType::FILE);
//     // }

//     // fn run(&self) {
//     //     loop {

//     //     }
//     // }
// }
