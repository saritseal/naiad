use edge::Edge;
//use std::collections::VecDeque;

pub struct AdjacencyList <'a>{
    edge_list : Vec<Edge<'a>>,

}

impl <'a> AdjacencyList<'a>{
    pub fn new() -> AdjacencyList<'a>{
        let vec = Vec::new();
        //vec.append(Edge::new());

        return AdjacencyList{edge_list: vec};
    }
}