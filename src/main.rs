use chrono::Datelike;
use chrono;
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

        println!("\nIs this your birthday? y/n\n{}/{}/{}",input_birthday.day,input_birthday.numbers_to_months((input_birthday.month -1).try_into().unwrap()),input_birthday.year);
        
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim() == "y" || answer.trim() == "" {
            let age_in_years = {
                let current_age_in_years = today.year - input_birthday.year;
                if input_birthday.month > today.month || input_birthday.month == today.month && input_birthday.day < today.day {current_age_in_years -1}
                else{current_age_in_years}
                };
            
            let age_in_months = {
                let current_age_in_months = age_in_years * 12;
                if input_birthday.month > today.month {current_age_in_months+(input_birthday.month - today.month)}
                else{current_age_in_months}
            };

            //calculate the remaining weeks in the difference between day of birth and day currently into weeks
            let age_in_weeks = {
                let current_age_in_weeks : f32 = age_in_months as f32 * 4.34524;
                current_age_in_weeks
            };

            //calculate the difference difference between day of birth and day currently and add unto current montha instead of weeks
            let age_in_days = {
                let current_age_in_days : f32 = age_in_weeks * 7.0;
                current_age_in_days
            };

            println!("\nYour age in :\nDays : {}\nWeeks : {}\nMonths : {}\nYears : {}\n\n", age_in_days,age_in_weeks,age_in_months,age_in_years);


        }else{
            continue;
        }
    };
}


