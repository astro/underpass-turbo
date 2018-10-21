use item::Item;
use set::Set;
use pbf_source::PbfSource;
use ql::{Statement, Filter, TagSpec, QueryType};

pub fn eval_filter(filter: &Filter, item: &Item) -> bool {
    match filter {
        &Filter::Id(id) =>
            item.id == id,
        &Filter::QueryType(query_type) =>
            match query_type {
                QueryType::Node =>
                    item.is_node(),
                QueryType::Way =>
                    item.is_way(),
                QueryType::Relation =>
                    item.is_relation(),
                _ =>
                    panic!("Not implemented"),
            },
        &Filter::BoundingBox { s, w, n, e } =>
            item.get_lat_lon()
            .map(
                |(lat, lon)|
                s <= lat && lat <= n &&
                    w <= lon && lon <= e
            ).unwrap_or(false),
        &Filter::TagEqual { ref k, ref v } =>
            match k {
                TagSpec::String(s) =>
                    item.tags.get(s)
                    .map(|tv| v.test(tv))
                    .unwrap_or(false),
                _ =>
                    item.tags.iter()
                    .any(
                        |(tk, tv)| k.test(tk) && v.test(tv)
                    ),
            },
        _ => panic!("Not implemented: {:?}", filter),
    }
}
