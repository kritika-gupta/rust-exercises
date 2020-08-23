// there can be 4 different commands
#[derive(Debug)]
pub enum Command {
    Add(String, String),
    ListDept(String),
    ListComp,
    Exit,
}

impl Command {
     
    pub fn create(string: &str) -> Option<Command>{
        
        let lower = string.to_lowercase();        
        let mut words = lower.split_whitespace();
        // chaining the above two will not work as the result of to_lowercase() will not be stored
        // anywhere and will be a temporary - consumed by split_whitespaces which results in slices
        // to the input, i.e., refs to the temporary which dies at the end of the statement.

        match words.next()? {
            "add" => {
                let person = words.next()?;
                let to = words.next()?;
                let dept = words.next()?;
                if to != "to" {
                    return None;
                }
                match words.next() {
                    None => (),
                    Some(_x) => return None
                }
                return Some(Command::Add(dept.to_string(), person.to_string()));
                
            },
            "list" => {
                let dept = words.next();
                match dept {
                    Some(x) => return Some(Command::ListDept(x.to_string())),
                    None => return Some(Command::ListComp)
                }
            },
            "exit" => return Some(Command::Exit),
            _ => None
        }

    }

}


