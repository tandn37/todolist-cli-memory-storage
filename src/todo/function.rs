use chrono::{DateTime, Utc, Local};
use std::time::{UNIX_EPOCH, Duration};
use std::num::ParseIntError;

use super::{Todo,get_text_input};

pub fn list(todo_list: &Vec<Todo>) {
  if todo_list.len() == 0 {
      return println!("List is empty");
  }
  println!("List:");
  for (i, todo) in todo_list.iter().enumerate() {
      let d = UNIX_EPOCH + Duration::from_secs(todo.created_at);
      let datetime = DateTime::<Local>::from(d);
      let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
      let status = if todo.is_done {
          "Done"
      } else {
          "NotDone"
      };
      println!("{}: ({}) {} - ({})", i + 1, status, todo.content, time_str);
  }
}

pub fn create(text: String) -> Todo {
  let now = Utc::now();
  let created_at = now.timestamp();
  Todo {
      content: text,
      is_done: false,
      created_at: created_at as u64,
  }
}

pub fn add(todo_list: &mut Vec<Todo>) {
  println!("Please input your todo:");
  let content = get_text_input();
  let todo = create(content);
  todo_list.push(todo);
  println!("Added successful!");
  list(todo_list);
}

pub fn remove(todo_list: &mut Vec<Todo>) {
  println!("Please input todo id you want to remove:");
  let id = get_text_input();
  let index: Result<usize, ParseIntError> = id.trim().parse();
  match index {
      Err(_) => {
          return println!("Invalid todo id");
      },
      Ok(value) => {
          if value < 1 || value > todo_list.len() {
              return println!("Invalid todo id");
          }
          todo_list.remove(value - 1);
      }
  }
  println!("Removed successful!");
  list(todo_list);
}

pub fn update(todo_list: &mut Vec<Todo>) {
  println!("Please input action you want to update:");
  println!("
      1, Update Text
      2, Mark as done
      3, Mark as not done
  ");
  let action = get_text_input();
  println!("Please input todo id:");
  let id = get_text_input();
  let index: Result<usize, ParseIntError> = id.trim().parse();
  match index {
      Err(_) => {
          return println!("Invalid todo id");
      },
      Ok(value) => {
          if value < 1 || value > todo_list.len() {
              return println!("Invalid todo id");
          }
      
          let ind = value - 1;
          match action.trim() {
              "1" => {
                  println!("Please input new text:");
                  let text = get_text_input();
                  todo_list[ind].content = text;
              },
              "2" => {
                  todo_list[ind].is_done = true;
              },
              "3" => {
                  todo_list[ind].is_done = false;
              },
              _ => println!("Invalid option"),
          }
      }
  }
  println!("Updated successful!");
  list(todo_list);
}