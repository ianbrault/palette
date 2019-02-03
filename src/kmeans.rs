/*
 * src/kmeans.rs
 * implements a k-means clustering algorithm
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use std::cmp;
use std::fmt::Debug;

use rand::prelude::*;
use rand::distributions::{Uniform, WeightedIndex};

// generic vector used in the k-means clustering algorithm
pub trait KVector<Element=Self>: Clone + Debug {
    fn distance(&self, other: &Element) -> u32;
    fn average(vectors: &Vec<Element>) -> Element;
}

fn update_weights<KV>(weights: Vec<u32>, vec: &KV, data: &Vec<KV>) -> Vec<u32> where KV: KVector {
    let mut new_weights = Vec::with_capacity(weights.len());
    for (v, w) in data.iter().zip(weights.iter()) {
        new_weights.push(cmp::min(w, &vec.distance(v)).clone());
    }
    new_weights
}

// k-means++ implementation
pub fn k_means_pp<KV>(k: u32, data: &Vec<KV>) -> Vec<KV> where KV: KVector {
    let mut centers = Vec::with_capacity(k as usize);
    // the weight for each vector is the minimum distance to a previously-generated center
    let mut weights = vec![std::u32::MAX; data.len()];

    let mut rng = thread_rng();
    let udist = Uniform::new(0, data.len());

    let initial = data.get(rng.sample(udist)).unwrap().clone();
    weights = update_weights(weights, &initial, data);
    centers.push(initial);

    for _ in 1..k {
        let dist = WeightedIndex::new(&weights).unwrap();
        let center = data.get(dist.sample(&mut rng)).unwrap().clone();
        weights = update_weights(weights, &center, data);
        centers.push(center);
    }

    centers
}

// k-means clustering implementation
pub fn k_cluster<KV>(k: u32, data: Vec<KV>) where KV: KVector {
    let centroids = k_means_pp(k, &data);
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

        let center1 = data.get(1).unwrap();
        weights = update_weights(weights, center1, &data);
        assert_eq!(weights, vec![77, 0, 269, 1456, 321]);

        let center2 = data.get(3).unwrap();
        weights = update_weights(weights, center2, &data);
        assert_eq!(weights, vec![77, 0, 269, 0, 321]);
    }
}
