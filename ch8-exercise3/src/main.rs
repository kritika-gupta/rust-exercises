mod commands;
use crate::commands::Command;

mod company;
use crate::company::Company;

use std::io;

fn main() {
    println!("\nCommands:\n-Add <person> to <department>\n-List <department>\n-List\n-Exit\n\nAll commands are case-insensitive.\n");

    let mut company = Company::new();
    loop {

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match Command::create(&command) {
            Some(Command::Add(dept, person)) => company.add_employee(&dept, &person),
            Some(Command::ListDept(dept)) => company.list_dept(&dept),
            Some(Command::ListComp) => company.list_all(),
            Some(Command::Exit) => return,
            None => println!("Invalid Command!\n"),
        }
    }
}



