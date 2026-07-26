use crate::{error::AppError, net::packet::ServerPacket};

use sonettobuf::CmdId;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn send_raw_server_message<W: AsyncWrite + Unpin + Send>(
    socket: &mut W,
    cmd_id: CmdId,
    payload: Vec<u8>,
    result_code: i16,
    up_tag: u8,
    down_tag: u8,
) -> Result<(), AppError> {
    let packet = ServerPacket {
        cmd_id: cmd_id as i16,
        result_code: result_code as u16,
        up_tag,
        down_tag,
        data: payload,
    };

    socket.write_all(&packet.encode()).await?;
    Ok(())
}

pub fn encode_message<T: prost::Message>(msg: &T) -> Result<Vec<u8>, AppError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)
        .map_err(|e| AppError::Custom(e.to_string()))?;
    Ok(buf)
}
