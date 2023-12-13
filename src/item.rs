pub struct ItemType {
    pub name: String,
    pub tags: Vec<String>,
}

impl ItemType {
    pub fn new(name: &str, tags: Vec<&str>) -> Self {
        ItemType {
            name: name.to_owned(),
            tags: tags.into_iter().map(|t| t.to_owned()).collect(),
        }
    }
}

pub struct Item {
    pub itemtype: usize,
    pub amount: u16,
}

impl Item {
    pub fn new(types: &[ItemType], typename: &str, amount: u16) -> Self {
        Self {
            itemtype: types
                .iter()
                .position(|t| t.name.to_uppercase() == typename.to_uppercase())
                .unwrap(),
            amount,
        }
    }
}

pub struct Inventory(Vec<Item>);

impl Inventory {
    pub fn find_item_mut(&mut self, itemtype: usize) -> &mut Item {
        &mut self.0[itemtype]
    }

    pub fn find_item(&self, itemtype: usize) -> &Item {
        &self.0[itemtype]
    }

    pub fn find_items_with_tag(&self, itemtypes: &ItemTypes, tag: String) -> Vec<&Item> {
        self.0
            .iter()
            .filter(|item| item.amount > 0 && itemtypes.types[item.itemtype].tags.contains(&tag))
            .collect()
    }

    pub fn count_items_with_tag(&self, itemtypes: &ItemTypes, tag: &str) -> u32 {
        self.0
            .iter()
            .filter(|item| {
                item.amount > 0 && itemtypes.types[item.itemtype].tags.iter().any(|t| t == tag)
            })
            .map(|item| item.amount as u32)
            .sum()
    }
}

pub struct Ingredient {
    pub tag: String,
    pub amount: u16,
    pub consumed: bool,
}

impl Ingredient {
    pub fn new(tag: &str, amount: u16, consumed: bool) -> Self {
        Ingredient {
            tag: tag.to_owned(),
            amount,
            consumed,
        }
    }

    pub fn satisfied(&self, itemtypes: &ItemTypes, inventory: &Inventory) -> bool {
        inventory.count_items_with_tag(itemtypes, &self.tag) >= self.amount as u32
    }
}

pub struct Recipe {
    pub ingredients: Vec<Ingredient>,
    pub output: Item,
}

impl Recipe {
    pub fn new(output: Item) -> Self {
        Recipe {
            ingredients: vec![],
            output,
        }
    }

    pub fn requires(mut self, ingredient: Ingredient) -> Self {
        self.ingredients.push(ingredient);
        self
    }
}

pub struct ItemTypes {
    pub types: Vec<ItemType>,
    pub recipes: Vec<Recipe>,
}

impl ItemTypes {
    pub fn new_empty_inventory(&self) -> Inventory {
        Inventory(
            (0..self.types.len())
                .map(|it| Item {
                    itemtype: it,
                    amount: 0,
                })
                .collect(),
        )
    }

    pub fn get_item_type(&self, itemtype: usize) -> &ItemType {
        &self.types[itemtype]
    }
}

impl Default for ItemTypes {
    // WIP: more elegant way to define item types
    fn default() -> Self {
        let types = vec![
            ItemType::new("Long grass blade", vec![]),
            ItemType::new("Grass thread", vec!["thread"]),
        ];

        let recipes = vec![Recipe::new(Item::new(&types, "Grass thread", 3))
            .requires(Ingredient::new("thread", 5, true))];

        Self { types, recipes }
    }
}
