// 定义 Allergies 结构体，包含一个 score 字段，表示过敏分数
pub struct Allergies {
    score: u32,
}

// 定义过敏原枚举
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

// 实现 From<Allergen> for u32，用于将过敏原转换为对应的位掩码
impl From<Allergen> for u32 {
    fn from(allergen: Allergen) -> Self {
        match allergen {
            Allergen::Eggs => 1 << 0,         // 鸡蛋：第 0 位
            Allergen::Peanuts => 1 << 1,      // 花生：第 1 位
            Allergen::Shellfish => 1 << 2,    // 贝类：第 2 位
            Allergen::Strawberries => 1 << 3, // 草莓：第 3 位
            Allergen::Tomatoes => 1 << 4,     // 西红柿：第 4 位
            Allergen::Chocolate => 1 << 5,    // 巧克力：第 5 位
            Allergen::Pollen => 1 << 6,       // 花粉：第 6 位
            Allergen::Cats => 1 << 7,         // 猫：第 7 位
        }
    }
}

impl Allergies {
    // 定义一个常量数组，包含所有可能的过敏原
    const ALLERGENS: [Allergen; 8] = [
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    // 创建一个新的 Allergies 实例
    pub fn new(score: u32) -> Self {
        // 对 score 进行取模运算，确保 score 的值在 0-255 范围内
        Self { score: score % 256 }
    }

    // 判断是否对指定的过敏原过敏
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // 将过敏原转换为对应的位掩码，并与 score 进行与运算
        // 如果结果不为 0，意味着 self.score 中与 allergen 对应的位是 1，表示对该过敏原过敏。如果结果为 0，则表示对应的位是 0，因此不过敏。
        self.score & u32::from(*allergen) != 0
    }

    // 返回一个包含所有过敏原的 Vec<Allergen>
    pub fn allergies(&self) -> Vec<Allergen> {
        // 遍历 ALLERGENS 数组，使用 filter 方法过滤出所有过敏的过敏原
        Self::ALLERGENS
            .iter() // 创建一个迭代器
            .filter(|allergen| self.is_allergic_to(allergen)) // 过滤出过敏的过敏原
            .cloned() // 克隆过敏原，因为 iter() 返回的是引用
            .collect() // 将过滤后的过敏原收集到一个新的 Vec<Allergen> 中
    }
}
