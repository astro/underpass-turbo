use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use osm_pbf_iter::{Primitive, Node, Way, Relation, RelationMemberType};

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    // info,
    pub tags: HashMap<String, String>,
    specific: ItemSpecific,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ItemSpecific {
    Node {
        lat: f64,
        lon: f64,
    },
    Way {
        refs: Vec<i64>,
    },
    Relation {
        members: Vec<(String, u64, RelationMemberType)>,
    },
}

impl Item {
    pub fn is_node(&self) -> bool {
        match self.specific {
            ItemSpecific::Node { .. } => true,
            _ => false,
        }
    }

    pub fn is_way(&self) -> bool {
        match self.specific {
            ItemSpecific::Way { .. } => true,
            _ => false,
        }
    }

    /// Is a closed way?
    pub fn is_area(&self) -> bool {
        match self.specific {
            ItemSpecific::Way { ref refs } if refs.len() > 0 => {
                let first = refs[0];
                let last = refs[refs.len() - 1];
                first == last
            }
            _ => false,
        }
    }

    pub fn is_relation(&self) -> bool {
        match self.specific {
            ItemSpecific::Relation { .. } => true,
            _ => false,
        }
    }

    pub fn get_lat_lon(&self) -> Option<(f64, f64)> {
        match self.specific {
            ItemSpecific::Node { lat, lon } =>
                Some((lat, lon)),
            _ =>
                None,
        }
    }

}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Item {
}

impl<'a> Hash for Item {
    fn hash<H>(&self, state: &mut H) 
    where
        H: Hasher,
    {
        self.id.hash(state)
    }
}

impl<'a> From<Primitive<'a>> for Item {
    fn from(primitive: Primitive<'a>) -> Self {
        match primitive {
            Primitive::Node(node) =>
                node.into(),
            Primitive::Relation(rel) =>
                rel.into(),
            Primitive::Way(way) =>
                way.into(),
        }
    }
}

impl<'a> From<Node<'a>> for Item {
    fn from(node: Node<'a>) -> Self {
        Item {
            id: node.id,
            tags: node.tags.iter()
                .map(
                    |(k, v)| (k.to_string(), v.to_string())
                ).collect(),
            specific: ItemSpecific::Node {
                lat: node.lat,
                lon: node.lon,
            },
        }
    }
}

impl<'a> From<Way<'a>> for Item {
    fn from(way: Way<'a>) -> Self {
        Item {
            id: way.id,
            tags: way.tags()
                .map(
                    |(k, v)| (k.to_string(), v.to_string())
                ).collect(),
            specific: ItemSpecific::Way {
                refs: way.refs().collect(),
            },
        }
    }
}

impl<'a> From<Relation<'a>> for Item {
    fn from(rel: Relation<'a>) -> Self {
        Item {
            id: rel.id,
            tags: rel.tags()
                .map(
                    |(k, v)| (k.to_string(), v.to_string())
                ).collect(),
            specific: ItemSpecific::Relation {
                members: rel.members()
                    .map(
                        |(role, id, typ)| (role.to_string(), id, typ)
                    ).collect(),
            },
        }
    }
}
