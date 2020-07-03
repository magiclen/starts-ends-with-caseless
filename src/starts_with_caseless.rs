#[cfg(feature = "std")]
extern crate cow_utils;

#[cfg(feature = "std")]
use cow_utils::CowUtils;

/// To extend types which implement `AsRef<str>` to have `starts_with_caseless_ascii` and `starts_with_caseless` methods.
pub trait StartsWithCaseless {
    /// Returns `true` if the given string slice case-insensitively (only ignoring ASCII case) matches a prefix of this string slice .
    fn starts_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool;
    #[cfg(feature = "std")]
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a prefix of this string slice .
    fn starts_with_caseless<S: AsRef<str>>(&self, s: S) -> bool;
}

impl<T: AsRef<str>> StartsWithCaseless for T {
    fn starts_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        let b_length = b.len();

        if b_length == 0 {
            return true;
        }

        let a_length = a.len();

        if a_length >= b_length {
            a[..b_length].eq_ignore_ascii_case(b)
        } else {
            false
        }
    }

    #[cfg(feature = "std")]
    fn starts_with_caseless<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        if b.is_empty() {
            return true;
        }

        {
            let au = a.cow_to_uppercase();
            let bu = b.cow_to_uppercase();

            let au_length = au.len();
            let bu_length = bu.len();

            if au_length >= bu_length && &au.as_bytes()[..bu_length] == bu.as_bytes() {
                return true;
            }
        }

        let al = a.cow_to_lowercase();
        let bl = b.cow_to_lowercase();

        let al_length = al.len();
        let bl_length = bl.len();

        if al_length >= bl_length {
            &al.as_bytes()[..bl_length] == bl.as_bytes()
        } else {
            false
        }
    }
}
