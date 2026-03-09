use std::fs::{self,File};
use std::io::{self,Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task{
    id :usize,
    description :String,
    completed :bool,
}

fn main(){
    let mut tasks: Vec<Task>=load_tasks();

    loop {
        println!("tolist menu:");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. mark task as completed");
        println!("4. delete task");
        println!("5. exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim(){
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().
        read_line(&mut input).
        expect("Failed to read input");
    input
}

fn load_tasks() -> Vec<Task>{
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
}

fn add_task(tasks: &mut Vec<Task>){
    let description = get_input("Enter task description: ");
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task = Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    };
    tasks.push(task);
    save_tasks(tasks);
    println!("Task added successfully.");
}

fn view_tasks(tasks: &Vec<Task>){
    if tasks.is_empty(){
        println!("There are no tasks.");
    }else {
        for task in tasks{
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{}: {} [{}]", task.id, task.description, status);
        }
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>){
    let id = get_input("Enter task ID to mark as completed: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id){
            task.completed = true;
            println!("Task marked as completed.");
            save_tasks(tasks);
        }else {
            println!("Task not found.");
        }
    }else {
        println!("Invalid ID.");
    }
}

fn delete_task(tasks: &mut Vec<Task>){
    let id = get_input("Enter task ID to delete: ");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(pos) = tasks.iter().position(|task| task.id == id){
            tasks.remove(pos);
            println!("Task deleted successfully.");
            save_tasks(tasks);
        }else {
            println!("Task not found.");
        }
    }else {
        println!("Invalid ID.");
    }
}