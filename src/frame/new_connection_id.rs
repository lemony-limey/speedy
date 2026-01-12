use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct NewConnectionID
{
    frame_type:            VariableLengthInteger,
    sequence_number:       VariableLengthInteger,
    retire_prior_to:       VariableLengthInteger,
    length:                u8,
    connection_id:         Bytes,  // At most 20 bytes
    stateless_reset_token: u128,
}

impl NewConnectionID
{
    pub fn new(
        sequence_number:       VariableLengthInteger,
        retire_prior_to:       VariableLengthInteger,
        length:                u8,
        connection_id:         Bytes,  // At most 20 bytes
        stateless_reset_token: u128,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::NewConnectionID),
            sequence_number,
            retire_prior_to,
            length,
            connection_id,
            stateless_reset_token,
        }
    }
}
