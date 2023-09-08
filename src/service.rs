#![allow(unused)]
#![allow(dead_code)]

use crate::*;
use calamine::{DataType, Excel, Range};
use chrono::prelude::*;
use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Employee {
    emp_id: i32,
    emp_name: String,
    dept_id: i32,
    mobile_no: String,
    email: String,
}

#[derive(Debug)]
pub struct Department {
    dept_id: i32,
    dept_title: String,
    dept_strength: i32,
}

#[derive(Debug)]
pub struct Salary {
    emp_id: i32,
    salary_id: i32,
    salary_date: String,
    salary: f64,
    salary_status: String,
}

#[derive(Debug)]
pub struct Leave {
    emp_id: i32,
    leave_id: i32,
    leave_from: f64,
    leave_to: f64,
    leave_type: String,
    leave_count: i32,
}

#[derive(Debug)]
struct Output {
    emp_id: i32,
    emp_name: String,
    dept_title: String,
    mobile_no: String,
    email: String,
    salary_status: String,
    on_leave: i32,
}

//Function to update emp_map:
pub fn update_empmap(cmd_path: String, map: &mut HashMap<i32, Employee>) -> Result<(), io::Error> {

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(cmd_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    //The flag is to skip the header row of the table.
    let mut flag = true;
    for line in content.lines() {
        if flag {
            flag = false;
            continue;
        }

        // split and bind data
        let mut data = line.split('|');

        let emp_id = data.next().expect("No emp id").parse::<i32>().unwrap();
        let emp_name_data = data.next().expect("No emp name").to_string();
        let dept_id = data.next().expect("No emp name").parse::<i32>().unwrap();
        let mobile_no = data.next().expect("No emp name").to_string();
        let email = data.next().expect("No emp name").to_string();

        //Adding data to Emp_hashmap:

        let emp = Employee {
            emp_id: emp_id,
            emp_name: emp_name_data,
            dept_id: dept_id,
            mobile_no: mobile_no,
            email: email,
        };

        map.insert(emp_id, emp);
    }
    Ok(())
}

//Function to update the data of department_map:
pub fn update_deptmap(cmd_path: String,map: &mut HashMap<i32, Department>,) -> Result<(), io::Error> {

    let mut excel = Excel::open(cmd_path).unwrap();
    let table = excel.worksheet_range("Dept").unwrap();

    let mut flag = true;
    for row in table.rows() {
        if flag {
            flag = false;
            continue;
        }

        let d_id = match row[0] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };
        let d_title = match &row[1] {
            DataType::String(v) => v,
            _ => "",
        };
        let d_strength = match row[2] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };
        let dept = Department {
            dept_id: d_id,
            dept_title: d_title.to_string(),
            dept_strength: d_strength,
        };

        //Adding data to the department_map;
        map.insert(d_id, dept);
    }
    Ok(())
}

//Function to update the data of salary map:
pub fn update_salarymap(cmd_path: String, map: &mut HashMap<i32, Salary>) -> Result<(), io::Error> {
    let mut excel = Excel::open(cmd_path).unwrap();
    let table = excel.worksheet_range("Sheet1").unwrap();

    let mut flag = true;
    for row in table.rows() {
        if flag {
            flag = false;
            continue;
        }

        let e_id = match row[0] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };

        let sal_id = match row[1] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };

        let sal_date = match &row[2] {
            DataType::String(v) => v,
            _ => "",
        };

        let sal = match row[3] {
            DataType::Float(v) => v,
            _ => 0_f64,
        };

        let sal_status = match &row[4] {
            DataType::String(v) => v,
            _ => "Not Credited",
        };
        
        let salary_struct = Salary {
            emp_id: e_id,
            salary_id: sal_id,
            salary_date: sal_date.to_string(),
            salary: sal,
            salary_status: sal_status.to_string(),
        };

        //given month & year:
        let mut date = sal_date.split(" ");
        let given_month = date.next().unwrap();
        let given_year = date.next().unwrap().parse::<i32>().unwrap();

        //current month and year
        let current_date = chrono::Utc::now();
        let curr_year = current_date.year();
        let curr_month = current_date.month();

        //Adding to salary_map:
        //Add only if the date is of current month and current year:
        //if(curr_year == given_year) && (curr_month == getmon(given_month)){
            map.insert(e_id, salary_struct);
        //}
    }
    Ok(())
}

//funciton to updating leave_map:
pub fn update_leavemap(cmd_path: String, map: &mut HashMap<i32, Leave>) -> Result<(), io::Error> {
    let mut excel = Excel::open(cmd_path).unwrap();
    let table = excel.worksheet_range("Sheet1").unwrap();

    let mut flag = true;
    for row in table.rows() {
        let emp_id = match row[0] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };

        let leave_id = match row[1] {
            DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
            _ => 0,
        };

        let leave_from = match row[2] {
            DataType::Float(v) => v,
            _ => 0_f64,
        };

        let leave_to = match row[3] {
            DataType::Float(v) => v,
            _ => 0_f64,
        };

        let leave_type = match &row[4] {
            DataType::String(v) => v,
            _ => "",
        };

        //Calculating the no. of days of leave:
        let mut leave_count = 0;
        let leave_till_month = get_date_from_float(leave_to).month();
        let leave_day = get_date_from_float(leave_to).day();

        let leave_from_month = get_date_from_float(leave_from).month();
        let curr_month = Utc::now().month();

        if curr_month == leave_till_month {
            if curr_month == leave_from_month {
                leave_count = (leave_to - leave_from) as i32;
            } else {
                leave_count = leave_day as i32;
            }
        }


        //Creating leave struct to add to map:
        let leave = Leave {
            emp_id: emp_id,
            leave_id: leave_id,
            leave_from: leave_from,
            leave_to: leave_to,
            leave_type: leave_type.to_string(),
            leave_count: leave_count,
        };

        //Updating data into leave map:
        map.insert(emp_id, leave);
    }
    Ok(())
}


//Funtion to generate output.txt:
pub fn generate_output(emp_map: &mut HashMap<i32, Employee>,dept_map: &mut HashMap<i32, Department>,salary_map: &mut HashMap<i32, Salary>,leave_map: &mut HashMap<i32, Leave>) {
    
    //Adding header to output.txt:
    let mut content = String::new();
    let heading: String = format!(
        "{}#{}#{}#{}#{}#{}#{}\n",
        "Emp_Id", "Emp_name", "Dept_Title", "Mobile_No", "Email", "Salary_status", "On_Leave"
    );

    content.push_str(&heading);


    //Iterate over emp_map to get all employees:
    //Taking related values and inserting it to output structs and adding the data to the output.txt.
    for (key, value) in emp_map.iter() {
        //e_id:
        let mut e_id = *key;

        //Emp_name:
        let e_name = emp_map.get(&e_id).expect("no ename found").emp_name.clone();

        //Dept_Title:
        let d_id = emp_map
            .get(&e_id)
            .expect("no dept id found")
            .dept_id
            .clone();
        let d_title = dept_map
            .get(&d_id)
            .expect("No dept title found")
            .dept_title
            .clone();

        //Mobile:
        let mobile = emp_map
            .get(&e_id)
            .expect("no mobile no found")
            .mobile_no
            .clone();

        //Email;
        let e_mail = emp_map.get(&e_id).expect("no email found").email.clone();

        //Salary Status:
        let sal_status2=match salary_map.get(&e_id){
            Some(val)=>&val.salary_status,
            _=>"",
        };

        //On leave:
        let l_count = leave_map
            .get(&e_id)
            .expect("no leave count found")
            .leave_count
            .clone();


        //Output struct:
        let output = Output {
            emp_id: e_id,
            emp_name: e_name,
            dept_title: d_title,
            mobile_no: mobile,
            email: e_mail,
            salary_status: sal_status2.to_string(),
            on_leave: l_count,
        };

        //Writing into output.txt:
        let file_data = format!(
            "{:?}#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n",
            output.emp_id,
            output.emp_name,
            output.dept_title,
            output.mobile_no,
            output.email,
            output.salary_status,
            output.on_leave
        );

        content.push_str(&file_data);
    }
    //Finally moving all data to output.txt:
    std::fs::write("Data/output.txt", content);
}

fn get_date_from_float(num: f64) -> NaiveDate {
    let start = NaiveDate::from_ymd_opt(1900, 1, 1).expect("DATE");
    let v = start.checked_add_signed(Duration::days(
        (num - 2.0).to_string().parse::<i64>().unwrap(),
    ));
    v.unwrap()
}

fn getmon(s:&str)->u32{
    if s=="Jan"{1}
    else if s=="Feb"{2}
    else if s=="Mar"{3}
    else if s=="Apr"{4}
    else if s=="May"{5}
    else if s=="Jun"{6}
    else if s=="Jul"{7}
    else if s=="Aug"{8}
    else if s=="Sep"{9}
    else if s=="Oct"{10}
    else if s=="Nov"{11}
    else if s=="Dec"{12}
    else {0}
}