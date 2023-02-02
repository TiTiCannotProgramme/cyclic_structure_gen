use std::cell::RefCell;
use std::cmp::max;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub parent: WeakNodePtr,
    pub index: isize,
    pub value: isize,
    pub height: isize,
    pub left_child: Option<NodePtr>,
    pub right_child: Option<NodePtr>,
}

pub type NodePtr = Rc<RefCell<Node>>;
pub type WeakNodePtr = Weak<RefCell<Node>>;

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.index == other.index
            && self.value == other.value
            && self.left_child == other.left_child
            && self.right_child == other.right_child
    }
}

impl Node {
    pub fn new(index: isize, value: isize) -> Node {
        Node {
            parent: Weak::new(),
            index,
            value,
            height: 1,
            left_child: None,
            right_child: None,
        }
    }

    fn same_index_value_to(&self, node_ptr: &NodePtr) -> bool {
        self.index == node_ptr.borrow().index && self.value == node_ptr.borrow().value
    }

    fn get_left_child_height(&self) -> isize {
        match &self.left_child {
            Some(refcell) => refcell.borrow().height,
            None => 0,
        }
    }

    fn get_right_child_height(&self) -> isize {
        match &self.right_child {
            Some(refcell) => refcell.borrow().height,
            None => 0,
        }
    }

    pub fn get_balance_factor(&self) -> isize {
        self.get_right_child_height() - self.get_left_child_height()
    }

    pub fn set_height(&mut self) {
        self.height = max(self.get_right_child_height(), self.get_left_child_height()) + 1;
    }

    pub fn get_value(&self) -> isize {
        self.value
    }

    pub fn get_left_child(&self) -> Option<NodePtr> {
        match &self.left_child {
            Some(node_ptr) => Some(Rc::clone(node_ptr)),
            None => None,
        }
    }

    pub fn get_right_child(&self) -> Option<NodePtr> {
        match &self.right_child {
            Some(node_ptr) => Some(Rc::clone(node_ptr)),
            None => None,
        }
    }

    pub fn is_left_child(&self) -> bool {
        let parent_ptr_option = self.parent.upgrade();
        match parent_ptr_option {
            Some(parent_ptr) => {
                let left_child_ptr_option = parent_ptr.borrow().get_left_child();
                match left_child_ptr_option {
                    Some(left_child_ptr) => left_child_ptr.borrow().eq(self),
                    None => false,
                }
            }
            None => true,
        }
    }

    pub fn is_right_child(&self) -> bool {
        !self.is_left_child()
    }
}
