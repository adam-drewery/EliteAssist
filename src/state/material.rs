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
    pub name: String,
    pub rarity: u8,
    pub count: u16,
    pub locations: Option<String>,
}