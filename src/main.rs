use anyhow::{Context, Result};
use rand::Rng;
use serde::Deserialize;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

#[derive(Debug, Deserialize)]
struct Quote {
    quotation: String,
    speaker: String,
}

fn main() -> Result<()> {
    let f = File::open("../datasets/quotes-2019-nytimes.json")
        .context("Failed to open main json file,\n
        the program expects the dataset to be located at ../datasets/ \n
        from where you execute it")?;
    let mut rdr = BufReader::new(f);
    let mut line = String::new();
    let mut quotes = Vec::new();
    while rdr.read_line(&mut line).is_ok() {
        if line.is_empty() {
            break;
        }
        let quote: Quote =
            serde_json::from_str(line.trim()).context("Failed to turn file into vec<Quote>")?;

        quotes.push(quote);
        line.clear();
    }
    let mut s = String::new();
    let mut rng = rand::thread_rng();
    println!("Enter q to quit, Enter any other string for 10 random quotes");
    loop {
        stdin()
            .read_line(&mut s)
            .context("Failed to read user input")?;
        if s.trim() == "q" {
            break;
        } else {
            (0..10)
                .map(|_| rng.gen_range(0..quotes.len()))
                .map(|index| (&quotes[index].quotation, &quotes[index].speaker))
                .for_each(|(quotation, speaker)| {
                    println!("...");
                    println!("{} --- {}", quotation, speaker);
                });
        }
    }
    Ok(())
}
