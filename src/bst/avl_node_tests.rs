#[cfg(test)]
mod tests {
    use super::super::avl_node::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn setup_node1() -> Node {
        Node::new(10, 20)
    }

    fn setup_node2() -> Node {
        let left_node = Rc::new(RefCell::new(Node::new(5, 10)));
        let right_node = Rc::new(RefCell::new(setup_node1()));
        let mut my_node = Node::new(1, 15);
        my_node.left_child = Some(Rc::clone(&left_node));
        my_node.right_child = Some(Rc::clone(&right_node));
        my_node
    }

    fn setup_node3() -> Node {
        let right_node = Rc::new(RefCell::new(setup_node1()));
        let mut my_node = Node::new(1, 15);
        my_node.right_child = Some(Rc::clone(&right_node));
        my_node
    }

    fn setup_node4() -> NodePtr {
        let left_node = Rc::new(RefCell::new(Node::new(5, 10)));
        let right_node = Rc::new(RefCell::new(setup_node1()));
        let mut my_node = Node::new(1, 15);
        my_node.left_child = Some(Rc::clone(&left_node));
        my_node.right_child = Some(Rc::clone(&right_node));
        let my_new_node = Rc::new(RefCell::new(my_node));
        left_node.borrow_mut().parent = Rc::downgrade(&my_new_node);
        right_node.borrow_mut().parent = Rc::downgrade(&my_new_node);
        my_new_node
    }

    #[test]
    fn test_node1_fields() {
        let node = setup_node1();
        assert_eq!(node.index, 10);
        assert_eq!(node.value, 20);
        assert!(node.parent.upgrade().is_none());
        assert!(node.left_child.is_none());
        assert!(node.right_child.is_none());
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_node1_equal_self() {
        assert_eq!(setup_node1(), setup_node1());
    }

    #[test]
    fn test_node1_not_equal_node2() {
        assert_ne!(setup_node1(), setup_node2());
    }

    #[test]
    fn test_node1_height() {
        let mut node = setup_node1();
        node.set_height();
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_node2_left_child() {
        let node = setup_node2();
        assert_eq!(node.left_child.as_ref().unwrap().borrow().value, 10);
        assert_eq!(node.left_child.as_ref().unwrap().borrow().index, 5);
    }

    #[test]
    fn test_node2_height() {
        let mut node = setup_node2();
        node.set_height();
        assert_eq!(node.height, 2);
    }

    #[test]
    fn test_node2_balance_factor() {
        let node = setup_node2();
        assert_eq!(node.get_balance_factor(), 0);
    }

    #[test]
    fn test_node3_balance_factor() {
        let node = setup_node3();
        assert_eq!(node.get_balance_factor(), 1);
    }

    #[test]
    fn test_node4_left_child() {
        let node = setup_node4();
        let left_child_ptr = node.borrow().get_left_child().unwrap();
        assert!(left_child_ptr.borrow().is_left_child());
    }
}
