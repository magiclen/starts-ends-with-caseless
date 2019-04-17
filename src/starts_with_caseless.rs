/// To extend types which implement `AsRef<str>` to have `starts_with_caseless_ascii` and `starts_with_caseless` methods.
pub trait StartsWithCaseless {
    /// Returns `true` if the given string slice case-insensitively (only ignoring ASCII case) matches a prefix of this string slice .
    fn starts_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool;
    /// Returns `true` if the given string slice case-insensitively (using case-folding) matches a prefix of this string slice .
    fn starts_with_caseless<S: AsRef<str>>(&self, s: S) -> bool;
}

impl<T: AsRef<str>> StartsWithCaseless for T {
    fn starts_with_caseless_ascii<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        let a_len = a.len();
        let b_len = b.len();

        if a_len < b_len {
            false
        } else {
            let a = &a[..b_len];

            let mut acs = a.chars();
            let mut bcs = b.chars();

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

    fn starts_with_caseless<S: AsRef<str>>(&self, s: S) -> bool {
        let a = self.as_ref();
        let b = s.as_ref();

        {
            let au = a.to_uppercase();
            let bu = b.to_uppercase();

            let au_len = au.len();
            let bu_len = bu.len();

            if au_len >= bu_len {
                let mut aucs = au.chars();
                let mut bucs = bu.chars();

                let pass = loop {
                    match bucs.next() {
                        Some(buc) => {
                            match aucs.next() {
                                Some(auc) => {
                                    if auc != buc {
                                        break false;
                                    }
                                }
                                None => {
                                    break false;
                                }
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
            let mut alcs = al.chars();
            let mut blcs = bl.chars();

            loop {
                match blcs.next() {
                    Some(blc) => {
                        match alcs.next() {
                            Some(alc) => {
                                if alc != blc {
                                    return false;
                                }
                            }
                            None => {
                                return false;
                            }
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