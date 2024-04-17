use core::fmt;
use slotmap::{new_key_type, SecondaryMap, SlotMap, SparseSecondaryMap};

use crate::{LinkedListWalker, Walker};

new_key_type! {
    pub struct LinkedListIndex;
}

#[derive(Debug)]
pub struct LinkedListItem<T> {
    pub index: LinkedListIndex,
    pub value: T,
    pub next_index: Option<LinkedListIndex>,
    pub prev_index: Option<LinkedListIndex>,
}

/**
# Associated data
With slotmap you can get `SecondaryMap<K, V>` for any V for the same K.\

We want to store a `Vec<SecondaryMap<K, V>` for any K and mixed V.\

One solution would be to use an `Enum` for `V`, but that would induce some overhead.\
For example, if you store a `Enum{Boolean(bool), ReallyBigStruct{...}}``
then the size of the Enum will be `C + sizeof(ReallyBigStruct)`.

We might be able to somewhat mitigate this by saying that
`V: EnumContaining<X>` where `T: Associate<K, V>` implies that `T` can be turned into a value of Option<V> for any possible V (but it will be None for all V except one).
Or alternatively, we can say tha:
`V: Into<ParentEnum>
 */
pub trait TodoTrait {}

/// A doubly linked list using SlotMap for better cache performance than a linked list using pointers and which also solves the ABA problem.
pub struct LinkedList<T = ()> {
    pub head: Option<LinkedListIndex>,
    pub tail: Option<LinkedListIndex>,
    items: SlotMap<LinkedListIndex, LinkedListItem<T>>,
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.items.values()).finish()
    }
}

impl<T> LinkedList<T> {
    /// Create a new empty list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            items: SlotMap::with_key(),
        }
    }

    // pub fn head_index(&self) -> Option<LinkedListIndex> {
    //     self.head
    // }

    // pub fn tail_index(&self) -> Option<LinkedListIndex> {
    //     self.tail
    // }

    pub fn contains_key(&self, index: LinkedListIndex) -> bool {
        self.items.contains_key(index)
    }
    
    #[inline]
    pub fn head(&self) -> Option<&LinkedListItem<T>> {
        if let Some(head) = self.head {
            self.get(head)
        } else {
            None
        }
    }

    #[inline]
    pub fn head_mut(&mut self) -> Option<&mut LinkedListItem<T>> {
        if let Some(head) = self.head {
            self.get_mut(head)
        } else {
            None
        }
    }

    #[inline]
    pub fn tail(&self) -> Option<&LinkedListItem<T>> {
        if let Some(tail) = self.tail {
            self.get(tail)
        } else {
            None
        }
    }

    #[inline]
    pub fn tail_mut(&mut self) -> Option<&mut LinkedListItem<T>> {
        if let Some(tail) = self.tail {
            self.get_mut(tail)
        } else {
            None
        }
    }

    /// Returns a secondary map of type V
    pub fn new_data<V>(&self) -> SecondaryMap<LinkedListIndex, V> {
        SecondaryMap::new()
    }

    /// Returns a sparse secondary map of type V
    pub fn new_data_sparse<V>(&self) -> SparseSecondaryMap<LinkedListIndex, V> {
        SparseSecondaryMap::new()
    }

    /// Get an item in the list.
    #[inline]
    pub fn get(&self, index: LinkedListIndex) -> Option<&LinkedListItem<T>> {
        self.items.get(index).map(|item| item)
    }

    /// Get a mutable reference to an item in the list.
    #[inline]
    pub fn get_mut(&mut self, index: LinkedListIndex) -> Option<&mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        item
    }

    /// Get the item after the item with the given index if it exists.
    #[inline]
    pub fn next_of(&self, index: LinkedListIndex) -> Option<&LinkedListItem<T>> {
        self.items
            .get(index)
            .and_then(|item| item.next_index.and_then(|next| self.items.get(next)))
    }

    /// Get the item before the item with the given index if it exists.
    #[inline]
    pub fn prev_of(&self, index: LinkedListIndex) -> Option<&LinkedListItem<T>> {
        self.items
            .get(index)
            .and_then(|item| item.prev_index.and_then(|prev| self.items.get(prev)))
    }

    /// Get a mutable reference to the item after the item with the given index if it exists.
    pub fn next_of_mut(&mut self, index: LinkedListIndex) -> Option<&mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let next = item.and_then(|item| item.prev_index);
        if let Some(next) = next {
            self.items.get_mut(next)
        } else {
            None
        }
    }

    /// Get a mutable reference to the item before the item with the given index if it exists.
    pub fn prev_of_mut(&mut self, index: LinkedListIndex) -> Option<&mut LinkedListItem<T>> {
        let item = self.items.get_mut(index);
        let prev = item.and_then(|item| item.prev_index);
        if let Some(prev) = prev {
            self.items.get_mut(prev)
        } else {
            None
        }
    }

    /// Insert an item after the given index and return the index of the new item.
    pub fn insert_after(&mut self, index: LinkedListIndex, value: T) -> LinkedListIndex {
        let next_index = self.items.get(index).unwrap().next_index;

        let new_index = self.items.insert_with_key(|i| LinkedListItem {
            index: i,
            value,
            next_index: next_index,
            prev_index: Some(index),
        });

        let items = &mut self.items;

        if let Some(next) = next_index {
            // If the element we insert after has a next element, we need to update the next element's `prev` to point to the new element.
            if let Some(next) = items.get_mut(next) {
                next.prev_index = Some(new_index);
            }
        } else {
            // If the element we insert after does not have a next element, we need to update the tail to point to the new element.
            self.tail = Some(new_index);
        }

        if let Some(item) = items.get_mut(index) {
            // Update the element we insert after to point its `prev` to the new element.
            item.next_index = Some(new_index);
        }

        // Return the new element
        new_index
    }

    /// Insert an item before the given index.
    pub fn insert_before(&mut self, index: LinkedListIndex, value: T) -> LinkedListIndex {
        let prev_index = self.items.get(index).unwrap().prev_index;

        let new_index = self.items.insert_with_key(|i| LinkedListItem {
            index: i,
            value,
            next_index: Some(index),
            prev_index: prev_index,
        });

        let items = &mut self.items;

        if let Some(prev) = prev_index {
            // If the element we insert before has a previous element, we need to update the previous element's `next` to point to the new element.
            if let Some(prev) = items.get_mut(prev) {
                prev.next_index = Some(new_index);
            }
        } else {
            // If the element we insert before does not have a previous element, we need to update the head to point to the new element.
            self.head = Some(new_index);
        }

        let item = items.get_mut(index).unwrap();
        // Update the element we insert before to point its `prev` to the new element.
        item.prev_index = Some(new_index);

        new_index
    }

    /// Add an item to the back of the list and return its index.
    pub fn push_back(&mut self, value: T) -> LinkedListIndex {
        let index = self.items.insert_with_key(|i| LinkedListItem {
            index: i,
            value,
            next_index: None,
            prev_index: self.tail,
        });

        match self.tail {
            Some(tail) => {
                if let Some(tail) = self.items.get_mut(tail) {
                    tail.next_index = Some(index);
                }
            }
            None => {
                self.head = Some(index);
            }
        }

        self.tail = Some(index);

        index
    }

    /// Push an item to the front of the list.
    pub fn push_front(&mut self, value: T) -> LinkedListIndex {
        let index = self.items.insert_with_key(|i| LinkedListItem {
            index: i,
            value,
            next_index: self.head,
            prev_index: None,
        });

        match self.head {
            Some(head) => {
                let head = self.items.get_mut(head);
                if let Some(head) = head {
                    head.prev_index = Some(index);
                }
            }
            None => {
                self.tail = Some(index);
            }
        }

        self.head = Some(index);

        index
    }

    /// Remove the last item in the list and return it (if it exists)
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.and_then(|old_tail| {
            let old_tail = self.items.remove(old_tail);

            if let Some(old_tail) = old_tail {

                self.tail = old_tail.prev_index;

                match old_tail.prev_index {
                    Some(prev) => {
                        let prev_mut = self.items.get_mut(prev);
                        if let Some(prev) = prev_mut {
                            prev.next_index = None;
                        }
                    }
                    None => {
                        self.head = None;
                    }
                }
    
                Some(old_tail.value)
            } else {
                None
            }

        })
    }

    /// Remove the first item in the list and return it (if it exists)
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.and_then(|old_head| {
            let old_head = self.items.remove(old_head);
            if let Some(old_head) = old_head {
                self.head = old_head.next_index;
                match old_head.next_index {
                    Some(next) => {
                        self.items.get_mut(next).unwrap().prev_index = None;
                    }
                    None => {
                        self.tail = None;
                    }
                }
                Some(old_head.value)
            } else {
                None
            }
        })
    }

    /// Convenience method for `list.iter_next(list.head.unwrap())`
    /// # Example
    /// ```
    /// use fast_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.extend(0..100);
    ///
    /// assert_eq!(list.iter_next(list.head.unwrap()).count(), 100);
    /// assert_eq!(list.iter().count(), 100);
    /// assert_eq!(
    ///     list.iter().next().unwrap().value,
    ///     list.iter_next(list.head.unwrap()).next().unwrap().value
    /// );
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.iter_next(self.head.unwrap())
    }

    #[inline]
    pub fn iter_unordered(&self) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.items.values()
    }

    #[inline]
    pub fn iter_next(&self, start: LinkedListIndex) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.cursor_next(start)
            .map(move |index| self.items.get(index).unwrap())
    }

    #[inline]
    pub fn iter_prev(&self, start: LinkedListIndex) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.cursor_prev(start)
            .map(move |index| self.items.get(index).unwrap())
    }

    pub fn cursor_next(
        &self,
        start: LinkedListIndex,
    ) -> impl Iterator<Item = LinkedListIndex> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| {
            items.get(*index).and_then(move |item| item.next_index)
        })
    }

    pub fn cursor_prev(
        &self,
        start: LinkedListIndex,
    ) -> impl Iterator<Item = LinkedListIndex> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| {
            items.get(*index).and_then(move |item| item.prev_index)
        })
    }

    /* // TODO
    Splits the list into two at the given index. Returns a new list containing everything after the given index, excluding the index.
    This operation should compute in O(n) time.
    */
    // pub fn split_off_ex(&mut self, index: LinkedListIndex) -> Self {
    //     let mut new_list = Self::new();
    //     let mut walker = LinkedListWalker::new(&self, index, false);
    //     while let Some(next) = walker.walk_next(&new_list) {
    //         self.remove(next).map(|item| new_list.items.insert(item));
    //     }
    //     return new_list
    // }

    /// Splits the list into two at the given index. Returns a new list containing everything after the given index, including the index.
    /// This operation should compute in O(n) time.
    ///
    /// T needs to implement Clone only to be able to insert the index, but not for the following items.
    ///
    /// The old indexes will become invalid after this operation and new indexes can be
    /// retrieved by iterating from the head & tail of the new list.
    pub fn split_off(&mut self, index: LinkedListIndex) -> Self {
        let mut new_list = Self::new();
        let mut walker = LinkedListWalker::new(&self, index, false);

        let first = self.remove(index);
        if let Some(first) = first {
            new_list.push_back(first.value);
        }
        while let Some(next) = walker.walk_next(&new_list) {
            self.remove(next).map(|item| new_list.push_back(item.value));
        }
        return new_list;
    }

    /// Returns the nth index by iterating from the head or tail, whichever is closer.
    #[inline]
    pub fn nth(&self, n: usize) -> Option<LinkedListIndex> {
        if n < self.len() / 2 {
            self.cursor_next(self.head.unwrap()).nth(n)
        } else {
            self.cursor_prev(self.tail.unwrap()).nth(self.len() - n - 1)
        }
    }

    /// Push many items to the back of the list.
    ///
    /// Returns the indexes of the new items
    pub fn extend<I>(&mut self, values: I) -> Vec<LinkedListIndex>
    where
        I: IntoIterator<Item = T>,
    {
        let mut indexes = Vec::new();
        for value in values {
            indexes.push(self.push_back(value));
        }
        indexes
    }

    /// Push many items to the front of the list.
    ///
    /// Returns the indexes of the new items
    pub fn extend_front<I>(&mut self, values: I) -> Vec<LinkedListIndex>
    where
        I: IntoIterator<Item = T>,
    {
        let mut indexes = Vec::new();
        for value in values {
            indexes.push(self.push_front(value));
        }
        indexes
    }

    /// Get the number of items in the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Remove an item from the list, returning the value at the key if the key was not previously removed.

    pub fn remove(&mut self, index: LinkedListIndex) -> Option<LinkedListItem<T>> {
        let item = self.items.remove(index)?;

        if let Some(prev) = item.prev_index {
            if let Some(prev_mut) = self.items.get_mut(prev) {
                prev_mut.next_index = item.next_index;
            }
        } else {
            self.head = item.next_index;
        }

        if let Some(next) = item.next_index {
            if let Some(next_mut) = self.items.get_mut(next) {
                next_mut.prev_index = item.prev_index;
            }
        } else {
            self.tail = item.prev_index;
        }

        Some(item)
    }

    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut current = self.head;
        while let Some(index) = current {
            let item = self.items.get(index).unwrap();
            let next = item.next_index;
            if !f(&item.value) {
                self.remove(index);
            }
            current = next;
        }
    }

    pub fn retain<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&T) -> bool,
        T: Clone,
        LinkedListItem<T>: Clone,
    {
        let mut new_list = Self::new();
        new_list.items = self.items.clone();
        new_list.head = self.head;
        new_list.tail = self.tail;
        new_list.retain_mut(f);
        new_list
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::{atomic::{AtomicI32, Ordering}, Arc, Mutex}, thread};

    use crossbeam::epoch::Atomic;

    //use crate::LinkedListCell;

    use super::*;

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
}
