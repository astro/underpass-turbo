extern crate osm_pbf_iter;
extern crate clap;
extern crate combine;
extern crate threadpool;
extern crate regex;

use clap::{Arg, App};

mod ql;
// use ql::parse_ql;
use ql::{Statement, Filter, TagSpec, QueryType};

mod item;
mod set;
mod pbf_source;
use pbf_source::PbfSource;
mod process;
use process::Runner;
mod filter;

fn main() {
    let matches = App::new("Underpass Turbo")
        .version("0.1.0")
        .author("Astro <astro@spaceboyz.net>")
        .arg(Arg::with_name("QUERY")
             .help("QL source")
             .required(true)
             .index(1)
        )
        .arg(Arg::with_name("PBF")
             .help("OpenStreetMap dump files (one or more)")
             .required(true)
             .multiple(true)
        ).get_matches();
    // let query = matches.value_of("QUERY")
    //     .expect("Query missing");
    // println!("parsed query: {:?}", parse_ql(query));

    let source_paths = matches.values_of_os("PBF")
        .expect("Source paths missing");
    let source = PbfSource::new(source_paths);
    let runner = Runner::new(source);

    let statement = Statement::Query {
        filters: vec![
            // Filter::QueryType(QueryType::Node),
            // Filter::Id(2331619771),
            // Filter::BoundingBox {
            //     // 51.0810832, 13.7286525
            //     s: 51.0810,
            //     n: 51.0811,
            //     w: 13.7286,
            //     e: 13.7287,
            // },
            // Filter::Id(372193022),
            Filter::TagEqual {
                k: TagSpec::from_string("leisure"),
                v: TagSpec::from_regex("space", true),
            }
        ],
    };
    let r = runner.run_all(statement);
    println!("{:?}", r);
}
