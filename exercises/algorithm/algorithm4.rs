/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
    where
        T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
    where
        T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
    where
        T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
    where
        T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut node = self.root.take();
        self.root = self.insertNode(node,value);
    }

    fn insertNode(&mut self, node :Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>>{
        match node {
            None => {
                Some(Box::new(TreeNode::new(value)))
            },
            Some(mut next) => {
                if value < next.value {
                    next.left = self.insertNode(next.left,value);
                }else if value > next.value {
                    next.right = self.insertNode(next.right,value);
                }
                Some(next)
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        return self.searchNode(self.root.as_ref(),value);
    }

    fn searchNode(&self, node:Option<&Box<TreeNode<T>>>, value:T) -> bool {
        match node {
            None => {
                return false;
            }
            Some(next) => {
                let mut res = false;
                if value < next.value {
                    res |= self.searchNode(next.left.as_ref(),value);
                }else if value == next.value{
                    res = true;
                }else {
                    res |= self.searchNode(next.right.as_ref(),value);
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);


        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
