extern crate alloc;

use alloc::vec::Vec;

/// To extend types which implement `AsRef<str>` to have `starts_with_caseless_ascii_multiple` and `starts_with_caseless_multiple` methods.
pub trait EndsWithCaselessMultiple {
    /// Returns `Some(usize)` if the given string slices case-insensitively (only ignoring ASCII case) matches a suffix of this string slice .
    fn ends_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> Option<usize>;
    #[cfg(feature = "std")]
    /// Returns `Some(usize)` if the given string slices case-insensitively (using case-folding) matches a suffix of this string slice .
    fn ends_with_caseless_multiple<S: AsRef<str>>(&self, s: S) -> Option<usize>;
}

impl<T: AsRef<str>> EndsWithCaselessMultiple for T {
    #[allow(unused_must_use)]
    fn ends_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> Option<usize> {
        let s_length = s.len();

        if s_length == 0 {
            return None;
        }

        let a = self.as_ref();

        let a_length = a.len();

        let mut bcss = Vec::with_capacity(s_length);

        for (i, s) in s.iter().enumerate().rev() {
            let s = s.as_ref();

            let s_length = s.len();

            if s_length == 0 {
                return Some(i);
            } else if s_length <= a_length {
                bcss.push((i, s.bytes().rev()));
            }
        }

        let mut acs = a.bytes().rev();

        loop {
            let bcss_len = bcss.len();

            match acs.next() {
                Some(ac) => {
                    let acl = ac.to_ascii_lowercase();

                    if bcss_len == 0 {
                        return None;
                    }

                    for i in (0..bcss_len).rev() {
                        let bcs = &mut bcss[i];

                        match bcs.1.next() {
                            Some(bc) => {
                                let bcl = bc.to_ascii_lowercase();

                                if acl != bcl {
                                    bcss.remove(i);
                                }
                            }
                            None => {
                                return Some(bcs.0);
                            }
                        }
                    }
                }
                None => {
                    break if bcss_len == 0 {
                        None
                    } else {
                        for mut bcs in bcss.into_iter().rev() {
                            if bcs.1.next().is_none() {
                                return Some(bcs.0);
                            }
                        }

                        unreachable!()
                    };
                }
            }
        }
    }

    #[cfg(feature = "std")]
    fn ends_with_caseless_multiple<S: AsRef<str>>(&self, _s: S) -> Option<usize> {
        // TODO: Implement this after `ends_with_caseless` can be done without `to_uppercase` or `to_lowercase` methods.
        unimplemented!()
    }
}
