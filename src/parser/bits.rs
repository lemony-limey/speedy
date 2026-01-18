use nom::Parser;
use nom::bits::complete::take;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::IResult;
use crate::parser::BitInput;


/// Takes one bit from the input, returning true for 1 and false for 0.
/// This function is a modified version of one of the same name from:
/// https://blog.adamchalmers.com/nom-bits/
///
/// If the bit is 1, true is returned.
/// If the bit is 0, false is returned.
pub(crate) fn take_bit_bool(input: BitInput) -> IResult<BitInput, bool>
{
    map( take(1usize), |bit: u8| bit != 0 ).parse(input)
}

pub(crate) fn take_bits_u8(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u8>
{
    // If the number passed in is greater than 8, return Err.
    if number_of_bits > 8
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

pub(crate) fn take_bits_u16(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u16>
{
    // If the number passed in is greater than 16, return Err.
    if number_of_bits > 16
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

pub(crate) fn take_bits_u32(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u32>
{
    // If the number passed in is greater than 32, return Err.
    if number_of_bits > 32
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

pub(crate) fn take_bits_u64(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u64>
{
    // If the number passed in is greater than 64, return Err.
    if number_of_bits > 64
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}