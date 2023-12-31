use std::cmp::max;
use std::ops::Range;

const INPUT: &str = include_str!("../inputs/day_5.txt");

#[derive(Debug)]
struct Mapping {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

pub fn star_1() {
    // each section
    let mut sections = INPUT.split("\n\n");
    let mut seeds = sections
        // first section
        .next()
        .unwrap()
        // split by space
        .split(' ')
        // skip first entry, which is the label
        .skip(1)
        // parse to nums
        .map(|s| s.parse::<i64>().unwrap())
        // collect
        .collect::<Vec<i64>>();
    for section in sections {
        let mut map: Vec<Mapping> = vec![];
        // first entry is label, skip and theniterate
        for entry in section.split('\n').skip(1) {
            if entry.is_empty() {
                break;
            }
            // iterate over split by spaces to get 3 data points
            let mut nums = entry.split(' ');
            map.push(Mapping {
                destination_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                source_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                range_length: nums.next().unwrap().parse::<i64>().unwrap(),
            })
        }
        for seed in seeds.iter_mut() {
            for entry in map.iter() {
                // if seed is in the mapped range
                if (entry.source_range_start..entry.source_range_start + entry.range_length)
                    .contains(seed)
                {
                    // map the seed
                    *seed += entry.destination_range_start - entry.source_range_start;
                    break;
                }
            }
        }
    }
    println!("{}", seeds.iter().min().unwrap())
}

struct RangeOverlap {
    overlap: Option<Range<i64>>,
    non_overlap_left: Option<Range<i64>>,
    non_overlap_right: Option<Range<i64>>,
}

fn is_in_ascending_order(list: &[i64]) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

fn ranges_overlap(range: &Range<i64>, overlapper: &Range<i64>) -> RangeOverlap {
    // there is almost certianly a better way to do this but this works and my brain hurts
    if range.start == overlapper.end {
        dbg!(&range, &overlapper);
    }
    // range       ------
    // overlapper -----
    if is_in_ascending_order(&[overlapper.start, range.start, overlapper.end, range.end]) {
        RangeOverlap {
            overlap: Some(range.start..overlapper.end),
            non_overlap_right: Some(overlapper.end..range.end),
            non_overlap_left: None,
        }
    }
    // range       ---
    // overlapper -----
    else if is_in_ascending_order(&[overlapper.start, range.start, range.end, overlapper.end]) {
        RangeOverlap {
            overlap: Some(range.clone()),
            non_overlap_right: None,
            non_overlap_left: None,
        }
    }
    // range      -----
    // overlapper  ---
    else if is_in_ascending_order(&[range.start, overlapper.start, overlapper.end, range.end]) {
        RangeOverlap {
            overlap: Some(overlapper.clone()),
            non_overlap_right: Some(range.start..overlapper.start),
            non_overlap_left: Some(overlapper.end..range.end),
        }
    }
    // range      -----
    // overlapper   -----
    else if is_in_ascending_order(&[range.start, overlapper.start, range.end, overlapper.end]) {
        RangeOverlap {
            overlap: Some(overlapper.start..range.end),
            non_overlap_right: None,
            non_overlap_left: Some(range.start..overlapper.start),
        }
    }
    // range      -----
    // overlapper       -----
    else if is_in_ascending_order(&[overlapper.start, overlapper.end, range.start, range.end])
        || is_in_ascending_order(&[range.start, range.end, overlapper.start, overlapper.end])
    {
        // yes the non-overlap can be left or right, but it doesnt matter, theyre treated the same and only named differently for convenience
        RangeOverlap {
            overlap: None,
            non_overlap_left: Some(range.clone()),
            non_overlap_right: None,
        }
    } else {
        panic!("unknown order {overlapper:#?} {range:#?}");
    }
}

fn merge_ranges(vc: &mut Vec<Range<i64>>) {
    // dbg!(&vc);
    let mut i = 0;
    while i + 1 < vc.len() {
        vc.sort_by_key(|v| v.start);
        if vc[i].end >= vc[i + 1].start {
            let lower = vc[i].clone();
            let upper = vc[i + 1].clone();
            vc.remove(i);
            vc.remove(i);
            vc.insert(i, lower.start..max(lower.end, upper.end))
        } else {
            i += 1;
        }
    }
    // dbg!(&vc);
}

pub fn star_2() -> i64 {
    // each section
    let mut sections = INPUT.split("\n\n");
    let mut seeds_ranges = sections
        // first section
        .next()
        .unwrap()
        // split by space
        .split(' ')
        // skip first entry, which is the label
        .skip(1)
        // parse to nums
        .map(|s| s.parse::<i64>().unwrap())
        // collect
        .collect::<Vec<i64>>()
        // chunk into 2s
        .chunks(2)
        // map to tuple
        .map(|s| s[0]..s[0] + s[1])
        // collect
        .collect::<Vec<Range<i64>>>();
    // might not be necessary
    // seeds_ranges.sort_by_key(|r| r.start);
    for section in sections {
        let mut map: Vec<Mapping> = vec![];
        // first entry is label, skip and then iterate
        for entry in section.split('\n').skip(1) {
            if entry.is_empty() {
                break;
            }
            // iterate over split by spaces to get 3 data points
            let mut nums = entry.split(' ');
            map.push(Mapping {
                destination_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                source_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                range_length: nums.next().unwrap().parse::<i64>().unwrap(),
            })
        }
        // dbg!(&seeds_ranges);
        let mut new_seeds: Vec<Range<i64>> = vec![];
        for seed in seeds_ranges {
            let mut this_seed: Vec<Range<i64>> = vec![seed.clone()];
            let mut mapped_seeds: Vec<Range<i64>> = vec![];
            for entry in map.iter() {
                // let mut unmapped_seeds: Vec<Seed> = seeds_ranges.iter().map(|s| Seed { range: s.clone() }).collect();
                // let mut mapped_seeds: Vec<Range<i64>> = vec!();

                let mut i = 0;
                while i < this_seed.len() {
                    let overlap = ranges_overlap(
                        &this_seed[i],
                        &(entry.source_range_start..entry.source_range_start + entry.range_length),
                    );
                    let mut replace: Vec<Range<i64>> = vec![];
                    if let Some(l) = overlap.non_overlap_right {
                        if !l.is_empty() {
                            replace.push(l);
                        }
                    }
                    if let Some(l) = overlap.non_overlap_left {
                        if !l.is_empty() {
                            replace.push(l);
                        }
                    }
                    if let Some(mut l) = overlap.overlap {
                        // dbg!(&l);
                        l.start += entry.destination_range_start - entry.source_range_start;
                        l.end += entry.destination_range_start - entry.source_range_start;
                        // dbg!(&l);
                        if !l.is_empty() {
                            mapped_seeds.push(l);
                        }
                    }
                    let len = replace.len();
                    this_seed.splice(i..i + 1, replace);

                    i += len;
                }
            }
            new_seeds.append(&mut this_seed);
            new_seeds.append(&mut mapped_seeds);
        }
        seeds_ranges = new_seeds;
        // merge_ranges(&mut seeds_ranges);
    }
    seeds_ranges.sort_by_key(|r| r.start);
    dbg!(&seeds_ranges);
    let yippee = seeds_ranges.iter().map(|r| r.start).min().unwrap();
    println!("{yippee}");
    yippee
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_5::*;
    use test::Bencher;

    #[bench]
    fn bench_star_2(b: &mut Bencher) {
        b.iter(star_2);
    }

    #[bench]
    fn bench_star_1(b: &mut Bencher) {
        b.iter(star_1);
    }
}
