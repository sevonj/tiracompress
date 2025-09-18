use super::HuffmanCode;

use std::collections::HashMap;
use std::io::Read;

/// The node struct used during tree construction.
#[derive(Debug)]
pub struct HuffmanTreeNode {
    // Box puts the wrapped type in heap.
    // Because a struct can't contain itself.
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    freq: u32,
    value: u8,
}

impl HuffmanTreeNode {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn new(value: u8) -> Self {
        Self {
            left: None,
            right: None,
            freq: 0,
            value,
        }
    }

    fn with_freq(value: u8, freq: u32) -> Self {
        Self {
            left: None,
            right: None,
            freq,
            value,
        }
    }

    /// Generate a tree from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let nodes = collect_nodes(reader)?;
        Ok(Self::build_tree(nodes))
    }

    fn build_tree(mut nodes: Vec<HuffmanTreeNode>) -> Self {
        while nodes.len() > 1 {
            join_nodes(&mut nodes);
        }
        nodes.pop().unwrap()
    }

    /// Turn the tree into a lookup table
    pub fn into_codes(self) -> HashMap<u8, HuffmanCode> {
        let mut map = HashMap::new();
        let mut stack = vec![(0, 0, self)]; // tuple: (depth, code, node)

        while let Some((depth, code, mut node)) = stack.pop() {
            if node.is_leaf() {
                let v = HuffmanCode::new(depth, code);
                println!("{:#x} => {v}", node.value);
                map.insert(node.value, v);
                continue;
            }

            // println!("{code:0fill$b}  ", fill = depth as usize);

            if let Some(right) = node.right.take() {
                let new_code = code << 1;
                stack.push((depth + 1, new_code, *right));
            }
            if let Some(left) = node.left.take() {
                let new_code = (code << 1) + 1;
                stack.push((depth + 1, new_code, *left));
            }
        }

        map
    }
}

/// Joins two bottom nodes into a new parent node. Right >= Left.  
fn join_nodes(nodes: &mut Vec<HuffmanTreeNode>) {
    nodes.sort_unstable_by(|a, b| b.freq.cmp(&a.freq));

    let l = Box::new(nodes.pop().unwrap());
    let r = Box::new(nodes.pop().unwrap());
    let freq = l.freq + r.freq;

    nodes.push(HuffmanTreeNode {
        left: Some(l),
        right: Some(r),
        freq,
        value: 0,
    });
}

/// Iterates over the bytes and initializes a node for each unique value.
fn collect_nodes<R: Read>(reader: &mut R) -> Result<Vec<HuffmanTreeNode>, std::io::Error> {
    let mut nodes: HashMap<u8, HuffmanTreeNode> = HashMap::new();
    for byte in reader.bytes() {
        let byte = byte?; // The question mark operator immediately returns here if IO operation failed.
        nodes.entry(byte).or_insert(HuffmanTreeNode::new(byte)).freq += 1;
    }
    let nodes: Vec<HuffmanTreeNode> = nodes.into_values().collect();
    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::*;

    /// Shortcut for creating a reader and calling [collect_nodes()]
    #[macro_export]
    macro_rules! bytes_to_nodes {
        ( $x:expr  ) => {{
            let mut reader = BufReader::new(Cursor::new($x));
            collect_nodes(&mut reader)
        }};
    }

    #[test]
    fn test_collect_nodes() {
        let bytes = [11, 11, 11, 11, 3, 3];
        let nodes = bytes_to_nodes!(bytes).unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes.iter().find(|n| n.value == 11).unwrap().freq, 4);
        assert_eq!(nodes.iter().find(|n| n.value == 3).unwrap().freq, 2);

        let bytes = [11, 33, 11, 22, 3, 4, 5, 0, 0, 0];
        let nodes = bytes_to_nodes!(bytes).unwrap();
        assert_eq!(nodes.len(), 7);
        assert_eq!(nodes.iter().find(|n| n.value == 11).unwrap().freq, 2);
        assert_eq!(nodes.iter().find(|n| n.value == 33).unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == 22).unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == 3).unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == 4).unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == 5).unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == 0).unwrap().freq, 3);
    }

    #[test]
    fn test_collect_nodes_2() {
        let bytes = b"What a nice day to write unit tests.";
        let nodes = bytes_to_nodes!(bytes).unwrap();
        assert_eq!(nodes.len(), 17);
        assert_eq!(nodes.iter().find(|n| n.value == b' ').unwrap().freq, 7);
        assert_eq!(nodes.iter().find(|n| n.value == b't').unwrap().freq, 6);
        assert_eq!(nodes.iter().find(|n| n.value == b'a').unwrap().freq, 3);
        assert_eq!(nodes.iter().find(|n| n.value == b'i').unwrap().freq, 3);
        assert_eq!(nodes.iter().find(|n| n.value == b'e').unwrap().freq, 3);
        assert_eq!(nodes.iter().find(|n| n.value == b'n').unwrap().freq, 2);
        assert_eq!(nodes.iter().find(|n| n.value == b's').unwrap().freq, 2);
        assert_eq!(nodes.iter().find(|n| n.value == b'h').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'c').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'd').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'y').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'o').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'r').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'u').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'w').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'W').unwrap().freq, 1);
        assert_eq!(nodes.iter().find(|n| n.value == b'.').unwrap().freq, 1);
    }

    #[test]
    fn test_join_nodes() {
        let a = HuffmanTreeNode::with_freq(1, 5);
        let b = HuffmanTreeNode::with_freq(2, 5);
        let c = HuffmanTreeNode::with_freq(3, 2);
        let d = HuffmanTreeNode::with_freq(4, 2);
        let e = HuffmanTreeNode::with_freq(5, 2);

        // freqs: [5,5,2,2,2]
        let mut nodes = vec![a, b, c, d, e];

        // freqs: [5,5,4,2]
        join_nodes(&mut nodes);
        assert_eq!(nodes.len(), 4);
        let merged = nodes.iter().find(|n| n.value == 0).unwrap();
        assert_eq!(merged.freq, 4);
        assert_eq!(merged.left.as_ref().unwrap().freq, 2);
        assert_eq!(merged.right.as_ref().unwrap().freq, 2);
        assert_eq!(nodes.iter().find(|n| n.value == 1).unwrap().freq, 5);
        assert_eq!(nodes.iter().find(|n| n.value == 2).unwrap().freq, 5);

        // freqs: [6,5,5]
        join_nodes(&mut nodes);
        assert_eq!(nodes.len(), 3);
        let merged = nodes.iter().find(|n| n.value == 0).unwrap();
        assert_eq!(merged.freq, 6);
        assert_eq!(merged.left.as_ref().unwrap().freq, 2);
        assert_eq!(merged.right.as_ref().unwrap().freq, 4);
        assert_eq!(nodes.iter().find(|n| n.value == 1).unwrap().freq, 5);
        assert_eq!(nodes.iter().find(|n| n.value == 2).unwrap().freq, 5);

        // freqs: [10,6]
        join_nodes(&mut nodes);
        assert_eq!(nodes.len(), 2);
        nodes.sort_unstable_by(|a, b| b.freq.cmp(&a.freq));
        assert_eq!(nodes[0].freq, 10);
        assert_eq!(nodes[1].freq, 6);

        // freqs: [16]
        join_nodes(&mut nodes);
        assert_eq!(nodes.len(), 1);
        nodes.sort_unstable_by(|a, b| b.freq.cmp(&a.freq));
        assert_eq!(nodes[0].freq, 16);

        /* The tree should look exactly like this:
            Root(16)
                -R:(10)
                    -R:[5]
                    -L:[5]
                -L:(6)
                    -R:(4)
                        -R:[2]
                        -L:[2]
                    -L:[2]

            legend: (node), [leaf]
        */
    }

    #[test]
    fn test_build_tree() {
        let a = HuffmanTreeNode::with_freq(1, 5);
        let b = HuffmanTreeNode::with_freq(2, 5);
        let c = HuffmanTreeNode::with_freq(3, 2);
        let d = HuffmanTreeNode::with_freq(4, 2);
        let e = HuffmanTreeNode::with_freq(5, 2);

        let nodes = vec![a, b, c, d, e];
        let root = HuffmanTreeNode::build_tree(nodes);

        /* The tree should look exactly like this:
            Root(16)
                -R:(10)
                    -R:[5]
                    -L:[5]
                -L:(6)
                    -R:(4)
                        -R:[2]
                        -L:[2]
                    -L:[2]

            legend: (node), [leaf]
        */

        assert_eq!(root.freq, 16);
        assert_eq!(root.right.as_ref().unwrap().freq, 10);
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().freq, 5);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().freq, 5);
        assert_eq!(root.left.as_ref().unwrap().freq, 6);
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().freq, 4);
        assert_eq!(
            root.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .freq,
            2
        );
        assert_eq!(
            root.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .freq,
            2
        );
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().freq, 2);
    }
}
