use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Section {
    start: u32,
    end: u32,
}

impl Section {
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlap(&self, other: &Self) -> bool {
        self.contains(other)
            || other.contains(self)
            || self.start <= other.start && self.end >= other.start
            || self.start >= other.start && self.start <= other.end
    }
}

#[derive(Debug, Clone)]
pub struct ParseSectionError;

impl FromStr for Section {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("-").map(|part| part.parse::<u32>()).collect();
        if parts.len() != 2 {
            return Err(ParseSectionError);
        }

        let start = &parts[0];
        let end = &parts[1];
        match (start, end) {
            (Ok(start), Ok(end)) => Ok(Self {
                start: *start,
                end: *end,
            }),
            _ => Err(ParseSectionError),
        }
    }

    type Err = ParseSectionError;
}
