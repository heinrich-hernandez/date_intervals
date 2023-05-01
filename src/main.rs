use chrono::Datelike;
use chrono;
use std::io;


struct Dates {
    day: u32,
    month: u32,
    year: u32, //can't use u32 for some reason
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

        let input_bithday = Dates{
            day : day_of_birth, 
            month : month_of_birth,
            year : year_of_birth,
        };

        println!("Is this your birthday? y/n\n{}/{}/{}",input_bithday.day,input_bithday.numbers_to_months((input_bithday.month -1).try_into().unwrap()),input_bithday.year);
        
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut month_of_birth)
            .expect("Failed to read line");

        if answer.trim() == 'y'{
            let age_in_years = {
                //try to make this a function
                let current_age = today.year - input_bithday.year;
                if input_bithday.month > today.month{current_age - 1}
                else if input_birthday.month == today.month && input_birthday < today.day{current_age -1}
                else{current_age}
                }
            
            let age_in_months = {

            }

            let age_in_weeks = {

            }

            let age_in_days = {
                
            }
        }else{
            continue;
        }
    };
}


