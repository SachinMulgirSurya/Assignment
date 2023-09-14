#![allow(unused)]
#![allow(dead_code)]

use crate::*;
use std::fs::OpenOptions;
use protobuf::Message;
use std::io::Read;
use std::io::Write;
use Persons::Person;


include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));


//Program to convert text to byte file:
pub fn text_to_byte(cmdpath:&CmdPath) {

    //Opening file reading the cmd path:
    let mut i_file = OpenOptions::new().read(true).open(&cmdpath.i).expect("Error opening input file");
    let mut o_file = OpenOptions::new().append(true).open(&cmdpath.o.clone).expect("Error opening output file");

    //Reading input file to String: [more readable]
    let mut data = String::new();

    input_file.read_to_string(&mut data)
    .expect("reading the input to data is not possible.");

    //iterate over data rows and store values:
    for line in data.lines() {
        
        let mut values = line.split(',');
        let last_name = values.next().expect("Error getting lastname");
        let first_name = values.next().expect("Error getting firstname");
        let dob = values.next().expect("Error getting D.O.B");

        //Person Object to store the attributes:
        let mut output = Person::new();
        output.last_name = lastname.to_string();
        output.first_name = firstname.to_string();
        output.dob = dob.to_string();


        //changing the data into byte_data:
        let byte_data: Vec<u8> = output.write_to_bytes().unwrap_or(Vec::new());

        if byte_data.len() == 0 {                                             // if row is empty
            continue;
        }
        let row_len = byte_data.len() as u64;                           // if row contains data
        let mut data = String::new();

        //creating file and vector to store data:
        let mut record_data = format!(""); 
        let mut data_buffer = Vec::new();

        //Writing byte into data_buffer using protobuf:
        protobuf::CodedOutputStream::vec(&mut data_buffer)
            .write_raw_varint64(row_len)
            .expect("Write-variant64 :writing error in protobuf");
        output_file.write(&data_buffer).expect("file writing in protobuf failed");


        //Iterate over the byte_data to store values in record_data:
        for byte in &byte_data {
            record_data.push_str(&format!(" {}", byte));
        }
        record_data.push_str(&format!("\n"));        //line break

        data.push_str(&record_data);
        output_file.write(data.as_bytes()).expect("Writing in output file failed");
    }
}


