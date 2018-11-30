use std::collections::HashMap;
    
pub struct Vertex <'a>{
    id: & 'a str,
    properties: HashMap< & 'a str, & 'a str>
}

impl <'a> Vertex <'a> {
    // add code here

    pub fn new( props: HashMap<& 'a str, & 'a str>) -> Vertex<'a> {
        return Vertex{ id: "dd" , properties : props};   
    }
}