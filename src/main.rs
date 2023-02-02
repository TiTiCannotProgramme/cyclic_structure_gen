mod bst;

fn main() {
    let node = bst::avl_node::Node::new(1, 2);
    let bf = node.get_balance_factor();
    println!("Hello, world!");
    println!("{}", node.value);
    println!("{}", bf);
}
