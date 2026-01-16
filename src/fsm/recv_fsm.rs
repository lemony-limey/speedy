pub(crate) enum RecvStreamState
{
    Recv,
    SizeKnown,
    DataRecvd,
    DataRead,
    ResetRecvd,
    ResetRead,
}

pub(crate) enum RecvStreamAction
{
    ReceiveResetStreamFrame,
    ReceiveStreamAndFinFrames,
    ReceiveAllData,
    ApplicationReadsAllData,
    ApplicationReadsReset,
}
pub(crate) struct RecvFSM
{
    current_state: RecvStreamState,
}

impl RecvFSM
{
    pub(crate) fn new() -> Self
    {
        Self { current_state: RecvStreamState::Recv }
    }
}
