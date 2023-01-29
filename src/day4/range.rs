pub struct Range {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl Range {
    pub(crate) fn includes(self: &Range, range: &Range) -> bool {
        range.start >= self.start && range.end <= self.end
    }

    fn includes_idx(self: &Range, idx: u32) -> bool {
        idx >= self.start && idx <= self.end
    }

    pub(crate) fn overlaps(first: &Range, second: &Range) -> bool {
        (*first).includes_idx(second.start) || second.includes_idx(first.start)
    }

    pub(crate) fn fully_overlaps(first: &Range, second: &Range) -> bool {
        first.includes(second) || second.includes(first)
    }

    pub(crate) fn from_range_str(range: &str) -> Range {
        let (start, end) =
            range
                .split_once("-")
                .expect("Range should have a '-' as delimiter");

        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();

        Range {
            start,
            end,
        }
    }
}
