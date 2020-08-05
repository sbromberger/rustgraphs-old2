use std::path::Path;

pub trait Graph<V> {
    type VIterator: Iterator<Item=V>;

    fn nv(&self) -> V;
    fn ne(&self) -> usize;
    fn vertices(&self) -> Self::VIterator;
    fn in_degree(&self, v:V) -> V;
    fn out_degree(&self, v:V) -> V;
    fn in_neighbors(&self, v:V) -> &[V];
    fn out_neighbors(&self, v:V) -> &[V];
    fn has_edge(&self, u:V, v:V) -> bool;
    fn from_edge_file(fname: &Path) -> Self;
}
