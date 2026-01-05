use bytes::Bytes;
use nom::combinator::map_res;
use nom::IResult;
use crate::packets::Packet;
use crate::variable_length_integer::VariableLengthInteger;

pub fn parse_packet(input: Bytes) -> anyhow::Result<Packet>
{
    todo!()
}

fn parse_variable_length_integer(input: &str) -> IResult<&str, VariableLengthInteger>
{
    todo!()
    // map_res(
    //     // Get some bytes or something
    //     // Parse them into a variable length integer
    // ).parse(input)
}
