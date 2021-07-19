use std::collections::{HashMap, BTreeSet};
use crate::util::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::variant::Variant;
use std::env::var;
use std::alloc::Global;

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

impl Region {
    pub fn new(chrom: String, begin: u32, end: u32) -> Region {
        let interval = Interval { begin, end };
        Region { chrom, interval }
    }
}

impl RegionsBuffer {
    fn new() -> RegionsBuffer {
        let by_chrom = HashMap::<String, BTreeSet<Interval>>::new();
        RegionsBuffer { by_chrom }
    }
    fn add(&mut self, region: Region) {
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
        let mut by_chrom: HashMap::<String, Vec<Interval>> = HashMap::new();
        for (chrom, interval_set) in &self.by_chrom {
            let intervals = RegionsBuffer::consolidate_intervals(&interval_set);
            by_chrom.insert(chrom.clone(), intervals);
        }
        Regions { by_chrom }
    }
}

impl Regions {
    fn load(file: String) -> Result<Regions, Error> {
        let reader = BufReader::new(File::open(file)?);
        let mut regions_buffer = RegionsBuffer::new();
        for line_result in reader.lines() {
            let line = line_result?;
            let mut parts = line.split('\t');
            let _id = parts.next().ok_or_else(|| { "Need at least four columns." })?;
            let chrom = parts.next().ok_or_else(|| { "chrom column missing." })?;
            let begin =
                parts.next().ok_or_else(|| { "begin column missing." })?.parse::<u32>()?;
            let end = parts.next().ok_or_else(|| { "end column missing." })?.parse::<u32>()?;
            let region = Region::new(chrom.to_string(), begin, end);
            regions_buffer.add(region);
        }
        Ok(regions_buffer.as_regions())
    }
    pub(crate) fn overlap(&self, variant: Variant) -> bool {
        let chrom = variant.chrom;
        match self.by_chrom.get(chrom.as_str()) {
            None => { false }
            Some(_) => {
                todo!()
            }
        }
    }
}
