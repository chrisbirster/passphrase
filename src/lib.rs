use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

/// Roll simulates a roll of five dice
pub fn roll() -> Result<String, Box<dyn std::error::Error>> {
    // Generate a random seed
    let mut rng = rand::thread_rng();

    // Even though we are generating integers from the random seed lets parse this into a String to
    // make it easier to concatenate a bunch of numbers together
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
    // Open the filepath into a buffer
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Parse the file by line into a Hashmap. The key is the number index followed by the word on
    // the same line. This way we can just do a O(1) lookup of a generated dice roll from the Hashmap
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
    // Test that a roll can be performed
    fn test_roll() -> Result<(), Box<dyn std::error::Error>> {
        let dice_roll = roll()?;
        assert_eq!(dice_roll.len(), 5);
        Ok(())
    }

    #[test]
    // Test that a two-column (number | word) list can be parsed by searching for a number and
    // receiving a word. This represents the EFF word list document
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

        // Closing the temp directory will automatically remove any artifacts created
        dir.close()?;
        Ok(())
    }
}
