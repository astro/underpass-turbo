use std::collections::HashSet;

use item::Item;

type Contents = HashSet<Item>;

#[derive(Debug, Clone)]
pub struct Set {
    contents: Contents,
}

impl Set {
    pub fn empty() -> Self {
        Set { contents: HashSet::new() }
    }

    pub fn new(contents: Contents) -> Self {
        Set { contents }
    }

    pub fn merge<I>(mut sets: I) -> Self
    where
        I: Iterator<Item=Self>,
    {
        let mut contents = match sets.next() {
            None =>
                // Empty case
                HashSet::new(),
            Some(set) => set.contents,
        };
        for set in sets.into_iter() {
            for item in set.contents {
                contents.insert(item);
            }
        }
        Set { contents }
    }

    pub fn insert(&mut self, item: Item) {
        self.contents.insert(item);
    }
}
