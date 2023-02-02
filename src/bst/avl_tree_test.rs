#[cfg(test)]
mod tests {
    use super::super::avl_node::*;
    use super::super::avl_tree::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn setup_empty_tree() -> AvlTree {
        AvlTree::new()
    }

    fn setup_tree1() -> AvlTree {
        let mut tree = AvlTree::new();
        let node = setup_node1();
        tree.insert(node);
        tree
    }

    fn setup_node1() -> Node {
        Node::new(10, 20)
    }

    fn setup_tree2() -> AvlTree {
        let mut tree = AvlTree::new();
        let node1 = setup_node1();
        let node2 = Node::new(10, 10);
        let node3 = Node::new(10, 30);
        let node4 = Node::new(10, 12);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);
        tree.insert(node4);
        tree
    }

    fn setup_tree3() -> AvlTree {
        let mut tree = AvlTree::new();
        let node1 = setup_node1();
        let node2 = Node::new(10, 30);
        let node3 = Node::new(10, 40);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);
        tree
    }

    fn setup_tree4() -> AvlTree {
        let mut tree = AvlTree::new();
        let node1 = setup_node1();
        let node2 = Node::new(10, 10);
        let node3 = Node::new(10, 5);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);
        tree
    }

    #[test]
    fn test_tree3_left_rotate() {
        let mut tree = setup_tree3();
        // tree.left_rotate(Rc::clone(&tree.root.as_ref().unwrap()));
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![30, 20, 40]));
    }

    #[test]
    fn test_tree4_right_rotate() {
        let mut tree = setup_tree4();
        // tree.right_rotate(Rc::clone(&tree.root.as_ref().unwrap()));
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![10, 5, 20]));
    }

    #[test]
    fn test_tree1_as_list() {
        let tree = setup_tree1();
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![20]));
    }

    #[test]
    fn test_tree2_as_list() {
        let tree = setup_tree2();
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![20, 10, 30, 12]));
    }

    #[test]
    fn test_empty_tree_insert() {
        let mut tree = setup_empty_tree();
        let node = setup_node1();
        tree.insert(node);
        assert_eq!(tree.root, Some(Rc::new(RefCell::new(setup_node1()))));
    }

    #[test]
    fn test_tree1_insert1() {
        let mut tree = setup_tree1();
        let node = Node::new(10, 10);
        tree.insert(node);
        let left_child = tree.root.as_ref().unwrap().borrow().get_left_child();
        assert_eq!(left_child, Some(Rc::new(RefCell::new(Node::new(10, 10)))));
    }

    #[test]
    fn test_tree1_insert1_parent() {
        let mut tree = setup_tree1();
        let node = Node::new(10, 10);
        tree.insert(node);
        let left_child = tree.root.as_ref().unwrap().borrow().get_left_child();
        let parent = left_child.unwrap().borrow().parent.upgrade();
        assert_eq!(parent, tree.root);
    }

    #[test]
    fn test_tree1_insert1_bf() {
        let mut tree = setup_tree1();
        let node = Node::new(10, 10);
        tree.insert(node);
        assert_eq!(tree.root.unwrap().borrow().get_balance_factor(), -1);
    }

    #[test]
    fn test_tree1_insert1_height() {
        let mut tree = setup_tree1();
        let node = Node::new(10, 10);
        tree.insert(node);
        assert_eq!(tree.root.unwrap().borrow().height, 2);
    }

    #[test]
    fn test_tree1_insert2() {
        let mut tree = setup_tree1();
        let node1 = Node::new(10, 10);
        let node2 = Node::new(10, 5);
        tree.insert(node1);
        tree.insert(node2);
        // let left_child_1 = tree.root.as_ref().unwrap().borrow().get_left_child();
        // let left_child_2 = left_child_1.as_ref().unwrap().borrow().get_left_child();
        // assert_eq!(left_child_2, Some(Rc::new(RefCell::new(Node::new(10, 5)))));
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![10, 5, 20]));
    }

    #[test]
    fn test_tree1_insert2_bf() {
        let mut tree = setup_tree1();
        let node1 = Node::new(10, 10);
        let node2 = Node::new(10, 5);
        tree.insert(node1);
        tree.insert(node2);
        assert_eq!(tree.root.unwrap().borrow().get_balance_factor(), 0);
    }

    #[test]
    fn test_tree1_insert3() {
        let mut tree = setup_tree1();
        let node1 = Node::new(10, 10);
        let node2 = Node::new(10, 5);
        let node3 = Node::new(10, 12);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);

        // let left_child_1 = tree.root.as_ref().unwrap().borrow().get_left_child();
        // let right_child_2 = left_child_1.as_ref().unwrap().borrow().get_right_child();
        // assert_eq!(
        //     right_child_2,
        //     Some(Rc::new(RefCell::new(Node::new(10, 12))))
        // );
        let result = convert_node_to_vec(tree.root.unwrap());
        assert_eq!(result, VecDeque::from(vec![10, 5, 20, 12]));
    }
}
