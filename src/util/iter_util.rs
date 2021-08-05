use std::fmt::{Display, Formatter};

pub(crate) fn fmt_vec<T: Display>(pre: &str, vec: &Vec<T>, post: &str, f: &mut Formatter<'_>)
    -> std::fmt::Result {
    pre.fmt(f)?;
    let mut iter = vec.iter();
    if let Some(value0) = iter.next() {
        value0.fmt(f)?;
        for value in iter {
            ", ".fmt(f)?;
            value.fmt(f)?;
        }
    }
    post.fmt(f)
}
