use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[✓]" } else { "[ ]" };
            println!("{}: {} - {}", i + 1, status, task.description);
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        }
    }

    fn save_to_file(&self, filename: &str) {
        let serialized = serde_json::to_string(&self.tasks).unwrap();
        fs::write(filename, serialized).expect("Gagal menyimpan ke file");
    }

    fn load_from_file(filename: &str) -> TodoList {
        let data = fs::read_to_string(filename).unwrap_or("[]".to_string());
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();
        TodoList { tasks }
    }
}

fn main() {
    let mut todo_list = TodoList::load_from_file("todo_list.json");

    loop {
        println!("\n== To-Do List ==");
        println!("1. Tambah Task");
        println!("2. Tandai Task Selesai");
        println!("3. Tampilkan Task");
        println!("4. Simpan & Keluar");
        print!("Pilih opsi: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca input");
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Deskripsi Task: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Gagal membaca input");
                let description = description.trim().to_string();
                todo_list.add_task(description);
                println!("Task ditambahkan.");
            }
            "2" => {
                print!("Nomor Task yang selesai: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Gagal membaca input");
                let index: usize = index.trim().parse().expect("Nomor tidak valid");
                todo_list.complete_task(index - 1);
                println!("Task ditandai selesai.");
            }
            "3" => {
                todo_list.list_tasks();
            }
            "4" => {
                todo_list.save_to_file("todo_list.json");
                println!("Daftar task disimpan. Keluar...");
                break;
            }
            _ => {
                println!("Opsi tidak valid, coba lagi.");
            }
        }
    }
}
