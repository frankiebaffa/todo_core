use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;
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
