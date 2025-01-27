// 三方库 Hash 库
// 在开头，我们提到过如果现有的 SipHash 1-3 的性能无法满足你的需求，那么可以使用社区提供的替代算法。

// 例如其中一个社区库的使用方式如下：
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
// 引入第三方的哈希函数
use twox_hash::XxHash64;

fn main() {
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
