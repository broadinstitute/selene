use std::fmt::{Display, Formatter};
use std::collections::HashMap;

pub(crate) fn fmt_vec<T: Display>(pre: &str, vec: &[T], post: &str, f: &mut Formatter<'_>)
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

fn fmt_entry<K: Display, V: Display>(key: K, value: V, f: &mut Formatter<'_>)
    -> std::fmt::Result {
    key.fmt(f)?;
    ": ".fmt(f)?;
    value.fmt(f)
}

pub(crate) fn fmt_map<K: Display, V: Display>(pre: &str, map: &HashMap<K, V>,
                                              post: &str, f: &mut Formatter<'_>)
                                  -> std::fmt::Result {
    pre.fmt(f)?;
    let mut iter = map.iter();
    if let Some((key0, value0)) = iter.next() {
        fmt_entry(key0, value0, f)?;
        for (key, value) in iter {
            ", ".fmt(f)?;
            fmt_entry(key, value, f)?;
        }
    }
    post.fmt(f)
}
