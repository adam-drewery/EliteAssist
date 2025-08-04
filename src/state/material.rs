#[derive(Clone, Default)]
pub struct Materials {
    pub raw: Vec<MaterialGroup>,
    pub manufactured: Vec<MaterialGroup>,
    pub encoded: Vec<MaterialGroup>,
}

#[derive(Clone)]
pub struct MaterialGroup {
    pub name: String,
    pub materials: Vec<Material>,
}

#[derive(Clone)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub count: u64,
    pub locations: Vec<String>,
}