use element::{Element, ElementPad, ElementPadType, ElementPadDataType};


pub struct BaseElement<'a> {
     //x: &'a i32,

    //pipeline : &'a PipeLine,
    name : String,
    next : &'a Element,
    pipeline : &'a Element,
    inputs : Vec<ElementPad>,
    output : ElementPad,
}

impl<'a> BaseElement<'a> {
    pub fn new(name: String) -> Self {
        BaseElement{
            name : name,
            inputs : Vec::new(),
            output : ElementPad::new ("output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
        }
    }

    pub fn add_input(&mut self, name : String, pad_type : ElementPadType, pad_data_type : ElementPadDataType) {
        //let mut map = self.inputs; 
        self.inputs.push(ElementPad::new (name, pad_type, pad_data_type));
    }
}


impl<'a> Element for BaseElement<'a> {

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
        loop {

        }
    }
}
