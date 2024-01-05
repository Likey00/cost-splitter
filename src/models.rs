pub struct NameList(pub Vec<String>);

pub struct ItemList(pub Vec<Item>);

pub struct ExtrasList(pub Vec<Extra>);

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub name: String,
    pub price: String,
    pub people: Vec<bool>,
    pub count: usize,
    pub active: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub name: String,
    pub price: String,
}
