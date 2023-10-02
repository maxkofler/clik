// NOTE: Taken from StackOverflow, shellfish
/// Use this macro to wrap an asynchronous function so that it can be used
/// as a function pointer.
///
/// The first argument is a Type, which is the state. The second is the
/// async function.
#[macro_export]
#[cfg_attr(nightly, doc(cfg(feature = "async")))]
macro_rules! async_fn {
    ($state:ty, $inc:expr) => {{
       // I think the error message referred to here is spurious, but why take a chance?
       fn rustc_complains_if_this_name_conflicts_with_the_environment_even_though_its_probably_fine(
           state: &mut $state,
           args: Vec<String>
       ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = Result<(), Box<dyn ::std::error::Error>>> + Send + '_ >> {
            Box::pin($inc(state, args))
        }
        rustc_complains_if_this_name_conflicts_with_the_environment_even_though_its_probably_fine
    }}
}
