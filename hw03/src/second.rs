#[derive(Default, Debug)]
pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Clone, Copy)]
enum Way { Left, Right }

impl<T> Node<T> where T: PartialOrd + Ord {
    pub fn new(value: T) -> Self {
        Node { value, left: None, right: None } 
    }

    pub fn insert(&mut self, way: Way, value: T) {
        let node = Some(Box::new(Node::new(value)));
        match way {
            Way::Left => self.left = node,
            Way::Right => self.right = node,
        }
    }
}

impl<T> BST<T> where T: PartialOrd + Ord {
    pub fn insert(&mut self, value: T) -> bool {
        let mut node = &mut self.root;
        if node.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            return true;
        }

        while let Some(ref mut next) = *node {
            if next.value < value {
                match next.left {
                    Some(_) => node = &mut next.left,
                    None    => {
                        next.insert(Way::Left, value);
                        return true;
                    }
                }
            } else if next.value > value {
                match next.right {
                    Some(_) => node = &mut next.right,
                    None    => {
                        next.insert(Way::Right, value);
                        return true;
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn search(&self, value: &T) -> bool {
        let mut node = &self.root; 
        while let Some(ref boxed) = *node {
            let next = &(*boxed);
            if next.value < *value {
                node = &next.left; 
            } else if next.value > *value {
                node = &next.right; 
            } else {
                return true; 
            }
        }
        false
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
