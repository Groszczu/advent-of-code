use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub struct File {
    size: u32,
}

impl File {
    pub fn new(size: u32) -> Self {
        Self { size }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Debug)]
pub enum ParseFileErr {
    MissingDelimiterErr,
    InvalidSizeErr(ParseIntError),
}

impl FromStr for File {
    type Err = ParseFileErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, _) = s.split_once(" ").ok_or(ParseFileErr::MissingDelimiterErr)?;

        let size = size.parse::<u32>().map_err(ParseFileErr::InvalidSizeErr)?;

        Ok(Self::new(size))
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub enum ParseDirectoryErr {
    MissingDirPrefixErr,
}

impl FromStr for Directory {
    type Err = ParseDirectoryErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix("dir ")
            .map(|dir_name| Directory::new(dir_name.to_owned()))
            .ok_or(ParseDirectoryErr::MissingDirPrefixErr)
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    FileNode(File),
    DirectoryNode(Directory),
}

impl FromStr for Node {
    type Err = ParseFileErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Directory::from_str(s)
            .map(Self::DirectoryNode)
            .or(File::from_str(s).map(Self::FileNode))
    }
}
