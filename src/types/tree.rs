#[derive(Default)]
pub struct TreeNode {
    pub key: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_key(key: i32) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }
}

pub type Tree = Option<Box<TreeNode>>;

const SYMBOL_NULL: char = 'n';
const DELIMITER: char = '/';

pub fn serialize(root: &Tree) -> String {
    let mut result = String::new();
    encode(root, &mut result);
    result
}

fn encode(n: &Tree, result: &mut String) {
    match n {
        Some(n) => {
            result.push_str(&n.key.to_string());
            result.push(DELIMITER);
            encode(&n.left, result);
            encode(&n.right, result);
        }
        None => {
            result.push(SYMBOL_NULL);
            result.push(DELIMITER);
        }
    }
}

pub fn deserialize(code: &str) -> Tree {
    let mut tokens: Vec<_> = code.split(DELIMITER).rev().collect();
    decode(&mut tokens)
}

fn decode(tokens: &mut Vec<&str>) -> Tree {
    let cur = tokens.pop().unwrap();

    if cur == SYMBOL_NULL.to_string() {
        None
    } else {
        let mut root = Box::new(TreeNode::with_key(cur.parse().unwrap()));
        root.left = decode(tokens);
        root.right = decode(tokens);
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_ser() {
        let code = "5/3/2/n/n/4/n/n/6/n/7/n/n/";
        let tree = deserialize(code);
        let result = serialize(&tree);
        assert_eq!(code, result);
    }
}
