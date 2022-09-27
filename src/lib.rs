#![cfg_attr(test, feature(pin_macro))]

use core::{ops::Deref, pin::Pin};

/// Produces `true` if the destructor of `P` is guaranteed to be run.
/// 
/// ## Examples
/// ```
/// # #![feature(pin_macro)]
/// use std::pin::pin;
/// use unforgettable::is_unforgettable;
/// 
/// let pinned = pin!(52);
/// assert_eq!(is_unforgettable(pinned.as_ref()), true);
/// 
/// let pinned = Box::pin(52);
/// assert_eq!(is_unforgettable(pinned.as_ref()), false);
/// ```
pub fn is_unforgettable<P: Deref>(pinned: Pin<P>) -> bool
{
    let pointer = pinned.as_ref().get_ref();
    is_stack_pointer(pointer)
}

/// Produces true if the given `pointer` is on the stack.
fn is_stack_pointer<T: ?Sized>(pointer: *const T) -> bool {
    let pointer = pointer as *const () as usize;
    let stack_pointer = psm::stack_pointer() as usize;
    match psm::StackDirection::new() {
        psm::StackDirection::Ascending => 
            pointer <= stack_pointer,
        psm::StackDirection::Descending => 
            pointer >= stack_pointer,
    }
}

#[test]
fn test() {
    use std::pin::pin;

    let pinned = pin!(52);
    assert_eq!(is_unforgettable(pinned.as_ref()), true);

    let pinned = Box::pin(52);
    assert_eq!(is_unforgettable(pinned.as_ref()), false);
}