use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use clap::{Arg, App};
use std::process;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct Record {
    name: String,
    value: f64,
}


fn main() {
    let matches = App::new("CSV Processor")
        .version("1.0")
        .author("Your Name")
        .about("Processes a CSV file")
        .arg(Arg::with_name("input")
             .help("Sets the input CSV file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("sort")
             .long("sort")
             .takes_value(false)
             .help("Sorts the records by the value field"))
        .arg(Arg::with_name("min")
             .long("min")
             .takes_value(false)
             .help("Returns the record with the minimum value"))
        .arg(Arg::with_name("max")
             .long("max")
             .takes_value(false)
             .help("Returns the record with the maximum value"))
        .arg(Arg::with_name("average")
             .long("average")
             .takes_value(false)
             .help("Returns the average of the value field"))
        .get_matches();

    let filepath = matches.value_of("input").unwrap();

    if let Err(err) = run(&filepath, &matches) {
        println!("Error: {}", err);
        process::exit(1);
    }
}

fn run(filepath: &str, matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(filepath)?;

    let records: Vec<Record> = rdr.deserialize().filter_map(Result::ok).collect();

    if matches.is_present("sort") {
        let mut sorted_records = records.clone();
        sorted_records.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal));
        println!("Sorted Records:");
        for record in &sorted_records {
            println!("{:?}", record);
        }
    }

    if matches.is_present("min") {
        if let Some(min_record) = records.iter().min_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal)) {
            println!("Record with the Minimum Value: {:?}", min_record);
        }
    }

    if matches.is_present("max") {
        if let Some(max_record) = records.iter().max_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal)) {
            println!("Record with the Maximum Value: {:?}", max_record);
        }
    }

    if matches.is_present("average") {
        let sum: f64 = records.iter().map(|rec| rec.value).sum();
        let avg = sum / records.len() as f64;
        println!("Average Value: {}", avg);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_records() {
        let mut records = vec![
            Record { name: "Banana".to_string(), value: 2.0 },
            Record { name: "Apple".to_string(), value: 3.0 },
        ];
        records.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        assert_eq!(records[0].name, "Banana");
        assert_eq!(records[1].name, "Apple");
    }

    #[test]
    fn test_average_value() {
        let records = vec![
            Record { name: "Cherry".to_string(), value: 5.0 },
            Record { name: "Date".to_string(), value: 15.0 },
        ];
        let total: f64 = records.iter().map(|r| r.value).sum();
        let average = total / records.len() as f64;
        assert_eq!(average, 10.0);
    }

    #[test]
    fn test_min_value() {
        let records = vec![
            Record { name: "Fig".to_string(), value: 7.5 },
            Record { name: "Elderberry".to_string(), value: 2.5 },
        ];
        let min_record = records.iter().min_by(|a, b| a.value.partial_cmp(&b.value).unwrap()).unwrap();
        assert_eq!(min_record.value, 2.5);
    }

    #[test]
    fn test_max_value() {
        let records = vec![
            Record { name: "Grape".to_string(), value: 1.0 },
            Record { name: "Apple".to_string(), value: 3.0 },
        ];
        let max_record = records.iter().max_by(|a, b| a.value.partial_cmp(&b.value).unwrap()).unwrap();
        assert_eq!(max_record.value, 3.0);
    }
}