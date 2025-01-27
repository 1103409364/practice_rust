// 字典 树 定义
#[derive(Default, Debug)]
struct Trie {
    root: Node,
}
// 节点
#[derive(Default, Debug)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], // 字符节点列表
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
            let next = &mut node.children[index]; // 用索引表示字符。不是在 Node 存具体的值
                                                  // get_or_insert_with 是一个用于处理 Option 类型的方法，常用于 std::collections 库中的数据结构，比如 HashMap 或 BTreeMap。它的主要功能是在 Option 中获取一个值，如果值不存在，则插入一个新值。
            node = next.get_or_insert_with(Box::<Node>::default); // Node 实现了 Default Trait，默认 Node 实例：Node { children: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], end: false }
        }
        node.end = true;
    }

    fn search(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end) // map_or 是 Option 类型的方法，第一个参数是默认值。用于对 Option 中的值进行处理。如果 Option 是 Some，会应用一个函数并返回结果；如果是 None，则返回一个提供的默认值。这使得在处理可选值时方法更加灵活。
    }

    // 判断是否存在以某个前缀开头的单词
    fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some() // is_some 是 Option 类型的方法，用于检查 Option 是否包含一个值。具体来说，如果 Option 是 Some，is_some 返回 true；如果是 None，返回 false
    }

    // 前缀字符串
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
