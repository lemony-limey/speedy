use crate::frame::stream::{Fin, Len, Off, Stream};
use bytes::Bytes;
use nom::error::ErrorKind;
use nom::{IResult, Parser};
use nom::bits::bytes;
use nom::bytes::complete::take;
use nom::number::complete::be_u128;
use crate::frame::{Frame, FrameType};
use crate::frame::ack::{Ack, AckRange, ECNCounts};
use crate::frame::connection_close::ConnectionClose;
use crate::frame::crypto::Crypto;
use crate::frame::data_blocked::DataBlocked;
use crate::frame::max_data::MaxData;
use crate::frame::max_stream_data::MaxStreamData;
use crate::frame::max_streams::MaxStreams;
use crate::frame::new_connection_id::NewConnectionID;
use crate::frame::new_token::NewToken;
use crate::frame::path_challenge::PathChallenge;
use crate::frame::path_response::PathResponse;
use crate::frame::reset_stream::ResetStream;
use crate::frame::retire_connection_id::RetireConnectionID;
use crate::frame::stop_sending::StopSending;
use crate::frame::stream_data_blocked::StreamDataBlocked;
use crate::frame::streams_blocked::StreamsBlocked;
use crate::parser::BitInput;
use crate::parser::bits::{take_bit_bool, take_bits_u64, take_bits_u8};
use crate::parser::variable_length_integer::parse_variable_length_integer;
use crate::stream::StreamID;
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
        FrameType::ResetStream => {
            let (input, frame) = parse_reset_stream(input)?;
            Ok((input, Frame::ResetStream(frame)))
        },
        FrameType::StopSending => {
            let (input, frame) = parse_stop_sending(input)?;
            Ok((input, Frame::StopSending(frame)))
        },
        FrameType::Crypto => unimplemented!(),
        FrameType::NewToken => {
            let (input, frame) = parse_new_token(input)?;
            Ok((input, Frame::NewToken(frame)))
        },
        FrameType::StreamNoneSet      | FrameType::StreamFin
            | FrameType::StreamLen    | FrameType::StreamLenFin
            | FrameType::StreamOff    | FrameType::StreamOffFin
            | FrameType::StreamOffLen | FrameType::StreamOffLenFin => {
            let (input, frame) = parse_stream(input)?;
            Ok((input, Frame::Stream(frame)))
        },
        FrameType::MaxData => {
            let (input, frame) = parse_max_data(input)?;
            Ok((input, Frame::MaxData(frame)))
        },
        FrameType::MaxStreamData => {
            let (input, frame) = parse_max_stream_data(input)?;
            Ok((input, Frame::MaxStreamData(frame)))
        },
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
        FrameType::StreamDataBlocked => {
            let (input, frame) = parse_stream_data_blocked(input)?;
            Ok((input, Frame::StreamDataBlocked(frame)))
        },
        FrameType::StreamsBlockedBidirectional => {
            let (input, frame) = parse_streams_blocked(input, frame_type)?;
            Ok((input, Frame::StreamsBlockedBidirectional(frame)))
        },
        FrameType::StreamsBlockedUnidirectional => {
            let (input, frame) = parse_streams_blocked(input, frame_type)?;
            Ok((input, Frame::StreamsBlockedBidirectional(frame)))
        },
        FrameType::NewConnectionID => {
            let (input, frame) = parse_new_connection_id(input)?;
            Ok((input, Frame::NewConnectionID(frame)))
        },
        FrameType::RetireConnectionID => {
            let (input, frame) = parse_retire_connection_id(input)?;
            Ok((input, Frame::RetireConnectionID(frame)))
        },
        FrameType::PathChallenge => {
            let (input, frame) = parse_path_challenge(input)?;
            Ok((input, Frame::PathChallenge(frame)))
        },
        FrameType::PathResponse => {
            let (input, frame) = parse_path_response(input)?;
            Ok((input, Frame::PathResponse(frame)))
        },
        FrameType::ConnectionCloseSuccessOrQuicError => {
            let (input, frame) = parse_connection_close(input, frame_type)?;
            Ok((input, Frame::ConnectionCloseSuccessOrQuicError(frame)))
        },
        FrameType::ConnectionCloseApplicationError => {
            let (input, frame) = parse_connection_close(input, frame_type)?;
            Ok((input, Frame::ConnectionCloseApplicationError(frame)))
        },
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

fn parse_bytes(input: BitInput, len: u64) -> IResult<BitInput, Bytes>
{
    let len = len as usize;

    let (input, token) = bytes(take::<usize, &[u8], nom::error::Error<_>>(len)).parse(input)?;
    let token = Bytes::copy_from_slice(token);

    Ok((input, token))
}

fn parse_crypto(input: BitInput) -> IResult<BitInput, Crypto>
{
    let (input, offset) = parse_variable_length_integer(input)?;
    let (input, length) = parse_variable_length_integer(input)?;
    let (input, data) = parse_bytes(input, length.into())?;

    Ok((
        input,
        Crypto::new(
            offset,
            length,
            data,
        )
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
    let (input, error_code) = parse_variable_length_integer(input)?;

    let (input, error_frame_type) = if frame_type == FrameType::ConnectionCloseSuccessOrQuicError {
        let (input, error_frame_type) = parse_variable_length_integer(input)?;
        (input, Some(error_frame_type))
    } else {
        // Application error
        (input, None)
    };

    let (input, reason_phrase_length) = parse_variable_length_integer(input)?;
    let (input, reason_phrase) = parse_bytes(input, reason_phrase_length.into())?;

    Ok((
        input,
        ConnectionClose::new(
            frame_type,
            error_code,
            error_frame_type,
            reason_phrase_length,
            reason_phrase,
        )
    ))
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

fn parse_max_stream_data(input: BitInput) -> IResult<BitInput, MaxStreamData>
{
    let (input, stream_id) = parse_stream_id(input)?;
    let (input, maximum_stream_data) = parse_variable_length_integer(input)?;

    Ok((
        input,
        MaxStreamData::new(
            stream_id,
            maximum_stream_data,
        ),
    ))
}

fn parse_new_connection_id(input: BitInput) -> IResult<BitInput, NewConnectionID>
{
    let (input, sequence_number) = parse_variable_length_integer(input)?;
    let (input, retire_prior_to) = parse_variable_length_integer(input)?;

    let (input, length) = take_bits_u8(input, 8usize)?;
    let (input, connection_id) = parse_bytes(input, length.into())?;

    let (input, stateless_reset_token) = bytes(be_u128::<&[u8], nom::error::Error<_>>).parse(input)?;

    Ok((
        input,
        NewConnectionID::new(
            sequence_number,
            retire_prior_to,
            length,
            connection_id,
            stateless_reset_token,
        )
    ))
}

fn parse_new_token(input: BitInput) -> IResult<BitInput, NewToken>
{
    // token_length is the length of the token in bytes.
    let (input, token_length) = parse_variable_length_integer(input)?;
    let (input, token) = parse_bytes(input, token_length.into())?;

    Ok((
        input,
        NewToken::new(
            token_length,
            token,
        )
    ))
}

fn parse_path_challenge(input: BitInput) -> IResult<BitInput, PathChallenge>
{
    let (input, data) = take_bits_u64(input, 64usize)?;
    Ok((input, PathChallenge::new(data)))
}

fn parse_path_response(input: BitInput) -> IResult<BitInput, PathResponse>
{
    let (input, data) = take_bits_u64(input, 64usize)?;
    Ok((input, PathResponse::new(data)))
}

fn parse_reset_stream(input: BitInput) -> IResult<BitInput, ResetStream>
{
    let (input, stream_id) = parse_stream_id(input)?;
    let (input, application_protocol_error_code) = parse_variable_length_integer(input)?;
    let (input, final_size) = parse_variable_length_integer(input)?;

    Ok((
        input,
        ResetStream::new(
            stream_id,
            application_protocol_error_code,
            final_size,
        ),
    ))
}

fn parse_retire_connection_id(input: BitInput) -> IResult<BitInput, RetireConnectionID>
{
    let (input, sequence_number) = parse_variable_length_integer(input)?;

    Ok((
        input,
        RetireConnectionID::new(sequence_number),
    ))
}

fn parse_stop_sending(input: BitInput) -> IResult<BitInput, StopSending>
{
    let (input, stream_id) = parse_stream_id(input)?;
    let (input, application_protocol_error_code) = parse_variable_length_integer(input)?;

    Ok((
        input,
        StopSending::new(
            stream_id,
            application_protocol_error_code,
        ),
    ))
}

fn parse_stream(input: BitInput) -> IResult<BitInput, Stream>
{
    let (input, off) = take_bit_bool(input)?;
    let off = Off(off);
    let (input, len) = take_bit_bool(input)?;
    let len = Len(len);
    let (input, fin) = take_bit_bool(input)?;
    let fin = Fin(fin);

    let (input, stream_id) = parse_stream_id(input)?;

    // Offset field is only present if the OFF bit is set
    let (input, offset) = if off.0 {
        let (input, offset) = parse_variable_length_integer(input)?;
        (input, Some(offset))
    } else {
        (input, None)
    };

    // Length field is only present if the LEN bit is set
    let (input, length, data) = if len.0 {
        let (input, length) = parse_variable_length_integer(input)?;
        let (input, data) = parse_bytes(input, length.into())?;
        (input, Some(length), data)
    } else {
        // If the LEN bit is set to 0, the data field consumes all remaining bytes in the packet.
        // let data = input.;
        let (input_bytes, _bit_position) = input;
        let data = Bytes::copy_from_slice(input_bytes);

        (("".as_bytes(), 0), None, data)
    };

    Ok((
        input,
        Stream::new(
            off,
            len,
            fin,
            stream_id,
            offset,
            length,
            data,
        )
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

fn parse_stream_data_blocked(input: BitInput) -> IResult<BitInput, StreamDataBlocked>
{
    let (input, stream_id) = parse_stream_id(input)?;
    let (input, maximum_stream_data) = parse_variable_length_integer(input)?;

    Ok((
        input,
        StreamDataBlocked::new(stream_id, maximum_stream_data),
    ))
}

fn parse_stream_id(input: BitInput) -> IResult<BitInput, StreamID>
{
    let (input, stream_id) = parse_variable_length_integer(input)?;
    Ok((input, StreamID(stream_id)))
}

#[cfg(test)]
mod tests
{

}
