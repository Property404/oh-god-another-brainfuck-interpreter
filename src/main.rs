#![allow(dead_code)]
use std::env;
use std::fs;
use std::io::prelude::*;
enum Token{
    Start,
    Shift(isize),
    Add(isize),
    Skip(usize),
    Goto(usize),
    Put,
    Get,
}

impl Token{
    fn print(&self) {
        match self {
            Token::Start => {print!("[Start]");},
            Token::Shift(n) => {print!("[> {}]", n);},
            Token::Add(n) => {print!("[+ {}]", n);},
            Token::Skip(n) => {print!("[Skip {}]", n);},
            Token::Goto(n) => {print!("[Goto {}]", n);},
            _ => {print!("[Misc]");}
        }
    }
}

fn tokenize(code: &str) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip_stack = Vec::new();

    tokens.push(Token::Start);
    for i in 0..code.as_bytes().len() {
        let c = code.as_bytes()[i] as char;

        match c {
            '[' => {
                skip_stack.push(tokens.len());
                tokens.push(Token::Skip(0));
            },
            ']' => {
                if let Some(popped) = skip_stack.pop() {
                    tokens[popped] = Token::Skip(tokens.len());
                    tokens.push(Token::Goto(popped - 1));
                } else {
                    println!("Time to panic - skip queue exceeded!");
                }
            },

            '>'|'<' => {
                tokens.push(Token::Shift(if c == '>' {1} else {-1}));
            },
            '+'|'-' => {
                tokens.push(Token::Add(if c == '+' {1} else {-1}));
            },
            '.' => {
                tokens.push(Token::Put);
            },
            ',' => {
                tokens.push(Token::Get);
            },

            _ =>{}
        }

    };

    tokens
}

fn interpret(tokens: &Vec<Token>)
{
    let mut tape:Vec<u8> = Vec::new();
    let mut data_pointer:isize = 0;

    let mut instruction_pointer = 0;
    while instruction_pointer < tokens.len()
    {
        let token = &(tokens[instruction_pointer]);
        //token.print();
        //println!(" dp:{} ip:{}", data_pointer, instruction_pointer);
        match token {
            Token::Shift(amount) => {
                data_pointer += amount;
                if data_pointer < 0 {
                    println!("Time to panic!");
                }
            },
            Token::Add(amount) => {
                tape.reserve(1 + data_pointer as usize);
                while tape.len() < 1 + data_pointer as usize {
                    tape.push(0);
                }
                let idx = data_pointer as usize;
                tape[idx] = (tape[idx] as isize + amount) as u8;
            },
            Token::Put => {
                print!("{}", tape[data_pointer as usize] as char);
            },
            Token::Get => {
                println!("Get not implemented");
            },
            Token::Skip(location) => {
                while tape.len() < 1 + data_pointer as usize {
                    tape.push(0);
                }
                if tape[data_pointer as usize] == 0 {
                    instruction_pointer = *location;
                };
            },
            Token::Goto(location) => {
                instruction_pointer = *location;
            },

            Token::Start => {}
        };

        instruction_pointer += 1;
    };
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).expect("Can't open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file!");
    let tokens = tokenize(&contents[..]);
    interpret(&tokens);
}
