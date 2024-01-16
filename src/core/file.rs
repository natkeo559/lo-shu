use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

/// Read a file with type T on new lines into a collection.
/// # Errors
/// - If full path does not exist
/// # Panics
/// - If reading encountered a parse error
pub fn read_file<T: FromStr, R: FromIterator<T>, P: AsRef<Path>>(
    path: P,
) -> Result<R, Box<dyn std::error::Error>>
where
    <T as FromStr>::Err: Debug,
{
    let data = read_to_string(path)?
        .lines()
        .map(|line| line.trim().parse::<T>().unwrap())
        .collect::<R>();

    Ok(data)
}

/// Write the contents of a collection to file seperated by newlines.
/// # Panics
/// - If full file path does not exist
/// - If formatting the string fails
/// - If `write_fmt` fails
pub fn write_file<T: Iterator, P: AsRef<Path>>(data: T, path: P)
where
    <T as Iterator>::Item: std::fmt::Display,
{
    let mut outfile = File::create(path).unwrap();
    for i in data {
        writeln!(outfile, "{i}").unwrap();
    }
}

/// Write serialized data to file.
/// # Errors
/// - If full file path does not exist
/// - If formatting the string fails
/// - If `write_fmt` fails
pub fn write_serial<T: IntoIterator + serde::Serialize, P: AsRef<Path>>(
    data: &T,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    let fstr = serde_json::to_string_pretty(&data)?;
    write!(file, "{fstr}")?;

    Ok(())
}

/// Read serialized data from file to a type T.
/// # Errors
/// - If full file path does not exist
/// - If deserialization fails
pub fn read_serial<T: IntoIterator + serde::de::DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, Box<dyn std::error::Error>> {
    let fstr: String = read_to_string(path)?;
    let data = serde_json::from_str::<T>(&fstr)?;

    Ok(data)
}
