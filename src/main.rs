enum TokenType{
    Shift,
    Add,
    Skip,
    Goto,
    Put,
    Get,
}

struct Token{
    ttype: TokenType,
    value: isize
}

fn tokenize(code: &str) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip_queue = Vec::new();

    let mut skips = 0;

    for i in 1..code.as_bytes().len() {
        let c = code.as_bytes()[i] as char;

        match c {
            '[' => {
                skip_queue.push(tokens.len());
                tokens.push(
                    Token {
                        ttype:TokenType::Skip,
                        value: 0 
                    }
                );
                skips += 1;
            },
            ']' => {
                skips -= 1;
                tokens[skip_queue[skips]].value = tokens.len();
                tokens.push(Token{
                    ttype:TokenType::Goto,
                    value:skip_queue[skips] - 1
                });
            },

            '>'|'<' => {
                tokens.push(Token {
                    ttype:TokenType::Shift,
                    value: if c == '>' {1} else {-1}
                });
            },
            '+'|'-' => {
                tokens.push(Token {
                    ttype:TokenType::Add,
                    value: if c == '+' {1} else {-1}
                });
            },
            '.' => {
                tokens.push(Token {
                    ttype:TokenType::Put,
                    value: 1
                });
            },
            ',' => {
                tokens.push(Token {
                    ttype:TokenType::Get,
                    value: 1
                });
            },


            _ =>{}
        }

    };

    tokens
}

fn main() {
    let tokens = tokenize("Hello, world!");
}
