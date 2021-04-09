// You have to write 2 fn
// for each Command, Each fn is an implementation
// of the Fn trait

type Migration<'a> = Box<dyn Fn() -> &'a str>;

pub struct Schema<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>,
}

impl<'a> Schema<'a> {
    pub fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    pub fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }
    pub fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    pub fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn add_field() -> &'static str {
        "add field"
    }
    fn remove_field() -> &'static str {
        "remove field"
    }
    #[test]
    fn fn_pointers_command_test() {
        let mut schema = Schema::new();
        schema.add_migration(|| "create table", || "drop table");
        schema.add_migration(add_field, remove_field);
        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
