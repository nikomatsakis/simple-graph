pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new(),
                edges: Vec::new(), }
    }

    fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}


mod test {
    use super::*;

    #[test]
    fn example() {

        // N0 ---E0---> N1 ---E1---> 2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let mut graph = Graph::new();

        let n0 = graph.add_node();
        let n1 = graph.add_node();
        let n2 = graph.add_node();
        let n3 = graph.add_node();

        graph.add_edge(n0, n1); // e0
        graph.add_edge(n1, n2); // e1
        graph.add_edge(n0, n3); // e2
        graph.add_edge(n3, n2); // e3

        let successors: Vec<NodeIndex> = graph.successors(n0).collect();
        assert_eq!(&successors[..], &[n3, n1]);
    }
}
