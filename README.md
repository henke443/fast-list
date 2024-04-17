# Fast Linked List

A linked list that is:
- On average ~2-3x faster than `std::collections::LinkedList` for all operations.
- On average ~2-3x faster than `Vec` & `VecDeque` for random insertions (random removals are about the same as of now)
- Only slightly slower than `Vec` & `VecDeque` for most other operations.
- Safe against [ABA problem] by using a [SlotMaps] internally, which means you can safely iterate & mutate the list across multiple threads. An advantage over just using a SlotMap is that the order when iterating is not arbitrary.
- Written in 100% safe Rust.
- Using indices into a stack allocated arena (slotmap) instead of pointers for improved cache locality.

[ABA problem]: https://en.wikipedia.org/wiki/ABA_problem
[SlotMaps]: (https://docs.rs/slotmap/latest/slotmap/index.html)
