#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
impl TreeNode {
    fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = TreeNode::max_depth(node.left.as_ref());
                let right_depth = TreeNode::max_depth(node.right.as_ref());
                1 + i32::max(left_depth, right_depth)
            }
        }
    }
}

fn main() {
    let mut root = None;

    loop {
        println!("Enter a number to insert into the tree (or type 'done' to finish):");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq("done") {
            break;
        }

        let val = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        if let Some(ref mut r) = root {
            r.insert(val);
        } else {
            root = Some(Box::new(TreeNode::new(val)));
        }
    }

    let depth = TreeNode::max_depth(root.as_ref());
    println!("The maximum depth of the tree is: {}", depth);
}
