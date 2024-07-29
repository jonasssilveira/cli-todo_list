use std::io;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    let mut todo_list: Vec<HashMap<String, bool>> = Vec::new();
    loop {
        println!("--------------------------------------");
        println!("Bem-vindo ao gerenciador de todo-list ");
        println!("--------------------------------------");
        println!("Options");
        println!("1 para adicionar");
        println!("2 para listar todos os todo");
        println!("3 para listar os todo done");
        println!("4 para listar os todo open");
        println!("5 para remover");
        println!("6 para finalizar um todo");
        let option = get_user_input();
        match option.as_str() {
            "1" => {
                println!("Insira a nova todo_list");
                add_item(&mut todo_list, get_user_input())
            }
            "2" => list_todo_list(&todo_list),
            "3" => list_todo_list_done(&todo_list),
            "4" => list_todo_list_open(&todo_list),
            "5" => {
                println!("Insira o numero do todo list para remover");
                for (index, value) in todo_list.iter().enumerate() {
                    for (k, v) in value {
                        println!("{} {}", index, k);
                    }
                }
                let index_to_delete = get_user_input();
                todo_list.remove(index_to_delete.parse().unwrap());
            }
            "6" => set_done(&mut todo_list),
            _ => println!("opcao invalida")
        }
    }
}

fn list_todo_list(todo_list: &Vec<HashMap<String, bool>>) {
    for (value) in todo_list {
        for (k, v) in value {
            println!("{}", k);
        }
    }
}

fn list_todo_list_done(todo_list: &Vec<HashMap<String, bool>>) {
    for (value) in todo_list {
        for (k, v) in value {
            if *v {
                println!("{}", k);
            }
        }
    }
}

fn list_todo_list_open(todo_list: &Vec<HashMap<String, bool>>) {
    for (value) in todo_list {
        for (k, v) in value {
            if !*v {
                println!("{}", k);
            }
        }
    }
}

fn add_item(todo_list: &mut Vec<HashMap<String, bool>>, input: String) {
    let mut map: HashMap<String, bool> = HashMap::new();
    map.insert(input, false);
    todo_list.push(map);
}

fn set_done(todo_list: &mut Vec<HashMap<String, bool>>) {
    println!("Digite o todo que deseja dar como done");
    let input = get_user_input();
    for (todo) in todo_list {
        todo.insert(input.clone(), true);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string();
}