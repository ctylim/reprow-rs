use crate::config::Config;
use csv::StringRecord;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};

pub fn read_line_from_csv<R: std::io::Read>(conf: &Config, reader: &mut csv::Reader<R>) -> Result<(), Box<dyn Error>> {
    let io_writer: Box<dyn Write> = match &conf.destination {
        Some(file_name) => Box::new(File::create(file_name)?),
        None => Box::new(stdout()),
    };

    let mut writer: csv::Writer<Box<dyn std::io::Write>> = csv::Writer::from_writer(io_writer);
    let mut header = StringRecord::default();
    reader.read_record(&mut header)?;
    let mut header_ind: Option<usize> = None;
    for (ind, col) in header.iter().enumerate() {
        if col == conf.rep_col {
            header_ind = Some(ind);
        }
    }
    let header_ind = header_ind.ok_or("header column not found")?;
    info!("header: {:?}", header);
    info!("started writing to {:?}", &conf.destination);
    writer.write_record(&header)?;
    writer.flush()?;
    for result in reader.records() {
        let record = result?;
        trace!("{:?}", record);
        reprow(&mut writer, &record, &header_ind)?;
    }
    info!("finished writing to {:?}", &conf.destination);
    Ok(())
}

pub fn reprow(writer: &mut csv::Writer<Box<dyn std::io::Write>>, record: &StringRecord, header_ind: &usize) -> Result<(), Box<dyn Error>> {
    let rep_count: i32 = record.get(*header_ind).ok_or("record with col index not found")?.parse::<i32>()?;
    for _ in 0..rep_count {
        writer.write_record(record)?;
        writer.flush()?;
    }
    Ok(())
}
