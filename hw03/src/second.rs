use std::vec;

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

impl<T> IntoIterator for BST<T> where T: PartialOrd + Ord {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values: Vec<T> = Vec::new();
        let mut stack: Vec<Link<T>> = Vec::new();
        stack.push(self.root);

        loop {
            match stack.pop() {
                Some(Some(boxed)) => {
                    values.push(boxed.value);
                    stack.push(boxed.left);
                    stack.push(boxed.right);
                },
                Some(None) => continue,
                None => break,
            } 
        }
        values.sort_unstable();
        values.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> where T: PartialOrd + Ord {
    type Item = &'a mut T;
    type IntoIter = vec::IntoIter<&'a mut T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values: Vec<&'a mut T> = Vec::new();
        let mut stack: Vec<&'a mut Link<T>> = Vec::new();
        stack.push(&mut self.root);

        loop {
            match stack.pop() {
                Some(&mut Some(ref mut boxed)) => {
                    values.push(&mut boxed.value);
                    stack.push(&mut boxed.left);
                    stack.push(&mut boxed.right);
                },
                Some(&mut None) => continue,
                None => break,
            } 
        }
        values.sort_unstable();
        values.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a BST<T> where T: PartialOrd + Ord {
    type Item = &'a T;
    type IntoIter = vec::IntoIter<&'a T>;
    
    fn into_iter(self) -> Self::IntoIter {
        let mut values: Vec<&'a T>  = Vec::new();
        let mut stack: Vec<&'a Link<T>> = Vec::new();
        stack.push(&self.root);

        loop {
            match stack.pop() {
                Some(&Some(ref boxed)) => {
                    values.push(&boxed.value);
                    stack.push(&boxed.left);
                    stack.push(&boxed.right);
                },
                Some(&None) => continue,
                None => break,
            } 
        }
        values.sort_unstable();
        values.into_iter()
    }
}

mod tests {
    #![cfg(test)]
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_empty() {
        let _bst: BST<i32> = Default::default();
    }

    #[test]
    fn test_insert() {
        let mut bst: BST<i32> = Default::default();
        for i in 0..10000 {
            bst.insert(i);
        }
    }

    #[test]
    fn test_search() {
        let mut bst: BST<i32> = Default::default();
        for i in 0..10000 {
            bst.insert(i);
            assert_eq!(bst.search(&i), true);
        }
    }

    #[test]
    fn test_negative() {
        let mut bst: BST<i32> = Default::default();
        for i in 0..10000 {
            bst.insert(i);
            assert_eq!(bst.search(&(i + 1)), false);
        }
    }

    #[test]
    fn test_into_iter() {
        let mut bst: BST<i32> = Default::default(); 
        let mut vec: Vec<i32> = (0..100000).collect();
        let slice = vec.as_mut_slice();
        thread_rng().shuffle(slice);

        for i in vec.into_iter() {
            bst.insert(i);
        }
        let mut n = 0;
        for i in bst.into_iter() {
            assert_eq!(i, n);
            n += 1;
        }
    }

    #[test]
    fn test_into_iter_ref() {
        let mut bst: BST<i32> = Default::default(); 
        let mut vec: Vec<i32> = (0..100000).collect();
        let slice = vec.as_mut_slice();
        thread_rng().shuffle(slice);

        for i in vec.into_iter() {
            bst.insert(i);
        }
        let mut n = 0;
        for i in (&bst).into_iter() {
            assert_eq!(*i, n);
            n += 1;
        }
    }

    #[test]
    fn test_into_iter_mut() {
        let mut bst: BST<i32> = Default::default(); 
        let mut vec: Vec<i32> = (0..100000).collect();
        let slice = vec.as_mut_slice();
        thread_rng().shuffle(slice);

        for i in vec.into_iter() {
            bst.insert(i);
        }
        let mut n = 0;
        for i in (&mut bst).into_iter() {
            assert_eq!(*i, n);
            *i = *i + 10;
            n += 1;
        }

        n = 10;
        for i in (&bst).into_iter() {
            assert_eq!(*i, n);
            n += 1;
        }
    }
}
