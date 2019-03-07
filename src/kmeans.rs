/*
 * src/kmeans.rs
 * implements a k-means clustering algorithm
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use std::cmp;

use rand::prelude::*;
use rand::distributions::{Uniform, WeightedIndex};

use crate::pixel::Pixel;


// used to store a Pixel alongside its center assignment and the distance to it
struct ClusterVector {
    assignment: i32,
    min_distance: u32,
    vector: Pixel,
}

impl ClusterVector {
    fn new(vector: Pixel) -> ClusterVector {
        ClusterVector {
            assignment: -1,
            min_distance: std::u32::MAX,
            vector,
        }
    }

    fn from_vectors(data: Vec<Pixel>) -> Vec<ClusterVector> {
        data.into_iter().map(ClusterVector::new).collect()
    }
}


fn update_weights(weights: &mut Vec<u64>, vec: &Pixel, data: &[Pixel]) {
    for (i, v) in data.iter().enumerate() {
        weights[i] = cmp::min(weights[i], u64::from(vec.distance(v)));
    }
}

// k-means++ implementation
pub fn k_means_pp(k: u32, data: &[Pixel]) -> Vec<Pixel> {
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


fn assign_centers(centers: Vec<Pixel>, cluster_vecs: &mut Vec<ClusterVector>) -> bool {
    let mut n_changes = 0;

    for cv in cluster_vecs.iter_mut() {
        for (i, center) in centers.iter().enumerate() {
            let dist = cv.vector.distance(center);
            if dist < cv.min_distance {
                if cv.assignment != i as i32 {
                    n_changes += 1;
                    cv.assignment = i as i32;
                }
                cv.min_distance = dist;
            }
        }
    }

    // uses a 0.1% cutoff
    n_changes >= (cluster_vecs.len() / 100)
}

fn update_centers(n_centers: u32, cluster_vecs: &[ClusterVector]) -> Vec<Pixel> {
    let mut new_centers = Vec::with_capacity(n_centers as usize);

    for i in 0..n_centers {
        let cluster = cluster_vecs.iter()
            .filter(|cv| cv.assignment == i as i32)
            .map(|cv| &cv.vector);
        new_centers.push(Pixel::average(cluster));
    }

    new_centers
}

// k-means clustering implementation
pub fn k_cluster(k: u32, data: Vec<Pixel>) -> Vec<Pixel> {
    let mut centers = k_means_pp(k, &data);
    let mut cluster_vecs = ClusterVector::from_vectors(data);

    let mut change_made = true;
    while change_made {
        change_made = assign_centers(centers, &mut cluster_vecs);
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
