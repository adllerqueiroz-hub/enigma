use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::{push, task_events},
};
use logic::task::{ProductionLineAction, TaskEvent};
use prost::Message;
use sonettobuf::{
    AllotCritterRequest, AllotVehicleRequest, CmdId, CopyOtherRoomPlanRequest, DeleteRoadRequest,
    DeleteRoomPlanRequest, DeleteRoomShareRequest, GainProductionLineRequest, GenerateRoadRequest,
    GetBlockPermanentInfoRequest, GetOtherRoomObInfoRequest, GetRoomObInfoRequest,
    GetRoomPlanDetailsRequest, GetRoomShareRequest, GetRoomThemeCollectionBonusRequest,
    HideBlockPackageReddotRequest, HideBuildingReddotRequset, ProductionLineAccelerateRequest,
    ProductionLineInfoRequest, ProductionLineLvUpRequest, ReadRoomLogNewRequest,
    SetBlockColorRequest, SetRoomPlanCoverRequest, SetRoomPlanNameRequest, SetRoomPlanRequest,
    SetWaterTypeRequest, ShareRoomPlanRequest, StartProductionLineRequest, SwitchRoomPlanRequest,
    UnUseBlockRequest, UnUseBuildingRequest, UpdateOpenPush, UseBlockRequest, UseBuildingRequest,
    UseRoomPlanRequest, UseRoomShareRequest,
};

mod blocks;
mod hero;
mod info;
mod interaction;
mod layout;
mod plan;
mod production;
mod skin;
pub use blocks::*;
pub use hero::*;
pub use info::*;
pub use interaction::*;
pub use layout::*;
pub use plan::*;
pub use production::*;
pub use skin::*;
