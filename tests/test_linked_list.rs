use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

use crossbeam::epoch::Atomic;

//use crate::LinkedListCell;

use fast_list::{LinkedList, LinkedListIndex};

#[test]
fn test_fn_push_back_fn_next_of_fn_prev_of() {
    let mut list = LinkedList::new();
    let a = list.push_back(1);
    let b = list.push_back(2);
    let c = list.push_back(3);

    assert!(list.prev_of(a).is_none());
    assert_eq!(list.get(a).unwrap().value, 1);
    assert_eq!(list.next_of(a).unwrap().value, 2);

    assert_eq!(list.prev_of(b).unwrap().value, 1);
    assert_eq!(list.get(b).unwrap().value, 2);
    assert_eq!(list.next_of(b).unwrap().value, 3);

    assert_eq!(list.prev_of(c).unwrap().value, 2);
    assert_eq!(list.get(c).unwrap().value, 3);
    assert!(list.next_of(c).is_none());
}

/// Demonstrates the ABA problem.
///
/// Note on locking threads:
/// Many real-world implementations of mutexes,
/// including std::sync::Mutex on some platforms,
/// briefly behave like a spin lock before asking the operating system to put a thread to sleep.
/// This is an attempt to combine the best of both worlds,
/// although it depends entirely on the specific use case whether this behavior is beneficial or not.
/// https://marabos.nl/atomics/building-spinlock.html
#[test]
fn test_multithreading() {
    let mut list = LinkedList::new();
    let indexes = Arc::new(list.extend(0..10_000));

    // You can also get the ordered indexes with something like this:
    // let indexes = Arc::new(
    //     list.cursor_next(list.head.unwrap()).collect::<Vec<_>>()
    // );

    let list_mut = Arc::new(Mutex::new(list));

    let mut threads = Vec::new();
    for _ in 0..3 {
        let list_mut = Arc::clone(&list_mut);
        let indexes = Arc::clone(&indexes);
        let t = thread::spawn(move || {
            for index in indexes.iter().take(9_000) {
                list_mut.lock().unwrap().remove(*index); // returns None if the index does not exist
            }
        });
        threads.push(t);
    }

    {
        assert_eq!(list_mut.lock().unwrap().head().unwrap().value, 0);
    }

    for t in threads {
        t.join().unwrap();
    }

    // Even though remove() is called 20*3 times, only 20 items are removed.
    {
        assert_eq!(list_mut.lock().unwrap().head().unwrap().value, 9_000);
    }
}

/// This is not really lock-free, but it might be useful to have a LinkedListCell like this that takes
/// advantage of the slotmap's ABA problem fix to allow using less locks. Right now it's implemented using
/// a simple spinlock and is probably very buggy so I wouldn't trust it, yet.
// #[test]
// fn test_multithreading_lockfree() {
//     let values = (0..1000).map(|v| v).collect::<Vec<_>>();
//     let list = Arc::new(LinkedListCell::new(LinkedList::new()));
//     let indexes = Arc::new(list.extend(values));

//     let mut threads = Vec::new();
//     for _ in 0..3 {
//         let list = Arc::clone(&list);
//         let indexes = Arc::clone(&indexes);
//         let t = thread::spawn(move || {
//             for index in indexes.iter().take(100)  {
//                 list.get_mut(indexes[200], |item| item.value = 1337);
//                 let _was_removed = list.remove(*index); // returns None if the index does not exist
//             }
//         });
//         threads.push(t);
//     }

//     {
//         assert_eq!(list.head().unwrap().value, 0);
//         assert_eq!(list.get(indexes[200]).unwrap().value, 200);
//     }

//     for t in threads {
//         t.join().unwrap();
//     }

//     {
//         assert_eq!(list.head().unwrap().value, 100);
//         assert_eq!(list.get(indexes[200]).unwrap().value, 1337);
//     }
// }

#[test]
fn test_split_off() {
    let mut d = LinkedList::new();

    d.push_front(1);
    d.push_front(2);
    d.push_front(3);

    let mut split = d.split_off(d.nth(2).unwrap());

    assert_eq!(split.pop_front(), Some(1));
    assert_eq!(split.pop_front(), None);
}

#[test]
fn test_associated_data() {
    let mut list = LinkedList::new();
    let indexes = list.extend(0..100);

    let mut str_map = list.new_data();
    str_map.insert(indexes[0], String::from("Hello"));
    str_map.insert(indexes[1], String::from("World"));

    assert_eq!(str_map.get(indexes[0]).unwrap(), &String::from("Hello"));
    assert_eq!(str_map.get(indexes[1]).unwrap(), &String::from("World"));
    assert!(str_map.get(indexes[2]).is_none());
}

#[test]
fn test_fn_insert_after_fn_insert_before() {
    // a -> b -> c
    let mut list = LinkedList::new();

    let (a, b, c, d) = {
        let a = list.push_back(1);
        let b = list.push_back(2);
        let c = list.push_back(3);
        let d = list.insert_after(a.clone(), 4);

        // a -> d -> b -> c
        (a, b, c, d)
    };

    let prev_b = list.prev_of(b).unwrap();
    let next_d = list.next_of(d).unwrap();
    let next_a = list.next_of(a).unwrap();

    assert!(list.prev_of(a).is_none());
    assert_eq!(prev_b.value, 4);
    assert_eq!(next_d.value, 2);
    assert_eq!(next_a.value, 4);
}

#[test]
fn test_iter() {
    let mut list = LinkedList::new();
    let verticies: Vec<LinkedListIndex> = (0..100)
        .map(|i| list.push_back(format!("Node: {}", i.to_string())))
        .collect();

    for n in list.iter_next(verticies[0]) {
        println!("Value: \"{}\"", n.value);
    }

    for n in list.iter_next(verticies[0]) {
        println!("Value: \"{}\"", n.value);
    }
}

#[test]
fn test_popback() {
    let mut list = LinkedList::new();
    let _verticies: Vec<LinkedListIndex> = (0..100)
        .map(|i| list.push_back(format!("Node: {}", i.to_string())))
        .collect();

    let mut i = 99;
    while let Some(_popped) = list.pop_back() {
        i -= 1;

        //println!("Popped: {:?}", popped);
        let expected = format!("Node: {}", (i).to_string());
        if i >= 0 {
            let last = list.tail.unwrap();
            assert_eq!(list.get(last).unwrap().value, expected);
        }
    }
}
