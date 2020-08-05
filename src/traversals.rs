use std::mem;
use bitvec::prelude as bv;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use crate::traits::Graph;
use num::cast::AsPrimitive;
use num::traits::PrimInt;

pub fn bfs<V>(g: &impl Graph<V>, src: V) -> Vec<V> where V:PrimInt + AsPrimitive<usize> {
    let n = g.nv();
    let maxdeg = g
        .vertices()
        .map(|v| g.out_degree(v))
        .max()
        .expect("Invalid degree found")
        .as_();
    let mut visited: bv::BitVec<bv::Lsb0, u64> = bv::BitVec::repeat(false, n.as_());

    let mut levels: Vec<V> = vec![V::max_value(); n.as_()];
    let mut cur_level: Vec<V> = Vec::new();
    cur_level.reserve(maxdeg);

    let mut next_level: Vec<V> = Vec::new();
    next_level.reserve(maxdeg);

    let s = src.as_();
    visited.set(s, true);
    cur_level.push(src);
    levels[s] = V::zero();

    let mut n_level = V::one();

    // println!("cur_level = {:?}", cur_level);
    while !cur_level.is_empty() {
        for v in cur_level.iter() {
            for i in g.out_neighbors(*v) {
                // println!("neighbor {:?}", i);
                let ui = i.as_();
                if unsafe { !*visited.get_unchecked(ui) } {
                    // println!("{:?} -> {}", v, ui);
                    next_level.push(*i);
                    unsafe {
                        visited.set_unchecked(ui, true);
                        *levels.get_unchecked_mut(ui) = n_level;
                    }
                }
            }
        }
        n_level = n_level + V::one();
        // println!("next_level = {:?}", next_level);
        cur_level.clear();

        mem::swap(&mut cur_level, &mut next_level);
        cur_level.sort_unstable();
    }
    levels
}
pub fn dijkstra<V, W>(g: &impl Graph<V>, v: V, weights: fn(V, V) -> W) -> Vec<W>
where
    V: PrimInt + AsPrimitive<usize> + std::hash::Hash + std::cmp::Eq,
    W: num::Float,
{
    let vu = v.as_();
    let n = g.nv().as_();
    let mut visited: bv::BitVec<bv::Lsb0, u64> = bv::BitVec::repeat(false, n);
    let mut pq = PriorityQueue::<V, OrderedFloat<W>>::new();
    let mut dists = vec![W::infinity(); n];
    let mut parents = vec![V::max_value(); n];

    dists[vu] = W::zero();
    unsafe {
        visited.set_unchecked(vu, true);
    }
    pq.push(v, OrderedFloat(W::zero()));

    // println!("starting pq");
    while !pq.is_empty() {
        let (u, _) = pq.pop().unwrap();
        // println!("popped {}", u);
        let uu = u.as_();
        let d = dists[uu];
        for v in g.out_neighbors(u) {
            let vu = (*v).as_();
            let alt = d + weights(u, *v);
            if !visited[vu] {
                unsafe {
                    visited.set_unchecked(vu, true);
                }
                dists[vu] = alt;
                parents[vu] = u;
                pq.push(*v, OrderedFloat(alt));
            } else if alt < dists[vu] {
                dists[vu] = alt;
                parents[vu] = u;
                pq.change_priority(v, OrderedFloat(alt));
                // pq.push(*v, OrderedFloat(alt));
            }
        }
    }
    parents[vu] = V::max_value();
    dists
}

