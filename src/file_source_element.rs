use element::Element;
use base_element::BaseElement;

use std::fs::File;

pub struct FileSourceElement<'a> {
    base : BaseElement<'a>,
    file: File,
}

impl<'a> FileSourceElement<'a> {
    fn new(name: String, file : File) -> Self {
        let mut element = FileSourceElement { base : BaseElement::new(name), file :file };

        element.initalise();
        
        element
    }
}


impl<'a> Element for FileSourceElement<'a> {
    
    fn next(&self) -> &Element {
        return self.next;
    }

    fn pipeline(&self) -> &Element {
        return &self;
    }

    fn get_name(&self) -> &str {
        return "FileSourceElement";
    }

    fn initalise(&mut self) {
        //self.base.add_input("input_pad1_filesource".to_string(), ElementPadDataType::FILE);
       // self.base.add_output("data".to_string(), ElementPadType::OUTPUT, ElementPadDataType::FILE);
    }

    fn run(&self) {
   
    }
}


