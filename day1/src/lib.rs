use std::{char, collections::VecDeque, fmt::Display, iter};

// FIXME expensive cloning

#[derive(Debug, Clone)]
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

fn write_nodes<'a>(
    nodes: impl Iterator<Item = &'a Node>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let mut children = Vec::new();
    for node in nodes {
        write!(f, "{}", node.char)?;
        children.extend(node.nodes.iter());
    }

    if !children.is_empty() {
        writeln!(f)?;
        write_nodes(children.into_iter(), f)?;
    };
    writeln!(f)
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes = iter::once(self);
        write_nodes(nodes, f)
    }
}

#[derive(Debug)]
pub enum Find {
    Complete(u8),
    Partial(Vec<Node>),
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

    pub fn is_leaf(&self) -> bool {
        self.value.is_some()
    }
    fn new_tree<'a>(words: impl IntoIterator<Item = (&'a str, u8)>) -> Self {
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

    fn find(&self, c: char) -> Find {
        let nodes: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|node| match node.char == c {
                true => Some(node.clone()),
                false => None,
            })
            .collect();

        if nodes.is_empty() {
            return Find::NoMatch;
        }

        let perfect_match = nodes.iter().find_map(|node| node.value);
        if let Some(v) = perfect_match {
            return Find::Complete(v);
        }

        Find::Partial(nodes)
    }
}

fn find<'a>(c: char, nodes: impl Iterator<Item = &'a Node>) -> Find {
    let mut finds = nodes.map(|node| node.find(c));
    let mut nodes: Vec<Node> = Vec::new();
    if let Some(v) = finds.find(|res| match res {
        Find::Complete(_v) => true,
        Find::Partial(inners) => {
            nodes.extend(inners.iter().cloned());
            false
        }
        _ => false,
    }) {
        return v;
    }
    if nodes.is_empty() {
        return Find::NoMatch;
    }
    Find::Partial(nodes)
}

#[derive(Default, Debug)]
pub struct Tree {
    root: Node,
    current: Vec<Node>,
    partial: VecDeque<char>,
}
impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.current.is_empty() {
            write!(f, "{}", self.root)?
        }
        write_nodes(self.current.iter(), f)
    }
}

impl Tree {
    pub fn new<'a>(words: impl Into<Box<[(&'a str, u8)]>>) -> Self {
        let words: Box<[_]> = words.into();
        let max_len = words
            .iter()
            .map(|word| word.0.len())
            .max()
            .expect("input should have atleast one word");
        let root = Node::new_tree(words.into_vec());
        Self {
            root,
            partial: VecDeque::with_capacity(max_len),
            ..Default::default()
        }
    }
    fn reset(&mut self) {
        self.current.clear();
        self.partial.clear();
    }
    pub fn decend(&mut self, c: char) -> Option<u8> {
        if self.current.is_empty() {
            self.current.push(self.root.clone());
        }
        let find = find(c, self.current.iter());
        // dbg!(&find);

        match find {
            Find::Complete(v) => {
                self.reset();
                Some(v)
            }
            Find::Partial(inner_nodes) => {
                self.partial.push_back(c);
                self.current = inner_nodes;
                None
            }
            Find::NoMatch => {
                if self.partial.is_empty() {
                    self.reset();
                    return None;
                }
                // self.partial.pop_front();
                self.partial.push_back(c);
                // dbg!(&self.partial);
                let chars: Vec<_> = self.partial.drain(..).collect();

                // dbg!(&self.partial);
                chars.into_iter().find_map(|c| self.decend(c))
            }
        }
    }

    /**
    based on spelled out words for 1-9
    */
    pub fn digits() -> Self {
        let words = digits_words();
        Tree::new(words)
    }
    /**
    based on spelled out words for 1-9
    */
    pub fn rev_digits() -> Self {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let reverse: [String; 9] = words.map(|word| word.chars().rev().collect());
        let reverse = reverse.iter().map(|word| (word.as_str()));
        let input: Box<[_]> = reverse.zip(1..10).collect();

        Tree::new(input)
    }
}

fn digits_words() -> Box<[(&'static str, u8)]> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let words: Box<_> = words.into_iter().zip(1..10).collect();
    words
}

#[cfg(test)]
mod tests {
    use crate::Tree;

    #[test]
    fn test_forward() {
        let input = "alksjdlkajsdtwo123l23jlkfour";
        let expected = vec![2, 4];
        let mut tree = Tree::digits();
        let chars = input.chars();
        let res: Vec<_> = chars.filter_map(|c| tree.decend(c)).collect();
        assert_eq!(expected, res);
    }
    #[test]
    fn test_rev() {
        let input = "onen";
        let expected = 1;
        let mut tree = Tree::rev_digits();
        println!("{}\n", tree);
        let chars = input.chars().rev();
        let mut res = None;
        let mut inserted = Vec::new();
        for (i, c) in chars.enumerate() {
            res = tree.decend(c);
            inserted.push(c);
            println!("inserted: {:?}", inserted);
            println!("partial: {:?}", tree.partial);
            print!("i{i} {tree}");
            if res.is_some() {
                break;
            }
        }
        assert_eq!(expected, res.unwrap());
    }
}
