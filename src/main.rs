use std::mem;
use core::mem::swap;


struct Counter {
    counter: Vec<usize>,
    last_rollover: usize,
    capacity: usize,
}


impl Counter {
    fn new(num_plates: usize) -> Counter {
        let scale: usize = mem::size_of::<usize>() * 8;
        let mut real_size: usize = num_plates / scale;
        if num_plates % scale != 0 {
            real_size = real_size + 1;
        }


        let mut res: Counter = Counter {
            counter: Vec::with_capacity(real_size),
            capacity: real_size - 1,
            last_rollover: 0,
        };

        for _i in 0..real_size {
            res.counter.push(0);
        }

        return res;
    }


    fn increment(&mut self) {
        let mut word_index: usize = 0;
        let mut bit_index: usize = 0;
        while word_index <= self.capacity && self.counter[self.capacity - word_index] == usize::MAX {
            self.counter[self.capacity - word_index] = 0;
            word_index = word_index + 1;
        }
        self.counter[self.capacity - word_index] = self.counter[self.capacity - word_index] + 1;

        while bit_index < mem::size_of::<usize>() * 8 && (self.counter[self.capacity - word_index] >> bit_index) & 0x01 == 0 {
            bit_index = bit_index + 1;
        }
        self.last_rollover = word_index * mem::size_of::<usize>()  * 8 + bit_index;
    }
}


enum Disc {
    Disc {
        index: usize,
        next: Box<Disc>,
    },
    Plate,
}


struct Tower {
    name: String,
    stack: Box<Disc>,
}


impl Tower {
    fn empty(name: &str) -> Tower {
        return Tower {
            name: String::from(name),
            stack: Box::<Disc>::new(Disc::Plate),
        };
    }


    fn full(name: &str, size: usize) -> Tower {
        let mut res: Tower = Tower::empty(name);
        let mut disc: Disc;
        for i in 0..size {
            disc = Disc::Disc { index: size - (i + 1), next: Box::<Disc>::new(*res.stack) };
            *res.stack = disc;
        }
        return res;
    }


    fn push(&mut self, mut disc: Box<Disc>) {
        match *disc {
            Disc::Disc { ref mut next, .. } => {
                swap(next, &mut self.stack);
                swap(&mut self.stack, &mut disc);
            },
            Disc::Plate => { },
        }
    }


    fn pop(&mut self) -> Option<Box<Disc>> {
        match *self.stack {
            Disc::Disc { ref mut next, .. } => {
                let mut disc: Box<Disc> = Box::<Disc>::new(Disc::Plate);
                swap(&mut disc, next);
                swap(&mut self.stack, &mut disc);
                return Some(disc);
            },
            Disc::Plate => { },
        }
        return None;
    }


    fn peek<'a>(&'a self) -> &'a Disc {
        return &*self.stack;
    }


    fn move_to(&mut self, other: &mut Tower) {
        if !self.can_move(other) {
            println!("Error! Illegal move!");
        }
        let opt: Option<Box<Disc>> = self.pop();
        match opt {
            Some(disc) => {
                other.push(disc);
            },
            None => { },
        }
    }


    fn count(&self) -> usize {
        let mut disc: &Disc = &*self.stack;
        let mut i: usize = 0;
        loop {
            match *disc {
                Disc::Disc { ref next, .. } => {
                    i = i + 1;
                    disc = &**next;
                },
                Disc::Plate => {
                    return i;
                },
            }
        }
    }


    fn is_valid(&self) -> bool {
        let mut disc: &Disc = &*self.stack;
        loop {
            match *disc {
                Disc::Disc { ref index, ref next } => {
                    let smaller: usize = *index;
                    match **next {
                        Disc::Disc { index, .. } => {
                            if smaller >= index {
                                return false;
                            }
                        },
                        Disc::Plate => {
                            break;
                        }
                    }
                    disc = &**next;
                },
                Disc::Plate => {
                    break;
                },
            }
        }
        return true;
    }


    fn can_move(&self, other: &Tower) -> bool {
        match self.peek() {
            Disc::Disc { index, .. } => {
                let smaller: usize = *index;
                match other.peek() {
                    Disc::Disc { index, .. } => {
                        return smaller < *index;
                    },
                    Disc::Plate => {
                        return true;
                    }
                }
            },
            Disc::Plate => {
                return false;
            },
        }
    }
}


fn main() {

    const PRINT_BOARD: bool = false;
    const PRINT_CONCISE: bool = false;
    const STACK_SIZE: usize = 2000;

    let mut counter: Counter = Counter::new(STACK_SIZE);

    let mut a: Tower = Tower::full("A", STACK_SIZE);
    let mut b: Tower = Tower::empty("B");
    let mut c: Tower = Tower::empty("C");

    while c.count() < STACK_SIZE {
        if PRINT_BOARD {
            display_towers(&a, &b, &c);
            if !a.is_valid() { println!("A Invalid!"); }
            if !b.is_valid() { println!("B Invalid!"); }
            if !c.is_valid() { println!("C Invalid!"); }
        }

        counter.increment();

        match a.peek() {
            Disc::Disc { index, .. } => {
                if *index == counter.last_rollover {
                    if *index == 0 || a.can_move(&b) {
                        if PRINT_CONCISE { println!("{}: A -> B", *index); }
                        a.move_to(&mut b);
                    } else {
                        if PRINT_CONCISE { println!("{}: A -> C", *index); }
                        a.move_to(&mut c);
                    }
                    continue;
                }
            },
            Disc::Plate => { },
        }

        match b.peek() {
            Disc::Disc { index, .. } => {
                if *index == counter.last_rollover {
                    if *index == 0 || b.can_move(&c) {
                        if PRINT_CONCISE { println!("{}: B -> C", *index); }
                        b.move_to(&mut c);

                    } else {
                        if PRINT_CONCISE { println!("{}: B -> A", *index); }
                        b.move_to(&mut a);
                    }
                    continue;
                }
            },
            Disc::Plate => { },
        }

        match c.peek() {
            Disc::Disc { index, .. } => {
                if *index == counter.last_rollover {
                    if *index == 0 || c.can_move(&a) {
                        if PRINT_CONCISE { println!("{}: C -> A", *index); }
                        c.move_to(&mut a);
                    } else {
                        if PRINT_CONCISE { println!("{}: C -> B", *index); }
                        c.move_to(&mut b);
                    }
                    continue;
                }
            },
            Disc::Plate => { },
        }
    }
    
    display_towers(&a, &b, &c);
    if !a.is_valid() { println!("A Invalid!"); }
    if !b.is_valid() { println!("B Invalid!"); }
    if !c.is_valid() { println!("C Invalid!"); }
}


fn display_towers(a: &Tower, b: &Tower, c: &Tower) {
    let mut discs: [&Disc; 3] = [&a.stack, &b.stack, &c.stack];
    let mut plates: usize;
    println!("{:<12}|{:<12}|{:<12}", "A:", "B:", "C:");
    loop {
        plates = 0;
        match discs[0] {
            Disc::Disc { index, ref next } => {
                print!("{:<12}|", index);
                discs[0] = &**next;
            },
            Disc:: Plate => {
                print!("{:<12}|", " ");
                plates = plates + 1;
            },
        }
        match discs[1] {
            Disc::Disc { index, ref next } => {
                print!("{:<12}|", index);
                discs[1] = &(*next);
            },
            Disc::Plate => {
                print!("{:<12}|", " ");
                plates = plates + 1;
            },
        }
        match discs[2] {
            Disc::Disc { index, ref next } => {
                println!("{:<12}", index);
                discs[2] = &(*next);
            },
            Disc::Plate => {
                println!("{:<12}", " ");
                plates = plates + 1;
            },
        }
        if plates > 2 {
            println!("{:_<39}", "");
            return;
        }
    }
}
