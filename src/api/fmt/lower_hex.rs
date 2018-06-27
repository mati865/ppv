//! Implement LowerHex formatting

macro_rules! impl_fmt_lower_hex {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl ::fmt::LowerHex for $id {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                write!(f, "{}(", stringify!($id))?;
                for i in 0..$elem_count {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    self.extract(i).fmt(f)?;
                }
                write!(f, ")")
            }
        }
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _fmt_lower_hex] {
                use super::*;
                #[test]
                fn lower_hex() {
                    use arrayvec::{ArrayString,ArrayVec};
                    type TinyString = ArrayString<[u8; 512]>;

                    use fmt::Write;
                    let v = $id::splat($elem_ty::default());
                    let mut s = TinyString::new();
                    write!(&mut s, "{:#x}", v).unwrap();

                    let mut beg = TinyString::new();
                    write!(&mut beg, "{}(", stringify!($id)).unwrap();
                    assert!(s.starts_with(beg.as_str()));
                    assert!(s.ends_with(")"));
                    let s: ArrayVec<[TinyString; 64]> = s.replace(beg.as_str(), "").replace(")", "").split(",")
                        .map(|v| TinyString::from(v.trim()).unwrap()).collect();
                    assert_eq!(s.len(), $id::lanes());
                    for (index, ss) in s.into_iter().enumerate() {
                        let mut e = TinyString::new();
                        write!(&mut e, "{:#x}", v.extract(index)).unwrap();
                        assert_eq!(ss, e);
                    }
                }
            }
        }
    };
}