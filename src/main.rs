use chrono::Datelike;
use chrono; 
use std::io;

fn main() {
    let current_date = chrono::Utc::now().date_naive();
    let y = current_date.year();
    let x = current_date.day();
    let z = current_date.month().try_into().unwrap();

    let a = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
    let za = a[z-1];


    //purpose -> to see the age equivalent to days and weeks
    println!("The date today is {x}, {za}, {y}");

    loop {
        let aaa : u32 = y.try_into().unwrap();
        //input fields

        //day of birth
        println!("Please enter the day you were born: ");
        let mut day_born = String::new();
        io::stdin()
        .read_line(&mut day_born)
        .expect("Failed to read line");

        let day_born: u32 = match day_born.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        //month of birth
        println!("Please enter the month you were born: ");
        let mut month_born = String::new();

        io::stdin()
        .read_line(&mut month_born)
        .expect("Failed to read line");

        let month_born: u32 = match month_born.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };



        //year of birth
        println!("Please enter the year you were born: ");
        let mut year_born = String::new();

        io::stdin()
            .read_line(&mut year_born)
            .expect("Failed to read line");

        let year_born: u32 = match year_born.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let zab : usize = month_born.try_into().unwrap();
        let bbb = a[zab - 1]; 


        //question to start the program
        println!("\nIs this your Birth Date? {day_born} {bbb} {year_born} y/n");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim() == "y"{
            
            //current year subtracted by birth year
            //problem - can't get the proper output 
            let bca = {
                let aaaa = aaa - year_born;
                if zab > z {aaaa - 1}
                else if zab == z && x < day_born {aaaa - 1}
                else{aaaa}
            };
            //months existing in this world
            let aa = { 
                years_to_months(bca) + month_born
            };

            //weeks existing in this world
            let ba =  {
                months_to_weeks(aa as f32)
            };

            //days existing in this world
            let ca = {
                weeks_to_days(ba) + day_born as f32
            };

            println!("\nYou've been existing in this world by:  \n{ca} Days. \n{ba} Weeks. \n{aa} Months. \n{bca} Years.");
        }


        //question to end the program
        println!("\nDo you want to continue: y/n");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim() == "n"{
            println!("Process Completed.\nThank you!");
            break;
        }else{println!("\n");}
    };
 
}

fn years_to_months(x: u32) -> u32 {
    x * 12
}

fn months_to_weeks(x: f32) -> f32 {
    (x * 4.34524) as f32
}

fn weeks_to_days(x: f32) -> f32 {
    (x * 7.0) as f32
}

//FOOTER
//completed task
    //make a tool that would convert
    //-> years to weeks
    //-> weeks to days
    

    //-->add days total
    //-->add weeks total
    //-->months total