/// To extend types which implement `AsRef<str>` to have `ends_with_caseless_ascii` and `ends_with_caseless` methods.
pub trait EndsWithCaseless {
    /// Returns `true` if the given string slice case-insensitively (only ignoring ASCII case) matches a suffix of this string slice .
    fn ends_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool;
    #[cfg(not(feature = "no_std"))]
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a suffix of this string slice .
    fn ends_with_caseless<S: AsRef<str>>(&self, s: S) -> bool;
}

impl<T: AsRef<str>> EndsWithCaseless for T {
    fn ends_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        let b_len = b.len();

        if b_len == 0 {
            return true;
        }

        let a_len = a.len();

        if a_len < b_len {
            false
        } else {
            let a = &a[(a_len - b_len)..];

            let mut acs = a.chars().rev();
            let mut bcs = b.chars().rev();

            loop {
                match bcs.next() {
                    Some(bc) => {
                        let bcu = bc.to_ascii_uppercase();
                        let acu = acs.next().unwrap().to_ascii_uppercase();

                        if acu != bcu {
                            break false;
                        }
                    }
                    None => {
                        break true;
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "no_std"))]
    fn ends_with_caseless<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        if b.is_empty() {
            return true;
        }

        {
            let au = a.to_uppercase();
            let bu = b.to_uppercase();

            let au_len = au.len();
            let bu_len = bu.len();

            if au_len >= bu_len {
                let mut aucs = au.chars().rev();
                let mut bucs = bu.chars().rev();

                let pass = loop {
                    match bucs.next() {
                        Some(buc) => {
                            let auc =  aucs.next().unwrap();

                            if auc != buc {
                                break false;
                            }
                        }
                        None => {
                            break true;
                        }
                    }
                };

                if pass {
                    return true;
                }
            }
        }


        let al = a.to_lowercase();
        let bl = b.to_lowercase();

        let al_len = al.len();
        let bl_len = bl.len();

        if al_len < bl_len {
            false
        } else {
            let mut alcs = al.chars().rev();
            let mut blcs = bl.chars().rev();

            loop {
                match blcs.next() {
                    Some(blc) => {
                        let alc = alcs.next().unwrap();

                        if alc != blc {
                            return false;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            true
        }
    }
}