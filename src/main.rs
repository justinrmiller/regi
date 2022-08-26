extern crate serde;
use parquet::record::RowAccessor;
use serde::{Deserialize, Serialize};

use std::usize;
use std::collections::HashMap;
use std::{fs::File};
use std::io::Read;
use std::path::Path;
use parquet::file::reader::{FileReader, SerializedFileReader};
// use parquet::record::RowAccessor;
use regex::Regex;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct Rule {
    name: String,
    regex: String,
}

struct CompiledRule {
    name: String,
    regex: Regex,
}

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    columns: Vec<String>,
    rules: Vec<Rule>,
}

fn main() {
    let mut file = File::open("config.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let config: Config = serde_json::from_str(&buff).unwrap();

    println!("Processing Ruleset...");
    println!("=====================");
    println!("Name of Rule Set: {}", config.name);
    println!("Columns examined in Rule Set: {:?}", config.columns);

    for rule in &config.rules {
        println!("Rule Name: {} - Rule Regex: {}", rule.name, rule.regex);
    }
    println!("Number of rules: {}", config.rules.len());

    println!("=====================");

    let parquet_file = File::open(&Path::new("./yelp_academic_dataset_review.parquet")).unwrap();
    let reader = SerializedFileReader::new(parquet_file).unwrap();
    
    // extract schema and row counts
    let parquet_metadata = reader.metadata();
    let fields = parquet_metadata.file_metadata().schema().get_fields();
    let row_count = parquet_metadata.file_metadata().num_rows();

    println!("Total number of rows: {}", row_count);

    println!("=====================");

    for config_column in config.columns {
        let column_exists = fields.iter().find(|column| column.name() == config_column);
        match column_exists {
            Some(column) => {
                println!("Column exists in parquet file: {}, schema: {:?}", config_column, column);
            },
            None => {
                println!("Column does not exist in parquet file: {}", config_column);
                println!("Shutting Down...");
                return;
            },
        };
    }
    
    println!("=====================");
    println!("Scanning...");
    println!("=====================");

    let mut compiled_regexes: Vec<CompiledRule> = Vec::new();
    for rule in config.rules {
        compiled_regexes.push(
            CompiledRule {
                name: rule.name.clone(), 
                regex: Regex::new(&rule.regex).unwrap()
            }
        )
    ;}

    let mut count = 0;

    let mut regex_counts = HashMap::new();

    let mut start = Instant::now();

    let mut iter = reader.get_row_iter(None).unwrap();

    while let Some(row) = iter.next() {
        let text = row.get_string(7).unwrap();
        let column_name = "text";
        for regex in &compiled_regexes {
            if regex.regex.is_match(text) {
                let key = column_name.to_string() + "|" + &regex.name;
                if regex_counts.contains_key(&key) {
                    let current_count = regex_counts.get(&key).unwrap();
                    regex_counts.insert(key, current_count + 1); 
                } else {
                    regex_counts.insert(key, 1); 
                }
            }
            // println!("{}", regex.name);
        }
        if count != 0 && count % 10000 == 0 {
            let duration = start.elapsed();
            println!("{:?}", regex_counts);
            println!("^-- at count: {}", count);
            println!("Time taken for batch: {:?}", duration);
            start = Instant::now();
        }
        count += 1;
    }
}