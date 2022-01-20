use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;
use std::str::FromStr;
use std::path::PathBuf;
pub enum ExitCode {
    Success,
    NoListName,
    NoListItemMessage(PathBuf),
    NoListItemNumber(PathBuf),
    FileExists(PathBuf),
    FileDoesNotExist(PathBuf),
    //PathFailed(PathBuf),
    //FileCreationFailed(PathBuf),
    FailedToWrite(PathBuf),
    FailedToRead(PathBuf),
    FailedToOpen(PathBuf),
    FailedToDeserialize(serde_json::Error),
    FailedToSerialize(serde_json::Error),
}
impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::NoListName => 3,
            Self::NoListItemMessage(_) => 4,
            Self::NoListItemNumber(_) => 5,
            Self::FileExists(_) => 6,
            Self::FileDoesNotExist(_) => 7,
            //Self::PathFailed(_) => 8,
            //Self::FileCreationFailed(_) => 9,
            Self::FailedToWrite(_) => 10,
            Self::FailedToRead(_) => 11,
            Self::FailedToOpen(_) => 12,
            Self::FailedToDeserialize(_) => 13,
            Self::FailedToSerialize(_) => 14,
        }
    }
}
impl Display for ExitCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match self {
            Self::Success => write!(f, "Success"),
            Self::NoListName => f.write_str("No list name for list"),
            Self::NoListItemMessage(s) => {
                return f.write_str(&format!("No item-message for list \"{}\"", s.to_str().unwrap()));
            },
            Self::NoListItemNumber(s) => {
                return f.write_str(&format!("No item-number for list \"{}\"", s.to_str().unwrap()));
            },
            Self::FileExists(s) => {
                return f.write_str(&format!("File exists at path \"{}\"", s.to_str().unwrap()));
            },
            Self::FileDoesNotExist(s) => {
                return f.write_str(&format!("File does not exist at path \"{}\"", s.to_str().unwrap()));
            },
            //Self::PathFailed(s) => {
            //    return f.write_str(&format!("Failed to derive path from \"{}\"", s.to_str().unwrap()));
            //},
            //Self::FileCreationFailed(s) => {
            //    return f.write_str(&format!("Failed to create file at \"{}\"", s.to_str().unwrap()));
            //},
            Self::FailedToWrite(s) => {
                return f.write_str(&format!("Failed to write to file \"{}\"", s.to_str().unwrap()));
            },
            Self::FailedToRead(s) => {
                return f.write_str(&format!("Failed to read file \"{}\"", s.to_str().unwrap()));
            },
            Self::FailedToOpen(s) => {
                return f.write_str(&format!("Failed to open file \"{}\"", s.to_str().unwrap()));
            },
            Self::FailedToDeserialize(e) => {
                return f.write_str(&format!("Failed to deserialize json: {}", e));
            },
            Self::FailedToSerialize(e) => {
                return f.write_str(&format!("Failed to serialize to json: {}", e));
            },
        }
    }
}
pub enum PathExitCondition {
    Exists,
    NotExists,
    Ignore,
}
#[derive(Clone)]
pub enum PrintWhich {
    All,
    Complete,
    Incomplete,
}
impl Display for PrintWhich {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            Self::All => fmt.write_str("all"),
            Self::Complete => fmt.write_str("complete"),
            Self::Incomplete => fmt.write_str("incomplete"),
        }
    }
}
#[derive(Debug)]
pub struct ParsePrintWhichError;
impl Display for ParsePrintWhichError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        fmt.write_str("Failed to parse str to PrintWhich")
    }
}
impl std::error::Error for ParsePrintWhichError {}
impl FromStr for PrintWhich {
    type Err = ParsePrintWhichError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        match input {
            "all" => Ok(PrintWhich::All),
            "complete" => Ok(PrintWhich::Complete),
            "incomplete" => Ok(PrintWhich::Incomplete),
            _ => Err(ParsePrintWhichError {}),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ItemType {
    Todo,
    Note,
}
impl Display for ItemType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            Self::Todo => fmt.write_str("todo"),
            Self::Note => fmt.write_str("note"),
        }
    }
}
#[derive(Debug)]
pub struct ParseItemTypeError;
impl Display for ParseItemTypeError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        fmt.write_str("Failed to parse str to ItemType")
    }
}
impl std::error::Error for ParseItemTypeError {}
impl FromStr for ItemType {
    type Err = ParseItemTypeError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        match input {
            "todo" => Ok(Self::Todo),
            "note" => Ok(Self::Note),
            _ => Err(ParseItemTypeError {}),
        }
    }
}
