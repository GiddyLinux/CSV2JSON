// for creating json objects
use serde_json::{Map, Value};
// for working with files
use std::fs::File;
// for handling errors
use std::io::ErrorKind;
// for arguments
use std::env;

fn main() {


    // creates a new String vector that is made up of arguments provided
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() >= 1 {


        error("No arguments supplied", "");
        std::process::exit(0)

    }
    // binds the variable json_type to the first argument in arguments
    let json_type = &arguments[1];
    // binds the variable filename to the second argument in arguments
    let filename = &arguments[2];

    // sets the json_response variable to the output of convert_csv
    // it takes the filename, type of output (2d or key) and if headers exist
    let json_response = convert_csv(filename, json_type, false);
    // prints the json as string
    println!(
        "{}",
        String::from_utf8_lossy(&json_response)
            .trim()
            // removes all escaped characters
            .replace("\\\"", "")
            .replace("\\n", "\n")
            .replace("\\t", "\t")
    );
}

// initiates new function called convert_csv which takes filename  (&str), json_type (&str) and headers_exist (bool) as arguments
fn convert_csv(filename: &str, json_type: &str, headers_exist: bool) -> Vec<u8> {
    if !json_type.contains("key") && !json_type.contains("2d") {
        error("Invalid Json Type", json_type);
        std::process::exit(0)
    }
    // opens the file filename and does not unwrap any errors so it can be handled by the bellow catch function
    let open_file = File::open(filename);
    //
    let file = match open_file {
        // if there are no errors it just returns the value
        Ok(file) => file,
        // branch for errors
        Err(e) => {
            match e.kind() {
                // runs the error() function if the error if ErrorKind::NotFound
                ErrorKind::NotFound => {
                    error("File Not Found", filename);
                    // exits the program
                    std::process::exit(0)
                }
                // runs the function error() for all other errors (wildcard)
                _ => {
                    error("Error", &e.to_string());
                    std::process::exit(0);
                }
            }
        }
    };
    // creates a new ReaderBuilder used for reading csv files

    let mut reader = csv::ReaderBuilder::new()
        // sets headers depending on the value of headers_exist
        .has_headers(headers_exist)
        // sets filepath using the filename provided
        .from_reader(file);
    // initiates a new mutable vector
    let mut list_of_records = vec![];

    // checks if json_type equals 2d
    if json_type == "2d" {
        // iterates over each record and flattens it out the nested vector
        for record in reader.records().flatten() {
            // iterates through each value in the row and maps them into a String vector
            let row = record
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            // Adds [ and ] to the start and end of the vector respsectively | also put commas between them
            list_of_records.push(format!("[{}]", row.join(", ")));
        }
    // checks if json_type equals key
    } else if json_type == "key" {
        // gets the file headers
        let headers = reader.headers().unwrap().clone();
        // iterates through each value in the row and maps them into a String vector
        for result in reader.records().flatten() {
            // creates a new hashmap object
            let mut object = Map::new();
            // iterates over each header storing the index (i) and the value (header)
            for (i, header) in headers.iter().enumerate() {
                // inserts the header and its index number into the hashmap
                object.insert(header.to_owned(), Value::String(result[i].to_owned()));
            }
            // appends the object hashmap to the list_of_records vector as a string
            list_of_records.push(serde_json::to_string(&object).unwrap());
        }
    }
    // initiates new immutable variable
    let record_str = list_of_records
        // iterates throught each value in list_of_records
        .iter()
        // converts each value to a string
        .map(|value| value.to_string())
        // collects these values into a String vector
        .collect::<Vec<String>>()
        // adds a newline for easier readability
        .join("\n")
        .into();
    // returns the value record_str
    return record_str;
}
// initiates new function called error which takes error (&str) and error_value (&str)
fn error(error: &str, error_value: &str) {
    // prints information on the error as well as program usage
    println!("{}: {} \n\nUsage: <json_type> <filename>\n\nJson Types: \n    Key:\n        Turns headers into json keys\n        Places inside JSON object \n    Normal:\n        Does not implement headers\n        Places records in 2D list\n", error,error_value);
}
