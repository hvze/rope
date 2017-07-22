#[cfg(test)]
mod tests {
    use rope::{Rope, Node};
    
    #[test]
    fn rope_def_creation() {
        let rope: Rope = Default::default();
        assert_eq!(String::new(), rope.root().data());
    }

    #[test]
    fn rope_sz_creation() {
        let rope: Rope = Rope::
    }

    #[test]
    fn rope_chunk() {
        assert_eq!(vec!["ab", "cd", "e"], Node::chunk_str("abcde".to_string(), 2));
    }


}



mod rope {

    use std::default::Default;

    #[derive(Default)]
    pub struct Node {
        data:   String,
        size:   usize,
        left:   Box<Option<Node>>,
        right:  Box<Option<Node>>
    }

    impl Node {
        pub fn data(&self) -> String { self.data.clone() }
        pub fn ref_data(&self) -> &String { &self.data }
        pub fn left(&self) -> &Box<Option<Node>> { &self.left }
        pub fn right(&self) -> &Box<Option<Node>> { &self.right }
    
        pub fn new_raw(string: &str) -> Node { 
            Node::new(String::from(string))
        }


        // Forgiving Chunking..
        pub fn chunk_str(mut data: String, parts: usize) -> Vec<String> {
            let mut container: Vec<String> = Vec::new();
            while data.len() > 0 {
                if data.len() < parts {
                    container.push(data.drain(..).collect());
                } else {
                    container.push(data.drain(..parts).collect());
                }
            }
            container
        }

        pub fn new_sz(_from: String, sz: usize) -> Node {
            let rope: Rope = Rope { leaf_size: sz, .. Default::default() };
            let mut from: String = _from.clone();
            let mut nodes: Vec<Node> = Vec::new();
            for part in Node::chunk_str(from, sz) {
                println!("{}", part);
            }
            let d: Node = Default::default();
            d
        }

        pub fn new(from: String) -> Node { Node::new_sz(from, 2 as usize) }



    }

    #[derive(Default)]
    pub struct Rope {
        root:      Node,
        leaf_size: usize
    }

    impl Rope {
        pub fn root(&self) -> &Node { &self.root }
        pub fn root_mut(&self) -> &mut Node { &mut self.root }
        pub fn new(from: String) -> Rope {
            let mut rope: Rope = Rope { leaf_size: 2, .. Default::default() };
            rope.root_mut() = Node::new(from);
        }
    }

}