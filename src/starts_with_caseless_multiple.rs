#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

/// To extend types which implement `AsRef<str>` to have `starts_with_caseless_ascii_multiple` and `starts_with_caseless_multiple` methods.
pub trait StartsWithCaselessMultiple {
    /// Returns `Some(usize)` if the given string slices case-insensitively (only ignoring ASCII case) matches a prefix of this string slice .
    fn starts_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> Option<usize>;
    #[cfg(not(feature = "no_std"))]
    /// Returns `Some(usize)` if the given string slices case-insensitively (using case-folding) matches a prefix of this string slice .
    fn starts_with_caseless_multiple<S: AsRef<str>>(&self, s: S) -> Option<usize>;
}

impl<T: AsRef<str>> StartsWithCaselessMultiple for T {
    fn starts_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> Option<usize> {
        let s_len = s.len();

        if s_len == 0 {
            return None;
        }

        let a = self.as_ref();

        let a_len = a.len();

        let mut bs = Vec::with_capacity(s_len);

        for (i, s) in s.iter().enumerate() {
            let s = s.as_ref();

            let s_len = s.len();

            if s_len == 0 {
                return Some(i);
            } else if s_len <= a_len {
                bs.push((i, s.as_bytes()));
            }
        }

        let mut bcss: Vec<_> = bs.iter().rev().map(|&b| (b.0, b.1.iter())).collect();

        let mut acs = a.as_bytes().iter();

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
                            if let None = bcs.1.next() {
                                return Some(bcs.0);
                            }
                        }

                        unreachable!()
                    };
                }
            }
        }
    }

    #[cfg(not(feature = "no_std"))]
    fn starts_with_caseless_multiple<S: AsRef<str>>(&self, _s: S) -> Option<usize> {
        // TODO: Implement this after `starts_with_caseless` can be done without `to_uppercase` or `to_lowercase` methods.
        unimplemented!()
    }
}