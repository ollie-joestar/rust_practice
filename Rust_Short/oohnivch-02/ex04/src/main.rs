enum Command {
    Todo(String), // Command: "QUIT"
    Done(usize),  // Command: "DONE"
    Purge,        // Command: "PURGE"
    Quit,         // Command: "QUIT"
}

impl Command {
    fn prompt() -> Self {
        let string = ftkit::read_line();
        let cmd = if string.len() >= 5 {
            &string[0..5]
        } else {
            &string
        };
        match cmd.trim() {
            "DONE" => {
                if let Some(index) = string.strip_prefix("DONE ") {
                    match index.trim().parse::<usize>() {
                        Ok(value) => Command::Done(value),
                        Err(_) => {
                            println!("Invalid DONE format. Use: DONE <index>");
                            Command::prompt()
                        }
                    }
                } else {
                    println!("Something wrong");
                    Command::prompt()
                }
            }
            "TODO" => {
                if let Some(task) = string.strip_prefix("TODO ") {
                    Command::Todo(task.to_string())
                } else {
                    println!("Something wrong");
                    Command::prompt()
                }
            }
            "PURGE" => Command::Purge,
            "QUIT" => Command::Quit,
            _ => {
                println!("Unknown command. Use: TODO, DONE, PURGE or QUIT");
                Command::prompt()
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            dones: Vec::new(),
        }
    }

    fn display(&self) {
        if !self.todos.is_empty() || !self.dones.is_empty() {
            println!();
        }
        for i in 0..self.todos.len() {
            print!("    {i} [ ] {}", self.todos[i].as_str());
        }
        for i in 0..self.dones.len() {
            print!("      [x] {}", self.dones[i].as_str());
        }
        println!();
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo)
    }

    fn done(&mut self, index: usize) {
        if index < self.todos.len() {
            self.dones.push(self.todos[index].clone());
            self.todos.remove(index);
        } else {
            println!("Wrong index");
        }
    }

    fn purge(&mut self) {
        self.dones.clear();
    }
}

fn main() {
    let mut list = TodoList::new();
    loop {
        list.display();
        let command = Command::prompt();
        match command {
            Command::Todo(task) => list.add(task),
            Command::Done(index) => list.done(index),
            Command::Purge => list.purge(),
            Command::Quit => break,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn mock_prompt(input: &str) -> Command {
        let string = input.to_string();
        let cmd = if string.len() >= 5 {
            &string[0..5]
        } else {
            &string
        };
        match cmd.trim() {
            "DONE" => {
                if let Some(index) = string.strip_prefix("DONE ") {
                    match index.trim().parse::<usize>() {
                        Ok(value) => Command::Done(value),
                        Err(_) => panic!("Invalid DONE format."),
                    }
                } else {
                    panic!("Invalid DONE command.");
                }
            }
            "TODO" => {
                if let Some(task) = string.strip_prefix("TODO ") {
                    Command::Todo(task.to_string())
                } else {
                    panic!("Invalid TODO command.");
                }
            }
            "PURGE" => Command::Purge,
            "QUIT" => Command::Quit,
            _ => panic!("Unknown command."),
        }
    }

    #[test]
    fn test_todo_command() {
        let input = "TODO Buy milk";
        let command = mock_prompt(input);
        if let Command::Todo(task) = command {
            assert_eq!(task, "Buy milk");
        } else {
            panic!("Expected TODO command.");
        }
    }

    #[test]
    fn test_done_command() {
        let input = "DONE 1";
        let command = mock_prompt(input);
        if let Command::Done(index) = command {
            assert_eq!(index, 1);
        } else {
            panic!("Expected DONE command.");
        }
    }

    #[test]
    fn test_purge_command() {
        let input = "PURGE";
        let command = mock_prompt(input);
        assert!(matches!(command, Command::Purge));
    }

    #[test]
    fn test_quit_command() {
        let input = "QUIT";
        let command = mock_prompt(input);
        assert!(matches!(command, Command::Quit));
    }

    #[test]
    fn test_todo_list_add() {
        let mut list = TodoList::new();
        list.add("Buy groceries".to_string());
        assert_eq!(list.todos.len(), 1);
        assert_eq!(list.todos[0], "Buy groceries");
    }

    #[test]
    fn test_todo_list_done() {
        let mut list = TodoList::new();
        list.add("Buy groceries".to_string());
        list.done(0);
        assert!(list.todos.is_empty());
        assert_eq!(list.dones.len(), 1);
        assert_eq!(list.dones[0], "Buy groceries");
    }

    #[test]
    fn test_todo_list_purge() {
        let mut list = TodoList::new();
        list.add("Buy groceries".to_string());
        list.done(0);
        list.purge();
        assert!(list.dones.is_empty());
    }

    #[test]
    fn test_todo_list_wrong_done_index() {
        let mut list = TodoList::new();
        list.add("Buy groceries".to_string());
        list.done(1);
        assert_eq!(list.todos.len(), 1);
        assert_eq!(list.dones.len(), 0);
    }
}
