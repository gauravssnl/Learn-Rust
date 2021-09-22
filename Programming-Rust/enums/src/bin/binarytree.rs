// Enums in Memory - example
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value < node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
    }

    fn get_left_child(&self) -> &BinaryTree<T> {
        match *self {
            BinaryTree::Empty => &BinaryTree::Empty,
            BinaryTree::NonEmpty(ref node) => match &node.left {
                BinaryTree::Empty => &BinaryTree::Empty,
                left_tree @ BinaryTree::NonEmpty(..) => &left_tree,
            },
        }
    }

    fn get_right_child(&self) -> &BinaryTree<T> {
        match *self {
            BinaryTree::Empty => &BinaryTree::Empty,
            BinaryTree::NonEmpty(ref node) => match &node.right {
                BinaryTree::Empty => &BinaryTree::Empty,
                right_tree @ BinaryTree::NonEmpty(..) => &right_tree,
            },
        }
    }
}

fn main() {
    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    println!("mars_tree: {:?}", mars_tree);

    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    tree.add("Earth");
    println!("tree: {:?}", tree);
    println!("Left Child - {:?}", tree.get_right_child());
    println!("Right Child - {:?}", tree.get_left_child());
}
