use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub trait Graph<V>: Sized {
    type VIterator: Iterator<Item=V>;

    fn nv(&self) -> V;
    fn ne(&self) -> usize;
    fn vertices(&self) -> Self::VIterator;
    fn in_degree(&self, v:V) -> V;
    fn out_degree(&self, v:V) -> V;
    fn in_neighbors(&self, v:V) -> &[V];
    fn out_neighbors(&self, v:V) -> &[V];
    fn has_edge(&self, u:V, v:V) -> bool;
    fn from_edge_reader(reader: impl BufRead) -> Result<Self, Box<dyn Error>>;
    fn from_edge_file(fname: &Path) -> Result<Self, Box<dyn Error>> {
        let f = File::open(fname)?;
        let file = BufReader::new(&f);
        Self::from_edge_reader(file)
    }
}
