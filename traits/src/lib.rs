pub struct MediumPost {
    pub title: String,
    pub subtitle: String,
    pub text: String,
    pub author: String,
}

pub struct Email {
    pub subject: String,
    pub text: String,
    pub sender: String,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Summary goes here!")
    }
}

impl Summary for MediumPost {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.subtitle)
    }
}

// impl Summary for Email {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.sender, self.subject)
//     }
// }

// Use default implementation
impl Summary for Email {}
