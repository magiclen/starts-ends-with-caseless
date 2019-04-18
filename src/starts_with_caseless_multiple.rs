/// To extend types which implement `AsRef<str>` to have `starts_with_caseless_ascii_multiple` and `starts_with_caseless_multiple` methods.
pub trait StartsWithCaselessMultiple {
    /// Returns `true` if the given string slices case-insensitively (only ignoring ASCII case) matches a prefix of this string slice .
    fn starts_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> bool;
    /// Returns `true` if the given string slices case-insensitively (using case-folding) matches a prefix of this string slice .
    fn starts_with_caseless_multiple<S: AsRef<str>>(&self, s: S) -> bool;
}

impl<T: AsRef<str>> StartsWithCaselessMultiple for T {
    fn starts_with_caseless_ascii_multiple<S: AsRef<str>>(&self, s: &[S]) -> bool {
        let s_len = s.len();

        if s_len == 0 {
            return true;
        }

        let a = self.as_ref();

        let a_len = a.len();

        let mut bs = Vec::with_capacity(s_len);

        for s in s {
            let s = s.as_ref();

            let s_len = s.len();

            if s_len == 0 {
                return true;
            } else if s_len <= a_len {
                bs.push(s);
            }
        }

        let mut bcss: Vec<_> = bs.iter().map(|&b| b.chars()).collect();

        let mut acs = a.chars();

        loop {
            let bcss_len = bcss.len();

            match acs.next() {
                Some(ac) => {
                    let acu = ac.to_ascii_uppercase();

                    if bcss_len == 0 {
                        return false;
                    }

                    for i in (0..bcss_len).rev() {
                        let bcs = &mut bcss[i];

                        match bcs.next() {
                            Some(bc) => {
                                let bcu = bc.to_ascii_uppercase();

                                if acu != bcu {
                                    bcss.remove(i);
                                }
                            }
                            None => {
                                return true;
                            }
                        }
                    }
                }
                None => {
                    break bcss_len != 0;
                }
            }
        }
    }

    fn starts_with_caseless_multiple<S: AsRef<str>>(&self, _s: S) -> bool {
        // TODO: Implement this after `starts_with_caseless` can be done without `to_uppercase` or `to_lowercase` methods.
        unimplemented!()
    }
}