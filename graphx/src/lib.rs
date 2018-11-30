#![allow(dead_code)]
#![allow(unused_imports)]

mod vertex;
mod edge;
mod adjacency_list;

pub mod graph_x{
    
    use vertex::Vertex;
    use edge::Edge;

    use adjacency_list::AdjacencyList;

    struct Graph<'a>{
        vertices : Vec<Vertex<'a>>,
        edges : Vec<Edge<'a>>,
    }

    use std::collections::HashMap;
    pub fn create_graph() -> bool {

        let _hmap:HashMap<&str, &str> = HashMap::new();

        println!("generate graphs !!!");
        return true;
    }
}

#[cfg(test)]
mod tests {
    use graph_x;
    #[test]
    fn it_works() {
        assert_eq!(graph_x::create_graph(), true);
    }
}
