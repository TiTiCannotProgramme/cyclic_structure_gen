use crate::bst::avl_node::{Node, NodePtr};
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

        self.update_balance(child_ptr);
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
            if node_ptr.borrow().get_right_child().unwrap().borrow().get_balance_factor() < 0 {
                self.right_rotate(node_ptr.borrow().get_right_child().unwrap());
                self.left_rotate(node_ptr)
            } else {
                self.left_rotate(node_ptr)
            }
        } else {
            if node_ptr.borrow().get_left_child().unwrap().borrow().get_balance_factor() > 0 {
                self.left_rotate(node_ptr.borrow().get_left_child().unwrap());
                self.right_rotate(node_ptr)
            } else {
                self.right_rotate(node_ptr)
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
        } else {
            old_parent.as_ref().unwrap().borrow_mut().right_child =
                Some(Rc::clone(&old_right_child_ptr));
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
        } else {
            old_parent.as_ref().unwrap().borrow_mut().right_child =
                Some(Rc::clone(&old_left_child_ptr));
        }
        old_left_child_ptr.borrow_mut().right_child = Some(Rc::clone(&node_ptr));
        node_ptr.borrow_mut().parent = Rc::downgrade(&old_left_child_ptr);

        node_ptr.borrow_mut().set_height();
        old_left_child_ptr.borrow_mut().set_height();
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
