//! Local time provider for Live Clock mode.

pub struct ClockTime;

impl ClockTime {
    /// Returns current local time formatted as "HH:MM" or "HH:MM:SS"
    #[inline]
    pub fn now_str(show_seconds: bool) -> String {
        unsafe {
            let now = libc::time(std::ptr::null_mut());
            let mut tm: libc::tm = std::mem::zeroed();
            libc::localtime_r(&now, &mut tm);
            if show_seconds {
                format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
            } else {
                format!("{:02}:{:02}", tm.tm_hour, tm.tm_min)
            }
        }
    }
}
