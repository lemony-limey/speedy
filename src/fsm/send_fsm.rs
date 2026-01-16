pub(crate) enum SendStreamState
{
    Ready,
    Send,
    DataSent,
    DataRecvd,
    ResetSent,
    ResetRecvd,
}

pub(crate) enum SendStreamAction
{
    SendResetStreamFrame,
    SendStreamFrame,
    SendStreamDataBlockedFrame,
    SendStreamAndFinFrames,
    ReceiveAllAcks,
    ReceiveAck,
}

pub(crate) struct SendFSM
{
    current_state: SendStreamState,
}

impl SendFSM
{
    pub fn new() -> Self
    {
        Self { current_state: SendStreamState::Ready }
    }

    /// Takes an action and moves to the corresponding state.
    pub fn next_state(&mut self, action: SendStreamAction)
    {
        match action
        {
            SendStreamAction::SendResetStreamFrame => {}
            SendStreamAction::SendStreamFrame => {}
            SendStreamAction::SendStreamDataBlockedFrame => {}
            SendStreamAction::SendStreamAndFinFrames => {}
            SendStreamAction::ReceiveAllAcks => {}
            SendStreamAction::ReceiveAck => {}
        }
    }
}
