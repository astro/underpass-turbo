use std::path::PathBuf;
use std::sync::Arc;
use std::collections::HashSet;
use std::sync::mpsc::sync_channel;
use threadpool::{self, ThreadPool};
use osm_pbf_iter::{Primitive, PrimitiveBlock, Blob};

use item::Item;
use set::Set;
use pbf_source::PbfSource;
use ql::{Statement, Filter, TagSpec, RecurseType, QueryType};
use filter::eval_filter;
use trace_node::UniqueSet;
use process_node::ProcessNode;

enum Task {
    Blob(Blob),
    Finish,
}
unsafe impl Send for Task {}

pub struct QueryRunner {
    source: PbfSource,
    pool: ThreadPool,
}

impl QueryRunner {
    pub fn new(source: PbfSource) -> Self {
        let pool = threadpool::Builder::new().build();
        QueryRunner { source, pool }
    }

    // pub fn run_all(&self, processor_factories: &[QueryTarget]) -> Set {
    //     self.run(
    //         self.source.all()
    //             // TODO: don't drop path+offset
    //             .map(|t| t.2),
    //         processor_factories
    //     )
    // }

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

    // fn run<I>(&self, iter: I, processor_factories: &[ProcessorFactory]) -> Set
    // where
    //     I: Iterator<Item=Blob>,
    // {
    //     // Prepare workers
    //     let processor_factories = Arc::new(processor_factories);
    //     let worker_count = self.pool.max_count();
    //     let mut task_txs = Vec::with_capacity(worker_count);
    //     let mut res_rxs = Vec::with_capacity(worker_count);

    //     for _ in 0..worker_count {
    //         let processor_factories = processor_factories.clone();

    //         let (task_tx, task_rx) = sync_channel(1);
    //         task_txs.push(task_tx);

    //         let (res_tx, res_rx) = sync_channel(1);
    //         res_rxs.push(res_rx);

    //         self.pool.execute(move || {
    //             let mut processors = processor_factories.iter()
    //                 .map(
    //                     |processor_factory| processor_factory.generate()
    //                 );
    //             while let Ok(task) = task_rx.recv() {
    //                 match task {
    //                     Task::Blob(blob) => {
    //                         let data = blob.into_data();
    //                         let primitive_block = PrimitiveBlock::parse(&data);
    //                         for primitive in primitive_block.primitives() {
    //                             let item = primitive.into();
    //                             for processor in processors {
    //                                 processor.process(item);
    //                             }
    //                         }
    //                     }
    //                     Task::Finish => {
    //                         break
    //                     }
    //                 }
    //             }
    //             let set = processors.digest();
    //             res_tx.send(set);
    //         });
    //     }
    //     // Feed tasks
    //     let mut i = 0;
    //     for blob in iter {
    //         task_txs[i].send(Task::Blob(blob));

    //         i += 1;
    //         if i >= worker_count {
    //             i = 0;
    //         }
    //     }
    //     // Finish up
    //     let mut result_sets = Vec::with_capacity(worker_count);
    //     for (task_tx, res_rx) in task_txs.iter().zip(res_rxs) {
    //         task_tx.send(Task::Finish);

    //         let set = res_rx.recv().unwrap();
    //         result_sets.push(set);
    //     }
    //     let result_set = Set::merge(result_sets.into_iter());
    //     self.pool.join();
    //     result_set
    // }
}

pub enum QueryTarget {
    Query {
        filters: Arc<Vec<Filter>>,
    },
    Recurse(RecurseType),
}
