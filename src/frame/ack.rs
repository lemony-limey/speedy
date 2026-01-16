use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct Ack
{
    frame_type:           FrameType,  // 0x02
    largest_acknowledged: VariableLengthInteger,
    ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
    ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
    first_ack_range:      VariableLengthInteger,
    // Length of Vec is given by ack_range_count field. Ends on a byte boundary.
    ack_ranges:           Option<Vec<AckRange>>,
}

impl Ack
{
    pub fn new(
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,
        ack_range_count:      VariableLengthInteger,
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,
    ) -> Self
    {
        Self {
            frame_type: FrameType::Ack,
            largest_acknowledged,
            ack_delay,
            ack_range_count,
            first_ack_range,
            ack_ranges,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AckRange
{
    gap:              VariableLengthInteger,
    ack_range_length: VariableLengthInteger,
}
