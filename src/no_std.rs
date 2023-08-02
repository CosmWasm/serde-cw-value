#[cfg(feature = "std")]
pub use std::{
    borrow::ToOwned,
    boxed::Box,
    cmp::Ordering,
    collections::BTreeMap,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(feature = "std"))]
pub use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};
#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
