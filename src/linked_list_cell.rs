/// Not yet implemented. This is a work in progress (highly experimental).

use std::{cell::UnsafeCell, sync::{atomic::AtomicBool, Mutex, MutexGuard, PoisonError}};

use slotmap::SecondaryMap;

use crate::{LinkedList, LinkedListIndex, LinkedListItem};


/// The only unsafe code in this library at the moment.
/// 
/// Assumptions:
/// - Removing and adding items to the list is thread safe as long as the thread locks the item being removed or added.
/// - Writing an item is safe as long as the same item is not being written to by multiple threads.
/// - Reading an item is safe
///     - If the item is removed by another thread, the SlotMap will fix the ABA problem.
pub struct LinkedListCell<T: Sync + Send> {
    data: UnsafeCell<LinkedList<T>>,
    mutex: Mutex<()>,
    item_locks: UnsafeCell<SecondaryMap<LinkedListIndex, AtomicBool>>
}

impl<T: Sync + Send> LinkedListCell<T> {
    unsafe fn inner_mut(&self) -> &mut LinkedList<T> {
        unsafe { &mut (*self.data.get()) }
    }
    fn inner(&self) -> &LinkedList<T> {
        unsafe { &*self.data.get() }
    }   

    fn item_locks(&self) -> &SecondaryMap<LinkedListIndex, AtomicBool> {
        unsafe { &*self.item_locks.get() }
    }

    fn item_locks_mut(&self) -> &mut SecondaryMap<LinkedListIndex, AtomicBool> {
        unsafe { &mut *self.item_locks.get() }
    }

    /// Sets a lock on the item at the given index, preventing two threads from writing to the same item.
    fn lock_item(&self, index: LinkedListIndex) -> () {
        if !self.inner().contains_key(index) {
            return ();
        }
        if !self.item_locks().contains_key(index) {
            self.item_locks_mut().insert(index, AtomicBool::new(false)); 
        }
        while unsafe { self.item_locks_mut().get_unchecked_mut(index) }.swap(true, std::sync::atomic::Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }

    fn unlock_item(&self, index: LinkedListIndex) {
        unsafe { self.item_locks_mut().get_unchecked_mut(index) }.store(false, std::sync::atomic::Ordering::Release);
    }

    pub fn get_mut<F: Fn(&mut LinkedListItem<T>)>(&self, index: LinkedListIndex, cb: F) -> Option<()> {
        let inner = unsafe { self.inner_mut() };
        self.lock_item(index);
        cb(inner.get_mut(index)?);
        self.unlock_item(index);
        Some(())
    }

    pub fn get(&self, index: LinkedListIndex) -> Option<&LinkedListItem<T>> {
        self.inner().get(index)
    }
    
    pub fn new(list: LinkedList<T>) -> Self {
        LinkedListCell { 
            data: UnsafeCell::new(list), 
            mutex: Mutex::new(()), 
            item_locks: UnsafeCell::new(SecondaryMap::new()) 
        }
    }
    /// Thread-safe assuming that the functions that modify the length of the list lock the thread.
    pub fn len(&self) -> usize {
        self.inner().len()
    }

    pub fn contains_key(&self, index: LinkedListIndex) -> bool {
        self.inner().contains_key(index)
    }

    /// This function needs to lock the item being removed.
    pub fn remove(&self, index: LinkedListIndex) -> Option<LinkedListItem<T>> {
        unsafe { self.inner_mut().remove(index) }
    }

    pub fn push_back(&self, value: T) -> LinkedListIndex {
        unsafe { self.inner_mut().push_back(value) }
    }

    pub fn push_front(&self, value: T) -> LinkedListIndex {
        unsafe { self.inner_mut().push_front(value) }
    }

    pub fn pop_back(&self) -> Option<T> {
        let back = self.inner().tail?;
        self.lock_item(back);
        unsafe { self.inner_mut().pop_back() }
    }

    /// This function needs to lock the item being removed.
    pub fn pop_front(&self) -> Option<T> {
        let front = self.inner().head?;
        self.lock_item(front);
        unsafe { self.inner_mut().pop_front() }
    }

    pub fn extend(&self, values: impl IntoIterator<Item = T>) -> Vec<LinkedListIndex> {
        unsafe { self.inner_mut().extend(values) }
    }

    pub fn insert_after(&self, index: LinkedListIndex, value: T) -> LinkedListIndex {
        unsafe { self.inner_mut().insert_after(index, value) }
    }

    pub fn head(&self) -> Option<&LinkedListItem<T>> {
        self.inner().head()
    }
}

unsafe impl<T: Sync + Send> Sync for LinkedListCell<T> {}
unsafe impl<T: Sync + Send> Send for LinkedListCell<T> {}

