use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::str::FromStr;

pub struct Reader {}

impl Reader {
    pub fn read_file<T: FromStr, R: FromIterator<T>>(
        path: String,
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

    pub fn write_file<T: Iterator>(data: T, path: String)
    where
        <T as Iterator>::Item: std::fmt::Display,
    {
        let mut outfile = File::create(path).unwrap();
        for i in data {
            writeln!(outfile, "{}", i).unwrap();
        }
    }
}

#[cfg(test)]
mod test_file {}
