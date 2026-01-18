use crate::packet::Header;
use crate::parser::BitInput;
use nom::IResult;


pub(crate) fn parse_long_header(input: BitInput) -> IResult<BitInput, Header>
{
    todo!()
}

pub(crate) fn parse_short_header(input: BitInput) -> IResult<BitInput, Header>
{
    todo!()
}
