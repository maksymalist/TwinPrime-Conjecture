use std::fs;
use std::io::{Write, Result};
use std::time::{SystemTime, UNIX_EPOCH};

fn save_to_log(log: String) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(format!("./data/log/{:?}.txt", now)).unwrap();

    file.set_len(0).unwrap();

    file.write_all(log.as_bytes()).unwrap();

    Ok(())
}

pub fn save_data(max_steps: Vec<i32>, max_step_gap: Vec<i32>) -> Result<()> {
    println!("saving data...");
    // write max_steps

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("./data/max_steps.txt").unwrap();

    
    file.set_len(0).unwrap();

    for step in &max_steps {
        file.write_all(format!("{}\n", *step).as_bytes()).unwrap();
    }

    // write max_step_gap to file

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("./data/max_step_gap.txt").unwrap();


    file.set_len(0).unwrap();

    for step in &max_step_gap {
        file.write_all(format!("{}\n", *step).as_bytes()).unwrap();
    }

    // write log

    let formated_max_steps = max_steps.into_iter().map(|x| -> String { format!("{}, ", x) }).collect::<String>();
    let formated_max_step_gap = max_step_gap.into_iter().map(|x| -> String { format!("{}, ", x) }).collect::<String>();

    let log = format!("/// MAX STEPS ///:\n {:?}, \n///MAX STEP GAP ///:\n {:?}", formated_max_steps, formated_max_step_gap);

    save_to_log(log).expect("failed to save log :(");

    Ok(())
}