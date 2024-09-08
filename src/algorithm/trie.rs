// 字典 树 定义
#[derive(Default, Debug)]
struct Trie {
    root: Node,
}
// 节点
#[derive(Default, Debug)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], // 字 符 节 点 列 表
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    // 单词插入
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        // 逐 个 字 符 插 入
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            // get_or_insert_with 是一个用于处理 Option 类型的方法，常用于 std::collections 库中的数据结构，比如 HashMap 或 BTreeMap。它的主要功能是在 Option 中获取一个值，如果值不存在，则插入一个新值。
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.end = true;
    }

    fn search(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end)
    }

    // 判 断 是 否 存 在 以 某 个 前 缀 开 头 的 单 词
    fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    // 前缀 字 符串
    // wps: word_prefix_string
    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }
        Some(node)
    }
    fn print(&self) {
        self.print_node(&self.root, String::new());
    }

    fn print_node(&self, node: &Node, prefix: String) {
        if node.end {
            println!("{}", prefix); // 打印完整的单词
        }

        for (i, child) in node.children.iter().enumerate() {
            if let Some(ref child_node) = child {
                self.print_node(
                    child_node,
                    format!("{}{}", prefix, (b'a' + i as u8) as char),
                );
            }
        }
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("box");
    trie.insert("insert");
    trie.insert("apple");
    trie.insert("appeal");

    let res1 = trie.search("apple");
    let res2 = trie.search("apples");
    let res3 = trie.start_with("ins");
    let res4 = trie.start_with("ina");
    println!("word 'apple' in Trie: {res1}");
    println!("word 'apples' in Trie: {res2}");
    println!("prefix 'ins' in Trie: {res3}");
    println!("prefix 'ina' in Trie: {res4}");
    // println!("trie {:?}", trie)
     trie.print(); // 打印所有单词
}
