//! The utility crate for the [`crow_engine`].
//!
//! # TODO:
//!
//! * impl [`Iterator`] for [`Holder<T>`].
//!
//! [`crow_engine`]:https://crates.io/crates/crow_engine
//! [`Holder<T>`]: holder/struct.Holder.html
//! [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [`SelfRefHolder<T,U>`]: self_ref/struct.SelfRefHolder.html

pub mod holder;
pub mod traits;
pub mod pop_iter;
