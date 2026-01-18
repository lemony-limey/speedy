use nom::bits::bits;
use nom::error::ErrorKind;
use nom::{IResult, Parser};
use crate::frame::{Frame, FrameType};
use crate::frame::ack::{Ack, AckRange, ECNCounts};
use crate::frame::connection_close::ConnectionClose;
use crate::frame::data_blocked::DataBlocked;
use crate::frame::max_data::MaxData;
use crate::frame::max_streams::MaxStreams;
use crate::frame::streams_blocked::StreamsBlocked;
use crate::parser::BitInput;
use crate::parser::variable_length_integer::parse_variable_length_integer;
use crate::variable_length_integer::{VariableLengthDecode, VariableLengthInteger};

pub(crate) fn parse_frame(input: BitInput) -> IResult<BitInput, Frame>
{
    // Frame type is always the first field, and is encoded as a variable length integer.
    let (input, VariableLengthInteger::EightBit(frame_type)) = parse_variable_length_integer(input)? else {
        // This should only fail if data runs out.
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
    };

    let frame_type = FrameType::u8_to_frame_type(frame_type.decoded_value());

    match frame_type
    {
        FrameType::Padding => Ok((input, Frame::Padding)),
        FrameType::Ping    => Ok((input, Frame::Ping)),
        FrameType::Ack => {
            let (input, frame) = parse_ack(input, frame_type)?;
            Ok((input, Frame::Ack(frame)))
        },
        FrameType::AckWithECN => {
            let (input, frame) = parse_ack(input, frame_type)?;
            Ok((input, Frame::AckWithECN(frame)))
        },
        FrameType::ResetStream => unimplemented!(),
        FrameType::StopSending => unimplemented!(),
        FrameType::Crypto => unimplemented!(),
        FrameType::NewToken => unimplemented!(),
        FrameType::StreamNoneSet => unimplemented!(),
        FrameType::StreamFin => unimplemented!(),
        FrameType::StreamLen => unimplemented!(),
        FrameType::StreamLenFin => unimplemented!(),
        FrameType::StreamOff => unimplemented!(),
        FrameType::StreamOffFin => unimplemented!(),
        FrameType::StreamOffLen => unimplemented!(),
        FrameType::StreamOffLenFin => unimplemented!(),
        FrameType::MaxData => {
            let (input, frame) = parse_max_data(input)?;
            Ok((input, Frame::MaxData(frame)))
        },
        FrameType::MaxStreamData => unimplemented!(),
        FrameType::MaxStreamsBidirectional => {
            let (input, frame) = parse_max_streams(input, frame_type)?;
            Ok((input, Frame::MaxStreamsBidirectional(frame)))
        },
        FrameType::MaxStreamsUnidirectional => {
            let (input, frame) = parse_max_streams(input, frame_type)?;
            Ok((input, Frame::MaxStreamsUnidirectional(frame)))
        },
        FrameType::DataBlocked => {
            let (input, frame) = parse_data_blocked(input)?;
            Ok((input, Frame::DataBlocked(frame)))
        },
        FrameType::StreamDataBlocked => unimplemented!(),
        FrameType::StreamsBlockedBidirectional => {
            let (input, frame) = parse_streams_blocked(input, frame_type)?;
            Ok((input, Frame::StreamsBlockedBidirectional(frame)))
        },
        FrameType::StreamsBlockedUnidirectional => {
            let (input, frame) = parse_streams_blocked(input, frame_type)?;
            Ok((input, Frame::StreamsBlockedBidirectional(frame)))
        },
        FrameType::NewConnectionID => unimplemented!(),
        FrameType::RetireConnectionID => unimplemented!(),
        FrameType::PathChallenge => unimplemented!(),
        FrameType::PathResponse => unimplemented!(),
        FrameType::ConnectionCloseSuccessOrQuicError => unimplemented!(),
        FrameType::ConnectionCloseApplicationError => unimplemented!(),
        FrameType::HandshakeDone => Ok((input, Frame::HandshakeDone)),
    }
}

pub(crate) fn parse_ack(input: BitInput, frame_type: FrameType) -> IResult<BitInput, Ack>
{
    let (input, largest_acknowledged) = parse_variable_length_integer(input)?;
    let (input, ack_delay) = parse_variable_length_integer(input)?;
    let (input, ack_range_count) = parse_variable_length_integer(input)?;
    let (input, first_ack_range) = parse_variable_length_integer(input)?;

    let ack_range_count_u64 = ack_range_count.into();

    // Parse as many ack ranges as specified by ack_range_count
    let mut ack_ranges = Vec::with_capacity(ack_range_count_u64 as usize);

    // Add all AckRanges to the Vec
    let mut input = input;  // Need it mutable
    for _ in 0..ack_range_count_u64
    {
        let (remaining_input, ack_range) = parse_ack_ranges(input)?;
        ack_ranges.push(ack_range);
        input = remaining_input;
    }
    let input = input;  // Don't need it mutable any more

    // Only parse ECN counts if type is AckWithECN
    let (input, ecn_counts) = if frame_type == FrameType::Ack {
        (input, None)
    } else {
        // Parse ECNCounts
        let (input, ecn_counts) = parse_ecn_counts(input)?;
        (input, Some(ecn_counts))
    };

    Ok((
        input,
        Ack::new(
            frame_type,
            largest_acknowledged,
            ack_delay,
            ack_range_count,
            first_ack_range,
            ack_ranges,
            ecn_counts,
        ),
    ))
}

fn parse_ack_ranges(input: BitInput) -> IResult<BitInput, AckRange>
{
    let (input, gap) = parse_variable_length_integer(input)?;
    let (input, ack_range_length) = parse_variable_length_integer(input)?;

    Ok((
        input,
        AckRange::new(gap, ack_range_length),
    ))
}

fn parse_ecn_counts(input: BitInput) -> IResult<BitInput, ECNCounts>
{
    let (input, ect0_count) = parse_variable_length_integer(input)?;
    let (input, ect1_count) = parse_variable_length_integer(input)?;
    let (input, ecn_ce_count) = parse_variable_length_integer(input)?;

    Ok((
        input,
        ECNCounts::new(
            ect0_count,
            ect1_count,
            ecn_ce_count,
        )
    ))
}

fn parse_connection_close(input: BitInput, frame_type: FrameType) -> IResult<BitInput, ConnectionClose>
{
    todo!()
}

fn parse_data_blocked(input: BitInput) -> IResult<BitInput, DataBlocked>
{
    let (input, maximum_data) = parse_variable_length_integer(input)?;

    Ok((
        input,
        DataBlocked::new(maximum_data),
    ))
}

fn parse_max_data(input: BitInput) -> IResult<BitInput, MaxData>
{
    let (input, maximum_data) = parse_variable_length_integer(input)?;

    Ok((
        input,
        MaxData::new(maximum_data),
    ))
}

/// The only thing in a MaxStreams frame other than the type is another variable length integer
/// representing the maximum number of streams.
fn parse_max_streams(input: BitInput, frame_type: FrameType) -> IResult<BitInput, MaxStreams>
{
    let (input, maximum_streams) = parse_variable_length_integer(input)?;

    Ok((
        input,
        MaxStreams::new(frame_type, maximum_streams),
    ))
}

fn parse_streams_blocked(input: BitInput, frame_type: FrameType) -> IResult<BitInput, StreamsBlocked>
{
    let (input, maximum_streams) = parse_variable_length_integer(input)?;

    Ok ((
        input,
        StreamsBlocked::new(frame_type, maximum_streams),
    ))
}
