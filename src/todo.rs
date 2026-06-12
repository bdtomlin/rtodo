/// A sruct to represent a todo with id, text, and done
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            id: 0,
            text,
            done: false,
        }
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, text: {}, done: {}",
            self.id, self.text, self.done
        )
    }
}

#[cfg(test)]
#[path = "todo_tests.rs"]
mod tests;
