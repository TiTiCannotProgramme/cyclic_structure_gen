use crate::bst::avl_node::{Node, NodePtr, WeakNodePtr};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct AvlTree {
    pub root: Option<NodePtr>,
}

impl AvlTree {
    pub fn new() -> AvlTree {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, node: Node) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(node)));
            return;
        }

        let current_node_ptr = Rc::clone(self.root.as_ref().unwrap());
        let node_value = node.value;
        let leaf_node = AvlTree::find_leaf_node_for_insertion(current_node_ptr, node_value);

        let leaf_node_value: isize = leaf_node.borrow().value;
        let child_ptr = Rc::new(RefCell::new(node));
        child_ptr.borrow_mut().parent = Rc::downgrade(&leaf_node);
        let child_to_add: Option<NodePtr> = Some(Rc::clone(&child_ptr));
        if node_value < leaf_node_value {
            leaf_node.borrow_mut().left_child = child_to_add;
        } else {
            leaf_node.borrow_mut().right_child = child_to_add;
        }
        leaf_node.borrow_mut().set_height();
        self.update_balance(leaf_node);
    }

    fn find_leaf_node_for_insertion(
        current_node_ptr: NodePtr,
        insert_node_value: isize,
    ) -> NodePtr {
        let mut x = current_node_ptr;
        loop {
            let current_value: isize = x.borrow().value;
            if insert_node_value < current_value {
                let left_child = x.borrow().get_left_child();
                match left_child {
                    Some(node_ptr) => x = node_ptr,
                    None => break x,
                }
            } else {
                let right_child = x.borrow().get_right_child();
                match right_child {
                    Some(node_ptr) => x = node_ptr,
                    None => break x,
                }
            }
        }
    }

    fn update_balance(&mut self, node_ptr: NodePtr) {
        let parent_ptr_option = node_ptr.borrow().parent.upgrade();

        if node_ptr.borrow().get_balance_factor() < -1 || node_ptr.borrow().get_balance_factor() > 1
        {
            self.rebalance(node_ptr);
        }

        if parent_ptr_option.is_some() {
            let parent_ptr = parent_ptr_option.unwrap();

            let old_height = parent_ptr.borrow().height;
            parent_ptr.borrow_mut().set_height();
            let new_height = parent_ptr.borrow().height;

            if old_height != new_height {
                self.update_balance(parent_ptr);
            }
        }
    }

    fn rebalance(&mut self, node_ptr: NodePtr) {
        if node_ptr.borrow().get_balance_factor() > 0 {
            if node_ptr
                .borrow()
                .get_right_child()
                .unwrap()
                .borrow()
                .get_balance_factor()
                < 0
            {
                let right_child_ptr = node_ptr.borrow().get_right_child().unwrap();
                self.right_rotate(right_child_ptr);
                self.left_rotate(node_ptr);
            } else {
                self.left_rotate(node_ptr);
            }
        } else {
            if node_ptr
                .borrow()
                .get_left_child()
                .unwrap()
                .borrow()
                .get_balance_factor()
                > 0
            {
                let left_child_ptr = node_ptr.borrow().get_left_child().unwrap();
                self.left_rotate(left_child_ptr);
                self.right_rotate(node_ptr);
            } else {
                self.right_rotate(node_ptr);
            }
        }
    }

    pub fn left_rotate(&mut self, node_ptr: NodePtr) {
        let old_right_child_ptr = node_ptr.borrow().get_right_child().unwrap();
        let new_right_child_ptr_option = old_right_child_ptr.borrow().get_left_child();
        if new_right_child_ptr_option.is_some() {
            new_right_child_ptr_option
                .as_ref()
                .unwrap()
                .borrow_mut()
                .parent = Rc::downgrade(&node_ptr);
        }
        node_ptr.borrow_mut().right_child = new_right_child_ptr_option;

        let old_parent = node_ptr.borrow().parent.upgrade();
        if old_parent.is_none() {
            old_right_child_ptr.borrow_mut().parent = Weak::new();
            self.root = Some(Rc::clone(&old_right_child_ptr));
        } else if node_ptr.borrow().is_left_child() {
            old_parent.as_ref().unwrap().borrow_mut().left_child =
                Some(Rc::clone(&old_right_child_ptr));
            old_right_child_ptr.borrow_mut().parent = Rc::downgrade(&old_parent.as_ref().unwrap());
        } else {
            old_parent.as_ref().unwrap().borrow_mut().right_child =
                Some(Rc::clone(&old_right_child_ptr));
            old_right_child_ptr.borrow_mut().parent = Rc::downgrade(&old_parent.as_ref().unwrap());
        }
        old_right_child_ptr.borrow_mut().left_child = Some(Rc::clone(&node_ptr));
        node_ptr.borrow_mut().parent = Rc::downgrade(&old_right_child_ptr);

        node_ptr.borrow_mut().set_height();
        old_right_child_ptr.borrow_mut().set_height();
    }

    pub fn right_rotate(&mut self, node_ptr: NodePtr) {
        let old_left_child_ptr = node_ptr.borrow().get_left_child().unwrap();
        let new_left_child_ptr_option = old_left_child_ptr.borrow().get_right_child();
        if new_left_child_ptr_option.is_some() {
            new_left_child_ptr_option
                .as_ref()
                .unwrap()
                .borrow_mut()
                .parent = Rc::downgrade(&node_ptr);
        }
        node_ptr.borrow_mut().left_child = new_left_child_ptr_option;

        let old_parent = node_ptr.borrow().parent.upgrade();

        if old_parent.is_none() {
            old_left_child_ptr.borrow_mut().parent = Weak::new();
            self.root = Some(Rc::clone(&old_left_child_ptr));
        } else if node_ptr.borrow().is_left_child() {
            old_parent.as_ref().unwrap().borrow_mut().left_child =
                Some(Rc::clone(&old_left_child_ptr));
            old_left_child_ptr.borrow_mut().parent = Rc::downgrade(&old_parent.as_ref().unwrap());
        } else {
            old_parent.as_ref().unwrap().borrow_mut().right_child =
                Some(Rc::clone(&old_left_child_ptr));
            old_left_child_ptr.borrow_mut().parent = Rc::downgrade(&old_parent.as_ref().unwrap());
        }
        old_left_child_ptr.borrow_mut().right_child = Some(Rc::clone(&node_ptr));
        node_ptr.borrow_mut().parent = Rc::downgrade(&old_left_child_ptr);

        node_ptr.borrow_mut().set_height();
        old_left_child_ptr.borrow_mut().set_height();
    }

    pub fn search_tree_by_value(&self, value: isize) -> Option<NodePtr> {
        if self.root.is_none() {
            return None;
        }

        let mut current_node_ptr = Rc::clone(self.root.as_ref().unwrap());
        let result = loop {
            if value < current_node_ptr.borrow().value {
                let left_child_ptr_option = current_node_ptr.borrow().get_left_child();
                match left_child_ptr_option {
                    Some(left_child_ptr) => current_node_ptr = left_child_ptr,
                    None => break None,
                }
            } else if value > current_node_ptr.borrow().value {
                let right_child_ptr_option = current_node_ptr.borrow().get_right_child();
                match right_child_ptr_option {
                    Some(right_child_ptr) => current_node_ptr = right_child_ptr,
                    None => break None,
                }
            } else {
                break Some(current_node_ptr);
            }
        };
        result
    }

    pub fn get_minimum_child(node_ptr: NodePtr) -> NodePtr {
        let mut current_node_ptr = node_ptr;
        loop {
            let left_child_ptr_option = current_node_ptr.borrow().get_left_child();
            match left_child_ptr_option {
                Some(left_child_ptr) => current_node_ptr = left_child_ptr,
                None => break,
            }
        }
        current_node_ptr
    }

    pub fn get_maximum_child(node_ptr: NodePtr) -> NodePtr {
        let mut current_node_ptr = node_ptr;
        loop {
            let right_child_ptr_option = current_node_ptr.borrow().get_right_child();
            match right_child_ptr_option {
                Some(right_child_ptr) => current_node_ptr = right_child_ptr,
                None => break,
            }
        }
        current_node_ptr
    }

    fn delete_root(&mut self) {
        let root_ptr = Rc::clone(self.root.as_ref().unwrap());
        if root_ptr.borrow().has_no_child() {
            self.root = None;
        } else if root_ptr.borrow().has_only_left_child() {
            let left_child = root_ptr.borrow().get_left_child().unwrap();
            left_child.borrow_mut().parent = Weak::new();
            self.root = Some(left_child);
        } else if root_ptr.borrow().has_only_right_child() {
            let right_child = root_ptr.borrow().get_right_child().unwrap();
            right_child.borrow_mut().parent = Weak::new();
            self.root = Some(right_child);
        } else {
            self.delete_node_with_children(root_ptr);
        }
    }

    fn delete_node_with_children(&mut self, node_ptr: NodePtr) {
        let min_node = AvlTree::get_minimum_child(node_ptr.borrow().get_right_child().unwrap());
        node_ptr.borrow_mut().value = min_node.borrow().value;
        node_ptr.borrow_mut().index = min_node.borrow().index;
        self.delete_node(min_node);
    }

    // node_ptr must refers to a valid node in the tree
    pub fn delete_node(&mut self, node_ptr: NodePtr) {
        if self
            .root
            .as_ref()
            .unwrap()
            .borrow()
            .same_structure_to(&node_ptr)
        {
            self.delete_root();
            return;
        }

        if node_ptr.borrow().has_both_children() {
            self.delete_node_with_children(node_ptr);
            return;
        }

        // Here we know the node_ptr is not the root, so the parent can not be None
        let parent = node_ptr.borrow().parent.upgrade().unwrap();
        // Here we know the node_ptr can have at most one child
        if node_ptr.borrow().has_no_child() {
            AvlTree::set_new_child(&parent, node_ptr, None);
        } else if node_ptr.borrow().has_only_left_child() {
            let node_ptr_left_child = node_ptr.borrow().get_left_child();
            AvlTree::set_new_child(&parent, node_ptr, node_ptr_left_child);
        } else {
            let node_ptr_right_child = node_ptr.borrow().get_right_child();
            AvlTree::set_new_child(&parent, node_ptr, node_ptr_right_child);
        }
        parent.borrow_mut().set_height();
        self.update_balance(parent);
    }

    fn set_new_child(parent: &NodePtr, old_child: NodePtr, new_child: Option<NodePtr>) {
        if new_child.is_some() {
            new_child.as_ref().unwrap().borrow_mut().parent = Rc::downgrade(parent);
        }
        if old_child.borrow().is_left_child() {
            parent.borrow_mut().left_child = new_child;
        } else {
            parent.borrow_mut().right_child = new_child;
        }
    }
}

pub fn convert_node_to_vec(node_ptr: NodePtr) -> VecDeque<isize> {
    let mut value_vec: VecDeque<isize> = VecDeque::new();
    let mut node_vec: VecDeque<NodePtr> = VecDeque::from(vec![node_ptr]);

    while node_vec.len() > 0 {
        let node_ptr = node_vec.pop_front().unwrap();
        value_vec.push_back(node_ptr.borrow().value);
        if let Some(left_child_ptr) = node_ptr.borrow().get_left_child() {
            node_vec.push_back(left_child_ptr);
        };
        if let Some(right_child_ptr) = node_ptr.borrow().get_right_child() {
            node_vec.push_back(right_child_ptr);
        };
    }

    value_vec
}

pub fn build_free_from_values(values_list: Vec<isize>) -> AvlTree {
    let mut tree = AvlTree::new();
    for value in values_list.iter() {
        tree.insert(Node::new(0, *value));
    }
    tree
}

pub fn build_free_from_index_and_values(
    values_list: Vec<isize>,
    index_list: Vec<isize>,
) -> AvlTree {
    if values_list.len() != index_list.len() {
        panic!("index list and value list must have the same length");
    }
    let mut tree = AvlTree::new();
    for (index, value) in index_list.iter().zip(values_list.iter()) {
        tree.insert(Node::new(*index, *value));
    }
    tree
}
