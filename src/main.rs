#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;
use clap::Parser;
 
#[derive(Parser, Debug)]
struct CmdPath {
    #[clap(long,short)]
    e: String,

    #[clap(long,short)]
    d: String,

    #[clap(long,short)]
    s: String,

    #[clap(long,short)]
    l: String,

    #[clap(long,short)]
    o: String,
}

mod service;
use service::*;

fn main() {

    //Catching path from the cmd line using clap:
    let cmd_path = CmdPath::parse();

    //HashMaps for maintaining data:
    let mut emp_map: HashMap<i32, Employee> = HashMap::new();
    let mut dept_map: HashMap<i32, Department> = HashMap::new();
    let mut salary_map: HashMap<i32, Salary> = HashMap::new();
    let mut leave_map: HashMap<i32, Leave> = HashMap::new();


    //Updating emp_map:
    update_empmap(cmd_path.e, &mut emp_map).expect("Error updating emp_map");
    //let emp_id = emp_map.get(&cmd_path.id.to_string()).unwrap().emp_id;

    //Updating dept_map;
    //let dept_id = emp_map.get(&cmd_path.id.to_string()).unwrap().dept_id;
    update_deptmap(cmd_path.d, &mut dept_map).expect("Error updating dept_map");
    
    //Updating salary data:
    update_salarymap(cmd_path.s, &mut salary_map).expect("Error updating salary_map");

    //Updating leave data:
    update_leavemap(cmd_path.l, &mut leave_map).expect("Error updating leave_map");

    
    //Generating output out of all the maps:
    generate_output(&mut emp_map, &mut dept_map, & mut salary_map, &mut leave_map);


    println!("Project Running Successfully!!");

}


//cmd : cargo run -- --e="Data/Employee_data.txt" --d="Data/Dept_data.xlsx" --s="Data/Salary_data.xlsx" --l="Data/Leave_data.xlsx" --o="Data/output.txt"