use crate::{
    error::AppError,
    logic::activity,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::{push, task_events},
};
use database::db::game::tasks::TaskEvent;
use prost::Message;
use sonettobuf::{
    AcceptAct186SpBonusRequest, Act146EpisodeBonusRequest, Act160FinishMissionRequest,
    Act160GetInfoRequest, Act160UpdatePush, Act165GainMilestoneRewardRequest,
    Act165GenerateEndingRequest, Act165GetInfoRequest, Act165ModifyKeywordRequest,
    Act165RestartRequest, Act196GainRequest, Act197ExploreRequest, Act197RummageRequest,
    Act198GainRequest, Act199GainRequest, Act205FinishGameRequest, Act205GetGameInfoRequest,
    Act206ChooseDirectionRequest, Act206GetBonusRequest, Act208ReceiveBonusRequest,
    Act212BonusPush, Act212ReceiveBonusRequest, Act216TaskPush, Act218AcceptRewardRequest,
    Act218FinishGameRequest, Act221SelectRequest, Act221SummonRequest, Act228FlipGridRequest,
    Act228GetFinalBonusRequest, ActivityNewStageReadRequest, Answer154PuzzleRequest, CmdId,
    FinishAct125EpisodeRequest, FinishAct146EpisodeRequest, FinishAct216TaskRequest,
    Get101BonusRequest, Get101InfosRequest, Get101SpBonusRequest, Get104InfosRequest,
    Get123InfosRequest, Get153InfosRequest, Get154InfosRequest, Get158InfosRequest,
    Get166InfosRequest, Get199InfoRequest, Get217InfosRequest, Get218InfoRequest,
    GetAct125InfosRequest, GetAct146InfosRequest, GetAct172InfoRequest, GetAct186InfoRequest,
    GetAct186SpBonusInfoRequest, GetAct189InfoRequest, GetAct189OnceBonusRequest,
    GetAct208InfoRequest, GetAct209InfoRequest, GetAct212InfoRequest, GetAct216InfoRequest,
    GetAct216OnceBonusRequest, GetAct225InfoRequest, GetAct228InfoRequest, GetAct229InfoRequest,
    GetActivityInfosRequest, GetActivityInfosWithParamRequest, MarkActivity104StoryRequest,
    MarkEpisodeAfterStoryRequest, MarkPopSummaryRequest, MarkUnlockNewPhotoRedDotRequest,
    UnlockPermanentRequest,
};

fn default_activity_id_for_type(type_id: i32) -> Option<i32> {
    config::configs::get()
        .activity
        .iter()
        .filter(|activity| activity.type_id == type_id && is_open(activity.open_id))
        .map(|activity| activity.id)
        .max()
}

fn is_open(open_id: i32) -> bool {
    open_id == 0
        || config::configs::get()
            .open
            .get(open_id)
            .is_some_and(|open| open.is_online != 0)
}

pub async fn on_act1000_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.player()?;
    let msg = sonettobuf::Act1000GetInfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act1000GetInfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(1000)),
        account_bind_bonus: None,
    };

    ctx.send_reply(CmdId::Act1000GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act1001_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act1001GetInfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act1001GetInfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(1001)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Act1001GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_106_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get106InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get106InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(106)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get106InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_108_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get108InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get108InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(108)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get108InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act109_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct109InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct109InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(109)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct109InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_111_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get111InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get111InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(111)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get111InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_112_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get112InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get112InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(112)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get112InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act113_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct113InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct113InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(113)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct113InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_114_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get114InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get114InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(114)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get114InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act115_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct115InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct115InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(115)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct115InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_116_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get116InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get116InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(116)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get116InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act120_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct120InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct120InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(120)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct120InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_121_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    sonettobuf::Get121InfosRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::Get121InfosCmd,
        sonettobuf::Get121InfosReply::default(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_act122_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct122InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct122InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(122)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct122InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act124_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct124InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct124InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(124)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct124InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_126_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get126InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get126InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(126)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get126InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_128_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get128InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get128InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(128)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get128InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_129_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get129InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get129InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(129)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get129InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_130_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get130InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get130InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(130)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get130InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_131_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get131InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get131InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(131)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get131InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_132_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get132InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get132InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(132)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get132InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_133_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get133InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get133InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(133)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get133InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_134_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get134InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get134InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(134)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get134InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_139_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get139InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get139InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(139)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get139InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_140_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get140InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get140InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(140)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get140InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act142_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct142InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct142InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(142)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct142InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_144_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    sonettobuf::Get144InfosRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::Get144InfosCmd,
        sonettobuf::Get144InfosReply::default(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_145_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get145InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get145InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(145)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get145InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act147_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct147InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct147InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(147)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct147InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_148_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get148InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get148InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(148)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get148InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_149_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get149InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get149InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(149)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get149InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_152_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get152InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act152_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get152InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act152_accept_present(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = sonettobuf::Act152AcceptPresentRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .accept_act152_present(db, msg.activity_id, msg.present_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act152AcceptPresentCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_157_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get157InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get157InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(157)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get157InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_159_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get159InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get159InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(159)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get159InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act161_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act161GetInfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act161GetInfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(161)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Act161GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_163_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    sonettobuf::Get163InfosRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::Get163InfosCmd,
        sonettobuf::Get163InfosReply::default(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_act164_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct164InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct164InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(164)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct164InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act167_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct167InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct167InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(167)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct167InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_168_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get168InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get168InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(168)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get168InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_169_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get169InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get169InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(169)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get169InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_170_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    sonettobuf::Get170InfoRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::Get170InfoCmd,
        sonettobuf::Get170InfoReply::default(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_171_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get171InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get171InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(171)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get171InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act174_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct174InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct174InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(174)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct174InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act178_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct178InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct178InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(178)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct178InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_179_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get179InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get179InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(179)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get179InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_180_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get180InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get180InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(180)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get180InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_181_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get181InfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get181InfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(181)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get181InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act182_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct182InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct182InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(182)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct182InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act183_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act183GetInfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act183GetInfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(183)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Act183GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act184_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct184InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct184InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(184)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct184InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act185_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct185InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct185InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(185)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct185InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_187_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get187InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get187InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(187)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get187InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act188_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct188InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct188InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(188)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct188InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act190_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct190InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct190InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(190)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct190InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act191_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct191InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct191InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(191)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct191InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act192_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct192InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct192InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(192)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct192InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act194_get_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act194GetInfosRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act194GetInfosReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(194)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Act194GetInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_196_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get196InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act196_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get196InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act196_gain(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act196GainRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act196_gain(db, msg.activity_id, msg.id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act196GainCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_197_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get197InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act197_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get197InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act197_rummage(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act197RummageRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act197_rummage(db, msg.activity_id, msg.pool_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act197RummageCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_act197_explore(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act197ExploreRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let result = ctx
        .player_mut()?
        .activity
        .act197_explore(db, msg.activity_id, msg.r#type)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        result.rewards,
        result.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act197ExploreCmd, result.reply, 0, req.up_tag)
        .await
}

pub async fn on_act198_gain(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act198GainRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act198_gain(db, msg.activity_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act198GainCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_201_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get201InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Get201InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(201)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Get201InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act203_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct203InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct203InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(203)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct203InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act204_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct204InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct204InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(204)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct204InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act205_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act205GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act205_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act205GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act205_get_game_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act205GetGameInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act205_get_game_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act205GetGameInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act205_finish_game(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act205FinishGameRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act205_finish_game(
            db,
            msg.activity_id,
            msg.game_type,
            msg.game_info,
            msg.reward_id,
        )
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    task_events::notify(
        ctx,
        player_id,
        TaskEvent::Act205FinishGame {
            activity_id: claim.activity_id,
            game_type: claim.game_type,
            is_win: claim.is_win,
        },
    )
    .await?;

    ctx.send_reply(CmdId::Act205FinishGameCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_act206_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act206GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act206_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act206GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act206_choose_direction(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act206ChooseDirectionRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act206_choose_direction(db, msg.activity_id, msg.direction_id)
        .await?;

    ctx.send_reply(CmdId::Act206ChooseDirectionCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act206_get_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act206GetBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act206_get_bonus(db, msg.activity_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act206GetBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act210_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct210InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct210InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(210)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct210InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act211_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct211InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct211InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(211)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct211InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act215_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct215InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct215InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(215)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct215InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act220_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct220InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct220InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(220)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct220InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_221_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get221InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act221_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get221InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act221_summon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act221SummonRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act221_summon(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act221SummonCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act221_select(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act221SelectRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act221_select(db, msg.activity_id, msg.select)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act221SelectCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act223_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct223InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct223InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(223)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct223InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act224_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct224InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct224InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(224)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct224InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act226_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct226InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct226InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(226)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct226InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act231_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct231InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct231InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(231)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::GetAct231InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act235_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::GetAct235InfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::GetAct235InfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(235)),
        info: Some(sonettobuf::Act235Info {
            total_reward_count: Some(0),
            preparation_ids: Vec::new(),
            count_list: Vec::new(),
        }),
    };

    ctx.send_reply(CmdId::GetAct235InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act240_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act240GetInfoRequest::decode(&req.data[..])?;
    let reply = sonettobuf::Act240GetInfoReply {
        activity_id: msg
            .activity_id
            .or_else(|| default_activity_id_for_type(240)),
        ..Default::default()
    };

    ctx.send_reply(CmdId::Act240GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_136_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get136InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act136_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get136InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act136_select(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = sonettobuf::Act136SelectRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act136_select(db, msg.activity_id, msg.select_hero_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act136SelectCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_activity_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetActivityInfosRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = activity::activity_infos(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetActivityInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_activity_infos_with_param(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetActivityInfosWithParamRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply =
        activity::activity_infos_with_param(ctx.state.db, player_id, &msg.activity_ids).await?;

    ctx.send_reply(CmdId::GetActivityInfosWithParamCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_activity_new_stage_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ActivityNewStageReadRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = activity::activity_new_stage_read(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::ActivityNewStageReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_unlock_permanent(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = UnlockPermanentRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = activity::unlock_permanent(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::UnlockPermanentCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_unlock_new_photo_red_dot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkUnlockNewPhotoRedDotRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let (_, changed_info_ids) = ctx
        .player()?
        .red_dot
        .show(db, RedDotId::ActivityJieXiKaPhoto.id(), false)
        .await?;
    push::send_red_dot_push(
        ctx,
        RedDotId::ActivityJieXiKaPhoto.id(),
        changed_info_ids.clone(),
        true,
    )
    .await?;

    ctx.send_reply(
        CmdId::MarkUnlockNewPhotoRedDotCmd,
        sonettobuf::MarkUnlockNewPhotoRedDotReply {
            activity_id: msg.activity_id,
        },
        0,
        req.up_tag,
    )
    .await?;

    Ok(())
}

pub async fn on_get_101_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get101InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .get101_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get101InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_101_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Get101BonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get101_bonus(db, msg.activity_id, msg.id)
        .await?;

    if let Some(rewards) = &claim.rewards {
        push::send_item_change_push(
            ctx,
            player_id,
            rewards.item_ids.clone(),
            rewards.power_item_ids.clone(),
            rewards.insight_item_ids.clone(),
        )
        .await?;
        push::send_currency_change_push(ctx, player_id, rewards.currency_ids.clone()).await?;
        push::send_equip_update_push(ctx, player_id, rewards.equip_uids.clone()).await?;
        push::send_hero_update_push(ctx, player_id, rewards.hero_ids.clone()).await?;
        push::send_skin_gain_pushes(
            ctx,
            &rewards.skin_gains,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
        push::send_bp_score_update_pushes(ctx, &rewards.bp_scores).await?;
        push::send_material_change_push(
            ctx,
            claim.material_changes.clone(),
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    let activity_id = claim.reply.activity_id.unwrap_or_default();
    push::send_red_dot_value_push(
        ctx,
        RedDotId::ActivityNoviceTab.id(),
        vec![activity_id],
        false,
        i32::from(claim.has_claimable),
        0,
    )
    .await?;

    ctx.send_reply(CmdId::Get101BonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_101_sp_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Get101SpBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get101_sp_bonus(db, msg.activity_id, msg.id)
        .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        claim.rewards.item_ids.clone(),
        claim.rewards.power_item_ids.clone(),
        claim.rewards.insight_item_ids.clone(),
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, claim.rewards.currency_ids.clone()).await?;
    push::send_equip_update_push(ctx, player_id, claim.rewards.equip_uids.clone()).await?;
    push::send_hero_update_push(ctx, player_id, claim.rewards.hero_ids.clone()).await?;
    push::send_skin_gain_pushes(
        ctx,
        &claim.rewards.skin_gains,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &claim.rewards.bp_scores).await?;
    push::send_material_change_push(
        ctx,
        claim.material_changes.clone(),
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    let activity_id = claim.reply.activity_id.unwrap_or_default();
    push::send_red_dot_value_push(
        ctx,
        RedDotId::ActivityNoviceTab.id(),
        vec![activity_id],
        false,
        i32::from(claim.has_claimable),
        0,
    )
    .await?;

    ctx.send_reply(CmdId::Get101SpBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_104_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get104InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act104_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get104InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_episode_after_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkEpisodeAfterStoryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let layer = msg.layer.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_episode_after_story(db, activity_id, layer)
        .await?;

    ctx.send_reply(CmdId::MarkEpisodeAfterStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_activity104_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkActivity104StoryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_activity104_story(db, activity_id)
        .await?;

    ctx.send_reply(CmdId::MarkActivity104StoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_pop_summary(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkPopSummaryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_pop_summary(db, activity_id)
        .await?;

    ctx.send_reply(CmdId::MarkPopSummaryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act186_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct186InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act186_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct186InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act186_sp_bonus_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct186SpBonusInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .get_act186_sp_bonus_info(db, msg.activity_id, msg.act186_activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct186SpBonusInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_accept_act186_sp_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = AcceptAct186SpBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .accept_act186_sp_bonus(db, msg.activity_id, msg.act186_activity_id)
        .await?;

    ctx.send_reply(CmdId::AcceptAct186SpBonusCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act189_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct189InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act189_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct189InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act189_once_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetAct189OnceBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get_act189_once_bonus(db, msg.activity_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::GetAct189OnceBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_199_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get199InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act199_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get199InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act199_gain(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act199GainRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act199_gain(db, msg.activity_id, msg.hero_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act199GainCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act172_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct172InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act172_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct172InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act125_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct125InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act125_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct125InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act125_episode(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = FinishAct125EpisodeRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let episode_id = msg.episode_id.unwrap_or_default();
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act125_episode(db, msg.activity_id, msg.episode_id, msg.target_frequency)
        .await?;

    task_events::notify(ctx, player_id, TaskEvent::EpisodeFinish { episode_id }).await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        reply.rewards,
        reply.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    ctx.send_reply(CmdId::FinishAct125EpisodeCmd, reply.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act146_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct146InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act146_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct146InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act146_episode(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = FinishAct146EpisodeRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act146_episode(db, msg.activity_id, msg.episode_id)
        .await?;

    ctx.send_reply(CmdId::FinishAct146EpisodeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act146_episode_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act146EpisodeBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act146_episode_bonus(db, msg.activity_id, msg.episode_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act146EpisodeBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_123_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get123InfosRequest::decode(&req.data[..])?;
    let reply = activity::act123_infos(msg.activity_id);

    ctx.send_reply(CmdId::Get123InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_153_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get153InfosRequest::decode(&req.data[..])?;
    let reply = activity::act153_infos(msg.activity_id);

    ctx.send_reply(CmdId::Get153InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_154_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get154InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act154_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get154InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_answer154_puzzle(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Answer154PuzzleRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .answer154_puzzle(db, msg.activity_id, msg.puzzle_id, msg.option_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Answer154PuzzleCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_158_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get158InfosRequest::decode(&req.data[..])?;
    let reply = ctx.player()?.activity.act158_infos(msg.activity_id);

    ctx.send_reply(CmdId::Get158InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act160_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act160GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act160_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act160GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act160_finish_mission(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act160FinishMissionRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .finish_act160_mission(db, msg.activity_id, msg.id)
        .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        claim.rewards.item_ids.clone(),
        claim.rewards.power_item_ids.clone(),
        claim.rewards.insight_item_ids.clone(),
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, claim.rewards.currency_ids.clone()).await?;
    push::send_equip_update_push(ctx, player_id, claim.rewards.equip_uids.clone()).await?;
    push::send_hero_update_push(ctx, player_id, claim.rewards.hero_ids.clone()).await?;
    push::send_skin_gain_pushes(
        ctx,
        &claim.rewards.skin_gains,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &claim.rewards.bp_scores).await?;
    push::send_material_change_push(
        ctx,
        claim.material_changes.clone(),
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    for act160_info in claim.updates.iter().skip(1) {
        ctx.notify(
            CmdId::Act160UpdatePushCmd,
            Act160UpdatePush {
                activity_id: claim.reply.activity_id,
                act160_info: Some(*act160_info),
            },
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act160FinishMissionCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act165GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_modify_keyword(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165ModifyKeywordRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_modify_keyword(db, msg.activity_id, msg.story_id, msg.keyword_ids)
        .await?;

    ctx.send_reply(CmdId::Act165ModifyKeywordCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_generate_ending(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165GenerateEndingRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_generate_ending(db, msg.activity_id, msg.story_id)
        .await?;

    ctx.send_reply(CmdId::Act165GenerateEndingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_restart(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165RestartRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_restart(db, msg.activity_id, msg.story_id, msg.step_id)
        .await?;

    ctx.send_reply(CmdId::Act165RestartCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_gain_milestone_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act165GainMilestoneRewardRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act165_gain_milestone_reward(db, msg.activity_id, msg.story_id)
        .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        claim.rewards.item_ids.clone(),
        claim.rewards.power_item_ids.clone(),
        claim.rewards.insight_item_ids.clone(),
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, claim.rewards.currency_ids.clone()).await?;
    push::send_equip_update_push(ctx, player_id, claim.rewards.equip_uids.clone()).await?;
    push::send_hero_update_push(ctx, player_id, claim.rewards.hero_ids.clone()).await?;
    push::send_skin_gain_pushes(
        ctx,
        &claim.rewards.skin_gains,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &claim.rewards.bp_scores).await?;
    push::send_material_change_push(
        ctx,
        claim.material_changes.clone(),
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(
        CmdId::Act165GainMilestoneRewardCmd,
        claim.reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_166_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get166InfosRequest::decode(&req.data[..])?;
    let reply = ctx.player()?.activity.act166_infos(msg.activity_id);

    ctx.send_reply(CmdId::Get166InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act208_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct208InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act208_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct208InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act208_receive_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act208ReceiveBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .receive_act208_bonus(db, msg.activity_id, msg.id)
        .await?;

    if let Some(rewards) = &claim.rewards {
        push::send_item_change_push(
            ctx,
            player_id,
            rewards.item_ids.clone(),
            rewards.power_item_ids.clone(),
            rewards.insight_item_ids.clone(),
        )
        .await?;
        push::send_currency_change_push(ctx, player_id, rewards.currency_ids.clone()).await?;
        push::send_equip_update_push(ctx, player_id, rewards.equip_uids.clone()).await?;
        push::send_hero_update_push(ctx, player_id, rewards.hero_ids.clone()).await?;
        push::send_skin_gain_pushes(
            ctx,
            &rewards.skin_gains,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
        push::send_bp_score_update_pushes(ctx, &rewards.bp_scores).await?;
        push::send_material_change_push(
            ctx,
            claim.material_changes.clone(),
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act208ReceiveBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act209_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct209InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act209_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct209InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_217_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get217InfosRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = activity::act217_infos(ctx.state.db, player_id, msg.activity_id).await?;

    ctx.send_reply(CmdId::Get217InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act216_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct216InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act216_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct216InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act216_task(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = FinishAct216TaskRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .finish_act216_task(db, msg.activity_id, msg.task_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }
    ctx.notify(
        CmdId::Act216TaskPushCmd,
        Act216TaskPush {
            activity_id: claim.reply.activity_id,
            act216_tasks: vec![claim.task_info],
            delete_tasks: Vec::new(),
        },
    )
    .await?;

    ctx.send_reply(CmdId::FinishAct216TaskCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act216_once_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetAct216OnceBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get_act216_once_bonus(db, msg.activity_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::GetAct216OnceBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act225_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct225InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act225_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct225InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_218_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get218InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act218_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get218InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act218_finish_game(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act218FinishGameRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act218_game(db, msg.activity_id, msg.result, msg.game_record)
        .await?;

    ctx.send_reply(CmdId::Act218FinishGameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act218_accept_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act218AcceptRewardRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .accept_act218_reward(db, msg.activity_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act218AcceptRewardCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act212_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct212InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act212_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct212InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act212_receive_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act212ReceiveBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .receive_act212_bonus(db, msg.activity_id, msg.id)
        .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        claim.rewards.item_ids.clone(),
        claim.rewards.power_item_ids.clone(),
        claim.rewards.insight_item_ids.clone(),
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, claim.rewards.currency_ids.clone()).await?;
    push::send_equip_update_push(ctx, player_id, claim.rewards.equip_uids.clone()).await?;
    push::send_hero_update_push(ctx, player_id, claim.rewards.hero_ids.clone()).await?;
    push::send_skin_gain_pushes(
        ctx,
        &claim.rewards.skin_gains,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &claim.rewards.bp_scores).await?;
    push::send_material_change_push(
        ctx,
        claim.material_changes.clone(),
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    let info = ctx
        .player_mut()?
        .activity
        .act212_info(db, claim.reply.activity_id)
        .await?
        .act212_info;
    ctx.notify(
        CmdId::Act212BonusPushCmd,
        Act212BonusPush { act212_info: info },
    )
    .await?;

    ctx.send_reply(CmdId::Act212ReceiveBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act228_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct228InfoRequest::decode(&req.data[..])?;
    let reply = activity::act228_info(msg.activity_id);

    ctx.send_reply(CmdId::GetAct228InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act228_flip_grid(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act228FlipGridRequest::decode(&req.data[..])?;
    let reply = activity::act228_flip_grid(msg.activity_id);

    ctx.send_reply(CmdId::Act228FlipGridCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act228_get_final_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act228GetFinalBonusRequest::decode(&req.data[..])?;
    let reply = activity::act228_get_final_bonus(msg.activity_id);

    ctx.send_reply(CmdId::Act228GetFinalBonusCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act229_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetAct229InfoRequest::decode(&req.data[..])?;
    let reply = activity::act229_info(ctx.state.db, player_id, msg.activity_id).await?;

    ctx.send_reply(CmdId::GetAct229InfoCmd, reply, 0, req.up_tag)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_info_defaults_from_activity_table() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        assert!(default_activity_id_for_type(120).is_some());
    }
}
