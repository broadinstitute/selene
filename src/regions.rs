use std::collections::{HashMap, BTreeSet};

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
struct Interval {
    begin: u32,
    end: u32,
}

struct Region {
    chrom: String,
    interval: Interval,
}

struct Regions {
    by_chrom: HashMap<String, Vec<Interval>>,
}

struct RegionsBuffer {
    by_chrom: HashMap<String, BTreeSet<Interval>>,
}

impl Interval {
    pub fn overlaps(&self, other: &Interval) -> bool {
        self.begin < other.end && self.end > other.begin
    }
    pub fn touches(&self, other: &Interval) -> bool {
        self.begin <= other.end && self.end >= other.begin
    }
    pub fn absorb(&mut self, other: &Interval) {
        if self.begin > other.begin { self.begin = other.begin }
        if self.end < other.end { self.end = other.end }
    }
}

impl RegionsBuffer {
    pub fn add(&mut self, region: Region) {
        let chrom = region.chrom;
        let interval = region.interval;
        match self.by_chrom.get_mut(chrom.as_str()) {
            None => {
                let mut intervals = BTreeSet::new();
                intervals.insert(interval);
                self.by_chrom.insert(chrom, intervals);
            }
            Some(intervals) => {
                intervals.insert(interval);
            }
        }
    }
    fn consolidate_intervals(interval_set: &BTreeSet<Interval>) -> Vec<Interval> {
        let mut intervals_iter = interval_set.iter();
        let mut intervals_consolidated = Vec::<Interval>::new();
        if let Some(first_interval) = intervals_iter.next() {
            let mut current_interval = *first_interval;
            for interval in intervals_iter {
                if current_interval.touches(interval) {
                    current_interval.absorb(interval);
                } else {
                    intervals_consolidated.push(current_interval);
                    current_interval = *interval;
                }
            }
            intervals_consolidated.push(current_interval);
        }
        intervals_consolidated
    }
    pub fn as_regions(&self) -> Regions {
        let mut by_chrom : HashMap::<String, Vec<Interval>> = HashMap::new();
        for (chrom, interval_set) in self.by_chrom {
            let intervals = consolidate_intervals(interval_set);
            by_chrom.insert(chrom, intervals);
        }
        Regions { by_chrom }
    }
}
