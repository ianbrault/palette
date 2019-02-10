/*
 * src/kmeans.rs
 * implements a k-means clustering algorithm
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use std::cmp;

use rand::prelude::*;
use rand::distributions::{Uniform, WeightedIndex};

// generic vector used in the k-means clustering algorithm
pub trait GenericVector<Element=Self>: Clone + Sync {
    fn average<'a, I>(vectors: I) -> Element where Element: 'a, I: Iterator<Item=&'a Element>;
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
        data.into_iter().map(ClusterVector::new).collect()
    }
}


fn update_weights<V>(weights: &mut Vec<u64>, vec: &V, data: &[V])
    where V: GenericVector
{
    for (i, v) in data.iter().enumerate() {
        weights[i] = cmp::min(weights[i], u64::from(vec.distance(v)));
    }
}

// k-means++ implementation
pub fn k_means_pp<V>(k: u32, data: &[V]) -> Vec<V>
    where V: GenericVector
{
    let mut centers = Vec::with_capacity(k as usize);
    // the weight for each vector is the minimum distance to a previously-generated center
    let mut weights = vec![std::u64::MAX; data.len()];

    let mut rng = thread_rng();
    let udist = Uniform::new(0, data.len());

    // first center is chosen randomly
    let initial = data[rng.sample(udist)].clone();
    update_weights(&mut weights, &initial, data);
    centers.push(initial);

    // all other centers are chosen with a probability based on the distance to prior centers
    for _ in 1..k {
        let wdist = WeightedIndex::new(&weights).unwrap();
        let center = data[wdist.sample(&mut rng)].clone();
        update_weights(&mut weights, &center, data);
        centers.push(center);
    }

    centers
}


fn index_of_closest_center<V>(v: &V, centers: &[V]) -> i32
    where V: GenericVector
{
    let (index, _) = centers.iter()
        .enumerate()
        .fold((0, std::u32::MAX), |(acc_i, acc_min), (i, center)| {
            let dist = v.distance(center);
            if dist < acc_min {
                (i, dist)
            } else {
                (acc_i, acc_min)
            }
        });

    index as i32
}

fn assign_centers<V>(centers: &[V], cluster_vecs: &mut Vec<ClusterVector<V>>) -> bool
    where V: GenericVector
{
    let mut change_made = false;

    for cv in cluster_vecs.iter_mut() {
        let closest = index_of_closest_center(&cv.vector, &centers);
        if cv.assignment != closest {
            cv.assignment = closest;
            change_made = true;
        }
    }

    change_made
}

fn update_centers<V>(n_centers: u32, cluster_vecs: &[ClusterVector<V>]) -> Vec<V>
    where V: GenericVector
{
    let mut new_centers = Vec::with_capacity(n_centers as usize);

    for i in 0..n_centers {
        let cluster = cluster_vecs.iter()
            .filter(|cv| cv.assignment == i as i32)
            .map(|cv| &cv.vector);
        new_centers.push(V::average(cluster));
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
        centers = update_centers(k, &cluster_vecs);
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
        let mut weights = vec![std::u64::MAX; data.len()];

        let center1 = &data[1];
        update_weights(&mut weights, center1, &data);
        assert_eq!(weights, vec![77, 0, 269, 1456, 321]);

        let center2 = &data[3];
        update_weights(&mut weights, center2, &data);
        assert_eq!(weights, vec![77, 0, 269, 0, 321]);
    }
}
