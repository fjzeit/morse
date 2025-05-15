use super::morse;

#[derive(Debug)]
struct SearchNode {
    value: Option<char>,
    dit_node: Option<Box<SearchNode>>,
    dah_node: Option<Box<SearchNode>>,
}

impl SearchNode {
    fn new() -> Self {
        SearchNode {
            value: None,
            dit_node: None,
            dah_node: None,
        }
    }

    fn set_value(&mut self, value: char) {
        self.value = Some(value);
    }
    fn add_dit(&mut self) {
        self.dit_node = Some(Box::new(SearchNode::new()));
    }

    fn add_dah(&mut self) {
        self.dah_node = Some(Box::new(SearchNode::new()));
    }

    fn next(&self, c: char) -> &Option<Box<SearchNode>> {
        match c {
            c @ (morse::DIT | morse::DAH) => {
                if c == morse::DIT {
                    &self.dit_node
                } else {
                    &self.dah_node
                }
            }
            _ => &None,
        }
    }
}

// Construct a dichotomic search tree to be used for morse-to-character decoding.
fn build_tree() -> SearchNode {
    let mut root = SearchNode::new();

    for n in morse::MORSE_TABLE.iter() {
        let mut node = &mut root;
        if n.1.contains(morse::MEDIUM_GAP) || n.1.contains(morse::SHORT_GAP) {
            continue;
        }
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

    for c in morse.chars() {
        match c {
            morse::MEDIUM_GAP => {
                node.value.map(|v| result.push(v));
                result.push(' ')
            }
            morse::SHORT_GAP => {
                node.value.map(|v| result.push(v));
            }
            _ => {}
        }

        node = if let Some(next) = node.next(c).as_ref() {
            next
        } else {
            &tree
        }
    }

    node.value.map(|v| result.push(v));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest] // ensure the lengths in the tuple match the length of the string
    #[case("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789&'@)(:,=!.-+\"?/", "")]
    #[case("%", "0/0")]
    fn bounce_test(#[case] test_data: &str, #[case] expected: &str) {
        let expected: &str = if expected.len() == 0 {
            test_data
        } else {
            expected
        };
        let intermediate = crate::encoding::encode::to_morse(expected).unwrap();
        let actual = to_text(&intermediate);
        assert_eq!(expected, actual);
    }
}
