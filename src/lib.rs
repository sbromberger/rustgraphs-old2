use crate::traits::Graph;
use std::fmt;
use std::io::BufRead;
pub mod traits;
pub mod traversals;
pub mod triangles;

pub type Vertex = u32;
pub struct StaticGraph {
    adj: graph_matrix::GraphMatrix<Vertex>
}

pub struct StaticDiGraph {
    fadj: graph_matrix::GraphMatrix<Vertex>,
    badj: graph_matrix::GraphMatrix<Vertex>,
}

impl Graph<Vertex> for StaticGraph {
    type VIterator = std::ops::Range<Vertex>;
    fn nv(&self) -> Vertex {
        self.adj.dim() as Vertex
    }

    fn ne(&self) -> usize {
        self.adj.n()
    }

    fn vertices(&self) -> Self::VIterator {
        std::ops::Range {
            start: 0 as Vertex,
            end: self.nv()
        }
    }

    fn out_degree(&self, v:Vertex) -> Vertex {
        self.adj.row_len(v as usize)
    }

    fn in_degree(&self, v:Vertex) -> Vertex {
        self.adj.row_len(v as usize)
    }

    fn out_neighbors(&self, v:Vertex) -> &[Vertex] {
        self.adj.row(v)
    }
    fn in_neighbors(&self, v:Vertex) -> &[Vertex] {
        self.adj.row(v)
    }

    fn has_edge(&self, u:Vertex, v:Vertex) -> bool {
        let d1 = self.out_degree(u);
        let d2 = self.out_degree(v);
        if d1 < d2 {
            // dst out_degree is larger.
            self.adj.has_index(u, v)
        } else {
            self.adj.has_index(v, u)
        }
    }

    fn from_edge_reader(reader: impl BufRead) -> Result<Self, String> {
        let mut edgelist: Vec<(Vertex, Vertex)> = vec![];
        for line in reader.lines() {
            let l = line.expect("error reading reader"); // produces a std::string::String
            let l = l.trim(); // changes to &str
            if l.starts_with("#") {
                continue;
            }
            let mut eit = l.split_whitespace();
            let s1 = eit.next().ok_or("Invalid line (first field)")?;
            let s2 = eit.next().ok_or("Invalid line (second field)")?;
            if eit.next().is_some() {
                return Err(String::from("Invalid line (extra fields)"));
            }
            let src: u32 = s1.parse().map_err(|_| "Invalid parse (first field)")?;
            let dst: u32 = s2.parse().map_err(|_| "Invalid parse (second field)")?;
            edgelist.push((src, dst));
            edgelist.push((dst, src));
        }
        let adj = graph_matrix::GraphMatrix::from_edges(edgelist);
        Ok(StaticGraph { adj })
    }

}

impl Graph<Vertex> for StaticDiGraph {
    type VIterator = std::ops::Range<Vertex>;
    fn nv(&self) -> Vertex {
        self.fadj.dim() as Vertex
    }

    fn ne(&self) -> usize {
        self.fadj.n()
    }

    fn vertices(&self) -> Self::VIterator {
        std::ops::Range {
            start: 0 as Vertex,
            end: self.nv()
        }
    }

    fn out_degree(&self, v:Vertex) -> Vertex {
        self.fadj.row_len(v as usize)
    }

    fn in_degree(&self, v:Vertex) -> Vertex {
        self.badj.row_len(v as usize)
    }

    fn out_neighbors(&self, v:Vertex) -> &[Vertex] {
        self.fadj.row(v)
    }
    fn in_neighbors(&self, v:Vertex) -> &[Vertex] {
        self.badj.row(v)
    }

    fn has_edge(&self, u:Vertex, v:Vertex) -> bool {
        let d1 = self.out_degree(u);
        let d2 = self.out_degree(v);
        if d1 < d2 {
            // dst out_degree is larger.
            self.fadj.has_index(u, v)
        } else {
            self.badj.has_index(v, u)
        }
    }

    fn from_edge_reader(reader: impl BufRead) -> Result<Self, String> {
        let mut edgelist: Vec<(Vertex, Vertex)> = vec![];
        for line in reader.lines() {
            let l = line.expect("error reading reader"); // produces a std::string::String
            let l = l.trim(); // changes to &str
            if l.starts_with("#") {
                continue;
            }
            let mut eit = l.split_whitespace();
            let s1 = eit.next().ok_or("Invalid line (first field)")?;
            let s2 = eit.next().ok_or("Invalid line (second field)")?;
            if eit.next().is_some() {
                return Err(String::from("Invalid line (extra fields)"));
            }
            let src: u32 = s1.parse().map_err(|_| "Invalid parse (first field)")?;
            let dst: u32 = s2.parse().map_err(|_| "Invalid parse (second field)")?;
            edgelist.push((src, dst));
        }
        let bedges = edgelist.clone().iter().map(|x| (x.1, x.0)).collect();
        let fadj = graph_matrix::GraphMatrix::from_edges(edgelist);
        let badj = graph_matrix::GraphMatrix::from_edges(bedges);
        Ok(StaticDiGraph { fadj, badj })
    }
}

impl fmt::Display for StaticDiGraph
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) StaticDiGraph", self.nv(), self.ne())
    }
}

impl fmt::Display for StaticGraph
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) StaticGraph", self.nv(), self.ne())
    }
}
