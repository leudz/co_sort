#[macro_use]
extern crate criterion;
#[macro_use]
extern crate co_sort;

use co_sort::*;
use criterion::Criterion;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sort small", |b| {
        let order = Permutation::from([0, 7, 5, 6, 4, 2, 1, 3].as_ref());
        let mut slice = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        b.iter(|| order.co_sort(slice.as_mut()))
    });
    c.bench_function("sort stable small", |b| {
        let order = Permutation::from([0, 7, 5, 6, 4, 2, 1, 3].as_ref());
        let mut slice = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        b.iter(|| order.co_sort_stable(slice.as_mut()))
    });
    c.bench_function("sort small lots", |b| {
        let order = Permutation::from([0, 7, 5, 6, 4, 2, 1, 3].as_ref());
        let mut slice = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice2 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice3 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice4 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice5 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice6 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice7 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice8 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice9 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        b.iter(|| {
            order.co_sort((
                slice.as_mut(),
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort stable small lots", |b| {
        let order = Permutation::from([0, 7, 5, 6, 4, 2, 1, 3].as_ref());
        let mut slice = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice1 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice2 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice3 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice4 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice5 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice6 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice7 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice8 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut slice9 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        b.iter(|| {
            order.co_sort_stable((
                slice.as_mut(),
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort medium", |b| {
        let mut slice1 = (0..100).collect::<Vec<_>>();
        slice1.shuffle(&mut thread_rng());
        let order = Permutation::from(slice1.as_ref());
        let mut slice2 = vec!['a'; 100];
        b.iter(|| order.co_sort((slice1.as_mut(), slice2.as_mut())))
    });
    c.bench_function("sort stable medium", |b| {
        let mut slice1 = (0..100).collect::<Vec<_>>();
        slice1.shuffle(&mut thread_rng());
        let order = Permutation::from(slice1.as_ref());
        let mut slice2 = vec!['a'; 100];
        b.iter(|| order.co_sort_stable((slice1.as_mut(), slice2.as_mut())))
    });
    c.bench_function("sort medium lots", |b| {
        let mut slice = (0..100).collect::<Vec<_>>();
        slice.shuffle(&mut thread_rng());
        let order = Permutation::from(slice.as_ref());
        let mut slice1 = vec!['a'; 100];
        let mut slice2 = vec!['a'; 100];
        let mut slice3 = vec!['a'; 100];
        let mut slice4 = vec!['a'; 100];
        let mut slice5 = vec!['a'; 100];
        let mut slice6 = vec!['a'; 100];
        let mut slice7 = vec!['a'; 100];
        let mut slice8 = vec!['a'; 100];
        let mut slice9 = vec!['a'; 100];
        b.iter(|| {
            order.co_sort((
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort stable medium lots", |b| {
        let mut slice = (0..100).collect::<Vec<_>>();
        slice.shuffle(&mut thread_rng());
        let order = Permutation::from(slice.as_ref());
        let mut slice1 = vec!['a'; 100];
        let mut slice2 = vec!['a'; 100];
        let mut slice3 = vec!['a'; 100];
        let mut slice4 = vec!['a'; 100];
        let mut slice5 = vec!['a'; 100];
        let mut slice6 = vec!['a'; 100];
        let mut slice7 = vec!['a'; 100];
        let mut slice8 = vec!['a'; 100];
        let mut slice9 = vec!['a'; 100];
        b.iter(|| {
            order.co_sort_stable((
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort big", |b| {
        let mut slice1 = (0..10000).collect::<Vec<_>>();
        slice1.shuffle(&mut thread_rng());
        let order = Permutation::from(slice1.as_ref());
        let mut slice2 = vec!['a'; 10000];
        b.iter(|| order.co_sort((slice1.as_mut(), slice2.as_mut())))
    });
    c.bench_function("sort stable big", |b| {
        let mut slice1 = (0..10000).collect::<Vec<_>>();
        slice1.shuffle(&mut thread_rng());
        let order = Permutation::from(slice1.as_ref());
        let mut slice2 = vec!['a'; 10000];
        b.iter(|| order.co_sort_stable((slice1.as_mut(), slice2.as_mut())))
    });
    c.bench_function("sort big lots", |b| {
        let mut slice = (0..10000).collect::<Vec<_>>();
        slice.shuffle(&mut thread_rng());
        let order = Permutation::from(slice.as_ref());
        let mut slice1 = vec!['a'; 10000];
        let mut slice2 = vec!['a'; 10000];
        let mut slice3 = vec!['a'; 10000];
        let mut slice4 = vec!['a'; 10000];
        let mut slice5 = vec!['a'; 10000];
        let mut slice6 = vec!['a'; 10000];
        let mut slice7 = vec!['a'; 10000];
        let mut slice8 = vec!['a'; 10000];
        let mut slice9 = vec!['a'; 10000];
        b.iter(|| {
            order.co_sort((
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort stable big lots", |b| {
        let mut slice = (0..10000).collect::<Vec<_>>();
        slice.shuffle(&mut thread_rng());
        let order = Permutation::from(slice.as_ref());
        let mut slice1 = vec!['a'; 10000];
        let mut slice2 = vec!['a'; 10000];
        let mut slice3 = vec!['a'; 10000];
        let mut slice4 = vec!['a'; 10000];
        let mut slice5 = vec!['a'; 10000];
        let mut slice6 = vec!['a'; 10000];
        let mut slice7 = vec!['a'; 10000];
        let mut slice8 = vec!['a'; 10000];
        let mut slice9 = vec!['a'; 10000];
        b.iter(|| {
            order.co_sort_stable((
                slice1.as_mut(),
                slice2.as_mut(),
                slice3.as_mut(),
                slice4.as_mut(),
                slice5.as_mut(),
                slice6.as_mut(),
                slice7.as_mut(),
                slice8.as_mut(),
                slice9.as_mut(),
            ))
        })
    });
    c.bench_function("sort big macro", |b| {
        let mut slice1 = (0..10000).collect::<Vec<_>>();
        slice1.shuffle(&mut thread_rng());
        let mut slice2 = vec!['a'; 10000];
        b.iter(|| {
            let mut slice1 = slice1.clone();
            co_sort![slice1, slice2]
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
