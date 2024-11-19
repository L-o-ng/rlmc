// lmc model here: collection of structs that simulate the computer

use std::{collections::VecDeque, fmt::Error, process::Output};

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
    fn read(&self) -> Result<i32, Error> {
        Ok(self.display)
    }
    fn add(&mut self, num: i32) -> Result<(), Error> {
        if self.display + num > 999 {
            return Err(Error);
        }
        self.display += num;
        Ok(())
    }
    fn sub(&mut self, num: i32) -> Result<(), Error> {
        if self.display - num < 0 {
            return Err(Error);
        }

        self.display -= num;
        Ok(())
    }
}

struct Counter {
    count: i32
}
impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn tick(&mut self) -> Result<(), Error> {
        if self.count >= 99 {
            return Err(Error)
        }

        self.count += 1;
        Ok(())
    }
    fn reset(&mut self) -> Result<(), Error> {
        self.count = 0;
        Ok(())
    }
}

struct ITray {
    tray: VecDeque<i32>
}
impl ITray {
    fn new() -> Self {
        Self { tray: VecDeque::new() }
    }
    fn get_input(&mut self, input: i32) -> Result<(), Error> {
        self.tray.push_back(input);
        Ok(())
    }
    fn read_input(&mut self) -> Result<i32, Error> {
        if let Some(input) = self.tray.pop_front() {
            Ok(input)
        } else {
            Err(Error)
        }
    }
}
struct OTray {
    tray: VecDeque<i32>
}
impl OTray {
    fn new() -> Self {
        Self { tray: VecDeque::new() }
    }
    fn add_to_output(&mut self, out: i32) -> Result<(), Error> {
        self.tray.push_back(out);
        Ok(())
    }
    fn read_output(&mut self) -> Result<(), Error> {
        for output in &self.tray {
            println!("{}", output);    
        }
        self.tray.clear();
        Ok(())
    }
}

struct Flag {
    STOP: bool,
}
impl Flag {
    fn new() -> Self {
        Self { STOP: false, }
    }
    fn raise(&mut self) {
        self.STOP = true;
    }
    fn lower(&mut self) {
        self.STOP = false;
    }
}