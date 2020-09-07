//! # argst
//! 
//! `argst` is a simple one-macro library to hide some boilerplate when using [StructOpt]. See the actual [`args`] macro.
//! 
//! [StructOpt]: https://docs.rs/structopt/0.3.17/structopt/
//! [`args`]: macro.args.html
//! 
//! # Example
//! 
//! ```rust
//! # return;
//! use argst::args;
//! args!(
//!     /// The number of times to perform this operation
//!     #[structopt(long = "count")]
//!     n: usize,
//!     /// The path to the data file
//!     path: std::path::PathBuf,
//! );
//! println!("Running {} times over data at {}", n, path.display());
//! ```

/// A macro to generate StructOpt boilerplate and bind arguments
///
/// # Example
///
/// ```rust
/// # return;
/// use argst::args;
/// args!(
///     /// The number of times to perform this operation
///     #[structopt(long = "count")]
///     n: usize,
///     /// The path to the data file
///     path: std::path::PathBuf,
/// );
/// println!("Running {} times over data at {}", n, path.display());
/// ```
#[macro_export]
macro_rules! args {
    ($(
        $(#[$attr:meta])*
        $arg_name:ident : $arg_type:ty
    ),*$(,)?) => {
        use ::structopt::StructOpt;
        #[derive(StructOpt)]
        struct Args {
            $(
                $( #[$attr] )*
                $arg_name: $arg_type,
            )*
        }
        let Args {
            $($arg_name,)*
        } = Args::from_args();
    }
}
