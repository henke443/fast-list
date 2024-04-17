fast-list
=========
### A doubly linked list using [`SlotMap`] for improved cache locality, and to solve the ABA problem.

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docsrs-url]
[![Rust CI](https://github.com/henke443/fast-list/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/henke443/fast-list/actions/workflows/rust-ci.yml)
![MSRV][msrv-badge]


✅ On average ~2-3x faster than the standard [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html) for all operations.

✅ On average ~2-3x faster than [`Vec`] & [`VecDeque`] for random insertions (random removals are about the same as of now)

✅ Only slightly slower than [`Vec`] & [`VecDeque`] for most other operations.

✅ Safe against [ABA problem] by using a [`SlotMap`] internally, which means you can safely iterate & mutate the list across multiple threads. An advantage over just using a SlotMap is that the order when iterating is not arbitrary.

✅ Written in 100% safe Rust.

[ABA problem]: https://en.wikipedia.org/wiki/ABA_problem
[`SlotMap`]: https://docs.rs/slotmap/latest/slotmap/index.html
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html

# Examples

## Multithreaded iteration & mutation

```rust
use fast_list::LinkedList;
use std::thread;
use std::sync::{Arc, Mutex};

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
        for index in indexes.iter().take(9_000)  {
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

// Even though remove() is called 9000*3 times, only 9000 items are removed.
{
    assert_eq!(list_mut.lock().unwrap().head().unwrap().value, 9_000);
}

```


# Structure

 [`LinkedList`] - The main struct that holds the list.
```rust,ignore
// A doubly linked list using SlotMap for better cache performance than a linked list using pointers, and which also solves the ABA problem.
pub struct LinkedList<T = ()> {
    // The index of the first item in the list.
    pub head: Option<LinkedListIndex>,
    // The index of the last item in the list.
    pub tail: Option<LinkedListIndex>,
    // The items in the list.
    items: SlotMap<LinkedListIndex, LinkedListItem<T>>,
}
```

[`LinkedListIndex`] - An index into the list.
```rust,ignore
new_key_type! {
    /// A newtype for the index of an item in the list.
    pub struct LinkedListIndex;
}
```
 [`LinkedListItem`] - An item in the list.

```rust,ignore
pub struct LinkedListItem<T> {
    /// The index of the item in the list.
    pub index: LinkedListIndex,
    /// The value of the item.
    pub value: T,
    /// The index of the next item in the list.
    pub next_index: Option<LinkedListIndex>,
    /// The index of the previous item in the list.
    pub prev_index: Option<LinkedListIndex>,
}
```

 [`LinkedListWalker`] - **\[unstable\]** A walker type (like in petgraph) which can be used to iterate over the list.

[`LinkedListItem`]: https://docs.rs/fast_list/latest/struct.LinkedListItem.html
[`LinkedList`]: https://docs.rs/fast_list/latest/struct.LinkedList.html
[`LinkedListIndex`]: https://docs.rs/fast_list/latest/struct.LinkedListIndex.html
[`LinkedListWalker`]: https://docs.rs/fast_list/latest/struct.LinkedListWalker.html

[API documentation]: https://docs.rs/fast-list/
[docsrs-badge]: https://img.shields.io/docsrs/fast-list
[docsrs-url]: https://docs.rs/fast-list/latest/fast_graph
[crates-badge]: https://img.shields.io/crates/v/fast-list.svg
[crates-url]: https://crates.io/crates/fast-list
[msrv-badge]: https://img.shields.io/badge/rustc-1.64+-blue.svg
[RELEASES]: RELEASES.rst