use crate::frame::{AckRange, FrameType};
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct AckWithECN
{
    frame_type:           VariableLengthInteger,  // 0x03
    largest_acknowledged: VariableLengthInteger,
    ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
    ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
    first_ack_range:      VariableLengthInteger,
    // Length of Vec is given by ack_range_count field. Ends on a byte boundary.
    ack_ranges:           Option<Vec<AckRange>>,
    ecn_counts:           ECNCounts,
}

impl AckWithECN
{
    pub fn new(
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,
        ack_range_count:      VariableLengthInteger,
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,
        ecn_counts:           ECNCounts,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::AckWithECN),
            largest_acknowledged,
            ack_delay,
            ack_range_count,
            first_ack_range,
            ack_ranges,
            ecn_counts,
        }
    }
}


/// As per RFC 9000, Section 19.3.2.
///
/// The ACK frame uses the least significant bit of the type value (that is, type 0x03) to indicate
/// ECN feedback and report receipt of QUIC packets with associated ECN codepoints of ECT(0),
/// ECT(1), or ECN-CE in the packet's IP header. ECN counts are only present when the ACK frame
/// type is 0x03.
///
/// ECN counts are maintained separately for each packet number space.
#[derive(Copy, Clone, Debug)]
pub struct ECNCounts
{
    // A variable-length integer representing the total number of packets received with the ECT(0)
    // codepoint in the packet number space of the ACK frame (RFC 9000, Section 19.3.2).
    ect0_count:   VariableLengthInteger,
    // A variable-length integer representing the total number of packets received with the ECT(1)
    // codepoint in the packet number space of the ACK frame (RFC 9000, Section 19.3.2).
    ect1_count:   VariableLengthInteger,
    // A variable-length integer representing the total number of packets received with the ECN-CE
    // codepoint in the packet number space of the ACK frame (RFC 9000, Section 19.3.2).
    ecn_ce_count: VariableLengthInteger,
}
