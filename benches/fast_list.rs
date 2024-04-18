use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(feature = "unstable")]
use linked_list::{LinkedListWalker, Walker};

use fast_list::LinkedList as FastLinkedList;
use std::collections::LinkedList as StdLinkedList;
use std::collections::VecDeque;
use std::vec::Vec;

/// Long and complicated code to make the following benchmark functions slightly shorter.
///
/// Calls the `$init` block in the start of the loop, sets up a loop through `0..$x`
/// and then calls the `$inner` block.
///
/// ~If only one block is given, it is assumed to be the inner block and the outer block
/// defaults to initating a list based on the test name `$s` (if it starts with "std_" or "fast_")~
macro_rules! bench {
    // ($g: tt, $s: tt, $x: expr, {$($inner:tt)+}) => {
    //     #[allow(redundant_semicolons, unused_variables)]

    //     $g.bench_function($s, |b| {
    //         #[allow(unused_variables, unused_mut)]
    //         let mut list = ();
    //         if $s.starts_with("fast_") {
    //             b.iter(|| {
    //                 let mut list = FastLinkedList::new();
    //                 list.extend(0..$x);
    //                 for i in 0..$x {
    //                     $($inner)*
    //                 }
    //             });
    //         } else {
    //             b.iter(|| {
    //                 let mut list = StdLinkedList::new();
    //                 list.extend(0..$x);
    //                 for i in 0..$x {
    //                     $($inner)*
    //                 }
    //             });
    //         }

    //     });
    // };
    ($g: tt, $s: tt, $x: expr, {$($init:tt)+}, {$($inner:tt)+}) => {
        #[allow(redundant_semicolons, unused_variables)]
        $g.bench_function($s, |b| {
            #[allow(unused_variables, unused_mut)]
            b.iter(|| {
                $($init)*
                for i in 0..$x {
                    $($inner)*
                }
            });
        });
    };
}

pub fn bench_list_push_back(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    // 1000
    let mut group = c.benchmark_group("list_push_back_1k");
    bench!(
        group,
        "fast_list_push_back_1k",
        1000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_list_push_back_1k",
        1000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    group.finish();

    let mut group = c.benchmark_group("list_push_back_10k");

    // 10_000
    bench!(
        group,
        "fast_list_push_back_10k",
        10_000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_list_push_back_10k",
        10_000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_vec_deque_push_back_10k",
        10_000,
        {
            let mut list = VecDeque::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_vec_push_back_10k",
        10_000,
        {
            let mut list = Vec::new();
        },
        {
            list.push(i);
        }
    );
    group.finish();

    // 100_000
    let mut group = c.benchmark_group("list_push_back_100k");
    bench!(
        group,
        "fast_list_push_back_100k",
        100_000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_vec_deque_push_back_100k",
        100_000,
        {
            let mut list = VecDeque::new();
        },
        {
            list.push_back(i);
        }
    );
    bench!(
        group,
        "std_vec_push_back_100k",
        100_000,
        {
            let mut list = Vec::new();
        },
        {
            list.push(i);
        }
    );
    bench!(
        group,
        "std_list_push_back_100k",
        100_000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_back(i);
        }
    );
    group.finish();
}

pub fn bench_list_push_front(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    // 1000
    let mut group = c.benchmark_group("list_push_front_1k");
    bench!(
        group,
        "fast_list_push_front_1k",
        1000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_list_push_front_1k",
        1000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    group.finish();

    let mut group = c.benchmark_group("list_push_front_10k");

    // 10_000
    bench!(
        group,
        "fast_list_push_front_10k",
        10_000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_list_push_front_10k",
        10_000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_vec_deque_push_front_10k",
        10_000,
        {
            let mut list = VecDeque::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_vec_push_front_10k",
        10_000,
        {
            let mut list = Vec::new();
        },
        {
            list = vec![vec![i], list].concat();
        }
    );
    group.finish();

    // 100_000
    let mut group = c.benchmark_group("list_push_front_100k");
    bench!(
        group,
        "fast_list_push_front_100k",
        100_000,
        {
            let mut list = FastLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_list_push_front_100k",
        100_000,
        {
            let mut list = StdLinkedList::new();
        },
        {
            list.push_front(i);
        }
    );
    bench!(
        group,
        "std_vec_deque_push_front_100k",
        100_000,
        {
            let mut list = VecDeque::new();
        },
        {
            list.push_front(i);
        }
    );
    //  bench!(group, "std_vec_push_front_100k", 100_000,
    //      { let mut list = Vec::new();},
    //      { list = vec![vec![i], list].concat(); }
    //  );
    group.finish();
}

pub fn bench_list_pop_back(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    // 1000
    let mut group = c.benchmark_group("list_pop_back_1k");
    bench!(
        group,
        "fast_list_pop_back_1k",
        1000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_list_pop_back_1k",
        1000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..1000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    group.finish();

    let mut group = c.benchmark_group("list_pop_back_10k");

    // 10_000
    bench!(
        group,
        "fast_list_pop_back_10k",
        10_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_list_pop_back_10k",
        10_000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_deque_pop_back_10k",
        10_000,
        {
            let mut list = VecDeque::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_pop_back_10k",
        10_000,
        {
            let mut list = Vec::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop().expect("pop_back returned None");
            }
        }
    );
    group.finish();

    // 100_000
    let mut group = c.benchmark_group("list_pop_back_100k");
    bench!(
        group,
        "fast_list_pop_back_100k",
        100_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_deque_pop_back_100k",
        100_000,
        {
            let mut list = VecDeque::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_pop_back_100k",
        100_000,
        {
            let mut list = Vec::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list = list.drain(1..).collect();
            }
        }
    );
    bench!(
        group,
        "std_list_pop_back_100k",
        100_000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_back().expect("pop_back returned None");
            }
        }
    );
    group.finish();
}

pub fn bench_list_pop_front(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    // 1000
    let mut group = c.benchmark_group("list_pop_front_1k");
    bench!(
        group,
        "fast_list_pop_front_1k",
        1000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_list_pop_front_1k",
        1000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..1000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    group.finish();

    let mut group = c.benchmark_group("list_pop_front_10k");

    // 10_000
    bench!(
        group,
        "fast_list_pop_front_10k",
        10_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_list_pop_front_10k",
        10_000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_deque_pop_front_10k",
        10_000,
        {
            let mut list = VecDeque::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_pop_front_10k",
        10_000,
        {
            let mut list = Vec::new();
            list.extend(0..10_000);
        },
        {
            if i > 0 {
                list = list.drain(1..).collect();
            }
        }
    );
    group.finish();

    // 100_000
    let mut group = c.benchmark_group("list_pop_front_100k");
    bench!(
        group,
        "fast_list_pop_front_100k",
        100_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_deque_pop_front_100k",
        100_000,
        {
            let mut list = VecDeque::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    bench!(
        group,
        "std_vec_pop_front_100k",
        100_000,
        {
            let mut list = Vec::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list = list.drain(1..).collect();
            }
        }
    );
    bench!(
        group,
        "std_list_pop_front_100k",
        100_000,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..100_000);
        },
        {
            if i > 0 {
                list.pop_front().expect("pop_front returned None");
            }
        }
    );
    group.finish();
}

pub fn bench_list_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_iter_1k");
    bench!(
        group,
        "fast_list_iter_1k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1000);
        },
        {
            let count = list.iter().count();
            assert!(count == 1000);
        }
    );
    bench!(
        group,
        "std_list_iter_1k",
        1,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..1000);
        },
        {
            let count = list.iter().count();
            assert!(count == 1000);
        }
    );
    group.finish();

    // 10_000
    let mut group = c.benchmark_group("list_iter_10k");
    // fast-list
    #[cfg(feature = "unstable")]
    bench!(
        group,
        "fast_list_iter_walker_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let mut count = 1;
            let mut walker = LinkedListWalker::new(&list, list.head.unwrap(), false);
            while let Some(item) = walker.walk_next(&list) {
                count += 1;
            }
            assert_eq!(count, 10000);
        }
    );
    bench!(
        group,
        "fast_list_iter_unordered_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter_unordered().count();
            assert!(count == 10000);
        }
    );
    bench!(
        group,
        "fast_list_iter_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter_next(list.head.unwrap()).count();
            assert!(count == 10000);
        }
    );
    bench!(
        group,
        "fast_list_iter_conveniece_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().count();
            assert!(count == 10000);
        }
    );

    // std LinkedList
    bench!(
        group,
        "std_list_iter_10k",
        1,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().count();
            assert!(count == 10000);
        }
    );

    // std Vec
    bench!(
        group,
        "std_vec_iter_10k",
        1,
        {
            let mut list = Vec::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().count();
            assert!(count == 10000);
        }
    );

    // std VecDeque
    bench!(
        group,
        "std_vec_deque_iter_10k",
        1,
        {
            let mut list = VecDeque::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().count();
            assert!(count == 10000);
        }
    );
    group.finish();
}

pub fn bench_list_iter_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_iter_reverse_1k");
    bench!(
        group,
        "fast_list_iter_reverse_1k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1000);
        },
        {
            let count = list.iter_prev(list.tail.unwrap()).count();
            assert!(count == 1000);
        }
    );
    bench!(
        group,
        "std_list_iter_reverse_1k",
        1,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..1000);
        },
        {
            let count = list.iter().rev().count();
            assert!(count == 1000);
        }
    );
    group.finish();

    // 10_000
    let mut group = c.benchmark_group("list_iter_reverse_10k");
    // fast-list
    #[cfg(feature = "unstable")]
    bench!(
        group,
        "fast_list_iter_reverse_walker_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let mut count = 1;
            let mut walker = LinkedListWalker::new(&list, list.tail.unwrap(), true);
            while let Some(item) = walker.walk_next(&list) {
                count += 1;
            }
            assert_eq!(count, 10000);
        }
    );
    bench!(
        group,
        "fast_list_iter_reverse_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter_prev(list.tail.unwrap()).count();
            assert!(count == 10000);
        }
    );

    // std LinkedList
    bench!(
        group,
        "std_list_iter_reverse_10k",
        1,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().rev().count();
            assert!(count == 10000);
        }
    );

    // std Vec
    bench!(
        group,
        "std_vec_iter_reverse_10k",
        1,
        {
            let mut list = Vec::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().rev().count();
            assert!(count == 10000);
        }
    );

    // std VecDeque
    bench!(
        group,
        "std_vec_deque_iter_reverse_10k",
        1,
        {
            let mut list = VecDeque::new();
            list.extend(0..10000);
        },
        {
            let count = list.iter().rev().count();
            assert!(count == 10000);
        }
    );
    group.finish();
}

pub fn bench_list_split_off(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    let mut group = c.benchmark_group("list_split_off_10k");

    // fast list
    bench!(
        group,
        "fast_list_split_off_10k",
        1,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..10_000);
        },
        {
            let mut z = 0;
            while list.len() > 1000 {
                assert_eq!(list.len(), 10_000 - (z) * 1000);
                list = list.split_off(list.nth(1000).unwrap());
                z += 1;
            }
        }
    );

    // std LinkedList
    bench!(
        group,
        "std_list_split_off_10k",
        1,
        {
            let mut list = StdLinkedList::new();
            list.extend(0..10_000);
        },
        {
            let mut z = 0;
            while list.len() > 1000 {
                assert_eq!(list.len(), 10_000 - (z) * 1000);
                list = list.split_off(1000);
                z += 1;
            }
        }
    );

    // std Vec
    bench!(
        group,
        "std_vec_split_off_10k",
        1,
        {
            let mut list = Vec::new();
            list.extend(0..10_000);
        },
        {
            let mut z = 0;
            while list.len() > 1000 {
                assert_eq!(list.len(), 10_000 - (z) * 1000);
                list = list.split_off(1000);
                z += 1;
            }
        }
    );

    // std VecDeque
    bench!(
        group,
        "std_vec_deque_split_off_10k",
        1,
        {
            let mut list = VecDeque::new();
            list.extend(0..10_000);
        },
        {
            let mut z = 0;
            while list.len() > 1000 {
                assert_eq!(list.len(), 10_000 - (z) * 1000);
                list = list.split_off(1000);
                z += 1;
            }
        }
    );
    group.finish();
}

pub fn bench_list_insert(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("list_insert_1k");

    // 1_000
    bench!(
        group,
        "fast_list_insert_1k",
        1_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1_000);
        },
        {
            list.insert_after(list.nth(list.len() / 2).unwrap(), i);
        }
    );
    bench!(
        group,
        "fast_list_known_index_insert_1k",
        1_000,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1_000);
            let j = list.nth(list.len() / 2).unwrap();
        },
        {
            list.insert_after(j, i);
        }
    );
    // I'm not counting this benchmark because there has to be some better way to do this.
    //  bench!(group, "std_list_known_index_insert_1k", 1_000,
    //     {
    //         let mut list = StdLinkedList::new(); list.extend(0..1_000);
    //         let j = list.len()/2;
    //      },
    //     {
    //         let second_half = list.split_off(j);
    //         list.push_back(i);
    //         list.extend(second_half);
    //     }
    //  );
    bench!(
        group,
        "std_vec_deque_insert_1k",
        1_000,
        {
            let mut list = VecDeque::new();
            list.extend(0..1_000);
        },
        {
            list.insert(list.len() / 2, i);
        }
    );
    bench!(
        group,
        "std_vec_insert_1k",
        1_000,
        {
            let mut list = Vec::new();
            list.extend(0..1_000);
        },
        {
            list.insert(list.len() / 2, i);
        }
    );
    group.finish();
}

pub fn bench_list_remove(c: &mut Criterion) {
    // Just putting this here so I don't get out of scope errors because of the bench! macro.
    let i = 0;

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("list_remove_1k");

    // 1_000
    bench!(
        group,
        "fast_list_remove_1k",
        499,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1_000);
        },
        {
            list.remove(list.nth(list.len() / 2).unwrap());
        }
    );
    bench!(
        group,
        "fast_list_known_index_remove_1k",
        499,
        {
            let mut list = FastLinkedList::new();
            list.extend(0..1_000);
            let j = list.nth(list.len() / 2).unwrap();
        },
        {
            list.remove(j);
        }
    );
    // There has to be some better way to do this...
    // bench!(group, "std_list_known_index_remove_1k", 499,
    //     {
    //         let mut list = StdLinkedList::new(); list.extend(0..1_000);
    //         let j = list.len()/2;
    //      },
    //     {
    //         let second_half = list.split_off(j);
    //         list.push_back(i);
    //         list.extend(second_half.iter().skip(1));
    //     }
    //  );
    bench!(
        group,
        "std_vec_deque_remove_1k",
        499,
        {
            let mut list = VecDeque::new();
            list.extend(0..1_000);
            let j = list.len() / 2;
        },
        {
            list.remove(j);
        }
    );
    bench!(
        group,
        "std_vec_remove_1k",
        499,
        {
            let mut list = Vec::new();
            list.extend(0..1_000);
            let j = list.len() / 2;
        },
        {
            list.remove(j);
        }
    );
    group.finish();
}

criterion_group!(
    benches,
    bench_list_remove,
    bench_list_insert,
    bench_list_push_back,
    bench_list_push_front,
    bench_list_pop_back,
    bench_list_pop_front,
    bench_list_iter,
    bench_list_iter_reverse,
    bench_list_split_off
);

criterion_main!(benches);
