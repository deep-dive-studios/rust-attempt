#![deny(missing_docs)]
#![doc(html_root_url = "https://dabo.guru/rust/attempt/attempt/")]
//! Attempt!
//! ------
//!
//! Attempt is a new experimental rust error handling library, meant to assist and build on existing
//! error handling systems.
//!
//! Attempt exports two structs, `attempt::ErrorPoint` and `attempt::Error`. `attempt::Error` stores a
//! single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
//! which starts out with the original point of creation with `attempt!()`, and is added to every
//! time you propagate the error upwards with `pass!()`.
//!
//! *Attempt does not replace existing error handling systems*. The `attempt::Error` type has a type
//! parameter `E` which represents an internal error type stored. `attempt::Error` just wraps your
//! error type and stores ErrorPoints alongside it.
//!
//! Attempt helps you better keep track of your errors. Instead of seeing a generic "No such file or
//! directory" message, you get a stack trace of functions which propagated the error as well.
//!
//! Instead of:
//!
//! ```text
//! IO Error: failed to lookup address information: Name or service not known
//! ```
//!
//! Get:
//!
//! ```text
//! Error: IO Error: failed to lookup address information: Name or service not known
//!     at 79:17 in zaldinar::startup (src/startup.rs)
//!     at 104:4 in zaldinar::startup (src/startup.rs)
//!     at 28:17 in zaldinar_irclib (/home/daboross/Projects/Rust/zaldinar/zaldinar-irclib/src/lib.rs)
//! ```
//!
//! ---
//!
//! Using attempt!
//! ---
//!
//! The main way you use attempt is through two macros, `attempt!()` and `pass!()`. `attempt!()` is used
//! when you have a regular (non-attempt) result coming from some library function that you want to
//! propagate upwards in case of an error. `pass!()` is used when you have an error which was
//! created using `attempt!()` in a sub-function which you want to add an error point to and
//! propagate upwards.
//!
//! Here's an example of attempt in action:
//!
//! ```rust
//! #[macro_use]
//! extern crate attempt;
//!
//! use std::io::prelude::*;
//! use std::io;
//! use std::fs::File;
//!
//! fn read_log() -> Result<String, attempt::Error<io::Error>> {
//!     let mut file = attempt!(File::open("some_file.log"));
//!     let mut buf = String::new();
//!     attempt!(file.read_to_string(&mut buf));
//!     Ok((buf))
//! }
//!
//! fn do_things() -> Result<(), attempt::Error<io::Error>> {
//!     let log_contents = pass!(read_log());
//!     println!("Log contents: {}", log_contents);
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     let result = do_things();
//!     if let Err(e) = result {
//! #       /*
//!         panic!("{}", e);
//! #       */
//! #       assert_eq!(format!("{}", e), "Error: No such file or directory (os error 2)\
//! #       \n\tat 16:23 in rust_out (<anon>)\
//! #       \n\tat 9:19 in rust_out (<anon>)");
//!     }
//! }
//! ```
//!
//! This simple program behaves exactly as if `Result<_, io::Error>` directly when it functions
//! correctly. When the program encounters is when attempt really shines.  This will result in an
//! error message:
//!
//! ```text
//! Error: No such file or directory (os error 2)
//!    at 16:23 in main (src/main.rs)
//!    at 9:19 in main (src/main.rs)
//! ```
//!
//! These stack traces are stored inside attempt::Error, and are recorded automatically when
//! `attempt!()` or `pass!()` returns an Err value.
//!
//! In each `at` line, the `16:23` represents `line_num:column_num`, the `main` represents the
//! module path (for example `my_program::sub_module`), and `src/main.rs` represents the path of
//! the file in which `attempt!()` was used in.
//!
//! ---
//!
//! Throwing directly from a function is also supported, using `pass_new!()`:
//!
//! ```
//! # #[macro_use]
//! # extern crate attempt;
//! fn possibly_fails() -> Result<(), attempt::Error<&'static str>> {
//!     if true {
//!         // pass_new!() will always return directly
//!         pass_new!("oops");
//!     }
//!
//!     Ok(())
//! }
//!
//! fn main() {
//! #   /*
//!     possibly_fails().unwrap()
//! #   */
//! #   assert_eq!(format!("{}", possibly_fails().unwrap_err()), "Error: oops\
//! #   \n\tat 6:8 in rust_out (<anon>)")
//! }
//! ```
//!
//! ```text
//! called `Result::unwrap()` on an `Err` value: Error: "oops"
//!    at 6:8 in main (src/main.rs)
//! ```
//!
//! `pass_new!()` differs from `attempt!()` in that it takes a parameter directly to pass to a
//! `attempt::Error`, rather than a `Result<>` to match on. `pass_new!()` will always return
//! directly from the function.

use std::fmt;

/// Result alias for a result containing a attempt::Error.
pub type Result<T, E> = std::result::Result<T, Error<E>>;

/// Represents a location at which an error was thrown via attempt!()
pub struct ErrorPoint {
    line: u32,
    column: u32,
    module_path: &'static str,
    file: &'static str,
}

impl ErrorPoint {
    /// The line attempt!() occurred at, retrieved by line!()
    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The column attempt!() occurred at, retrieved by column!()
    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }

    /// The module attempt!() occurred in, retrieved by module_path!()
    #[inline]
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The file attempt!() occurred in, retrieved by file!()
    #[inline]
    pub fn file(&self) -> &'static str {
        self.file
    }

    #[doc(hidden)]
    pub fn __construct(line: u32, column: u32, module_path: &'static str, file: &'static str)
            -> ErrorPoint {
        ErrorPoint {
            line: line,
            column: column,
            module_path: module_path,
            file: file,
        }
    }
}

/// Represents an error. Stores an original error of type E, and any number of ErrorPoints at
/// which the error was propagated.
pub struct Error<E> {
    points: Vec<ErrorPoint>,
    original_error: E,
}

impl<E> Error<E> {
    /// Creates a new Error with no ErrorPoints
    pub fn new(error: E) -> Error<E> {
        Error {
            points: Vec::new(),
            original_error: error,
        }
    }

    /// For macro use only
    #[doc(hidden)]
    pub fn __push_point(&mut self, point: ErrorPoint) {
        self.points.push(point);
    }

    /// Gets all ErrorPoints where this Error was thrown. These are in reverse order, with the
    /// first time it was thrown first and the latest time it was thrown last.
    #[inline]
    pub fn points(&self) -> &[ErrorPoint] {
        &self.points
    }

    /// Gets the original error which this Error was constructed with.
    #[inline]
    pub fn original_error(&self) -> &E {
        &self.original_error
    }

    /// Transforms this Error<OldError> into Error<NewError>. This isn't implemented as an Into or
    /// From implementation because it would conflict with the blanket implementations in stdlib.
    pub fn transform<NE>(self) -> Error<NE> where E: Into<NE> {
        Error {
            points: self.points,
            original_error: self.original_error.into(),
        }
    }
}

impl<E> fmt::Display for Error<E> where E: fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        try!(write!(fmt, "Error: {}", self.original_error));
        for point in self.points.iter().rev() {
            try!(write!(fmt, "\n\tat {}:{} in {} ({})", point.line(), point.column(),
                point.module_path(), point.file()));
        }

        Ok(())
    }
}

impl<E> fmt::Debug for Error<E> where E: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        try!(write!(fmt, "Error: {:?}", self.original_error));
        for point in self.points.iter().rev() {
            try!(write!(fmt, "\n\tat {}:{} in {} ({})", point.line(), point.column(),
                point.module_path(), point.file()));
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! pass {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => {
                // re-assignment for a better error message if pass!() is used incorrectly
                let mut e: $crate::Error<_> = e.transform();
                e.__push_point($crate::ErrorPoint::__construct(
                    line!(),
                    column!(),
                    module_path!(),
                    file!(),
                ));
                return Err(e);
            },
        }
    );
}

#[macro_export]
macro_rules! attempt {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => pass_new!(e),
        }
    );
}

#[macro_export]
macro_rules! pass_new {
    ($e:expr) => ({
        let mut e: $crate::Error<_> = $crate::Error::new($e.into());
        e.__push_point($crate::ErrorPoint::__construct(
            line!(),
            column!(),
            module_path!(),
            file!(),
        ));
        return Err(e);
    })
}
