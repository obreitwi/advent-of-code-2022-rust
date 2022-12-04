use anyhow::Result;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

fn execute(path: &Path) -> Result<Vec<u64>> {
    let content = read_to_string(path)?;
    let mut retval = Vec::new();
    let mut current = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            retval.push(current);
            current = 0;
        } else {
            let value = line.parse::<u64>()?;
            current += value;
        }
    }
    Ok(retval)
}

fn get_max(path: &Path) -> Result<u64> {
    let values = execute(path)?;
    Ok(*values.iter().max().expect("no values"))
}

fn get_top_three(path: &Path) -> Result<u64> {
    let mut values = execute(path)?;
    values.sort();
    values.reverse();
    Ok(values.iter().take(3).sum())
}

fn main() -> Result<()> {
    println!("max: {}", get_max(&PathBuf::from("input.txt"))?);
    println!("top3: {}", get_top_three(&PathBuf::from("input.txt"))?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_max() {
        assert_eq!(get_max(&PathBuf::from("debug.txt")).unwrap(), 24000);
    }

    #[test]
    fn test_top_trhee() {
        assert_eq!(get_top_three(&PathBuf::from("debug.txt")).unwrap(), 45000);
    }
}
