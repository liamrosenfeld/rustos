use core::clone::Clone;
use core::fmt::{self, Debug};
use core::iter::Iterator;
use core::marker::Copy;
use core::option::Option::{self, None, Some};
use core::prelude::rust_2021::derive;

/// An _instrusive_ linked list of addresses.
///
/// A `LinkedList` maintains a list of `*mut ListNode`s. The user of the
/// `LinkedList` guarantees that the passed in pointer refers to valid, unique,
/// writeable memory at least `ListNode` in size.
///
/// The ListNode is a wrapper for the pointer to the next free chunk and the size
/// which is stored inside of the freed memory
///
/// # Usage
///
/// A list is created using `LinkedList::new()`. A new address can be prepended
/// using `push()`. The first address in the list, if any, can be removed and
/// returned using `pop()` or returned (but not removed) using `peek()`.
///
/// `LinkedList` exposes two iterators. The first, obtained via `iter()`,
/// iterates over all of the addresses in the list. The second, returned from
/// `iter_mut()`, returns `Cursor`s that point to a single address in the list. The
/// `value()` and `pop()` methods of `Cursor` can be used to read the value or pop
/// the value from the list, respectively.
///
#[derive(Copy, Clone)]
pub struct LinkedList {
    head: Option<*mut ListNode>,
}

#[derive(Debug, Copy, Clone)]
pub struct ListNode {
    next: Option<*mut ListNode>,
    size: usize,
}

impl ListNode {
    pub const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    pub fn ptr(&mut self) -> *mut u8 {
        self as *mut Self as *mut u8
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

unsafe impl Send for LinkedList {}

impl LinkedList {
    /// Returns a new, empty linked list.
    pub const fn new() -> LinkedList {
        LinkedList { head: None }
    }

    /// Returns `true` if the list is empty and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Pushes the address `item` to the front of the list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` refers to unique, writeable memory at
    /// least `usize` in size that is valid as long as `item` resides in `self`.
    /// Barring the uniqueness constraint, this is equivalent to ensuring that
    /// `*ptr = some_usize` is a safe operation as long as the pointer resides
    /// in `self`.
    pub unsafe fn push(&mut self, ptr: *mut u8, size: usize) {
        let node_ptr = ptr as *mut ListNode;
        node_ptr.write(ListNode {
            next: self.head.take(),
            size,
        });
        self.head = Some(node_ptr);
    }

    /// Removes and returns the first item in the list, if any.
    pub fn pop(&mut self) -> Option<*mut ListNode> {
        let value = self.head.take()?;
        unsafe {
            self.head = (*value).next;
        }
        Some(value)
    }

    /// Returns the first item in the list without removing it, if any.
    pub fn peek(&self) -> Option<*mut ListNode> {
        self.head
    }

    /// Returns an iterator over the items in this list.
    pub fn iter(&self) -> Iter {
        Iter {
            current: self.head,
            _list: self,
        }
    }

    /// Returns an iterator over the items in this list.
    ///
    /// The items returned from the iterator (of type `Node`) allows the given
    /// item to be removed from the linked list via the `Node::pop()` method.
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            prev: None,
            curr: self.head,
            list: self,
        }
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// An iterator over the items of the linked list.
pub struct Iter<'a> {
    _list: &'a LinkedList,
    current: Option<*mut ListNode>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current?;
        unsafe {
            self.current = (*value).next;
        }
        Some(value)
    }
}

/// An item returned from a mutable iterator of a `LinkedList`.
pub struct Cursor {
    prev: Option<*mut ListNode>,
    curr: *mut ListNode,
    list: *mut LinkedList,
}

impl Cursor {
    /// Removes and returns the value of this item from the linked list it
    /// belongs to.
    pub fn pop(self) -> *mut ListNode {
        unsafe {
            if let Some(prev) = self.prev {
                // middle of list
                (*(prev)).next = (*(self.curr)).next;
            } else {
                // first node
                (*(self.list)).head = (*(self.curr)).next;
            }
        }
        self.curr
    }

    /// Returns the value of this element.
    pub fn value(&self) -> *mut ListNode {
        self.curr
    }
}

/// An iterator over the items of the linked list allowing mutability.
pub struct IterMut<'a> {
    list: &'a mut LinkedList,
    prev: Option<*mut ListNode>,
    curr: Option<*mut ListNode>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        let old_curr = self.curr?;
        let old_prev = self.prev;
        self.prev = self.curr;
        unsafe {
            self.curr = (*old_curr).next;
        }
        Some(Cursor {
            prev: old_prev,
            curr: old_curr,
            list: self.list,
        })
    }
}
