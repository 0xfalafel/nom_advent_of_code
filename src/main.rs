use color_eyre::Result;
use nom::character::streaming::u8;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::Parser;
use nom::{bytes::take_while_m_n, combinator::map_res, IResult};
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Point> {
        let parse_two_numbers = separated_pair(
            parse_u8,
            char(','),
            parse_u8
        );
        map(parse_two_numbers, |(x, y)| Point { x, y }).parse(input)
    }
}


fn parse_u8(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse::<u8>).parse(input)
}


fn main() -> Result<()> {
    color_eyre::install()?;
    
    let input_data = fs::read_to_string("input.txt")?;
    
    for line in input_data.lines() {
        println!("{line}");
    }

    Ok(())
}
