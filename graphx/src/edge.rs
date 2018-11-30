
use vertex::Vertex;
use std::collections::HashMap;

pub struct Edge <'a>{
    id: & 'a str,
    start_vertex: Vertex<'a>,
    end_vertex: Vertex<'a>,
}

impl <'a> Edge<'a> {
    pub fn new() -> Edge<'a> {

        let mut map = HashMap::new();
        map.insert("SS", "DD");

        let end_vertex_properties:HashMap<&str, &str> = HashMap::new();

        return Edge{ id: "ss", start_vertex: Vertex::new(map), end_vertex: Vertex::new(end_vertex_properties)};
    }
}