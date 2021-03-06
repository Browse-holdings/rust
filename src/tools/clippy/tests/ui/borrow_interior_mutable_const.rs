#![warn(clippy::borrow_interior_mutable_const)]
#![allow(clippy::declare_interior_mutable_const, clippy::ref_in_deref)]

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;

const ATOMIC: AtomicUsize = AtomicUsize::new(5);
const CELL: Cell<usize> = Cell::new(6);
const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);
const INTEGER: u8 = 8;
const STRING: String = String::new();
const STR: &str = "012345";
const COW: Cow<str> = Cow::Borrowed("abcdef");
const NO_ANN: &dyn Display = &70;
static STATIC_TUPLE: (AtomicUsize, String) = (ATOMIC, STRING);
const ONCE_INIT: Once = Once::new();

trait Trait<T>: Copy {
    type NonCopyType;

    const ATOMIC: AtomicUsize;
}

impl Trait<u32> for u64 {
    type NonCopyType = u16;

    const ATOMIC: AtomicUsize = AtomicUsize::new(9);
}

fn main() {
    ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability
    assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability

    let _once = ONCE_INIT;
    let _once_ref = &ONCE_INIT; //~ ERROR interior mutability
    let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability
    let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability
    let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability
    let _atomic_into_inner = ATOMIC.into_inner();
    // these should be all fine.
    let _twice = (ONCE_INIT, ONCE_INIT);
    let _ref_twice = &(ONCE_INIT, ONCE_INIT);
    let _ref_once = &(ONCE_INIT, ONCE_INIT).0;
    let _array_twice = [ONCE_INIT, ONCE_INIT];
    let _ref_array_twice = &[ONCE_INIT, ONCE_INIT];
    let _ref_array_once = &[ONCE_INIT, ONCE_INIT][0];

    // referencing projection is still bad.
    let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability
    let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability
    let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability
    let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
    let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability
    let _ = &*ATOMIC_TUPLE.1; //~ ERROR interior mutability
    let _ = &ATOMIC_TUPLE.2;
    let _ = (&&&&ATOMIC_TUPLE).0;
    let _ = (&&&&ATOMIC_TUPLE).2;
    let _ = ATOMIC_TUPLE.0;
    let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
    let _ = ATOMIC_TUPLE.1.into_iter();
    let _ = ATOMIC_TUPLE.2;
    let _ = &{ ATOMIC_TUPLE };

    CELL.set(2); //~ ERROR interior mutability
    assert_eq!(CELL.get(), 6); //~ ERROR interior mutability

    assert_eq!(INTEGER, 8);
    assert!(STRING.is_empty());

    let a = ATOMIC;
    a.store(4, Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), 4);

    STATIC_TUPLE.0.store(3, Ordering::SeqCst);
    assert_eq!(STATIC_TUPLE.0.load(Ordering::SeqCst), 3);
    assert!(STATIC_TUPLE.1.is_empty());

    u64::ATOMIC.store(5, Ordering::SeqCst); //~ ERROR interior mutability
    assert_eq!(u64::ATOMIC.load(Ordering::SeqCst), 9); //~ ERROR interior mutability

    assert_eq!(NO_ANN.to_string(), "70"); // should never lint this.
}
