use crate::{
    po::misc::{StageConfigType, StageType},
    vo,
};

#[derive(Clone, Debug)]
/// 游戏中的备注文案，一般来说是在一大段文案中的下划线，点一下会有介绍遮罩
/// 如 【反震】 的介绍是：由特定「存护」命途祝福造成的附加伤害。
/// 这里 `name = "反震"`，`desc = "由特定「存护」命途祝福造成的附加伤害。"`
pub struct ExtraEffectConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
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

#[derive(Clone)]
pub struct RewardData<'a> {
    pub id: u32,
    /// 奖励可能是物品、光锥、角色，后两者常出现于活动奖励
    pub item_ids: &'a [u32; 6],
    /// 数量
    pub counts: &'a [u32; 6],
    /// 不明，目前全部都是 1
    pub levels: &'a [u8; 6],
    /// 不明，目前全部都是 1
    pub ranks: &'a [u8; 6],
    /// 星琼
    pub hcoin: u16,
    pub is_special: bool,
}

impl std::fmt::Debug for RewardData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let write_flat_array = |fmt: &mut std::fmt::Formatter<'_>, values: &[u32; 6]| {
            fmt.write_char('[')?;
            if let Some(value) = values.first() {
                fmt.write_str(&value.to_string())?;
            }
            for value in values.iter().skip(1) {
                fmt.write_str(", ")?;
                fmt.write_str(&value.to_string())?;
            }
            fmt.write_char(']')
        };

        f.debug_struct("RewardData")
            .field("id", &self.id)
            .field_with("item_ids", |f| write_flat_array(f, self.item_ids))
            .field_with("counts", |f| write_flat_array(f, self.counts))
            .field("hcoin", &self.hcoin)
            .field("is_special", &self.is_special)
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScheduleData {
    pub id: u32,
    pub begin_time: chrono::DateTime<chrono::FixedOffset>,
    pub end_time: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Clone, Copy, Debug)]
pub struct ScheduleDataGlobal {
    pub id: u32,
    pub begin_time: chrono::DateTime<chrono::FixedOffset>,
    pub end_time: chrono::DateTime<chrono::FixedOffset>,
    pub global_end_time: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Clone, Debug)]
pub struct StageConfig<'a> {
    pub id: u32,
    pub r#type: StageType,
    pub name: &'a str,
    pub hard_level_group: u16,
    pub level: u8,
    pub stage_config_data: fnv::FnvHashMap<StageConfigType, &'a str>,
    pub monster_list: Vec<Vec<vo::monster::MonsterConfig<'a>>>,
    pub forbid_auto_battle: bool,
    pub release: bool,
    pub forbid_exit_battle: bool,
}
