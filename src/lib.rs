/// A trait for fast substring search in strings.
///
/// # Examples
///
/// ```
/// use your_crate::fastcontains;
///
/// let haystack = "hello world";
/// assert!(haystack.fast_contains("world"));
/// assert!(!haystack.fast_contains("mars"));
/// ```
pub trait fastcontains {
    /// Returns `true` if the `needle` string is a substring of `self`.
    ///
    /// This method uses raw pointer arithmetic for fast byte-wise comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use fastcontains::fastcontains;
    ///
    /// let haystack = "hello world";
    /// assert!(haystack.fast_contains("world"));
    /// assert!(!haystack.fast_contains("mars"));
    /// ```
    fn fast_contains(&self, needle: &str) -> bool;
}

/// Implements `fastcontains` for string slices (`str`).
impl fastcontains for str {
    /// Returns `true` if the `needle` string is a substring of `self`.
    ///
    /// This method uses raw pointer arithmetic for fast byte-wise comparison.
    fn fast_contains(&self, needle: &str) -> bool {
        let haystack_ptr = self.as_ptr();
        let needle_ptr = needle.as_ptr();
        let haystack_len = self.len();
        let needle_len = needle.len();

        if needle_len <= haystack_len {
            let max_start = haystack_len - needle_len;
            let mut i = 0;
            while i <= max_start {
                let mut matched = true;
                let mut j = 0;
                unsafe {
                    while j < needle_len {
                        // Compare each byte using raw pointer arithmetic
                        if *haystack_ptr.add(i + j) != *needle_ptr.add(j) {
                            matched = false;
                            break;
                        }
                        j += 1;
                    }
                }
                if matched {
                    return true;
                }
                i += 1;
            }
            false
        } else {
            false
        }
    }
}

impl fastcontains for String {
    fn fast_contains(&self, needle: &str) -> bool {
        self.as_str().fast_contains(needle)
    }
}
