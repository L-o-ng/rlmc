// lmc model here: collection of structs that simulate the computer

use std::collections::VecDeque;

pub struct LMC {
    mailbox: Mailbox,
    calculator: Calculator,
    counter: Counter,
    input: ITray,
    output: OTray,
}
impl LMC {
    pub fn new() -> Self {
        todo!()
    }
}

struct Mailbox {
    boxes: Vec<i32>,
}
impl Mailbox {
    fn new() -> Self {
        todo!()
    }
}

struct Calculator {
    display: i32,
}
impl Calculator {
    fn new() -> Self {
        todo!()
    }
}

struct Counter {
    count: i32
}
impl Counter {
    fn new() -> Self {
        todo!()
    }
}

struct ITray {
    tray: VecDeque<i32>
}
impl ITray {
    fn new() -> Self {
        todo!()
    }
}
struct OTray {
    tray: VecDeque<i32>
}
impl OTray {
    fn new() -> Self {
        todo!()
    }
}