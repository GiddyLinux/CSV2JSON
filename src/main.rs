// uses the Map and Value functions from the serde_json crate
use serde_json::{Map, Value};

fn main() {
    // sets the json_response variable to the output of convert_csv
    // it takes the filename, type of output (normal or key) and if headers exist
    let json_response = convert_csv("deniro.csv", "normal", false);
    // prints the json as string
    println!("{}", String::from_utf8_lossy(&json_response));
}

// initiates new function called convert_csv which takes filename  (&str), json_type (&str) and headers_exist (bool) and arguments
fn convert_csv(filename: &str,json_type: &str, headers_exist: bool) ->  Vec<u8> {
    // creates a new ReaderBuilder used for reading csv files
    let mut file = csv::ReaderBuilder::new()
        // sets headers depending on the value of headers_exist
        .has_headers(headers_exist)
        // sets filepath using the filename provided
        .from_path(filename)
        .unwrap();
    // initiates a new mutable vector
    let mut list_of_records = vec![];

    // checks if json_type equals normal
    if json_type == "normal" {
        // iterates over each record and flattens it out the nested vector
        for record in file.records().flatten() {
            // iterates through each value in the row and maps them into a String vector
            let row = record.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            // Adds [ and ] to the start and end of the vector respsectively | also put commas between them 
            list_of_records.push(format!("[{}]", row.join(", ")));
                
        }
    // checks if json_type equals key
    } else if json_type == "key" {
        // gets the file headers
        let headers = file.headers().unwrap().clone();
        // iterates through each value in the row and maps them into a String vector
        for result in file.records().flatten() {
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
        .join("\n").into();   
    // returns the value record_str
    return record_str
}

    





