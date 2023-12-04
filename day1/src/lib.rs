#[derive(Debug)]
pub struct Node {
    char: char,
    nodes: Vec<Node>,
    value: Option<u8>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            char: '*',
            nodes: Vec::new(),
            value: None,
        }
    }
}

#[derive(Debug)]
pub enum Find<'a> {
    Complete(u8),
    Partial(&'a Node),
    NoMatch,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        Self {
            char: c,
            ..Default::default()
        }
    }
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.char == '*'
    }
    pub fn new_tree<'a>(words: impl IntoIterator<Item = (&'a str, u8)>) -> Node {
        let mut root = Node::default();
        for (word, value) in words {
            root.insert(word.chars(), value)
        }

        root
    }
    fn insert(&mut self, word: impl IntoIterator<Item = char>, value: u8) {
        let mut word = word.into_iter();
        match word.next() {
            Some(c) => {
                let find = self.nodes.iter_mut().find(|node| node.char == c);
                match find {
                    Some(node) => node.insert(word, value),
                    None => {
                        let mut new_node: Node = c.into();
                        new_node.insert(word, value);
                        self.nodes.push(new_node)
                    }
                };
            }
            _ => self.value = Some(value),
        };
    }

    pub fn decend(&self, c: char) -> Find {
        let find = self.nodes.iter().find(|node| node.char == c);
        match find {
            Some(node) => match node.value {
                Some(v) => Find::Complete(v),
                None => Find::Partial(node),
            },
            None => Find::NoMatch,
        }
    }

    pub fn find(&self, word: impl IntoIterator<Item = char>) -> Find {
        let mut word = word.into_iter();
        match word.next() {
            Some(c) => {
                let find = self.nodes.iter().find(|node| node.char == c);
                match find {
                    Some(node) => node.find(word),
                    None => Find::NoMatch,
                }
            }

            None => match self.value {
                Some(v) => Find::Complete(v),
                None => Find::Partial(self),
            },
        }
    }
}
