/*
 * src/kmeans.rs
 * implements a k-means clustering algorithm
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */


// generic vector used in the k-means clustering algorithm
pub trait KVector<Element=Self> {
    fn distance(&self, other: Element) -> u64;
    fn sq_average(vectors: Vec<Element>) -> Element;
}


pub fn k_cluster<KV>(centroids: Vec<KV>, vectors: Vec<KV>) where KV: KVector {

}
