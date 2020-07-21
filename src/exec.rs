use crate::config::Config;
use crate::io;
use std::error::Error;
use std::io::BufReader;

pub fn exec(conf: &Config) -> Result<(), Box<dyn Error>> {
    match &conf.source {
        Some(file_name) => {
            let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(&file_name)?;
            io::read_line_from_csv(&conf, &mut reader)?;
        }
        None => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(BufReader::new(std::io::stdin()));
            io::read_line_from_csv(&conf, &mut reader)?;
        }
    };
    Ok(())
}
