#![allow(unused)]
#![allow(dead_code)]

use crate::*;
use std::fs::OpenOptions;
use protobuf::CodedInputStream;
use protobuf::Message;
use std::io::Read;
use std::io::Write;
use Persons::Person;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

//Program to convert byte to text file:
pub fn byte_to_text(cmd_path: &CmdPath) {

    //read input file:
    let mut i_file = OpenOptions::new()
        .read(true)
        .open(&cmd_path.i)
        .expect("Cant open input file");

    //make output file to store the output:
    let mut o_file = OpenOptions::new()
        .append(true)
        .open(&cmd_path.o)
        .expect("cannot open output file");

    //reading input file:
    let mut data = String::new();
    i_file
        .read_to_string(&mut data)
        .expect("Data can't be read");


    //store and iterate over the data to get the rows individually:
    let mut content = String::new();
    for line in data.lines() {

        //split lines by spacing:
        let mut values = line.split(' ');
        let mut input = CodedInputStream::from_bytes(&mut values
            .next()
            .unwrap_or("")
            .as_bytes());


        let length = input
            .read_raw_varint64()
            .expect("raw_variant64 length error");

        let mut bytes: Vec<u8> = Vec::new();
        for _i in 1..=length {
            let temp = values.next().expect("No byte data present");
            bytes.push(temp.parse::<u8>().unwrap_or(0u8));
        }

        //Person object to store data in 
        //format -> [lastname, firstname, dob]
        let input_message = Person::parse_from_bytes(&bytes).unwrap_or(Person::new());
        content.push_str(input_message.last_name.as_str());
        content.push_str(",");
        content.push_str(input_message.first_name.as_str());
        content.push_str(",");
        content.push_str(input_message.dob.as_str());
        content.push_str("\n");
    }

    //Writing output for 2nd program:
    o_file
        .write(content.as_bytes())
        .expect("writing byte into data failed");

}