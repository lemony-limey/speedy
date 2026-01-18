use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct Ack
{
    // No ECN:   0x02
    // WIth ECN: 0x03
    frame_type:           FrameType,
    largest_acknowledged: VariableLengthInteger,
    ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
    ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
    first_ack_range:      VariableLengthInteger,
    // Length of Vec is given by ack_range_count field. Ends on a byte boundary.
    ack_ranges:           Vec<AckRange>,
    ecn_counts:           Option<ECNCounts>,
}

impl Ack
{
    pub fn new(
        frame_type:           FrameType,
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,
        ack_range_count:      VariableLengthInteger,
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Vec<AckRange>,
        ecn_counts:           Option<ECNCounts>,
    ) -> Self
    {
        // If FrameType indicates ECN is not to be used, ecn_counts must be None.
        // Similarly, if FrameType indicates ECN is used, ecn_counts must be Some.
        // This should never fail as only the parser should be able to access this.
        assert!((frame_type == FrameType::Ack && ecn_counts.is_none())
            || (frame_type == FrameType::AckWithECN && ecn_counts.is_some()));

        Self {
            frame_type,
            largest_acknowledged,
            ack_delay,
            ack_range_count,
            first_ack_range,
            ack_ranges,
            ecn_counts,
        }
    }
}


/// RFC 9000, Section 19.3.1
#[derive(Clone, Copy, Debug)]
pub struct AckRange
{
    /// A variable-length integer indicating the number of contiguous unacknowledged packets
    /// preceding the packet number one lower than the smallest in the preceding ACK Range.
    gap:              VariableLengthInteger,
    /// A variable-length integer indicating the number of contiguous acknowledged packets
    /// preceding the largest packet number, as determined by the preceding Gap.
    ack_range_length: VariableLengthInteger,
}

impl AckRange
{
    pub(crate) fn new(
        gap:              VariableLengthInteger,
        ack_range_length: VariableLengthInteger,
    ) -> Self
    {
        Self { gap, ack_range_length }
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
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
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

impl ECNCounts
{
    pub fn new(
        ect0_count:   VariableLengthInteger,
        ect1_count:   VariableLengthInteger,
        ecn_ce_count: VariableLengthInteger,
    ) -> Self
    {
        Self {
            ect0_count,
            ect1_count,
            ecn_ce_count,
        }
    }
}
