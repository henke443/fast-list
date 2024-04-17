# Fast Linked List
A fast doubly linked list using [`SlotMap`] for better cache performance and to solve the ABA problem.

- [x] On average ~2-3x faster than the standard [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html) for all operations.
- [x] On average ~2-3x faster than [`Vec`] & [`VecDeque`] for random insertions (random removals are about the same as of now)
- [x] Only slightly slower than [`Vec`] & [`VecDeque`] for most other operations.
- [x] Safe against [ABA problem] by using a [`SlotMap`] internally, which means you can safely iterate & mutate the list across multiple threads. An advantage over just using a SlotMap is that the order when iterating is not arbitrary.
- [x] Written in 100% safe Rust.

[ABA problem]: https://en.wikipedia.org/wiki/ABA_problem
[`SlotMap`]: https://docs.rs/slotmap/latest/slotmap/index.html
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html


# Structure

 [`LinkedList`] - The main struct that holds the list.

 [`LinkedListIndex`] - An index into the list.

 [`LinkedListItem`] - An item in the list.

 [`LinkedListWalker`] - **\[unstable\]** A walker type (like in petgraph) which can be used to iterate over the list.

# Examples

```rust
use fast_list::LinkedList;

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