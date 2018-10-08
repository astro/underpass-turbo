use std::path::PathBuf;
use std::sync::Arc;
use std::collections::HashSet;
use std::sync::mpsc::sync_channel;
use threadpool::{self, ThreadPool};
use osm_pbf_iter::{Primitive, PrimitiveBlock, Blob};

use item::Item;
use set::Set;
use pbf_source::PbfSource;
use ql::{Statement, Filter, TagSpec, QueryType};

enum Task {
    Blob(Blob),
    Finish,
}
unsafe impl Send for Task {}

pub struct Runner {
    source: PbfSource,
    pool: ThreadPool,
}

impl Runner {
    pub fn new(source: PbfSource) -> Self {
        let pool = threadpool::Builder::new().build();
        Runner { source, pool }
    }

    pub fn run_all(&self, statement: Statement) -> Set {
        let processor_factory = ProcessorFactory::from_statement(statement);
        self.run(
            self.source.all()
                // TODO: don't drop path+offset
                .map(|t| t.2),
            processor_factory
        )
    }

    // pub fn run_segments<I, F>(paths: I, mut f: F)
    // where
    //     I: Iterator<Item=Arc<PathBuf>>,
    //     F: FnMut(Primitive),
    // {
    //     let mut paths_set = HashSet::new();
    //     for path in paths {
    //         paths_set.insert(path);
    //     }

    //     let (tx, rx) = channel();
    //     for path in paths_set.into_iter() {
    //     }
    // }

    fn run<I>(&self, iter: I, processor_factory: ProcessorFactory) -> Set
    where
        I: Iterator<Item=Blob>,
    {
        // Prepare workers
        let processor_factory = Arc::new(processor_factory);
        let worker_count = self.pool.max_count();
        let mut task_txs = Vec::with_capacity(worker_count);
        let mut res_rxs = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let processor_factory = processor_factory.clone();

            let (task_tx, task_rx) = sync_channel(1);
            task_txs.push(task_tx);

            let (res_tx, res_rx) = sync_channel(1);
            res_rxs.push(res_rx);

            self.pool.execute(move || {
                let mut processor = processor_factory.generate();
                while let Ok(task) = task_rx.recv() {
                    match task {
                        Task::Blob(blob) => {
                            let data = blob.into_data();
                            let primitive_block = PrimitiveBlock::parse(&data);
                            for primitive in primitive_block.primitives() {
                                let item = primitive.into();
                                processor.process(item);
                            }
                        }
                        Task::Finish => {
                            break
                        }
                    }
                }
                let set = processor.digest();
                res_tx.send(set);
            });
        }
        // Feed tasks
        let mut i = 0;
        for blob in iter {
            task_txs[i].send(Task::Blob(blob));

            i += 1;
            if i >= worker_count {
                i = 0;
            }
        }
        // Finish up
        let mut result_sets = Vec::with_capacity(worker_count);
        for (task_tx, res_rx) in task_txs.iter().zip(res_rxs) {
            task_tx.send(Task::Finish);

            let set = res_rx.recv().unwrap();
            result_sets.push(set);
        }
        let result_set = Set::merge(result_sets.into_iter());
        self.pool.join();
        result_set
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorFactory {
    statement: Statement,
}

impl ProcessorFactory {
    pub fn from_statement(statement: Statement) -> Self {
        ProcessorFactory { statement }
    }
    
    pub fn generate(&self) -> Processor {
        Processor::from_statement(self.statement.clone())
    }
}

pub struct Processor {
    filters: Vec<Filter>,
    results: Set,
}

impl Processor {
    pub fn from_statement(statement: Statement) -> Self {
        let results = Set::empty();
        match statement {
            Statement::Query { filters } => {
                Processor {
                    filters,
                    results,
                }
            },
            _ => panic!("Not implemented: {:?}", statement),
        }
    }

    pub fn process(&mut self, item: Item) {
        let is_match = self.filters.iter()
            .all(|filter| eval_filter(filter, &item));
        if is_match {
            self.results.insert(item);
        }
    }

    pub fn digest(self) -> Set {
        self.results
    }
}

fn eval_filter(filter: &Filter, item: &Item) -> bool {
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
