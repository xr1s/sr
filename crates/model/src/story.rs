use std::num::NonZero;

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "PascalCase")]
pub struct Value<T> {
    pub value: T,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "$type")]
#[serde(deny_unknown_fields)]
pub struct RogueSimpleTalk {
    #[serde(rename = "TalkBgID")]
    pub talk_bg_id: Option<NonZero<u8>>,
    #[serde(rename = "TalkSentenceID")]
    pub talk_sentence_id: u32,
    pub text_speed: Option<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum OptionIconType {
    ChatContinueIcon,
    ChatMissionIcon,
    ChatOutIcon,
    RogueHeita,
    ShopIcon,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueOptionTalk {
    #[serde(rename = "TalkSentenceID")]
    pub talk_sentence_id: Option<NonZero<u32>>,
    #[serde(rename = "OptionTextMapID")]
    pub option_textmap_id: Option<NonZero<i32>>,
    pub option_icon_type: Option<OptionIconType>,
    #[serde(rename = "RogueOptionID")]
    pub rogue_option_id: Option<NonZero<u32>>,
    pub trigger_custom_string: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EmitterType {
    DefaultEmitter,
    LocalPlayer,
    NPC,
    Prop,
    TargetEvaluator,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "$type")]
#[serde(deny_unknown_fields)]
pub enum Task {
    #[serde(rename = "RPG.GameCore.PlayAndWaitRogueSimpleTalk")]
    #[serde(rename_all = "PascalCase")]
    PlayAndWaitRogueSimpleTalk {
        simple_talk_list: Vec<RogueSimpleTalk>,
    },
    #[serde(rename = "RPG.GameCore.PlayRogueOptionTalk", rename_all = "PascalCase")]
    PlayRogueOptionTalk { option_list: Vec<RogueOptionTalk> },
    #[serde(rename = "RPG.GameCore.SetAsRogueDialogue")]
    SetAsRogueDialogue,
    #[serde(rename = "RPG.GameCore.ShowRogueTalkUI", rename_all = "PascalCase")]
    ShowRogueTalkUI { show: bool },
    #[serde(rename = "RPG.GameCore.ShowRogueTalkBg")]
    ShowRogueTalkBg {
        #[serde(rename = "TalkBgID")]
        talk_bg_id: u32,
    },
    #[serde(rename = "RPG.GameCore.TriggerCustomString", rename_all = "PascalCase")]
    TriggerCustomString { custom_string: Value<String> },
    #[serde(rename = "RPG.GameCore.TriggerDialogueEvent")]
    TriggerDialogueEvent {
        #[serde(rename = "DialogueEventID")]
        dialogue_event_id: u8,
    },
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
    #[serde(rename = "RPG.GameCore.FinishLevelGraph", rename_all = "PascalCase")]
    FinishLevelGraph {
        #[serde(default)]
        make_owner_entity_die: bool,
    },
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
