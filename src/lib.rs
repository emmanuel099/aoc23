use std::fs;
use std::io;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_to_string(filename)?
        .lines()
        .map(|s| s.to_owned())
        .collect())
}
