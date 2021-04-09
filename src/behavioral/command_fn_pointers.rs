// You have to write 2 fn
// for each Command
// fn pointers implement all three traits Fn, FnMut, and FnOnce

type FnPtr = fn() -> String;
pub struct Command {
    pub execute: FnPtr,
    pub rollback: FnPtr,
}

pub struct Schema {
    commands: Vec<Command>,
}

impl Default for Schema {
    fn default() -> Self {
        Schema::new()
    }
}

impl Schema {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
    pub fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }
    pub fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    pub fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn add_field() -> String {
        "add field".to_string()
    }

    fn remove_field() -> String {
        "remove field".to_string()
    }
    #[test]
    fn fn_pointers_command_test() {
        let mut schema = Schema::new();
        schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
        schema.add_migration(add_field, remove_field);
        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
