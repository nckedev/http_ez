pub enum HeaderItem {
    ContentLenght(u32),
    Other(String),
}

struct WeightedItem<'a, T: From<&'a str>> {
    item: &'a T,
    weight: i32,
}

impl<'a, T: From<&'a str>> WeightedItem<'a, T> {
    pub fn new(value: &'a T) -> Self {
        WeightedItem {
            item: value,
            weight: 0,
        }
    }
}
