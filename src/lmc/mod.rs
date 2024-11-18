// lmc model here: collection of structs that simulate the computer

use std::{collections::VecDeque, fmt::Error};

pub struct LMC {
    mailbox: Mailbox,
    calculator: Calculator,
    counter: Counter,
    input: ITray,
    output: OTray,
    flag: Flag,
}
impl LMC {
    pub fn new() -> Self {
        Self { mailbox: Mailbox::new(),  
            calculator: Calculator::new(), 
            counter: Counter::new(),
            input: ITray::new(),
            output: OTray::new(),
            flag: Flag::new(),
        }
    }
}

struct Mailbox {
    boxes: Vec<i32>,
}
impl Mailbox {
    fn new() -> Self {
      Self { boxes: vec![0; 100] }  
    }
    fn put(&mut self, data: i32, id: i32) -> Result<(), Error> {
        if id > 99 || id < 0 {
            return Err(Error);
        }
        
        self.boxes[id as usize] = data;
        Ok(())
    }
    fn open(&self, id: i32) -> Result<i32, Error> {
        if id > 99 || id < 0 {
            return Err(Error);
        }

        Ok(self.boxes[id as usize])
    }
}

struct Calculator {
    display: i32,
}
impl Calculator {
    fn new() -> Self {
        Self { display: 0 }
    }
    fn read() -> Result<i32, Error> {
        todo!()
    }
    fn add(num: i32) -> Result<i32, Error> {
        todo!()
    }
    fn sub(num: i32) -> Result<i32, Error> {
        todo!()
    }
}

struct Counter {
    count: i32
}
impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

struct ITray {
    tray: VecDeque<i32>
}
impl ITray {
    fn new() -> Self {
        Self { tray: VecDeque::new() }
    }
}
struct OTray {
    tray: VecDeque<i32>
}
impl OTray {
    fn new() -> Self {
        Self { tray: VecDeque::new() }
    }
}

struct Flag {
    NEG: bool,
}
impl Flag {
    fn new() -> Self {
        Self { NEG: false, }
    }
}