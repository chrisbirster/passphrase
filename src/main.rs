use std::env;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create some collections to keep track of rolls and words
    let mut rolls = Vec::<String>::new();
    let mut words = Vec::<String>::new();

    // Generate some dice rolls
    for _ in 0..6 {
        rolls.push(passphrase::roll()?);
    }

    // Do a one time conversion of the word list into a Hashmap so we can lookup the rolls
    // generated to get a list of words back
    let path = env::current_dir()?
        .as_path()
        .join("data/eff_large_word_list_2016_07_18.txt");
    let map = passphrase::parse_list(path)?;

    // Retrieve the word list
    for roll in rolls.iter() {
        if let Some(word) = map.get(roll) {
            words.push(word.into());
        }
    }

    // Output the passphrase
    let passphrase = words.join(" ");
    println!("{}", passphrase);
    Ok(())
}
