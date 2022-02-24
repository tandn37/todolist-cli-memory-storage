use std::io;
 
pub fn show_main_menu() {
  println!("\n============================
Please select your command:
      1, List
      2, Add
      3, Update
      4, Remove
      5, Exit
============================
  ");
}

pub fn get_text_input() -> String {
  let mut content = String::new();
      io::stdin()
          .read_line(&mut content)
          .expect("Failed to read todo");
  content
}