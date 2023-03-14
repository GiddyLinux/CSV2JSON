# csv2_json


## New Additions

### **Things not implemented in the [last](https://github.com/GiddyLinux/countdown-timer) mini project**
    - User supplied arguments
    - File library
    - Working with CSV and JSON files
    - Custom Error Function
    



## Features
    - Error handling 
        - NotFound Error
    - Fully commented for clarity
    - Sorted into clear functions

## Installation

### **Use the binary provided in releases**

### Self Compilation
This section is for people who want to compile the project themselves

 Requires [Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/) to compile

```sh
git clone https://github.com/GiddyLinux/csv2_json.git
cd csv2_json
cargo build --release --target <platform of choice>
```

## Usage

Below is an example demonstration of the program using the key option
```sh
$ ./target/csv2_json-x86_64-unknown-linux-gnu.run key src/key_test.csv
{" Score":" Score"," Title":" Title","Year":"Year"}
{" Score":"  86"," Title":" Greetings","Year":"1968"}
{" Score":"  17"," Title":" Bloody Mama","Year":"1970"}
{" Score":"  40"," Title":" Born to Win","Year":"1971"}
```
And again using 2D lists 
```sh
$ ./target/csv2_json-x86_64-unknown-linux-gnu.run 2d src/key_test.csv
[Year,  "Score",  "Title"]
[1968,   86,  "Greetings"]
[1970,   17,  "Bloody Mama"]
[1971,   40,  "Born to Win"]
```
