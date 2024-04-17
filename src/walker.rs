use crate::linked_list::{LinkedList, LinkedListIndex, LinkedListItem};

/// Shamelessly stolen/inspired by https://docs.rs/petgraph/0.4.13/src/petgraph/visit/traversal.rs.html#355-370

/// A walker is a traversal state, but where part of the traversal
/// information is supplied manually to each next call.
///
/// This for example allows graph traversals that don't hold a borrow of the
/// graph they are traversing.
pub trait Walker<Context> {
    type Item;
    /// Advance to the next item
    fn walk_next(&mut self, context: Context) -> Option<Self::Item>;

    /// Create an iterator out of the walker and given `context`.
    fn iter(self, context: Context) -> WalkerIter<Self, Context>
    where
        Self: Sized,
        Context: Clone,
    {
        WalkerIter {
            walker: self,
            context,
        }
    }
}

/// A walker and its context wrapped into an iterator.
#[derive(Clone, Debug)]
pub struct WalkerIter<W, C> {
    walker: W,
    context: C,
}

impl<W, C> WalkerIter<W, C>
where
    W: Walker<C>,
    C: Clone,
{
    pub fn context(&self) -> C {
        self.context.clone()
    }

    pub fn inner_ref(&self) -> &W {
        &self.walker
    }

    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.walker
    }
}

impl<W, C> Iterator for WalkerIter<W, C>
where
    W: Walker<C>,
    C: Clone,
{
    type Item = W::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.walker.walk_next(self.context.clone())
    }
}

impl<'a, C, W: ?Sized> Walker<C> for &'a mut W
where
    W: Walker<C>,
{
    type Item = W::Item;
    fn walk_next(&mut self, context: C) -> Option<Self::Item> {
        (**self).walk_next(context)
    }
}

pub struct LinkedListWalker {
    /// The current index in the list
    current: Option<LinkedListIndex>,
    /// If true walk in reverse order (from tail to head)
    reverse: bool,
}

impl LinkedListWalker {
    pub fn new<T>(list: &LinkedList<T>, start: LinkedListIndex, reverse: bool) -> Self {
        Self {
            current: Some(start),
            reverse: reverse,
        }
    }
}

impl<T> Walker<&LinkedList<T>> for LinkedListWalker {
    type Item = LinkedListIndex;

    /// Advance to the next item, returns the indices of the items in the list and
    /// does not hold a borrow of the list, allowing for mutation while traversing.
    ///
    /// ## Excludes the starting index
    fn walk_next(&mut self, context: &LinkedList<T>) -> Option<Self::Item> {
        if let Some(current) = self.current {
            return if self.reverse {
                context.get(current).and_then(|item| {
                    self.current = item.prev_index;
                    item.prev_index
                })
            } else {
                context.get(current).and_then(|item| {
                    self.current = item.next_index;
                    item.next_index
                })
            };
        }
        None
    }
}

// impl<G> Walker<G> for Dfs<G::NodeId, G::Map>
//     where G: IntoNeighbors + Visitable
// {
//     type Item = G::NodeId;
//     fn walk_next(&mut self, context: G) -> Option<Self::Item> {
//         self.next(context)
//     }
// }

// impl<G> Walker<G> for DfsPostOrder<G::NodeId, G::Map>
//     where G: IntoNeighbors + Visitable
// {
//     type Item = G::NodeId;
//     fn walk_next(&mut self, context: G) -> Option<Self::Item> {
//         self.next(context)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_list_walker() {
        let mut list = LinkedList::new();
        list.extend(0..=100);
        let mut count = 0;
        let mut walker = LinkedListWalker::new(&list, list.tail.unwrap(), true);

        assert_eq!(list.get(list.head.unwrap()).unwrap().value, 0);
        assert_eq!(list.get(list.tail.unwrap()).unwrap().value, 100);

        list.tail_mut().unwrap().value = 0;
        while let Some(index) = walker.walk_next(&list) {
            count += 1;
            list.get_mut(index).unwrap().value = count;
            // let value = list.get(index).unwrap().value;
            // println!("{:?} - {:?} - {:?}", value, index, count);
        }

        assert_eq!(count, 100);
        assert_eq!(list.get(list.head.unwrap()).unwrap().value, 100);
        assert_eq!(list.get(list.tail.unwrap()).unwrap().value, 0);
    }
}
