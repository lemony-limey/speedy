use nom::combinator::map_res;
use nom::IResult;
use crate::VariableLengthInteger;

fn parse_variable_length_integer(input: &str) -> IResult<&str, VariableLengthInteger>
{
    todo!()
    // map_res(
    //     // Get some bytes or something
    //     // Parse them into a variable length integer
    // ).parse(input)
}
