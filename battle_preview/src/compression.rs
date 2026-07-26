use std::io::{Cursor, Read};

use anyhow::{Context, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::GzDecoder;
use prost::Message;
use sonettobuf::FightStep;

pub fn expand_compressed_fight_steps(value: &mut serde_json::Value) -> anyhow::Result<()> {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(encoded) = map
                .get("fightStepBytes")
                .and_then(serde_json::Value::as_str)
                .filter(|encoded| !encoded.is_empty())
            {
                let compressed = STANDARD
                    .decode(encoded)
                    .context("fightStepBytes is not valid base64")?;
                let mut framed = Vec::new();
                GzDecoder::new(compressed.as_slice())
                    .read_to_end(&mut framed)
                    .context("fightStepBytes is not valid gzip")?;
                map.insert(
                    "fightStep".to_owned(),
                    serde_json::to_value(decode_fight_steps(&framed)?)?,
                );
                map.remove("fightStepBytes");
                map.remove("totalStep");
            }
            for value in map.values_mut() {
                expand_compressed_fight_steps(value)?;
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                expand_compressed_fight_steps(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn decode_fight_steps(framed: &[u8]) -> anyhow::Result<Vec<FightStep>> {
    let mut bytes = Cursor::new(framed);
    let count = read_u32(&mut bytes).context("fightStepBytes has no step count")?;
    let mut steps = Vec::with_capacity(count as usize);
    for index in 0..count {
        let len = read_u32(&mut bytes)
            .with_context(|| format!("fightStepBytes has no length for step {index}"))?
            as usize;
        let start = bytes.position() as usize;
        let end = start
            .checked_add(len)
            .filter(|end| *end <= framed.len())
            .with_context(|| format!("fightStepBytes step {index} exceeds its frame"))?;
        steps.push(
            FightStep::decode(&framed[start..end])
                .with_context(|| format!("fightStepBytes step {index} is invalid protobuf"))?,
        );
        bytes.set_position(end as u64);
    }
    if bytes.position() as usize != framed.len() {
        bail!("fightStepBytes contains trailing data");
    }
    Ok(steps)
}

fn read_u32(bytes: &mut Cursor<&[u8]>) -> Option<u32> {
    let mut value = [0; 4];
    bytes.read_exact(&mut value).ok()?;
    Some(u32::from_be_bytes(value))
}
