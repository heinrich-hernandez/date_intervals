use chrono::{NaiveDate};
use chrono::{Datelike};
use std::io;


struct Dates {
    day: u32,
    month: u32,
    year: u32,
}

impl Dates{
    fn numbers_to_months(&self, x : usize) -> &str {
        let months_to_str = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
        months_to_str[x]
    }
}


fn main() {
    let current_date = chrono::Local::now().date_naive();
    let today = Dates {
        day : current_date.day() as u32,
        month : current_date.month() as u32,
        year : current_date.year() as u32,
    };

    println!("The date today is {}/{}/{}.\n", today.day,today.numbers_to_months((today.month - 1).try_into().unwrap()),today.year);

    println!("Convert your age relative to Days, Weeks, Months, and Years.\nPlease Enter the Following:");

    loop{
        println!("Day you were Born :");
        let mut day_of_birth = String::new();
        io::stdin()
            .read_line(&mut day_of_birth)
            .expect("Failed to read line");
        
        if day_of_birth.trim() == "quit" || day_of_birth.trim() == "stop" {break;}

        let day_of_birth: u32 = match day_of_birth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        

        println!("Month you were Born :");
        let mut month_of_birth = String::new();
        io::stdin()
            .read_line(&mut month_of_birth)
            .expect("Failed to read line");
        
        //simplify the if conditions into a function---can also transpose into if conditions into match conditions instead
        if month_of_birth.trim() == "quit" || month_of_birth.trim() == "stop" {break;}

        let month_of_birth: u32 = match month_of_birth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Year you were Born :");
        let mut year_of_birth = String::new();
        io::stdin()
            .read_line(&mut year_of_birth)
            .expect("Failed to read line");

        if year_of_birth.trim() == "quit" || year_of_birth.trim() == "stop" {break;}
        
        let year_of_birth: u32 = match year_of_birth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let input_birthday = Dates{
            day : day_of_birth, 
            month : month_of_birth,
            year : year_of_birth,
        };

        let birth_date = format!("{}-{}-{}", input_birthday.year,input_birthday.month,input_birthday.day);
        let birthday = NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").unwrap();
        let age = current_date.signed_duration_since(birthday).num_days();

        println!("\nIs this your birthday? y/n\n{}/{}/{}",input_birthday.day,input_birthday.numbers_to_months((input_birthday.month -1).try_into().unwrap()),input_birthday.year);
        
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim() == "y" || answer.trim() == "" {
            let age_in_years = {
                age/365
                };
            
            let age_in_months = {
                age/30
            };

            //calculate the remaining weeks in the difference between day of birth and day currently into weeks
            let age_in_weeks = {
                age/7
            };

            //calculate the difference difference between day of birth and day currently and add unto current montha instead of weeks
            let age_in_days = {
                age
            };

            println!("\nYour age in :\nDays : {}\nWeeks : {}\nMonths : {}\nYears : {}\n\n", age_in_days,age_in_weeks,age_in_months,age_in_years);


        }else{
            continue;
        }
    };
}

//added new logic for age_in_days and age_in_weeks

/*
Problem:
    Isn't accurate enough if the birthday input is over the the day currently
Goal:
    To make it more accurate and using chrono to its limits or creating my own logic towards it
    problem logic in the future:
        =>if I can use chrono fully = create better project
        =>if not -> make it so that it is accurate to the second floating point digit


    Whilst studying chrono:
        I realized there are limitation towards the crate, such limitation is my understanding of the crate and of how I don't really much undertand what I'm trying to take away from it.

        =>Extract the age of input compared to the date now by:
            ->year
            ->month
            ->week
            ->day
            ->hrs
            ->min
            ->sec
    
        The problem with my own understanding is that it is very limitted at the moment. Although reading the documentation is a good idea. I don't think I would percieve it as much as I'd like to. I have to struggle understanding it and fully ingesting how it works. Although I see it as a must as I develop into rust further as it adheres to ISO-8601.
*/