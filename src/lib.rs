use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


#[derive(Debug)]
pub enum ElementError {
    FAILED_TO_READ_INPUT,
}

pub type ElementResult<T> = Result<T, Element>;

enum ElementPadType {
    STRING,
    NUMERIC,
    DATETIME,
    FILE
}

struct ElementPad {
    name : String,
    pad_type: ElementPadType,
    //comm : (Sender<i32>, Receiver<i32>),
}

impl ElementPad {
    pub fn new(name : String, pad_type: ElementPadType) -> Self {
        ElementPad{
            name : name,
            pad_type : pad_type,
            //comm : mpsc::channel(),
        }
    }

    // pub fn send_on_output(&self, index : u8) {
    //     //let mut map = self.inputs; 
    //     self.outputs[index].comm[0].send(id).unwrap();
    // }
}

struct BaseElement{
    name : String,
    inputs : Vec<ElementPad>,
    outputs : Vec<ElementPad>,
}

impl BaseElement {
    pub fn new(name: String) -> Self {
        BaseElement{
            name : name,
            inputs : Vec::new(),
            outputs : Vec::new(),
        }
    }

    pub fn add_input(&mut self, name : String, pad_type : ElementPadType) {
        //let mut map = self.inputs; 
        self.inputs.push(ElementPad::new (name, pad_type));
    }

    pub fn add_output(&mut self, name : String, pad_type : ElementPadType) {
        //let mut map = self.inputs; 
        self.outputs.push(ElementPad::new (name, pad_type));
    }
}

trait Element {
    fn initalise(&mut self);
    fn run(&self);
}

struct FileSourceElement {
    base : BaseElement,
    file: File,
}

impl FileSourceElement {
    fn new(name: String, file : File) -> Self {
        FileSourceElement { base : BaseElement::new(name),
                            file :file }
    }
}

impl Element for FileSourceElement {
    
    fn initalise(&mut self) {
        //self.base.add_input("input_pad1_filesource".to_string(), ElementPadType::FILE);
        self.base.add_output("output_pad_filedata".to_string(), ElementPadType::FILE);
    }

    fn run(&self) {
        // Read from file and stream data to the output pad.
        let mut reader = BufReader::new(&self.file);

        let (tx, rx): (Sender<String>, _) = mpsc::channel();
        for line in reader.lines() {
            let l = line.unwrap();
            //self.base.outputs[0].comm
            //println!("{}", l); 
            tx.send(l).unwrap();
        }           
    }
}



struct ConsoleEchoElement {
    base : BaseElement,
}

impl ConsoleEchoElement {
    fn new(name: String, file : File) -> Self {
        ConsoleEchoElement { base : BaseElement::new(name) }
    }
}

impl Element for ConsoleEchoElement {
    
    fn initalise(&mut self) {
        // TODO needs multiple types
        self.base.add_input("input_pad1_filesource".to_string(), ElementPadType::FILE);
    }

    fn run(&self) {
        let (_, rx): (_, Receiver<String>) = mpsc::channel();
        println!("{}", rx.recv().unwrap());       
    }
}


struct ElementPipeline {
    elements : Vec<Element>,
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use ::Element;
        use ::FileSourceElement;
        use std::fs::File;

        match File::open("data/test.txt") {
            Ok(file) => {
              let file_source_element = FileSourceElement::new("My Source".to_string(), file);
              file_source_element.run();
            },
            Err(e) => {
                // fallback in case of failure.
                // you could log the error, panic, or do anything else.
                println!("{}", e);
            }
        };

        
    }
}
