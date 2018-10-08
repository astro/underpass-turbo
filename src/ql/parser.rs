use combine::{Parser, choice, many};
use combine::char::{string, spaces};
use combine::error::StringStreamError;
use super::{Statement, QueryType};


/*pub fn parse_ql(text: &str) -> Result<Vec<Statement>, StringStreamError> {
    let prim_type = choice((
        string("node")
            .map(|_| QueryType::Node),
        string("way")
            .map(|_| QueryType::Way),
        string("relation")
            .map(|_| QueryType::Relation),
    ));
    let query = prim_type
        .map(|query_type| Statement::Query { query_type });
    let statement = (
        query
            .skip(spaces()),
        string(";")
    ).map(|r| r.0);
    let mut parser = many::<Vec<Statement>, _>(statement);
    parser.parse(text)
        .map(|r| r.0)
}
*/
