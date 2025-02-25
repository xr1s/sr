use std::num::NonZero;

pub mod performance;
pub mod talk;
pub use self::{
    performance::{CaptureNPC, CreateCharacter, PerformanceType},
    talk::{
        OptionTalk, RogueOptionTalk, RogueSimpleTalk, SimpleTalk, SimpleTalkBackground,
        TargetBehavior,
    },
};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Value<T> {
    pub value: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EmitterType {
    DefaultEmitter,
    LocalPlayer,
    NPC,
    Prop,
    TargetEvaluator,
}

#[inline]
const fn true_value() -> bool {
    true
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "$type")]
#[serde(deny_unknown_fields)]
pub enum Task {
    #[serde(rename = "RPG.GameCore.EndPerformance", rename_all = "PascalCase")]
    EndPerformance,
    #[serde(rename = "RPG.GameCore.FinishLevelGraph", rename_all = "PascalCase")]
    FinishLevelGraph {
        #[serde(default)]
        make_owner_entity_die: bool,
    },
    #[serde(rename = "RPG.GameCore.LevelPerformanceInitialize")]
    #[serde(rename_all = "PascalCase")]
    LevelPerformanceInitialize {
        create_character_list: Vec<CreateCharacter>,
        #[serde(rename = "CaptureNPCList")]
        capture_npc_list: Vec<CaptureNPC>,
        performance_type: PerformanceType,
        use_new_streaming_source_type: bool,
    },
    #[serde(rename = "RPG.GameCore.PlayAndWaitRogueSimpleTalk")]
    #[serde(rename_all = "PascalCase")]
    PlayAndWaitRogueSimpleTalk {
        simple_talk_list: Vec<RogueSimpleTalk>,
    },
    #[serde(rename = "RPG.GameCore.PlayAndWaitSimpleTalk")]
    #[serde(rename_all = "PascalCase")]
    PlayAndWaitSimpleTalk {
        backgrounds: Option<Vec<SimpleTalkBackground>>,
        #[serde(default)]
        black_mask: bool,
        #[serde(default = "true_value")]
        keep_display: bool,
        simple_talk_list: Vec<SimpleTalk>,
        #[serde(default)]
        need_fade_black_mask: bool,
        #[serde(default)]
        skip_first_bg_fade_in: bool,
        target_behaviors: Option<Vec<TargetBehavior>>,
        #[serde(default)]
        use_background: bool,
        #[serde(default)]
        use_target_behavior: bool,
    },
    #[serde(rename = "RPG.GameCore.PlayOptionTalk", rename_all = "PascalCase")]
    PlayOptionTalk {
        #[serde(default)]
        hide_button_auto: bool,
        #[serde(default)]
        hide_selected: bool,
        option_list: Vec<OptionTalk>,
        trigger_string: Option<String>,
        #[serde(default)]
        trigger_string_when_all_selected: bool,
    },
    #[serde(rename = "RPG.GameCore.PlayRogueOptionTalk", rename_all = "PascalCase")]
    PlayRogueOptionTalk { option_list: Vec<RogueOptionTalk> },
    #[serde(rename = "RPG.GameCore.SetAsRogueDialogue")]
    SetAsRogueDialogue,
    #[serde(rename = "RPG.GameCore.ShowRogueTalkBg")]
    ShowRogueTalkBg {
        #[serde(rename = "TalkBgID")]
        talk_bg_id: u32,
    },
    #[serde(rename = "RPG.GameCore.ShowRogueTalkUI", rename_all = "PascalCase")]
    ShowRogueTalkUI { show: bool },
    #[serde(rename = "RPG.GameCore.TriggerSound", rename_all = "PascalCase")]
    TriggerCustomSound {
        emitter_type: Option<EmitterType>,
        #[serde(rename = "GroupID")]
        group_id: Option<NonZero<u16>>,
        #[serde(rename = "ID")]
        id: Option<NonZero<u16>>,
        sound_name: Value<String>,
        target_type: (),
        task_enabled: (),
        unique_name: (),
    },
    #[serde(rename = "RPG.GameCore.TriggerCustomString", rename_all = "PascalCase")]
    TriggerCustomString { custom_string: Value<String> },
    #[serde(rename = "RPG.GameCore.TriggerDialogueEvent")]
    TriggerDialogueEvent {
        #[serde(rename = "DialogueEventID")]
        dialogue_event_id: u8,
    },
    #[serde(rename = "RPG.GameCore.TutorialTaskUnlock", rename_all = "PascalCase")]
    TutorialTaskUnlock { trigger_param: Value<String> },
    #[serde(rename = "RPG.GameCore.WaitCustomString", rename_all = "PascalCase")]
    WaitCustomString {
        custom_string: Value<String>,
        #[serde(default)]
        go_next_immediately: bool,
        #[serde(default)]
        reset_when_task_begin: bool,
        #[serde(default)]
        wait_owner_only: bool,
    },
    #[serde(rename = "RPG.GameCore.WaitPerformanceEnd")]
    WaitPerformanceEnd,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    #[serde(default)]
    pub is_loop: bool,
    pub order: Option<i8>,
    pub task_list: Vec<Task>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Story {
    pub on_start_sequece: Vec<Sequence>,
    pub on_init_sequece: Option<Vec<Sequence>>,
}
