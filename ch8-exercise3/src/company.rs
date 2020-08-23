use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<String>>
}

impl Company {

    pub fn new() -> Company {
        Company {
            departments: HashMap::new()
        }
    }

    pub fn add_employee(&mut self, dept: &str, name: &str){
        
        // if dept does not exist in map, add it with a value = empty vector
        let department = self.departments.entry(dept.to_string()).or_insert(vec![]);
        department.push(name.to_string());
        println!("Added {} to department {}\n", name, dept);
    }

    pub fn list_all(&self) {
        //iterate over
        let mut all_emps = Vec::new();
        for (_dept, emps) in &self.departments {
            all_emps.extend(emps);
        }
        all_emps.sort();
        println!("All employees:");
        for employee in &all_emps {
            println!("- {}", employee);
        }
        println!();
    }

    pub fn list_dept(&self, dept: &str) {
        match self.departments.get(dept) {
            Some(x) => {
                let mut employees = x.clone();
                employees.sort();
                println!("Employees in department {}:", dept);
                for employee in &employees {
                    println!("- {}", employee);
                }
            },
            None => println!("Department does not exist!"),
        };
        println!();
    }
}

