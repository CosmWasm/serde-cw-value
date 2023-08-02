#[cfg(feature = "std")]
pub use std::{
    boxed::Box,
    cmp::Ordering,
    collections::BTreeMap,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    string::String,
    vec::Vec,
};

#[cfg(not(feature = "std"))]
pub use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
