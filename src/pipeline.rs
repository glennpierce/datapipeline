//use element::*;
//use base_element::*;

use element::{Element, ElementPad, ElementPadType, ElementPadDataType};

//use base_element::BaseElement;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};


#[derive(Debug)]
pub enum PipeLineError {
    ELEMENT_ALREADY_EXISTS,
}

pub type PipelineResult<T> = Result<T, PipeLineError>;
pub type PipeLineStreamFormat = (String, String);

pub struct Pipeline {

    //base : BaseElement<'a>,
    elements : Vec<(Arc<Mutex<Element>>, Arc<AtomicUsize>)>,
    data_queue : Vec<PipeLineStreamFormat>,

}

impl Pipeline {

    pub fn new(name: String) -> Self {
        Pipeline{
            //name : name,
            //next : None,
            //pipeline : None,
            elements : Vec::new(),
            data_queue : Vec::new(),
        }
    }

    pub fn print_last_position(&self) {
        if self.elements.is_empty() {
            println!("0");
        }

        println!("{:?}", self.elements.last().unwrap().1);
    }

    pub fn add_element<T: Element + 'static>(&mut self, element: T) -> PipelineResult<()> {
        //     // if let Some(found_element) = self.find_element(element.get_name()) {
    //     //         debug!("Element with that name already exits in pipeline");
    //     //         return Err(PipeLineError::ELEMENT_ALREADY_EXISTS)
    //     // }; // <-- immutable borrow ends here
    //     // now you can re-borrow mutably
        self.elements.push((Arc::new(Mutex::new(element)), Arc::new(AtomicUsize::new(0))));
        Ok(()) 
    }

    pub fn attach_output_pad_to_input_pad(output : &Element, input : &Element, pad_name : &str) -> PipelineResult<()> {
        // Confirm pad name in correct

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
        for e in &self.elements {
            let elem = e.0.clone();
            let c = e.1.clone();
            handles.push(thread::spawn(move || {
                elem.lock().unwrap().run(c);
            }));
        }
        handles
    }

    pub fn quick_test(&self) {

        println!("quick_test");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);

        self.elements[0].1.fetch_add(100, Ordering::SeqCst);

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
