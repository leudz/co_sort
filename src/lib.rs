//! The simplest way to use this crate is the co_sort! and co_sort_stable! macros, it will sort the first array and swap elements of the others in order to mimic the changes in the first array. \
//! Usefull when you have multiple slices with an implicit relation.
//! ```
//! #[macro_use] extern crate co_sort;
//! use co_sort::*;
//! let mut names = ["Diego", "Maia", "Luciana", "Bruno", "Astrid", "Thierry"];
//! let mut ages =  [  73,      88,      21,        47,      4,        62    ];
//! // We want to sort the names but keep the ages synced
//! co_sort![names, ages];
//! assert_eq!(names, ["Astrid", "Bruno", "Diego", "Luciana", "Maia", "Thierry"]);
//! assert_eq!(ages,  [   4,       47,      73,       21,       88,      62    ]);
//! ```
//! If you want more control you can use the functions co_sort and co_sort_stable. \
//! co_sort_stable allocates O(n) memory and requires the types to implement Clone while co_sort is in place and doesn't require any trait. \
//! Performance wise co_sort scale well with the number of arrays but not with their size and co_sort_stable is the opposite.
//! ```
//! # use co_sort::*;
//! let mut names = ["Diego", "Maia", "Luciana", "Bruno", "Astrid", "Thierry"];
//! let mut ages =  [  73,      88,      21,       47,       4,        62    ];
//! let permutation = Permutation::from(names.as_ref());
//! permutation.co_sort((names.as_mut(), ages.as_mut()));
//! // or permutation.co_sort_stable((names.as_mut(), ages.as_mut()));
//! assert_eq!(names, ["Astrid", "Bruno", "Diego", "Luciana", "Maia", "Thierry"]);
//! assert_eq!(ages,  [   4,       47,      73,       21,       88,      62    ]);
//! ```

#[derive(Debug, Clone)]
pub struct Permutation(Vec<usize>);

impl Permutation {
    pub fn co_sort<T: CoSort>(&self, to_sort: T) {
        to_sort.co_sort_with(&self.0);
    }
    pub fn co_sort_stable<T: CoSortStable>(&self, to_sort: T) {
        to_sort.co_sort_stable_with(&self.0);
    }
}

impl<T: Ord> From<&[T]> for Permutation {
    fn from(slice: &[T]) -> Permutation {
        let mut order = (0..slice.len()).collect::<Vec<_>>();
        order.sort_unstable_by_key(|&i| &slice[i]);
        Permutation(order)
    }
}

impl std::ops::Deref for Permutation {
    type Target = [usize];
    fn deref(&self) -> &[usize] {
        &self.0
    }
}

impl std::ops::DerefMut for Permutation {
    fn deref_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
}

/// If you want to implement CoSort on your type nothing simpler.
/// ```
/// # use co_sort::*;
/// struct MyStruct(Vec<i32>);
/// impl CoSort for MyStruct {
///     fn co_sort_with(mut self, order: &[usize]) {
///         self.0.as_mut_slice().co_sort_with(order);
///     }
/// }
/// ```
pub trait CoSort {
    fn co_sort_with(self, order: &[usize]);
}

impl<T> CoSort for &mut [T] {
    fn co_sort_with(self, order: &[usize]) {
        assert_eq!(self.len(), order.len());
        let mut pos;
        for i in 0..order.len() {
            pos = unsafe { *order.get_unchecked(i) };
            while pos < i {
                pos = unsafe { *order.get_unchecked(pos) };
            }
            self.swap(i, pos);
        }
    }
}

impl<T> CoSort for (&mut [T],) {
    fn co_sort_with(self, order: &[usize]) {
        assert_eq!(self.0.len(), order.len());
        let mut pos;
        for i in 0..order.len() {
            pos = unsafe { *order.get_unchecked(i) };
            while pos < i {
                pos = unsafe { *order.get_unchecked(pos) };
            }
            self.0.swap(i, pos);
        }
    }
}

macro_rules! impl_co_sort {
    ($(($type: ident, $index: tt)),+; ($type1: ident, $index1: tt), $($tail: tt)*) => [
        impl<$type1, $($type),+> CoSort for (&mut [$type1], $(&mut [$type]),+) {
            fn co_sort_with(self, order: &[usize]) {
                assert_eq!(order.len(), self.$index1.len());
                $(
                    assert_eq!(order.len(), self.$index.len());
                )+
                let mut pos;
                for i in 0..order.len() {
                    pos = unsafe {*order.get_unchecked(i)};
                    while pos < i {
                        pos = unsafe {*order.get_unchecked(pos)};
                    }
                    self.$index1.swap(i, pos);
                    $(
                        self.$index.swap(i, pos);
                    )+
                }
            }
        }
        impl_co_sort![$(($type, $index),)+ ($type1, $index1); $($tail)*];
    ];
    ($(($type: ident, $index: tt)),+;) => [];
}

/// If you want to implement CoSortStable on your type nothing simpler.
/// ```
/// # use co_sort::*;
/// struct MyStruct(Vec<i32>);
/// impl CoSortStable for MyStruct {
///     fn co_sort_stable_with(mut self, order: &[usize]) {
///         self.0.as_mut_slice().co_sort_with(order);
///     }
/// }
/// ```
pub trait CoSortStable {
    fn co_sort_stable_with(self, order: &[usize]);
}

impl<U: Clone> CoSortStable for &mut [U] {
    fn co_sort_stable_with(self, order: &[usize]) {
        assert_eq!(order.len(), self.len());
        let copy = (0..self.len())
            .map(|i| unsafe { self.get_unchecked(*order.get_unchecked(i)) }.clone())
            .collect::<Vec<_>>();
        for (i, j) in copy.into_iter().enumerate() {
            unsafe { *self.get_unchecked_mut(i) = j };
        }
    }
}

impl<U: Clone> CoSortStable for (&mut [U],) {
    fn co_sort_stable_with(self, order: &[usize]) {
        assert_eq!(order.len(), self.0.len());
        let copy = (0..self.0.len())
            .map(|i| unsafe { self.0.get_unchecked(*order.get_unchecked(i)) }.clone())
            .collect::<Vec<_>>();
        for (i, j) in copy.into_iter().enumerate() {
            unsafe { *self.0.get_unchecked_mut(i) = j };
        }
    }
}

macro_rules! impl_co_sort_stable {
    ($(($type: ident, $index: tt)),+; ($type1: ident, $index1: tt), $($tail: tt)*) => [
        impl<$($type: Clone,)+ $type1: Clone> CoSortStable for ($(&mut [$type],)+ &mut [$type1]) {
            fn co_sort_stable_with(self, order: &[usize]) {
                assert_eq!(order.len(), self.$index1.len());
                $(
                    assert_eq!(order.len(), self.$index.len());
                )+
                $(
                    let copy = (0..self.$index.len()).map(|i| unsafe {self.$index.get_unchecked(*order.get_unchecked(i))}.clone()).collect::<Vec<_>>();
                    for (i, j) in copy.into_iter().enumerate() {
                        unsafe {*self.$index.get_unchecked_mut(i) = j};
                    }
                )+
                let copy = (0..self.$index1.len()).map(|i| unsafe {self.$index1.get_unchecked(*order.get_unchecked(i))}.clone()).collect::<Vec<_>>();
                for (i, j) in copy.into_iter().enumerate() {
                    unsafe {*self.$index1.get_unchecked_mut(i) = j};
                }
            }
        }
        impl_co_sort_stable![$(($type, $index),)+ ($type1, $index1); $($tail)*];
    ];
    ($(($type: ident, $index: tt)),+;) => [];
}

macro_rules! impl_traits {
    ($(($type: ident, $index: tt)),+; $($tail: tt)*) => [
        impl_co_sort![$(($type, $index)),+; $($tail)*];
        impl_co_sort_stable![$(($type, $index)),+; $($tail)*];
    ]
}
impl_traits![(A, 0); (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11),];

#[macro_export]
macro_rules! co_sort {
    ($slice: ident $(,)*) => [
        $slice.sort_unstable();
    ];
    ($slice1: ident $(, $slice: expr)+) => [
        Permutation::from(&$slice1[..]).co_sort((&mut $slice1[..], $(&mut $slice[..]),+))
    ];
}

#[macro_export]
macro_rules! co_sort_stable {
    ($slice: ident $(,)*) => [
        $slice.sort();
    ];
    ($slice1: ident $(, $slice: expr)+) => [
        Permutation::from(&$slice1[..]).co_sort_stable((&mut $slice1[..], $(&mut $slice[..]),+))
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort() {
        let order = Permutation::from([0, 5, 2, 4, 3, 1].as_ref());
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut slice2 = [147, 6478, 555, 264, 8, 52];
        order.co_sort((&mut slice1[..], &mut slice2[..]));
        assert_eq!(slice1, ['a', 'f', 'c', 'e', 'd', 'b']);
        assert_eq!(slice2, [147, 52, 555, 8, 264, 6478]);
    }
    #[test]
    fn sort_stable() {
        let slice0 = [0, 5, 2, 4, 3, 1];
        let order = Permutation::from(slice0.as_ref());
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut slice2 = [147, 6478, 555, 264, 8, 52];
        order.co_sort_stable((&mut slice1[..], &mut slice2[..]));
        assert_eq!(slice0, [0, 5, 2, 4, 3, 1]);
        assert_eq!(*order, [0, 5, 2, 4, 3, 1]);
        assert_eq!(slice1, ['a', 'f', 'c', 'e', 'd', 'b']);
        assert_eq!(slice2, [147, 52, 555, 8, 264, 6478]);
    }
    #[test]
    #[should_panic]
    fn len() {
        let order = Permutation::from([0, 5, 2, 4, 3, 1].as_ref());
        let mut slice1 = ['a', 'b', 'c', 'd', 'e'];
        order.co_sort(&mut slice1[..]);
    }
    #[test]
    fn macro_test() {
        let mut order = [0, 5, 2, 4, 3, 1];
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut slice2 = [147, 6478, 555, 264, 8, 52];
        co_sort![order, slice1, slice2];
        assert_eq!(slice1, ['a', 'f', 'c', 'e', 'd', 'b']);
        assert_eq!(slice2, [147, 52, 555, 8, 264, 6478]);
    }
    #[test]
    fn macro_test_stable() {
        let mut order = [0, 5, 2, 4, 3, 1];
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut slice2 = [147, 6478, 555, 264, 8, 52];
        co_sort_stable![order, slice1, slice2];
        assert_eq!(slice1, ['a', 'f', 'c', 'e', 'd', 'b']);
        assert_eq!(slice2, [147, 52, 555, 8, 264, 6478]);
    }
}
