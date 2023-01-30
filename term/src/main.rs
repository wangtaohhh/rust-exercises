use std::io::{stdout};
use crossterm::{ExecutableCommand, cursor};

fn main() {
    

let mut stdout = stdout();
stdout.execute(cursor::MoveTo(1,8));
}

