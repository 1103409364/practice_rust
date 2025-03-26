#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,         // 鸡蛋：第 0 位
    Peanuts = 1 << 1,      // 花生：第 1 位
    Shellfish = 1 << 2,    // 贝类：第 2 位
    Strawberries = 1 << 3, // 草莓：第 3 位
    Tomatoes = 1 << 4,     // 西红柿：第 4 位
    Chocolate = 1 << 5,    // 巧克力：第 5 位
    Pollen = 1 << 6,       // 花粉：第 6 位
    Cats = 1 << 7,         // 猫：第 7 位
}

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

pub struct Allergies {
    allergens: u32, // 一个 u32，其中每个位表示一个人是否对相应的过敏原过敏
}

impl Allergies {
    // 创建一个新的 `Allergies` 结构体
    pub fn new(score: u32) -> Self {
        Allergies { allergens: score }
    }

    // 检查一个人是否对特定的过敏原过敏
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // 使用位操作来确定一个人是否对特定的过敏原过敏
        self.allergens & (*allergen as u32) != 0
    }

    // 获取一个人过敏的所有过敏原的列表
    pub fn allergies(&self) -> Vec<Allergen> {
        // 使用一个迭代器来过滤 `ALLERGENS` 数组，并返回一个人过敏的所有过敏原的向量
        ALLERGENS
            .iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .cloned()
            .collect()
    }
}
