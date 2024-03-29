/// Prints time to execute given expression and returns the value of a given expression
/// for quick and dirty time mesuring.
///
/// # Example
///
/// ```
/// # use time_debug::time;
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// time!(sleep(Duration::new(2, 0)));
/// // prints: took 2s 60us 518ns: [src/lib.rs:42] sleep(Duration::new(2, 0))
/// ```
///
/// ```
/// # use time_debug::time;
/// assert_eq!(1 + 2, 3);
/// assert_eq!(time!(1 + 2), 3);
/// //         ^--- prints: took 0.01 msec [src/main.rs:2] 1 + 2
/// ```
#[macro_export]
macro_rules! time {
    ($fn:expr) => {{
        let start_time = std::time::Instant::now();
        let ret = $fn;
        let time = start_time.elapsed();
        eprintln!(
            "took {}: [{}:{}] {}",
            $crate::_format_duration(time),
            file!(),
            line!(),
            stringify!($fn),
        );
        ret
    }};
}

/// Mesure time to execute given expression and returns the value of a given expression
/// paired with expression's return value for quick and dirty time mesuring.
///
/// # Example:
///
/// ```
/// # use time_debug::with_time;
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// let (time, _ret) = with_time!(sleep(Duration::new(2, 0)));
/// ```
///
/// ```
/// # use time_debug::with_time;
/// let (time, ret) = with_time!(1 + 2);
/// assert_eq!(ret, 3);
/// ```
#[macro_export]
macro_rules! with_time {
    ($fn:expr) => {{
        let start_time = std::time::Instant::now();
        let ret = $fn;
        let time = start_time.elapsed();
        (time, ret)
    }};
}

pub use humantime::format_duration as _format_duration;

#[test]
fn test() {
    use std::thread::sleep;
    use std::time::Duration;
    time!(sleep(Duration::new(2, 0)));
}
