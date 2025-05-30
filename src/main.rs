use color_eyre::Result;
use nom::{bytes::take_while_m_n, combinator::map_res, IResult};
use nom::character::complete::digit1;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

fn is_digit(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn parse_int(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(1, 1, digit1),
        is_digit
    ).parse(input)
}


fn main() -> Result<()> {
    color_eyre::install()?;
    
    let input_data = fs::read_to_string("input.txt")?;
    
    for line in input_data.lines() {
        println!("{line}");
    }

    Ok(())
}
