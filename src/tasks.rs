use core::fmt;
use std::process::Command;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub command: String,
    pub arguments: Option<Vec<String>>,
}
impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = match &self.arguments {
            Some(args) => args.join(", "),
            None => String::from("None"),
        };

        write!(
            f,
            "  name: {}\n  command: {}\n  arguments: {:?}\n",
            self.name, self.command, arguments
        )
    }
}

#[derive(Clone, Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Recipe>,
}

impl Rukefile {
    pub fn from_str(raw: &str) -> Result<Self, toml::de::Error> {
        toml::from_str::<Rukefile>(raw)
    }

    fn find_recipe(&self, name: String) -> Option<Recipe> {
        let recipe = self.tasks.iter().find(|recipe| recipe.name.eq(&name));

        recipe.cloned()
    }

    pub fn run_recipe(&self, name: String, quiet: bool) {
        let recipe = self.find_recipe(name).expect("recipe does not exist");

        let command = recipe.command.split(' ').collect::<Vec<&str>>();

        let positional_arguments = command[1..].iter().map(|argument| argument.to_string());

        let arguments = match recipe.arguments {
            Some(mut arguments) => {
                positional_arguments.for_each(|argument| arguments.push(argument));

                arguments
            }
            None => positional_arguments.collect::<Vec<String>>(),
        };

        let output = Command::new(command[0])
            .args(arguments)
            .output()
            .expect("failed to execute command");

        if output.status.success() && !quiet {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", stderr);
        }
    }

    pub fn list_tasks(&self) {
        println!("Tasks in recipe:");
        for t in self.tasks.iter() {
            println!("  {}", t.name);
        }
    }

    pub fn all_tasks(&self) {
        println!("All tasks in recipe:");
        for t in self.tasks.iter() {
            println!("{}", t);
        }
    }
}
