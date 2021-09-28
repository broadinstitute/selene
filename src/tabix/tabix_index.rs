use crate::genomics::variant;
use crate::genomics::variant::Variant;

const INTERVAL_SIZE: variant::Pos = 1 << 14;  //  16,384

pub(crate) fn pos_to_i_interval(pos: variant::Pos) -> u32 { pos / INTERVAL_SIZE }

// ported from https://samtools.github.io/hts-specs/tabix.pdf

pub(crate) fn variant_to_bins(variant: &Variant) -> Vec<u32> {
    let rbeg = variant.pos;
    let rend = variant.end() - 1;
    let reasonable_initial_capacity: usize = 8;
    let mut bins = Vec::<u32>::with_capacity(reasonable_initial_capacity);
    bins.push(0);
    for k in (1 + (rbeg >> 26))..=(1 + (rend >> 26)) { bins.push(k) }
    for k in (9 + (rbeg >> 23))..=(9 + (rend >> 23)) { bins.push(k) }
    for k in (73 + (rbeg >> 20))..=(73 + (rend >> 20)) { bins.push(k) }
    for k in (585 + (rbeg >> 17))..=(585 + (rend >> 17)) { bins.push(k) }
    for k in (4681 + (rbeg >> 14))..=(4681 + (rend >> 14)) { bins.push(k) }
    bins
}
