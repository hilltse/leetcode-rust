use std::collections::HashSet;
use crate::types::tree::*;

#[derive(Default)]
pub struct Finder {
    seen: HashSet<i32>,
    target: i32,
}

impl Finder {
    pub fn new(target: i32) -> Self {
        Self {
            seen: HashSet::new(),
            target,
        }
    }

    pub fn find(&mut self, root: &Tree) -> bool {
        match root {
            Some(n) => {
                if self.seen.contains(&(self.target - n.key)) {
                    return true;
                }
                self.seen.insert(n.key);
                self.find(&n.left) || self.find(&n.right)
            }
            None => false,
        }
    }
}

pub fn two_sum_bst(root: &Tree, target: i32) -> bool {
    Finder::new(target).find(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc653() {
        let tree = deserialize("5/3/2/n/n/4/n/n/6/n/7/n/n/");
        assert!(two_sum_bst(&tree, 9));
    }
}
