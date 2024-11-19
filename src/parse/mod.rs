use crate::lmc::LMC;

pub fn parse_pause_for_input(mut lmc: LMC, code: Vec<i32>) {
    if code.len() == 0 || code.len() > 99 { return; }

    // load code into memory
    for i in 0..code.len() {
        lmc.mailbox.put(code[i], i as i32).unwrap()
    }

    // carry out execution until stop or flag
    while !lmc.flag.is_raised() {
        // fetch
        let count = lmc.counter.read();
        lmc.counter.tick().unwrap();
        let instruction = lmc.mailbox.open(count).unwrap();

        // decode
        /* 
        match instruction / 100 {
            0 => ;
            1 => ;
            2 => ;
            3 => ;
            5 => ;
            9 => ;
            _ => ;
        }
        */
    }
}

pub fn parse_input_from_file() {
    todo!()
}