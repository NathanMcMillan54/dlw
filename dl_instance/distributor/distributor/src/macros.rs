/// This is intended to be used by any part of a distributor that would normally go through a loop but currently 
/// doesn't need to go through the entire loop because of some condition. This macro sleeps for a short amount of time
/// if ``$condition`` is true so that the loop is not constantly running as fast as possible.
#[macro_export]
macro_rules! sleep_condition {
    ($condition:expr) => {
        if $condition {
            std::thread::sleep($crate::IDLE_SLEEP);
            continue;
        }
    }
}

/// Handles errors that come from ``std::io::Result``
#[macro_export]
macro_rules! io_handle {
    () => {
        unimplemented!()
    };
}
