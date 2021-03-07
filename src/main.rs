use std::env;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get 6 rolls
    let mut rolls = Vec::<String>::new();
    let mut words = Vec::<String>::new();
    for _ in 0..6 {
        rolls.push(passphrase::roll()?);
    }

    // get word list
    let path = env::current_dir()?
        .as_path()
        .join("data/eff_large_word_list_2016_07_18.txt");
    let map = passphrase::parse_list(path)?;

    // generate a passphrase
    for roll in rolls.iter() {
        if let Some(word) = map.get(roll) {
            words.push(word.into());
        }
    }
    let passphrase = words.join(" ");
    println!("{}", passphrase);
    Ok(())
}
