fast-list
=========
### A fast doubly linked list using [`SlotMap`] for better cache performance and to solve the ABA problem.

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
[`LinkedListIndex`] : https://docs.rs/fast_list/latest/struct.LinkedListIndex.html
[`LinkedListWalker`]: https://docs.rs/fast_list/latest/struct.LinkedListWalker.html

