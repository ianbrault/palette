/*
 * src/kdtree.rs
 * implements a k-d tree for 3 dimensional pixels
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use crate::pixel::Pixel;

type KDTreeNode = Option<Box<KDTree>>;

pub struct KDTree {
    pixel: Pixel,
    left: KDTreeNode,
    right: KDTreeNode,
}

impl KDTree {
    fn new_node(pixel: Pixel, left: KDTreeNode, right: KDTreeNode) -> Box<KDTree> {
        Box::new(KDTree { pixel, left, right })
    }

    fn new_rec(dimension: u8, data: &mut [Pixel]) -> KDTreeNode {
        match data.len() {
            0 => None,
            1 => Some(KDTree::new_node(data[0].clone(), None, None)),
            n => {
                // sort by current dimension
                if dimension == 0 {
                    data.sort_by(|p1, p2| p1.r.cmp(&p2.r));
                } else if dimension == 1 {
                    data.sort_by(|p1, p2| p1.g.cmp(&p2.g));
                } else {
                    data.sort_by(|p1, p2| p1.b.cmp(&p2.b));
                }
                // select pivot and recursively build subtrees
                let pivot = n / 2;
                let next_dimension = (dimension + 1) % 3;
                let left = KDTree::new_rec(next_dimension, &mut data[0..pivot]);
                let right = KDTree::new_rec(next_dimension, &mut data[(pivot+1)..]);

                Some(KDTree::new_node(data[pivot].clone(), left, right))
            },
        }
    }

    pub fn new(data: &mut [Pixel]) -> KDTree {
        let n = data.len();
        let pivot = n / 2;

        // sort by first dimension
        data.sort_by(|p1, p2| p1.r.cmp(&p2.r));
        // select pivot and recursively build subtrees
        let pixel = data[pivot].clone();
        let left = KDTree::new_rec(1, &mut data[0..pivot]);
        let right = KDTree::new_rec(1, &mut data[(pivot+1)..]);

        KDTree { pixel, left, right }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_pixels() -> Vec<Pixel> {
        vec![
            Pixel::new(1,4,7), Pixel::new(6,0,1), Pixel::new(2,5,3),
            Pixel::new(4,4,4), Pixel::new(7,8,9), Pixel::new(3,2,1),
        ]
    }

    #[test]
    fn test_new() {
        let mut data = get_test_pixels();
        let kdtree = KDTree::new(&mut data);

        assert_eq!(kdtree.pixel, Pixel::new(4,4,4));
        assert!(kdtree.left.is_some());
        assert!(kdtree.right.is_some());

        let left = kdtree.left.unwrap();
        assert_eq!(left.pixel, Pixel::new(1,4,7));
        assert!(left.left.is_some());
        assert!(left.right.is_some());

        let right = kdtree.right.unwrap();
        assert_eq!(right.pixel, Pixel::new(7,8,9));
        assert!(right.left.is_some());
        assert!(right.right.is_none());

        let left_left = left.left.unwrap();
        assert_eq!(left_left.pixel, Pixel::new(3,2,1));
        assert!(left_left.left.is_none());
        assert!(left_left.right.is_none());

        let left_right = left.right.unwrap();
        assert_eq!(left_right.pixel, Pixel::new(2,5,3));
        assert!(left_right.left.is_none());
        assert!(left_right.right.is_none());

        let right_left = right.left.unwrap();
        assert_eq!(right_left.pixel, Pixel::new(6,0,1));
        assert!(right_left.left.is_none());
        assert!(right_left.right.is_none());
    }
}
