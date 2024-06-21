use serenity::prelude::TypeMapKey;
use std::collections::HashMap;

pub struct Data {
    pub codes: HashMap<u64, String>,
}

impl TypeMapKey for Data {
    type Value = HashMap<u64, String>;
}