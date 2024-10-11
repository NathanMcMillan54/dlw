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

/// Check if there was an error while calling an ``std::io`` function. If this macro returns ``true`` then the socket/stream
/// should be disconnected, it will return ``false`` if ``TimedOut`` or ``WouldBlock`` is returned from the function.
/// 
/// ``($code:expr)`` and ``($code:expr, $log:expr)`` run the same code except ``($code:expr, $log:expr)`` is intended to log
/// any information if an error comes up.
#[macro_export]
macro_rules! io_err_check {
    ($code:expr) => {
        {
            let tmp_ret = $code;

            if tmp_ret.is_err() {
                match tmp_ret.err().unwrap().kind() {
                    std::io::ErrorKind::NotConnected | std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionAborted => {
                        true
                    },
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => false,
                    _ => {
                        true
                    },
                }
            } else {
                tmp_ret.unwrap();
                false
            }
        }
    };

    ($code:expr, $log:expr) => {
        {
            let tmp_ret = $code;

            if tmp_ret.is_err() {
                match tmp_ret.err().unwrap().kind() {
                    std::io::ErrorKind::NotConnected | std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::ConnectionAborted => {
                        $log;
                        true
                    },
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => false,
                    _ => {
                        true
                    },
                }
            } else {
                tmp_ret.unwrap();
                false
            }
        }
    };
}