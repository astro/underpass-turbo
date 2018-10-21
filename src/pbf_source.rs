use std::fs::File;
use std::io::{Seek, SeekFrom, BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use osm_pbf_iter::{Blob, BlobReader, Primitive};

#[derive(Debug, Clone)]
pub struct PbfSource {
    paths: Vec<Arc<PathBuf>>,
}

impl PbfSource {
    pub fn new<I, E>(paths: I) -> Self
    where
        I: Iterator<Item=E>,
        PathBuf: From<E>
    {
        PbfSource {
            paths: paths
                .map(From::from)
                .map(Arc::new)
                .collect(),
        }
    }

    pub fn all(&self) -> All {
        All {
            path_file: None,
            remain_paths: self.paths.clone(),
        }
    }

    // pub fn segments() {
    // }
}

pub struct All {
    path_file: Option<(Arc<PathBuf>, BufReader<File>)>,
    remain_paths: Vec<Arc<PathBuf>>,
}

impl Iterator for All {
    type Item = (Arc<PathBuf>, u64, Blob);

    fn next(&mut self) -> Option<Self::Item> {
        match self.path_file.take() {
            Some((path, mut file)) => {
                let position = file.seek(SeekFrom::Current(0)).ok()?;
                let result = BlobReader::read_blob(&mut file)
                    .map(|blob| (path.clone(), position, blob));
                match result {
                    Some(_) => {
                        self.path_file = Some((path, file));
                        result
                    }
                    None => {
                        self.path_file = None;
                        // Retry next file
                        self.next()
                    }
                }
            }
            None if self.remain_paths.len() > 0 => {
                let path = self.remain_paths.pop().unwrap();
                let file = BufReader::new(File::open(path.as_ref()).unwrap());
                self.path_file = Some((path, file));
                self.next()
            }
            None => None
        }
    }
}
