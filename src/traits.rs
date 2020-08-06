use std::fs::File;
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
    fn from_edge_reader(reader: impl BufRead) -> Result<Self, String>;
    fn from_edge_file(fname: &Path) -> Result<Self, String> {
        let f = match File::open(fname) {
            Err(why) => {
                return Err(format!("{}", why))
            },
            Ok(file) => file,
        };
        let file = BufReader::new(&f);
        Self::from_edge_reader(file)
    }
}
