use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

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

pub fn write_file<T: Iterator, P: AsRef<Path>>(data: T, path: P)
where
    <T as Iterator>::Item: std::fmt::Display,
{
    let mut outfile = File::create(path).unwrap();
    for i in data {
        writeln!(outfile, "{}", i).unwrap();
    }
}

pub fn write_serial<T: IntoIterator + serde::Serialize, P: AsRef<Path>>(
    data: T,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    let str = serde_json::to_string_pretty(&data)?;
    write!(file, "{}", str)?;

    Ok(())
}

pub fn read_serial<T: IntoIterator + serde::de::DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, Box<dyn std::error::Error>> {
    let str: String = read_to_string(path)?;
    let data = serde_json::from_str::<T>(&str)?;

    Ok(data)
}

#[cfg(test)]
mod test_file {
    use crate::{Enumerable, Permutation, Square, O3};
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_write_serial() -> Result<(), Box<dyn std::error::Error>> {
        let data = (300..302)
            .map(|a| Permutation::<O3>::kth(a))
            .collect::<BTreeSet<_>>();

        write_serial(data, "examples/tests/orderthree/test_serial.txt")?;
        Ok(())
    }

    #[test]
    fn test_read_serial() -> Result<(), Box<dyn std::error::Error>> {
        let data = (300..302)
            .map(|a| Permutation::<O3>::kth(a))
            .collect::<BTreeSet<_>>();

        write_serial(data, "examples/tests/orderthree/test_serial.txt")?;

        let expected: BTreeSet<Permutation<O3>> = BTreeSet::from([
            Permutation {
                square: Square {
                    data: [1, 2, 3, 6, 7, 8, 4, 5, 9],
                },
            },
            Permutation {
                square: Square {
                    data: [1, 2, 3, 6, 7, 8, 4, 9, 5],
                },
            },
        ]);

        let read: BTreeSet<Permutation<O3>> =
            read_serial("examples/tests/orderthree/test_serial.txt")?;

        assert_eq!(read, expected);

        Ok(())
    }
}
