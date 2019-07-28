extern crate nom;

use nom::bytes::complete::take_while;
use nom::multi::separated_list;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use nom::sequence::tuple;

#[derive(Debug)]
enum Token {
    A,
    B,
    C,
}

fn token(i: &str) -> IResult<&str, Token> {
    let (i, val) = alt((tag("a"), tag("b"), tag("c")))(i)?;
    unsafe {
        Ok((i, match val {
            "a" => Token::A,
            "b" => Token::B,
            "c" => Token::C,
            v => panic!(format!("Unknown token {}", v)),
        }))
    }
}

fn tokens(i: &str) -> IResult<&str, Vec<Token>> {
    let (i, (_, list, _)) = tuple((sp, separated_list(sp, token), sp))(i)?;
    Ok((i, list))
}

fn sp(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

pub fn main() {
    println!("{:?}", tokens("a b b a c c "));
}
