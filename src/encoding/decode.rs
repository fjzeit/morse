use super::morse;

#[derive(Debug)]
struct SearchNode {
    value: char,
    dit_node: Option<Box<SearchNode>>,
    dah_node: Option<Box<SearchNode>>,
}

impl SearchNode {
    fn new() -> Self {
        SearchNode {
            value: '?',
            dit_node: None,
            dah_node: None,
        }
    }

    fn set_value(&mut self, value: char) {
        self.value = value;
    }
    fn add_dit(&mut self) {
        self.dit_node = Some(Box::new(SearchNode::new()));
    }

    fn add_dah(&mut self) {
        self.dah_node = Some(Box::new(SearchNode::new()));
    }
}

// Construct a dichotomic search tree to be used for morse-to-character decoding.
fn build_tree() -> SearchNode {
    let mut root = SearchNode::new();

    for n in morse::MORSE_TABLE.iter() {
        if n.1.contains('/') {
            continue;
        };

        let mut node = &mut root;
        for c in n.1.chars() {
            match c {
                morse::DIT => {
                    if node.dit_node.is_none() {
                        node.add_dit();
                    }
                    node = node.dit_node.as_mut().unwrap();
                }
                morse::DAH => {
                    if node.dah_node.is_none() {
                        node.add_dah();
                    }
                    node = node.dah_node.as_mut().unwrap();
                }
                _ => panic!("Unexpected character found in morse definition"),
            }
        }
        node.set_value(n.0);
    }

    root
}

pub fn to_text(morse: &str) -> String {
    let mut result = String::new();
    let tree = build_tree();
    let mut node = &tree;

    for (i, c) in morse.chars().enumerate() {
        if c == morse::MEDIUM_GAP || c == morse::SHORT_GAP {
            result.push(node.value);
        }

        node = match c {
            morse::DIT => if let Some(n) = &node.dit_node { n } else { panic!("Invalid character at {}", i)} ,
            morse::DAH => if let Some(n) = &node.dah_node { n } else { panic!("Invalid character at {}", i)} ,
            _ => &tree
        }
    }
    
    result.push(node.value);

    result
}
