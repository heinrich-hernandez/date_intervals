use chrono::Datelike;
use chrono; 

fn main() {
    let current_date = chrono::Utc::now().date_naive();
    let y = current_date.year();
    let x = current_date.day();
    let z : usize = current_date.month().try_into().unwrap();

    let a = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
    let za = a[z-1];
    //make a tool that would convert
    //-> years to weeks
    //-> weeks to days
    
    //-->days to weeks

    //-->add days total
    //-->add weeks total

    //purpose -> to see the age equivalent to days and weeks
    println!("The date is {x}, {za}, {y}");


    //input fields

    //day of birth
    println!("Please enter the day you were born: ");
    let mut day_born = String::new();
    io::stdin()
    .read_line(&mut day_born)
    .expect("Failed to read line");

    let day_born: i32 = match day_born.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


    //month of birth
    println!("Please enter the month you were born: ");
    let mut month_born = String::new();

    io::stdin()
    .read_line(&mut month_born)
    .expect("Failed to read line");

    let month_born: i32 = match month_born.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


    //year of birth
    println!("Please enter the year you were born: ");
    let mut year_born = String::new();

    io::stdin()
        .read_line(&mut year_born)
        .expect("Failed to read line");

    let year_born: i32 = match year_born.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


}

fn years_to_months(x: i32) {
    x * 12
}

fn months_to_weeks(x: i32) {
    x * 4.34524
}

fn weeks_to_days(x: i32) {
    x * 7
}