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

    pub fn same_structure_to(&self, node_ptr: &NodePtr) -> bool {
        let same_index_value =
            self.index == node_ptr.borrow().index && self.value == node_ptr.borrow().value;
        let same_children_structure = self.get_left_child().is_none()
            == node_ptr.borrow().get_left_child().is_none()
            && self.get_right_child().is_none() == node_ptr.borrow().get_right_child().is_none();
        let same_parent_structure = self.parent.upgrade().is_none() == node_ptr.borrow().parent.upgrade().is_none();
        same_index_value && same_children_structure && same_parent_structure
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

    pub fn has_both_children(&self) -> bool {
        self.left_child.is_some() && self.right_child.is_some()
    }

    pub fn has_no_child(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }

    pub fn has_only_left_child(&self) -> bool {
        !self.has_both_children() && self.left_child.is_some()
    }

    pub fn has_only_right_child(&self) -> bool {
        !self.has_both_children() && self.right_child.is_some()
    }
}
