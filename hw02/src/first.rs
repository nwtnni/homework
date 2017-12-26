#[derive(Default, Debug)]
pub struct BST {
    root: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Link,
    right: Link,
}

#[derive(Clone, Copy)]
enum Way { Left, Right }

impl Node {
    pub fn new(value: i32) -> Self {
        Node { value, left: None, right: None } 
    }

    pub fn insert(&mut self, way: Way, value: i32) {
        let node = Some(Box::new(Node::new(value)));
        match way {
            Way::Left => self.left = node,
            Way::Right => self.right = node,
        }
    }

    pub fn length(&self) -> i32 {
        let mut len = 1;
        if let Some(ref left) = self.left {
            len += left.length(); 
        }
        if let Some(ref right) = self.right {
            len += right.length(); 
        }
        len
    }
}

impl BST {
    pub fn insert(&mut self, value: i32) -> bool {
        let mut node = &mut self.root;
        if let None = *node {
            self.root = Some(Box::new(Node::new(value)));
            return true;
        }

        while let Some(ref mut next) = *node {
            if next.value < value {
                match next.left {
                    Some(_) => node = &mut next.left,
                    None    => next.insert(Way::Left, value),
                }
            } else if next.value > value {
                match next.right {
                    Some(_) => node = &mut next.right,
                    None    => next.insert(Way::Right, value),
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn search(&self, value: i32) -> bool {
        let mut node = &self.root; 
        while let Some(ref boxed) = *node {
            let next = &(*boxed);
            if next.value < value {
                node = &next.left; 
            } else if next.value > value {
                node = &next.right; 
            } else {
                return true; 
            }
        }
        false
    }

    pub fn length(&self) -> i32 {
        match self.root {
            Some(ref node) => node.length(),
            None => 0,
        }
    }
}

mod tests {
    #![cfg(test)]

    use super::*;

    #[test]
    fn test_empty() {
        let bst: BST = Default::default();
        assert_eq!(bst.length(), 0);
    }

    #[test]
    fn test_insert() {
        let mut bst: BST = Default::default(); 
        for i in 0..10000 {
            bst.insert(i);
        }
        assert_eq!(bst.length(), 10000)
    }

    #[test]
    fn test_search() {
        let mut bst: BST = Default::default();
        for i in 0..10000 {
            bst.insert(i);
            assert_eq!(bst.search(i), true);
        }
    }

    #[test]
    fn test_negative() {
        let mut bst: BST = Default::default();
        for i in 0..10000 {
            bst.insert(i);
            assert_eq!(bst.search(i + 1), false);
        }
    }
}
