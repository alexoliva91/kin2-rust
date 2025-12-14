use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use terminal_menu::{menu, label, button, string, numeric, run, mut_menu};

fn main(){
    
    let amu_in_mev = 931.4940954;

    println!("Insert the file name: ");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).expect("Failed to read line");
    file_name.pop();

    let file_exists = std::path::Path::new(&file_name).exists();
    //if file exists overwrite it
    if file_exists {
        println!("File exists, should I overwrite it? (Y/n)");
    }
    

    //create file
    let mut file = File::create(file_name).unwrap();

    
    
    // open file
    //let file = File::open("src/words.txt").unwrap();
    //let reader = BufReader::new(file);

    let menu = menu(vec![
        label("strings and numerics"),

        // string:
        //  a string of characters
        //  the last arguments specifies if empty strings are allowed

        // empty strings allowed:
        string("ste", "default", true),

        // empty strings not allowed:
        string("stn", "default", false),

        // numeric:
        //  a floating point number
        numeric("num",
            // default
            4.5,

            // step
            Some(1.5),

            // minimum
            None,

            // maximum
            Some(150.0)
        ),

        button("exit")
    ]);
    run(&menu);
    {
        let mm = mut_menu(&menu);
        println!("{}", mm.selection_value("ste"));
        println!("{}", mm.selection_value("stn"));
        println!("{}", mm.numeric_value("num"));
    }



}