/*
 * src/kmeans.rs
 * implements a k-means clustering algorithm
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use std::cmp;

use rand::prelude::*;
use rand::distributions::{Uniform, WeightedIndex};

// generic vector used in the k-means clustering algorithm
pub trait GenericVector<Element=Self>: Clone {
    fn average(vectors: &Vec<Element>) -> Element;
    fn distance(&self, other: &Element) -> u32;
}


// used to store a GenericVector alongside its cluster assignment
struct ClusterVector<V> where V: GenericVector {
    assignment: i32,
    vector: V,
}

impl<V> ClusterVector<V> where V: GenericVector {
    fn new(v: V) -> ClusterVector<V> {
        ClusterVector {
            assignment: -1,
            vector: v,
        }
    }

    fn from_vectors(data: Vec<V>) -> Vec<ClusterVector<V>> {
        data.into_iter().map(|v| ClusterVector::new(v)).collect()
    }
}


fn update_weights<V>(weights: &mut Vec<u32>, vec: &V, data: &Vec<V>)
    where V: GenericVector
{
    for (i, v) in data.iter().enumerate() {
        weights[i] = cmp::min(weights[i], vec.distance(v));
    }
}

// k-means++ implementation
pub fn k_means_pp<V>(k: u32, data: &Vec<V>) -> Vec<V>
    where V: GenericVector
{
    let mut centers = Vec::with_capacity(k as usize);
    // the weight for each vector is the minimum distance to a previously-generated center
    let mut weights = vec![std::u32::MAX; data.len()];

    let mut rng = thread_rng();
    let udist = Uniform::new(0, data.len());

    let initial = data[rng.sample(udist)].clone();
    update_weights(&mut weights, &initial, data);
    centers.push(initial);

    for _ in 1..k {
        let dist = WeightedIndex::new(&weights).unwrap();
        let center = data[dist.sample(&mut rng)].clone();
        update_weights(&mut weights, &center, data);
        centers.push(center);
    }

    centers
}


fn min_index(vec: &Vec<i64>) -> usize {
    let mut min = std::i64::MAX;
    let mut min_index: i32 = -1;

    for (i, v) in vec.iter().enumerate() {
        if v < &min {
            min = *v;
            min_index = i as i32;
        }
    }

    min_index as usize
}

fn assign_centers<V>(centers: &Vec<V>, cluster_vecs: &mut Vec<ClusterVector<V>>) -> bool
    where V: GenericVector
{
    let mut change_made = false;
    for cv in cluster_vecs.iter_mut() {
        let closest = min_index(&centers.iter().map(|c| cv.vector.distance(c) as i64).collect()) as i32;

        if cv.assignment != closest {
            cv.assignment = closest;
            change_made = true;
        }
    }

    change_made
}

fn update_centers<V>(centers: Vec<V>, cluster_vecs: &Vec<ClusterVector<V>>) -> Vec<V>
    where V: GenericVector
{
    let n_centers = centers.len();
    let mut new_centers = Vec::with_capacity(n_centers);

    for i in 0..n_centers {
        let cluster = cluster_vecs.iter()
            .filter(|cv| cv.assignment == i as i32)
            .map(|cv| cv.vector.clone())
            .collect();
        new_centers.push(V::average(&cluster));
    }

    new_centers
}

// k-means clustering implementation
pub fn k_cluster<V>(k: u32, data: Vec<V>) -> Vec<V>
    where V: GenericVector
{
    let mut centers = k_means_pp(k, &data);
    let mut cluster_vecs = ClusterVector::from_vectors(data);

    let mut change_made = true;
    while change_made {
        change_made = assign_centers(&centers, &mut cluster_vecs);
        centers = update_centers(centers, &cluster_vecs);
    }

    centers
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixel::Pixel;

    #[test]
    fn test_update_weights() {
        let data = vec![
            Pixel::new(0,0,0), Pixel::new(4,5,6), Pixel::new(12,8,20), Pixel::new(16,1,42), Pixel::new(20, 12, 2)
        ];
        let mut weights = vec![std::u32::MAX; data.len()];

        let center1 = &data[1];
        update_weights(&mut weights, center1, &data);
        assert_eq!(weights, vec![77, 0, 269, 1456, 321]);

        let center2 = &data[3];
        update_weights(&mut weights, center2, &data);
        assert_eq!(weights, vec![77, 0, 269, 0, 321]);
    }
}
