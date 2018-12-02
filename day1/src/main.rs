use std::io::{Result, BufReader, Error};
use std::fs::File;

enum FrequencyChange {
    Decrease(u32),
    Increase(u32),
}

impl FrequencyChange {
    pub fn change(&self, n: i32) -> i32 {
        match *self {
            FrequencyChange::Increase(val) => n + (val as i32),
            FrequencyChange::Decrease(val) => n - (val as i32),
        }
    }
}

fn get_frequency_change_list(f: BufReader<File>) -> Result<Vec<FrequencyChange>> {
    use std::io::BufRead;

    let mut last_error: Option<Error> = None;

    let freq = f
        .lines()
        .map(|s| {
            match s {
                Ok(s) => {
                    use std::str::FromStr;

                    let mut chars = s.chars();
                    let prefix = chars.next();

                    if let Some(c) = prefix {
                        let num_str: String = chars.collect();
                        let val = u32::from_str(&*num_str).unwrap();

                        match c {
                            '+' => FrequencyChange::Increase(val),
                            '-' => FrequencyChange::Decrease(val),
                            _ => unimplemented!(),
                        }
                    } else {
                        unimplemented!();
                    }
                }
                Err(e) => {
                    last_error = Some(e);

                    FrequencyChange::Increase(0)
                }
            }
        })
        .collect();

    if let Some(e) = last_error {
        Err(e)
    } else {
        Ok(freq)
    }
}

fn calculate_final_frequency(changes: &[FrequencyChange]) -> i32 {
    changes
        .iter()
        .fold(0, move |acc, change|
            change.change(acc))
}

fn find_first_frequency_reached_twice(changes: &[FrequencyChange]) -> i32 {
    let mut freq = 0;
    let mut seen_freqs = vec![0];

    loop {
        for change in changes {
            freq = change.change(freq);

            if seen_freqs.contains(&freq) {
                return freq;
            }

            seen_freqs.push(freq);
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let f = File::open("./day1/input")?;
    let f = BufReader::new(f);

    let changes = get_frequency_change_list(f)?;

    println!("final frequency: {}", calculate_final_frequency(&*changes));
    println!("first duplicate frequency: {}", find_first_frequency_reached_twice(&*changes));

    Ok(())
}
