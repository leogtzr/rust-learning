use std::io;
use std::io::Write;
use std::collections::HashMap;

const EXPECTED_NUMBER_OF_TOKENS_FOR_ADD: usize = 4;

fn add_person_to_department(words: &Vec<&str>, personnel: &mut HashMap<String, Vec<String>>) -> Result<(), String> {
    if words.len() != EXPECTED_NUMBER_OF_TOKENS_FOR_ADD {
        return Err(format!("Wrong number of elements: {}, expected: {}", words.len(), EXPECTED_NUMBER_OF_TOKENS_FOR_ADD));
    }

    let person = words[1].to_string();
    let department = words[words.len() - 1].to_string();

    let dept_entry = personnel.entry(department).or_insert(Vec::new());
    dept_entry.push(person);
    
    Ok(()) 
}

fn view_members_of_department<'a>(department: &str, personnel: &'a HashMap<String, Vec<String>>) -> Result<&'a Vec<String>, String> {
   personnel.get(department).ok_or_else(|| format!("Department '{}' not found", department)) 
}

fn departments_by_person<'a>(personnel: &'a HashMap<String, Vec<String>>, person: &str) -> Vec<&'a String> {
    personnel
        .iter()
        .filter_map(|(departamento, personas)| {
            if personas.iter().any(|p| p == person) {
                Some(departamento)
            } else {
                None
            }
        })
    .collect()
}

fn print_department_count(personnel: &HashMap<String, Vec<String>>) {
    for (department, people) in personnel {
        println!("{}: {}", department, people.len());
    }
}

fn main() {
    println!("Departments REPL v0.0");
    let mut personnel: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read users input")
            ;

        let words: Vec<&str> = user_input.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        let command = words[0];

        match command.to_lowercase().as_str() {
            "add" => {
                match add_person_to_department(&words, &mut personnel) {
                    Ok(()) => println!("Ok"),
                    Err(reason) => println!("Wow: something happened: {}", reason),
                }
            },
            "view" => {
                if words.len() < 2 {
                    println!("Please specify a department");
                    continue;
                }
                let department = words[1];
                match view_members_of_department(department, &personnel) {
                    Ok(members) => println!("Members: {:?}", members),
                    Err(reason) => println!("Wow: something bad happened: {}", reason)
                }
            },
            "find" => {
                if words.len() < 2 {
                    println!("Please specify a person");
                    continue;
                }
                let person = words[1];
                let depts = departments_by_person(&personnel, person);
                println!("{} works in: {:?}", person, depts);
            },
            "count" => {
                print_department_count(&personnel);

            },
            _ => println!("do another shit")
        };
    }
}
