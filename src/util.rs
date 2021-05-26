//! Utility functions that typically operate on common types

use itertools::{rev, sorted};
use std::cmp::{Eq, Ord};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/* TODO: work this out
pub trait Partition {
    fn partition<T, F, Label>(self, f: F) -> Vec<Vec<T>>
    where
        T: Clone,
        F: Fn(&T) -> Label,
        Label: Eq + Ord + Hash;
}
impl<Item> Partition for Vec<Item>
where
    Item: Clone,
{
    fn partition<T: Item, F, Label>(self, f: F) -> Vec<Vec<T>>
    where
        T: Clone,
        F: Fn(&T) -> Label,
        Label: Eq + Ord + Hash,
    {
        partition(f, self.into_iter())
    }
}
*/

/// partition accepts an iterator and a predicate function
/// and returns a Vec of Vecs that contains each group
///
/// The predicate function must accept an item from the iterator
/// and return a label value that is used to identify the group
///
/// The results are returned in an arbitrary order
pub fn partition<T, F, Label>(f: F, v: T) -> Vec<Vec<T::Item>>
where
    T: IntoIterator,
    T::Item: Clone,
    F: Fn(&T::Item) -> Label,
    Label: Eq + Ord + Hash,
{
    let mut ls: HashMap<Label, Vec<T::Item>> = HashMap::new();
    for i in v {
        let l = f(&i);
        ls.entry(l).or_insert(vec![]).push(i)
    }

    let keys = rev(sorted(ls.keys()));
    keys.map(|k| ls.get(k).unwrap()).cloned().collect()
}

/// partition_by accepts an iterator and a predicate function
/// and returns a Vec of Vecs that contains groups split by the predicate
///
/// The predicate function must accept an item from the iterator
/// and return a boolean value. True denotes an element that splits
/// the groups in the Iterator.
///
/// The results are returned in order
pub fn partition_by<T, F>(f: F, v: T) -> Vec<Vec<T::Item>>
where
    T: IntoIterator,
    T::Item: Clone + Debug,
    F: Fn(&T::Item) -> bool,
{
    v.into_iter().fold(vec![vec![]], |mut acc, n| {
        match f(&n) {
            true => acc.push(vec![]),
            false => acc.last_mut().expect("must not be null").push(n),
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let v: Vec<i32> = vec![1, 2, 3];
        let vs: Vec<Vec<i32>> = partition(|v| v % 2, v.into_iter());
        let actual_vs: Vec<Vec<i32>> = vec![vec![1, 3], vec![2]];
        assert_eq!(vs, actual_vs);
    }

    #[test]
    fn test_partition_struct() {
        #[derive(Clone, Debug, PartialEq)]
        struct Pt {
            x: u8,
            y: u8,
        }
        impl Eq for Pt {}

        let v = vec![Pt { x: 1, y: 2 }, Pt { x: 3, y: 4 }];
        let vs = partition(|v| v.x < 3, v.into_iter());
        let actual_vs = vec![vec![Pt { x: 1, y: 2 }], vec![Pt { x: 3, y: 4 }]];
        assert_eq!(vs, actual_vs);
    }

    #[test]
    fn test_partition_by() {
        let v: Vec<i32> = vec![1, 2, 3];
        let vs: Vec<Vec<i32>> = partition_by(|v| v % 2 == 0, v.into_iter());
        let actual_vs: Vec<Vec<i32>> = vec![vec![1], vec![3]];
        assert_eq!(vs, actual_vs);
    }
}
