use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

/// roll simulates a roll of a dice five times
pub fn roll() -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let mut dice_roll = String::new();
    for _ in 0..5 {
        let num = rng.gen_range(1..7);
        dice_roll.push_str(&num.to_string());
    }
    Ok(dice_roll)
}

/// parse_list takes the EFF wordlist and parses the pairings of dice roll and the associated word
pub fn parse_list<P>(filename: P) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::<String, String>::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let split: Vec<&str> = l.split("\t").collect();
            map.insert(split[0].into(), split[1].into());
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_roll() -> Result<(), Box<dyn std::error::Error>> {
        let dice_roll = roll()?;
        assert_eq!(dice_roll.len(), 5);
        Ok(())
    }

    #[test]
    fn test_parse_list() -> Result<(), Box<dyn std::error::Error>> {
        // set up test file
        let dir = tempdir()?;
        let file_path = dir.path().join("test-file.txt");
        let file_path_2 = file_path.clone();
        let mut file = File::create(file_path)?;
        write!(file, "123\ttest")?;

        // test fn
        let content = parse_list(file_path_2)?;
        assert_eq!(content.get("123").unwrap(), "test");

        // remove artifacts
        dir.close()?;
        Ok(())
    }
}
