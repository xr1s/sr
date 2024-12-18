use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
/// 游戏中的备注文案，一般来说是在一大段文案中的下划线，点一下会有介绍遮罩
/// 如 【反震】 的介绍是：由特定「存护」命途祝福造成的附加伤害。
/// 这里 `name = "反震"`，`desc = "由特定「存护」命途祝福造成的附加伤害。"`
pub struct ExtraEffectConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ExtraEffectConfig<'a> {
    type Model = model::misc::ExtraEffectConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        let arguments = format::Argument::from_array(&model.desc_param_list);
        Self {
            id: model.extra_effect_id,
            name: game.text(model.extra_effect_name),
            desc: format::format(game.text(model.extra_effect_desc), &arguments),
        }
    }
}

#[derive(Clone, Debug)]
/// 增益（模拟宇宙各种祝福方程增益、逐光捡金增益等）
pub struct MazeBuff<'a> {
    pub id: u32,
    /// 初始等级
    pub lv: u8,
    pub lv_max: u8,
    /// 祝福名称
    pub name: &'a str,
    /// 祝福详细文案
    pub desc: String,
    /// 祝福简单文案
    pub simple_desc: String,
    pub desc_battle: String,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MazeBuff<'a> {
    type Model = model::misc::MazeBuff;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        let params = format::Argument::from_array(&model.param_list);
        Self {
            id: model.id,
            lv: model.lv,
            lv_max: model.lv_max,
            name: game.text(model.buff_name),
            desc: format::format(game.text(model.buff_desc), &params),
            simple_desc: format::format(game.text(model.buff_simple_desc), &params),
            desc_battle: format::format(game.text(model.buff_desc_battle), &params),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct RewardData<'a> {
    pub id: u32,
    /// 奖励可能是物品、光锥、角色，后两者常出现于活动奖励
    pub item_ids: &'a [u32; 6],
    /// 数量
    #[educe(Debug(method = flat_array))]
    pub counts: &'a [u32; 6],
    /// 不明，目前全部都是 1
    #[educe(Debug(method = flat_array))]
    pub levels: &'a [u8; 6],
    /// 不明，目前全部都是 1
    #[educe(Debug(method = flat_array))]
    pub ranks: &'a [u8; 6],
    /// 星琼
    pub hcoin: u16,
    pub is_special: bool,
}

fn flat_array<T: std::fmt::Debug, const N: usize>(
    values: &[T; N],
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    use std::fmt::{Debug, Write};
    f.write_char('[')?;
    if !values.is_empty() {
        <T as Debug>::fmt(&values[0], f)?;
        for value in values.iter().skip(1) {
            f.write_str(", ")?;
            <T as Debug>::fmt(value, f)?;
        }
    }
    f.write_char(']')?;
    Ok(())
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RewardData<'a> {
    type Model = model::misc::RewardData;
    fn from_model(_game: &Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.reward_id,
            item_ids: &model.item_ids,
            counts: &model.counts,
            levels: &model.levels,
            ranks: &model.ranks,
            hcoin: model.hcoin,
            is_special: model.is_special,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScheduleData {
    pub id: u32,
    pub begin_time: chrono::DateTime<chrono::FixedOffset>,
    pub end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for ScheduleData {
    type Model = model::misc::ScheduleData;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            begin_time: model.begin_time,
            end_time: model.end_time,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScheduleDataGlobal {
    pub id: u32,
    pub begin_time: chrono::DateTime<chrono::FixedOffset>,
    pub end_time: chrono::DateTime<chrono::FixedOffset>,
    pub global_end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for ScheduleDataGlobal {
    type Model = model::misc::ScheduleDataGlobal;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.schedule.id,
            begin_time: model.schedule.begin_time,
            end_time: model.schedule.end_time,
            global_end_time: model.global_end_time,
        }
    }
}
