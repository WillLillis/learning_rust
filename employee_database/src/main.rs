use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut database: HashMap<String, String> = HashMap::new();

    loop {
        println!("Select an action: \'Add\', \'List-Dept\', \'List-All\', \'Quit\'");
        let mut user_in = String::new();    
        io::stdin().read_line(&mut user_in).expect("Failed to read line");

        match user_in.trim() { // converts from String to &str, and removes whitespace/ newlines
            "Add" => add_employee(&mut database),
            "List-Dept" => list_dept(&database),
            "List-All" => list_all(&database),
            "Quit" => break,
            _ => continue,
        };

    };

}


fn add_employee(database: &mut HashMap<String, String>) {
    let mut name = String::new();
    let mut dept = String::new();

    print!("Employee name: ");
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin().read_line(&mut name).expect("Failed to read the name");
    print!("Department: ");
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin().read_line(&mut dept).expect("Failed to read the department");
    name = name.trim().to_string();
    dept = dept.trim().to_lowercase();
    
    database.insert(name, dept);
}

fn list_dept(database: &HashMap<String, String>) {
    let mut dept_in = String::new();

    print!("Department: ");
    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin().read_line(&mut dept_in).expect("Failed to read the department");
    dept_in = dept_in.trim().to_lowercase();
    
    //let mut entries: Vec<String> = Vec::new();
    let mut entries: Vec<&String> = Vec::new();
    for (name_db, dept_db) in database {
        if dept_in == *dept_db {
            //entries.push(String::from(name_db));
            entries.push(name_db);
        }
    }

    if entries.is_empty() {
        println!("No corresponding entries");
    } else {
        entries.sort();
        for (i, name) in entries.iter().enumerate() {
            println!("{i}: {name}");
        }
    }
}

fn list_all(database: &HashMap<String, String>) {
    //let mut depts: Vec<String> = Vec::new();
    let mut depts: Vec<&String> = Vec::new();
    for dept_db in database.values() {
        if !depts.contains(&dept_db) {
            //depts.push(String::from(dept_db)); // making copies vs....
            depts.push(dept_db); // chasing around refs, idk which is preferred here...
        }
    }

    depts.sort();
    //let mut names: Vec<String> = Vec::new();
    let mut names: Vec<&String> = Vec::new();
    for (i, dept) in depts.iter().enumerate() {
        names.clear();
        println!("{i}: {dept}");
        for (name_db, dept_db) in database {
            if **dept == *dept_db {
                //names.push(String::from(name_db));
                names.push(name_db);
            }
        }

        names.sort();
        for (j, name) in names.iter().enumerate() {
            println!("\t{j}: {name}");
        } 
    }
}
