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

    fn setup_tree5() -> AvlTree {
        let mut tree = AvlTree::new();
        let node1 = setup_node1();
        let node2 = Node::new(10, 10);
        tree.insert(node1);
        tree.insert(node2);
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

    #[test]
    fn test_empty_tree_search() {
        let tree = setup_empty_tree();
        assert_eq!(tree.search_tree_by_value(19), None);
    }

    #[test]
    fn test_tree2_search() {
        let tree = setup_tree2();
        assert_eq!(tree.search_tree_by_value(30).unwrap().borrow().value, 30);
    }

    #[test]
    fn test_tree2_max() {
        let tree = setup_tree2();
        let node_ptr = AvlTree::get_maximum_child(tree.root.unwrap());
        assert_eq!(node_ptr.borrow().value, 30);
    }

    #[test]
    fn test_tree2_min() {
        let tree = setup_tree2();
        let node_ptr = AvlTree::get_minimum_child(tree.root.unwrap());
        assert_eq!(node_ptr.borrow().value, 10);
    }

    #[test]
    fn test_tree5_delete() {
        let mut tree = setup_tree5();
        let node = tree.search_tree_by_value(10);
        tree.delete_node(node.unwrap());
        assert_eq!(tree.root.as_ref().unwrap().borrow().value, 20);
        assert_eq!(tree.root.unwrap().borrow().parent.upgrade(), None);
    }

    #[test]
    fn test_left_right_rotate() {
        let mut tree = build_free_from_values(vec![30, 40]);
        tree.insert(Node::new(0, 35));
        let root = tree.root.as_ref().unwrap();
        let root_copy = Rc::clone(tree.root.as_ref().unwrap());
        assert_eq!(
            convert_node_to_vec(root_copy),
            VecDeque::from(vec![35, 30, 40])
        );
        assert_eq!(root.borrow().value, 35);
        assert_eq!(root.borrow().parent.upgrade(), None);
        assert_eq!(
            root.borrow()
                .get_left_child()
                .as_ref()
                .unwrap()
                .borrow()
                .parent
                .upgrade()
                .unwrap()
                .as_ref()
                .borrow()
                .value,
            35
        );
        assert_eq!(
            root.borrow()
                .get_right_child()
                .as_ref()
                .unwrap()
                .borrow()
                .parent
                .upgrade()
                .unwrap()
                .as_ref()
                .borrow()
                .value,
            35
        );
        assert!(root
            .borrow()
            .get_right_child()
            .as_ref()
            .unwrap()
            .borrow()
            .has_no_child());
        assert!(root
            .borrow()
            .get_left_child()
            .as_ref()
            .unwrap()
            .borrow()
            .has_no_child());
        assert!(root
            .borrow()
            .get_right_child()
            .as_ref()
            .unwrap()
            .borrow()
            .parent
            .upgrade()
            .unwrap()
            .as_ref()
            .borrow()
            .has_both_children());
    }

    #[test]
    fn test_right_left_rotate() {
        let mut tree = build_free_from_values(vec![30, 20]);
        tree.insert(Node::new(0, 25));
        let root = Rc::clone(tree.root.as_ref().unwrap());
        let node_20 = root.borrow().get_left_child().unwrap();
        let node_30 = root.borrow().get_right_child().unwrap();
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![25, 20, 30])
        );
        assert_eq!(
            node_20.borrow().parent.upgrade().unwrap().borrow().value,
            25
        );
        assert_eq!(
            node_30.borrow().parent.upgrade().unwrap().borrow().value,
            25
        );
    }

    #[test]
    fn test_left_rotate() {
        let mut tree = build_free_from_values(vec![30, 20, 10]);
        let root = Rc::clone(tree.root.as_ref().unwrap());
        let node_10 = root.borrow().get_left_child().unwrap();
        let node_30 = root.borrow().get_right_child().unwrap();
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![20, 10, 30])
        );
        assert_eq!(
            node_10.borrow().parent.upgrade().unwrap().borrow().value,
            20
        );
        assert_eq!(
            node_30.borrow().parent.upgrade().unwrap().borrow().value,
            20
        );
    }

    #[test]
    fn test_left_rotate2() {
        let mut tree = build_free_from_values(vec![15, 10, 20, 30, 40]);
        let root = Rc::clone(tree.root.as_ref().unwrap());
        let node_10 = root.borrow().get_left_child().unwrap();
        let node_30 = root.borrow().get_right_child().unwrap();
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![15, 10, 30, 20, 40])
        );
        assert_eq!(
            node_10.borrow().parent.upgrade().unwrap().borrow().value,
            15
        );
        assert_eq!(
            node_30.borrow().parent.upgrade().unwrap().borrow().value,
            15
        );
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = build_free_from_values(vec![30, 40, 50]);
        let root = Rc::clone(tree.root.as_ref().unwrap());
        let node_30 = root.borrow().get_left_child().unwrap();
        let node_50 = root.borrow().get_right_child().unwrap();
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![40, 30, 50])
        );
        assert_eq!(
            node_30.borrow().parent.upgrade().unwrap().borrow().value,
            40
        );
        assert_eq!(
            node_50.borrow().parent.upgrade().unwrap().borrow().value,
            40
        );
    }

    fn setup_tree6() -> AvlTree {
        build_free_from_values(vec![30, 50, 70, 65, 68])
    }

    #[test]
    fn test_tree6_root_correct() {
        let mut tree = setup_tree6();
        assert_eq!(tree.root.unwrap().borrow().value, 50);
    }

    #[test]
    fn test_tree6_init_correct() {
        let mut tree = setup_tree6();
        let root = Rc::clone(tree.root.as_ref().unwrap());
        let node_30 = root.borrow().get_left_child().unwrap();
        let node_68 = root.borrow().get_right_child().unwrap();
        assert_eq!(
            convert_node_to_vec(root),
            VecDeque::from(vec![50, 30, 68, 65, 70])
        );
        assert_eq!(
            node_30.borrow().parent.upgrade().unwrap().borrow().value,
            50
        );
        assert_eq!(
            node_68.borrow().parent.upgrade().unwrap().borrow().value,
            50
        );
    }

    #[test]
    fn test_tree6_insertion_correct() {
        let mut tree = setup_tree6();
        tree.insert(Node::new(0, 75));
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![68, 50, 70, 30, 65, 75])
        );
    }

    #[test]
    fn test_tree6_insertion_correct2() {
        let mut tree = setup_tree6();
        tree.insert(Node::new(0, 64));
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![65, 50, 68, 30, 64, 70])
        );
    }

    #[test]
    fn test_tree_min_child() {
        let tree = setup_tree6();
        let min_child = AvlTree::get_minimum_child(tree.root.unwrap());
        assert_eq!(min_child.borrow().value, 30);
    }

    #[test]
    fn test_tree6_deletion_correct() {
        let mut tree = setup_tree6();
        let node_found = tree.search_tree_by_value(30);
        assert_eq!(node_found.as_ref().unwrap().borrow().value, 30);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![68, 50, 70, 65])
        );
    }

    #[test]
    fn test_tree6_deletion_correct2() {
        let mut tree = setup_tree6();
        let node_found = tree.search_tree_by_value(70);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![50, 30, 68, 65])
        );
    }

    #[test]
    fn test_tree6_deletion_correct3() {
        let mut tree = setup_tree6();
        let node_found = tree.search_tree_by_value(50);
        assert_eq!(node_found.as_ref().unwrap().borrow().value, 50);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![65, 30, 68, 70])
        );
    }

    #[test]
    fn test_delete_root_with_no_child() {
        let mut tree = build_free_from_values(vec![10]);
        let node_found = tree.search_tree_by_value(10);
        assert_eq!(node_found.as_ref().unwrap().borrow().value, 10);
        tree.delete_node(node_found.unwrap());
        assert_eq!(tree.root, None);
    }

    #[test]
    fn test_delete_root_with_both_child() {
        let mut tree = build_free_from_values(vec![10, 5, 15]);
        let node_found = tree.search_tree_by_value(10);
        assert_eq!(node_found.as_ref().unwrap().borrow().value, 10);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![15, 5])
        );
    }

    #[test]
    fn test_delete_root_with_one_child() {
        let mut tree = build_free_from_values(vec![10, 5]);
        let node_found = tree.search_tree_by_value(10);
        assert_eq!(node_found.as_ref().unwrap().borrow().value, 10);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![5])
        );
    }

    #[test]
    fn test_delete_leaf() {
        let mut tree = build_free_from_values(vec![10, 5, 15]);
        let node_found = tree.search_tree_by_value(5);
        tree.delete_node(node_found.unwrap());
        assert_eq!(
            convert_node_to_vec(tree.root.unwrap()),
            VecDeque::from(vec![10, 15])
        );
    }
}
