use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();

    loop {
        match menu() {
            Some(1) => ask_add_employee(&mut employees),
            Some(2) => ask_list_department(&employees),
            Some(3) => list_all(&employees),
            Some(4) => {
                break;
            }
            Some(n) => println!("{} is not a valid choice.", n),
            _ => println!("Please enter a number.")
        }
    }
}

fn get_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Failed to flush stdout");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    String::from(s.trim())
}

fn menu() -> Option<i32> {
    println!("");
    println!("1. Add employee");
    println!("2. List department");
    println!("3. List all");
    println!("4. Quit");
    println!("");

    match get_line("> ").parse() {
        Result::Ok(n) => Some(n),
        _ => None
    }
}

fn ask_add_employee(employees: &mut HashMap<String, Vec<String>>) {
    println!("");
    let name = get_line("Name: ");
    let department = get_line("Department: ");

    println!("");
    println!("Added {} to {}.", name, department);
    let names = employees.entry(department).or_insert(Vec::new());
    names.push(name);
}

fn list_department(employees: &HashMap<String, Vec<String>>,
                   indent: &str,
                   department: &str) {
    if let Some(orig_names) = employees.get(department) {
        let mut names = orig_names.clone();
        names.sort();

        for name in &names {
            println!("{}{}", indent, name);
        }
    } else {
        println!("No such department {}.", department);
    }
}

fn ask_list_department(employees: &HashMap<String, Vec<String>>) {
    println!("");
    let department = get_line("Department: ");

    println!("");
    list_department(employees, "", &department);
}

fn list_all(employees: &HashMap<String, Vec<String>>) {
    println!("");

    let mut depts = Vec::new();
    for dept in employees.keys() {
        depts.push(dept);
    }

    depts.sort();
    for dept in &depts {
        println!("{}", dept);
        list_department(employees, "    ", dept);
    }
}
