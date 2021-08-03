use std::collections::{HashMap, BTreeSet};
use crate::util::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::variant::Variant;

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
struct Interval {
    begin: u32,
    end: u32,
}

struct Region {
    chrom: String,
    interval: Interval,
}

pub(crate) struct Regions {
    by_chrom: HashMap<String, Vec<Interval>>,
}

struct RegionsBuffer {
    by_chrom: HashMap<String, BTreeSet<Interval>>,
}

impl Interval {
    pub fn new(begin: u32, end: u32) -> Interval {
        Interval { begin, end }
    }
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
    pub(crate) fn load(file: &str) -> Result<Regions, Error> {
        let reader = BufReader::new(File::open(file)?);
        let mut regions_buffer = RegionsBuffer::new();
        for line_result in reader.lines() {
            let line = line_result?;
            let mut parts = line.split('\t');
            let _id = parts.next().ok_or("Need at least four columns.")?;
            let chrom = parts.next().ok_or("chrom column missing.")?;
            let begin = parts.next().ok_or("chrom column missing.")?.parse::<u32>()?;
            let end = parts.next().ok_or("end column missing.")?.parse::<u32>()?;
            let region = Region::new(chrom.to_string(), begin, end);
            regions_buffer.add(region);
        }
        Ok(regions_buffer.as_regions())
    }
    pub(crate) fn overlap(&self, variant: &Variant) -> bool {
        let interval = Interval::new(variant.pos, variant.end());
        let chrom = &variant.chrom;
        match self.by_chrom.get(chrom.as_str()) {
            None => { false }
            Some(intervals) => { overlaps_intervals(&interval, intervals) }
        }
    }
}

fn overlaps_intervals(interval: &Interval, intervals: &[Interval]) -> bool {
    if intervals.is_empty() {
        false
    } else {
        let mut i_min: usize = 0;
        let mut i_max: usize = intervals.len() - 1;
        loop {
            if i_min == i_max {
                break intervals[i_min].overlaps(interval);
            } else {
                let i_mid = (i_min + i_max) / 2;
                let interval_i_mid = intervals[i_mid];
                if interval_i_mid.end <= interval.begin {
                    i_min = i_mid + 1;
                } else if interval_i_mid.begin >= interval.end {
                    if i_mid > 0 {
                        i_max = i_mid - 1;
                    } else {
                        break false;
                    }
                } else {
                    break true;
                }
                if i_min > i_max {
                    break false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::error::Error;
    use crate::cache::regions::Regions;
    use crate::variant::Variant;
    use std::io::{BufWriter, Write};
    use std::fs::File;

    fn write_regions_line(writer: &mut BufWriter<File>, id: &str, chrom: &str, begin: u32,
                          end: u32) -> Result<(), Error> {
        let line = format!("{}\t{}\t{}\t{}\n", id, chrom, begin, end);
        writer.write(line.as_bytes())?;
        Ok(())
    }

    fn write_regions_file(path: &str) -> Result<(), Error> {
        let mut writer = BufWriter::new(File::create(path)?);
        write_regions_line(&mut writer, "region1", "1", 100, 200)?;
        write_regions_line(&mut writer, "region2", "2", 100, 200)?;
        write_regions_line(&mut writer, "region3", "2", 200, 300)?;
        write_regions_line(&mut writer, "region4", "3", 100, 200)?;
        write_regions_line(&mut writer, "region5", "3", 300, 400)?;
        writer.flush()?;
        Ok(())
    }

    fn assert_included(regions: &Regions, variant: &Variant) {
        assert!(regions.overlap(variant), "Regions should include variant {}, but don't.", variant)
    }

    fn assert_not_included(regions: &Regions, variant: &Variant) {
        assert!(!regions.overlap(variant), "Regions shouldn't include variant {}, but do.",
                variant)
    }

    fn new_variant(chrom: &str, pos: u32, ref_allele: &str, alt_allele: &str) -> Variant {
        Variant::new(chrom.to_string(), pos, ref_allele.to_string(),
                     alt_allele.to_string())
    }

    #[test]
    fn load_and_test_regions() -> Result<(), Error> {
        let regions_file_path = "tmp/regions.tsv";
        write_regions_file(&regions_file_path)?;
        let regions = Regions::load(regions_file_path)?;
        assert_eq!(regions.by_chrom.len(), 3);
        assert_eq!(regions.by_chrom.get("1").unwrap().len(), 1);
        assert_eq!(regions.by_chrom.get("2").unwrap().len(), 1);
        assert_eq!(regions.by_chrom.get("3").unwrap().len(), 2);
        assert_not_included(&regions,
                            &new_variant("1", 50, "A", "T"));
        assert_included(&regions,
                        &new_variant("1", 150, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("1", 250, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("2", 50, "A", "T"));
        assert_included(&regions,
                        &new_variant("2", 150, "A", "T"));
        assert_included(&regions,
                        &new_variant("2", 250, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("2", 350, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("3", 50, "A", "T"));
        assert_included(&regions,
                        &new_variant("3", 150, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("3", 250, "A", "T"));
        assert_included(&regions,
                        &new_variant("3", 350, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("3", 450, "A", "T"));
        assert_not_included(&regions,
                            &new_variant("X", 50, "A", "T"));
        Ok(())
    }
}

