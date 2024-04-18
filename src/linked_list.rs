use core::fmt;
use slotmap::{new_key_type, SecondaryMap, SlotMap, SparseSecondaryMap};

#[cfg(feature = "unstable")]
use crate::{LinkedListWalker, Walker};

new_key_type! {
    /// A newtype for the index of an item in the list.
    pub struct LinkedListIndex;
}

#[derive(Debug)]
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

/// A doubly linked list using SlotMap for better cache performance than a linked list using pointers, and which also solves the ABA problem.
pub struct LinkedList<T = ()> {
    /// The index of the first item in the list.
    pub head: Option<LinkedListIndex>,
    /// The index of the last item in the list.
    pub tail: Option<LinkedListIndex>,
    /// The items in the list.
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

    /// Checks if the list contains the given index.
    pub fn contains_key(&self, index: LinkedListIndex) -> bool {
        self.items.contains_key(index)
    }

    /// Get the first item in the list.
    /// Can be None if the list is empty.
    #[inline]
    pub fn head(&self) -> Option<&LinkedListItem<T>> {
        if let Some(head) = self.head {
            self.get(head)
        } else {
            None
        }
    }

    /// Get a mutable reference to the first item in the list.
    /// Can be None if the list is empty.
    #[inline]
    pub fn head_mut(&mut self) -> Option<&mut LinkedListItem<T>> {
        if let Some(head) = self.head {
            self.get_mut(head)
        } else {
            None
        }
    }

    /// Get the last item in the list.
    /// Can be None if the list is empty.
    #[inline]
    pub fn tail(&self) -> Option<&LinkedListItem<T>> {
        if let Some(tail) = self.tail {
            self.get(tail)
        } else {
            None
        }
    }

    /// Get a mutable reference to the last item in the list.
    /// Can be None if the list is empty.
    #[inline]
    pub fn tail_mut(&mut self) -> Option<&mut LinkedListItem<T>> {
        if let Some(tail) = self.tail {
            self.get_mut(tail)
        } else {
            None
        }
    }

    /// Convenience method to return a slotmap::SecondaryMap of type V
    pub fn new_data<V>(&self) -> SecondaryMap<LinkedListIndex, V> {
        SecondaryMap::new()
    }

    /// Convenience method to return a slotmap::SparseSecondaryMap of type V
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
            items.get_mut(next).unwrap().prev_index = Some(new_index);
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
            items.get_mut(prev).unwrap().next_index = Some(new_index);
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
                head.unwrap().prev_index = Some(index);
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
                        prev_mut.unwrap().next_index = None
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

    /// Returns an iterator that iterates over the items of the list in no particular order.
    #[inline]
    pub fn iter_unordered(&self) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.items.values()
    }

    /// Returns an iterator that iterates over the items of the list.
    #[inline]
    pub fn iter_next(&self, start: LinkedListIndex) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.cursor_iter_next(start)
            .map(move |index| self.items.get(index).unwrap())
    }

    /// Returns an iterator that iterates over the items of the list in reverse order.
    #[inline]
    pub fn iter_prev(&self, start: LinkedListIndex) -> impl Iterator<Item = &LinkedListItem<T>> {
        self.cursor_iter_prev(start)
            .map(move |index| self.items.get(index).unwrap())
    }

    /// Returns the next index of the item with the given index.
    pub fn cursor_next(
        &self,
        item: LinkedListIndex,
    ) -> Option<LinkedListIndex> {
        self.items.get(item).and_then(|item| item.next_index)
    }

    /// Returns the previous index of the item with the given index.
    pub fn cursor_prev(
        &self,
        item: LinkedListIndex,
    ) -> Option<LinkedListIndex> {
        self.items.get(item).and_then(|item| item.next_index)
    }

    /// Returns an iterator that iterates over the indexes of the list.
    pub fn cursor_iter_next(
        &self,
        start: LinkedListIndex,
    ) -> impl Iterator<Item = LinkedListIndex> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| {
            items.get(*index).and_then(move |item| item.next_index)
        })
    }

    /// Returns an iterator that iterates over the indexes of the list in reverse order.
    pub fn cursor_iter_prev(
        &self,
        start: LinkedListIndex,
    ) -> impl Iterator<Item = LinkedListIndex> + '_ {
        let items = &self.items;
        std::iter::successors(Some(start), move |index| {
            items.get(*index).and_then(move |item| item.prev_index)
        })
    }

    /// Splits the list into two at the given index. Returns a new list containing everything after the given index, including the index.
    /// This operation should compute in O(n) time.
    ///
    /// T needs to implement Clone only to be able to insert the index, but not for the following items.
    ///
    /// The old indexes will become invalid after this operation and new indexes can be
    /// retrieved by iterating from the head & tail of the new list.
    pub fn split_off(&mut self, index: LinkedListIndex) -> Self {
        let mut new_list = Self::new();
        // let mut walker = LinkedListWalker::new(&self, index, false);

        // let first = self.remove(index);
        // if let Some(first) = first {
        //     new_list.push_back(first.value);
        // }
        // while let Some(next) = walker.walk_next(&new_list) {
        //     self.remove(next).map(|item| new_list.push_back(item.value));
        // }
        let mut current = index;


        let first = self.remove(index);
        if let Some(first) = first {
            new_list.push_back(first.value);
        }
        while let Some(next) = self.cursor_next(current) {
            let removed = self.remove(next);
            if let Some(removed) = removed {
                new_list.push_back(removed.value);
            }
        }
        return new_list;
    }

    /// Returns the nth index by iterating from the head or tail, whichever is closer.
    #[inline]
    pub fn nth(&self, n: usize) -> Option<LinkedListIndex> {
        let len = self.len();

        if n >= len {
            return None;
        }
        if len == 0 {
            return None;
        }
        if n < len / 2 {
            self.cursor_iter_next(self.head.unwrap()).nth(n)
        } else {
            self.cursor_iter_prev(self.tail.unwrap()).nth(len - n - 1)
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

    /// Retain only the elements specified by the predicate.
    /// This operation should compute in O(n) time.
    /// Modifies the list in place.
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

    /// Returns a cloned list retaining only the elements specified by the predicate.
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
