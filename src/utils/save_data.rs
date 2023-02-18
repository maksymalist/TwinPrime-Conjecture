use std::fs;
use std::io::{Write, Result};


pub fn save_data(max_steps: Vec<i32>, max_step_gap: Vec<i32>) -> Result<()> {
    println!("saving data...");

    
    // write max_steps

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("./data/max_steps.txt").unwrap();

    
    file.set_len(0).unwrap();

    for step in max_steps {
        file.write_all(format!("{}\n", step).as_bytes()).unwrap();
    }

    // write max_step_gap to file

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("./data/max_step_gap.txt").unwrap();


    file.set_len(0).unwrap();

    for step in max_step_gap {
        file.write_all(format!("{}\n", step).as_bytes()).unwrap();
    }

    Ok(())
}