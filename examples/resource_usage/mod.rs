//! Optional observed peak resident bytes. Darwin reports bytes; Linux procfs reports KiB.
#![allow(dead_code)]
#[cfg(any(target_os = "linux", test))]
fn linux_peak(status: &str) -> Option<u64> {
    let mut fields = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))?
        .split_whitespace();
    let amount = fields.next()?.parse::<u64>().ok()?;
    if fields.next()? != "kB" {
        return None;
    }
    amount.checked_mul(1024).filter(|n| *n > 0)
}
pub fn peak_rss_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    return std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| linux_peak(&s));
    #[cfg(target_os = "macos")]
    {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        // SAFETY: writable rusage storage with the native ABI; no borrowed pointers escape.
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
            return None;
        }
        // SAFETY: successful getrusage initialized usage. Darwin ru_maxrss uses bytes.
        u64::try_from(unsafe { usage.assume_init() }.ru_maxrss)
            .ok()
            .filter(|n| *n > 0)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rss_units_missing_and_overflow_are_explicit() {
        assert_eq!(linux_peak("VmHWM: 12 kB\n"), Some(12 * 1024));
        for input in [
            "",
            "VmHWM: 0 kB",
            "VmHWM: 3 bytes",
            "VmHWM: nope kB",
            "VmHWM: 18446744073709551615 kB",
        ] {
            assert_eq!(linux_peak(input), None);
        }
        assert!(peak_rss_bytes().is_none_or(|n| n > 0));
    }
}
