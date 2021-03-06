use rand::Rng;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    let mut rng = rand::thread_rng();
    let mut roll = String::new();
    for _ in 0..6 {
        let num = rng.gen_range(0..6);
        roll.push_str(&num.to_string());
    }
    println!("{:?}", roll);
    let mut cwd = env::current_dir()?;
    let mut path = writej!("{}/data/eff_large_word_list_2016_07_18.txt", src);
    path.push("data/eff_large_word_list_2016_07_18.txt");
    let lines = parse_list(path)?;
    Ok(())
}

fn parse_list<P>(filename: P) -> Result<String, failure::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
