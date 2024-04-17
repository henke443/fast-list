# Fast Linked List

A rust library that is:
- On average ~2-3x faster than std linked list for all operations.
- On average ~2-3x faster than Vec & VecDeque for random insertions (random removals are about the same as of now)
- Only slightly slower than Vec & VecDeque for most other operations.
- Safe against ABA problem by using a SlotMap internally, which means you can safely iterate & mutate the list across multiple threads. The main advantage over just using a SlotMap is that the order when iterating is not arbitrary.