use std::fs;
use std::io::Result;

pub enum DataType {
    TwinPrimeGaps,
    TwinPrimes,
}

pub fn save_data(data_type: DataType, data: Vec<i32>) -> Result<()> {

    match data_type {
        DataType::TwinPrimeGaps => {
            //
        },
        DataType::TwinPrimes => {
            //
        }
        _ => {
            println!("Error: data type not found");
        }
    }

    Ok(())
}