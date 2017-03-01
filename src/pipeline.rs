//use element::*;
//use base_element::*;

use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

//use base_element::BaseElement;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;


#[derive(Debug)]
pub enum PipeLineError {
    ELEMENT_ALREADY_EXISTS,
}

type PipelineResult<T> = Result<T, PipeLineError>;
type PipeLineStreamFormat = (String, String);

pub struct Pipeline {

    //base : BaseElement<'a>,
    elements : Vec<(Arc<Mutex<Element>>, Arc<AtomicUsize>)>,
//    data_queue : Vec<PipeLineStreamFormat>,

}

impl Pipeline {

    pub fn new(name: String) -> Self {
        Pipeline{
            //name : name,
            //next : None,
            //pipeline : None,
            elements : Vec::new(),
       //     data_queue : Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Arc<Mutex<Element>>) -> PipelineResult<()> {
        // if let Some(found_element) = self.find_element(element.get_name()) {
        //         debug!("Element with that name already exits in pipeline");
        //         return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
        // }; // <-- immutable borrow ends here
        // now you can re-borrow mutably
        self.elements.push((Arc::new(Mutex::new(element)), Arc::new(AtomicUsize::new(0))));
        Ok(()) 
    }

    // pub fn add_element(&mut self, element : &'a Element) -> PipelineResult<&'a Element> {
    //     if let Some(found_element) = self.find_element(element.get_name()) {
    //             debug!("Element with that name already exits in pipeline");
    //             return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
    //     }; // <-- immutable borrow ends here
    //     // now you can re-borrow mutably
    //     self.elements.push(element);
    //     Ok(element) 
    // }

    // pub fn add_element(&mut self, element : &'a Element) -> PipelineResult<&'a Element> {
     
    //     let found_element = (&self).find_element(element.get_name());
        
    //     match found_element {
    //         Some(found_element) => {
    //             // fallback in case of failure.
    //             // you could log the error, panic, or do anything else.
    //             debug!("Element with that name already exits in pipeline");
    //             return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
    //         }
    //         None => {
    //             self.elements.push(element);
    //             return Ok(element)  
    //         }
    //     };
    // }

    // pub fn get_first_element(&self) -> &Element {
    //     return self.elements[0];
    // }

    // pub fn find_element(&self, name : &str) -> Option<&&Element> {
    //     return self.elements.iter().find(|&&e| e.get_name() == name);
    // }

    pub fn run(&self) {

        loop {

            let e = &self.elements[0];

            thread::spawn(move || {
                e.0.run(e.1);
            });
        }
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
