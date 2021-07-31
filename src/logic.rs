use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use anyhow::{bail, Result};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn step(num: u128) -> u128 {
    if num % 2 == 0 {
        num / 2
    } else {
        num * 3 + 1
    }
}

pub fn solve_single_number(num: u128) -> Result<()> {
    if num < 1 {
        bail!("Numbers passed cannot be lower than 1.");
    }
    let mut num_buffer = num;
    let mut solve_buffer = 0;
    loop {
        num_buffer = step(num_buffer);
        println!("{}", Style::new().bold().paint(num_buffer.to_string()));
        if num_buffer == 4 {
            solve_buffer = 4;
        }
        if solve_buffer == 4 && num_buffer == 2 {
            solve_buffer = 2;
        }
        if solve_buffer == 2 && num_buffer == 1 {
            solve_buffer = 1;
        }
        if solve_buffer == 1 {
            println!("This number is not it.");
            break;
        }
    }
    Ok(())
}

pub fn prove_numbers(low: u128, high: u128) -> Result<()> {
    if low < 1 || high < 1 {
        bail!("Numbers passed cannot be lower than 1.");
    }
    if low > high {
        bail!("The upper range cannot be lower than the lower range.");
    }
    if low == high {
        bail!("The upper range cannot be the same as the lower range.");
    }
    let unproven_list: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(Vec::new()));
    (low..=high).into_par_iter().for_each(|num| {
        let unproven_list = Arc::clone(&unproven_list);
        let mut num_buffer = num;
        let mut solve_buffer = 0;
        let mut count = 0;
        loop {
            count += 1;
            num_buffer = step(num_buffer);
            if num_buffer == 4 {
                solve_buffer = 4;
            }
            if solve_buffer == 4 && num_buffer == 2 {
                solve_buffer = 2;
            }
            if solve_buffer == 2 && num_buffer == 1 {
                solve_buffer = 1;
            }
            if solve_buffer == 1 {
                println!(
                    "{}{}",
                    Style::new().bold().paint(num.to_string()),
                    Red.paint(" is not it.")
                );
                break;
            }
            if count > 1_000_000 {
                {
                    unproven_list.lock().unwrap().push(num);
                }
                println!(
                    "{}{}",
                    Style::new().bold().paint(num.to_string()),
                    Green.paint(" might be it.")
                );
            }
        }
    });
    println!("Numbers provided are not it.");
    let unproven_list = unproven_list.lock().unwrap();
    if !unproven_list.is_empty() {
        println!("Unproven numbers: {:?}", unproven_list);
    }
    Ok(())
}
