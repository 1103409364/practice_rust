#[derive(Debug)]
enum AvlTree<T> {
    Null,
    Tree(Box<AvlNode<T>>),
}

#[derive(Debug)]
struct AvlNode<T> {
    val: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    bfactor: i8,
}

use std::cmp::max;
use std::cmp::Ordering::*;
use std::mem::replace;
use AvlTree::*;

impl<T> AvlTree<T>
// 为实现节点数据的比较，数据需要满足排序 Ord 特性，所以引入了 Ordering 做比较。
where
    T: Ord,
{
    fn new() -> AvlTree<T> {
        Null
    }

    fn insert(&mut self, val: T) -> (bool, bool) {
        let ret = match *self {
            // 没有节点，直接插入
            Null => {
                let node = AvlNode {
                    val,
                    left: Null,
                    right: Null,
                    bfactor: 0,
                };
                *self = Tree(Box::new(node));

                (true, true)
            }
            Tree(ref mut node) => match node.val.cmp(&val) {
                // 比较节点值，再判断从哪边插入
                // inserted 表示是否插入
                // deepened 表示是否加深
                Equal => (false, false), // 相等，无需插入
                Less => {
                    let (inserted, deepened) = node.right.insert(val);

                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor += 1;

                        ret
                    } else {
                        (inserted, deepened)
                    }
                }
                Greater => {
                    let (inserted, deepened) = node.left.insert(val);

                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor -= 1;

                        ret
                    } else {
                        (inserted, deepened)
                    }
                }
            },
        };
        self.rebalance();

        ret
    }

    // 调整各个节点的平衡因子
    fn rebalance(&mut self) {
        match *self {
            // 没数据，不用调整
            Null => (),
            Tree(_) => match self.node().bfactor {
                // 右子树重
                -2 => {
                    let lbf = self.node().left.node().bfactor;
                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 { (0, 0) } else { (-1, 1) };
                        self.rotate_right(); // 不平衡，旋转
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if lbf == 1 {
                        let (a, b) = match self.node().left.node().right.node().bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };

                        // 先左旋再右旋
                        self.node().left.rotate_right();
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                // 左子树重
                2 => {
                    let rbf = self.node().right.node().bfactor;
                    if rbf == 1 || rbf == 0 {
                        let (a, b) = if rbf == 1 { (0, 0) } else { (1, -1) };
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if rbf == -1 {
                        let (a, b) = match self.node().right.node().left.node().bfactor {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        }; // 先右旋再左旋
                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                _ => (),
            },
        }
    }
    // 获取节点
    fn node(&mut self) -> &mut AvlNode<T> {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut n) => n,
        }
    }

    // 获取左右子树
    fn left_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.left,
        }
    }

    fn right_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.right,
        }
    }

    // 左右旋  通过旋转操作，维持着树的平衡。
    fn rotate_left(&mut self) {
        let mut v = replace(self, Null);
        let mut right = replace(v.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *v.right_subtree() = right_left;
        *right.left_subtree() = v;
        *self = right;
    }
    // 右旋
    fn rotate_right(&mut self) {
        let mut v = replace(self, Null);
        let mut left = replace(v.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *v.left_subtree() = left_right;
        *left.right_subtree() = v;
        *self = left;
    }
    // 树节点数是左右子树节点加根节点数，递归计算
    fn len(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => 1 + v.left.len() + v.right.len(),
        }
    }

    // 树深度是左右子树深度最大值 + 1，递归计算
    fn depth(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => max(v.left.depth(), v.right.depth()) + 1,
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Null => true,
            _ => false,
        }
    }

    fn search(&self, val: &T) -> bool {
        match *self {
            Null => true,
            Tree(ref v) => match v.val.cmp(&val) {
                Equal => true,
                Greater => match &v.left {
                    Null => false,
                    _ => v.left.search(val),
                },
                Less => match &v.right {
                    Null => false,
                    _ => v.right.search(val),
                },
            },
        }
    }
}

fn main() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        let (_r1, _r2) = avl.insert(i);
    }
    println!("empty: {}", avl.is_empty());
    println!("length: {}", avl.len());
    println!("depth: {}", avl.depth());
    println!("9 in avl: {}", avl.search(&9));
}
