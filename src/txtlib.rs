use crate::{txterror::EmptyTxtResult, util::get_user_arg};

pub type TxtCommand = (String, String, Box<dyn Fn() -> EmptyTxtResult>);
pub struct TxtLib {
    pub project_name: String,
    pub project_description: String,
    pub doc_url: String,
    pub commands: Vec<TxtCommand>,
}

impl TxtLib {
    pub fn new(project_name: &str, project_description: &str, doc_url: &str) -> TxtLib {
        TxtLib {
            project_name: String::from(project_name),
            project_description: String::from(project_description),
            doc_url: String::from(doc_url),
            commands: Vec::new(),
        }
    }

    pub fn add_command<T>(&mut self, command_name: &str, command_description: &str, command: T)
    where
        T: Fn() -> EmptyTxtResult + 'static,
    {
        self.commands.push((
            String::from(command_name),
            String::from(command_description),
            Box::new(command),
        ));
    }

    pub fn start(&self) {
        match self.run_command() {
            Err(err) => err.fire(),
            Ok(_) => (),
        };
    }

    fn run_command(&self) -> EmptyTxtResult {
        if let Ok(command) = get_user_arg(1) {
            let mut run_command = false;

            for current_command in &self.commands {
                if current_command.0 == command {
                    run_command = true;
                    current_command.2()?;
                }
            }

            if !run_command {
                self.help();
            }
        } else {
            self.help();
        }

        Ok(())
    }

    fn help(&self) {
        eprint!("{} - {}\n\n", self.project_name, self.project_description);

        for current_command in &self.commands {
            eprint!("{} - {}\n\n", current_command.0, current_command.1);
        }

        eprint!("more documentation is available at {}", self.doc_url);
    }
}
