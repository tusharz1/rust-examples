/*
 * A simple todo list utilising usage of Struct and Trait.
 */

struct TodoItem {
    content: String,
    is_done: bool
}

trait TodoList {
    fn add(&mut self, content: &str) -> Result<i32, String>;
    fn mark_done(&mut self, idx: i32) -> Result<(), String>;
    fn search(&mut self, needle: &str) -> Result<Vec<usize>, String>;
}

impl TodoList for Vec<TodoItem> {
    fn add(self: &mut Vec<TodoItem>, content: &str) -> Result<i32, String> {
            let x = TodoItem {
                content :content.to_string(),
                is_done : false
            };
            self.push(x);
            Ok(self.len() as i32)
    }

    fn mark_done(self: &mut Vec<TodoItem>, idx: i32) -> Result<(), String> {
        let some_val = &self[idx as usize];
        self[idx as usize] = TodoItem {
            content : some_val.content.clone(),
            is_done : true
        };
        Ok(())
    }

    fn search(self: &mut Vec<TodoItem>, needle: &str) 
            -> Result<Vec<usize>,String> {

        let mut res = Vec::<usize>::new();
        for (pos, ele) in self.iter().enumerate() {
            if ele.content.contains(needle) {
                res.push(pos);
            }
        }
        Ok(res)
    }
}

fn main() {
    let mut x = Vec::<TodoItem>::new();
    let _ = x.add("hello");
    let _ = x.add("world");
    let _ = x.mark_done(1);
    match x.search("ll") {
        Ok(res) => println!("{:?}", res),
        _ => println!("something went wrong"),
    }
}
