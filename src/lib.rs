use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

enum ElementPadType {
    STRING,
    NUMERIC,
    DATETIME,
    FILEPATH
}

struct ElementPad {
    name : String,
    pad_type: ElementPadType,
    comm : (Sender<i32>, Receiver<i32>),
}

impl ElementPad {
    pub fn new(name : String, pad_type: ElementPadType) -> Self {
        ElementPad{
            name : name,
            pad_type : pad_type,
            comm : mpsc::channel(),
        }
    }
}

struct BaseElement{
    name : String,
    inputs : Vec<ElementPad>,
    //output : ElementPad,
}

impl BaseElement {
    pub fn new(name: String) -> Self {
        BaseElement{
            name : name,
            inputs : Vec::new()
        }
    }

    pub fn add_input(&mut self, name : String, pad_type : ElementPadType) {
        //let mut map = self.inputs; 
        self.inputs.push(ElementPad::new (name, pad_type));
    }
}

trait Element {
    fn new(name: String) -> Self;
    fn run(&self);
}

struct FileSourceElement {
    base : BaseElement,
}

impl Element for FileSourceElement {
    fn new(name: String) -> Self {
        FileSourceElement { base : BaseElement::new(name) }
    }

    fn run(&self) {
        println!("Hi");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use ::Element;
        use ::FileSourceElement;

        let file_source_element = FileSourceElement::new("My Source".to_string());
    }
}
