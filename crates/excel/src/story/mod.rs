use std::num::NonZero;

use model::story::PerformanceType;

use crate::ExcelOutput;

pub mod talk;
pub use talk::{OptionTalk, RogueOptionTalk, RogueSimpleTalk, SimpleTalk};

#[derive(Clone, Debug)]
pub struct Story<'a> {
    pub on_start_sequence: Vec<Sequence<'a>>,
    pub on_init_sequence: Vec<Sequence<'a>>,
}

impl<'a> Story<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::Story) -> Self {
        Self {
            on_start_sequence: model
                .on_start_sequece
                .into_iter()
                .map(|seq| Sequence::from_model(game, seq))
                .collect(),
            on_init_sequence: model
                .on_init_sequece
                .map(|seq| {
                    seq.into_iter()
                        .map(|seq| Sequence::from_model(game, seq))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sequence<'a> {
    pub is_loop: bool,
    pub order: Option<i8>,
    pub task_list: Vec<Task<'a>>,
}

impl<'a> Sequence<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::Sequence) -> Self {
        Self {
            is_loop: model.is_loop,
            order: model.order,
            task_list: model
                .task_list
                .into_iter()
                .map(|task| Task::from_model(game, task))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Task<'a> {
    EndPerformance,
    FinishLevelGraph {
        make_owner_entity_die: bool,
    },
    LevelPerformanceInitialize {
        performance_type: PerformanceType,
        use_new_streaming_source_type: bool,
    },
    PlayAndWaitRogueSimpleTalk {
        simple_talk_list: Vec<RogueSimpleTalk<'a>>,
    },
    PlayAndWaitSimpleTalk {
        backgrounds: Vec<model::story::talk::SimpleTalkBackground>,
        black_mask: bool,
        keep_display: bool,
        simple_talk_list: Vec<SimpleTalk<'a>>,
        need_fade_black_mask: bool,
        skip_first_bg_fade_in: bool,
        target_behaviors: Vec<model::story::talk::TargetBehavior>,
        use_background: bool,
        use_target_behavior: bool,
    },
    PlayRogueOptionTalk {
        option_list: Vec<RogueOptionTalk<'a>>,
    },
    PlayOptionTalk {
        hide_button_auto: bool,
        hide_selected: bool,
        option_list: Vec<OptionTalk<'a>>,
        trigger_string: Option<String>,
        trigger_string_when_all_selected: bool,
    },
    SetAsRogueDialogue,
    ShowRogueTalkBg {
        talk_bg_id: u32,
    },
    ShowRogueTalkUI {
        show: bool,
    },
    TriggerCustomSound {
        emitter_type: Option<model::story::EmitterType>,
        group_id: u16,
        id: u16,
        sound_name: String,
        target_type: (),
        task_enabled: (),
        unique_name: (),
    },
    TriggerCustomString {
        custom_string: String,
    },
    TriggerDialogueEvent {
        dialogue_event_id: u8,
    },
    TutorialTaskUnlock {
        trigger_param: String,
    },
    WaitCustomString {
        custom_string: String,
        go_next_immediately: bool,
        reset_when_task_begin: bool,
        wait_owner_only: bool,
    },
    WaitPerformanceEnd,
}

impl<'a> Task<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::Task) -> Self {
        use model::story::Task;
        match model {
            Task::EndPerformance => Self::EndPerformance,
            Task::LevelPerformanceInitialize {
                performance_type,
                use_new_streaming_source_type,
                ..
            } => Self::LevelPerformanceInitialize {
                performance_type,
                use_new_streaming_source_type,
            },
            Task::PlayAndWaitRogueSimpleTalk { simple_talk_list } => {
                Self::PlayAndWaitRogueSimpleTalk {
                    simple_talk_list: simple_talk_list
                        .into_iter()
                        .map(|talk| RogueSimpleTalk::from_model(game, talk))
                        .collect(),
                }
            }
            Task::PlayAndWaitSimpleTalk {
                backgrounds,
                black_mask,
                keep_display,
                simple_talk_list,
                need_fade_black_mask,
                skip_first_bg_fade_in,
                target_behaviors,
                use_background,
                use_target_behavior,
            } => Self::PlayAndWaitSimpleTalk {
                backgrounds: backgrounds.unwrap_or_default(),
                black_mask,
                keep_display,
                need_fade_black_mask,
                simple_talk_list: simple_talk_list
                    .into_iter()
                    .map(|talk| SimpleTalk::from_model(game, talk))
                    .collect(),
                skip_first_bg_fade_in,
                target_behaviors: target_behaviors.unwrap_or_default(),
                use_background,
                use_target_behavior,
            },
            Task::PlayRogueOptionTalk { option_list } => Self::PlayRogueOptionTalk {
                option_list: option_list
                    .into_iter()
                    .map(|option| RogueOptionTalk::from_model(game, option))
                    .collect(),
            },
            Task::PlayOptionTalk {
                hide_button_auto,
                hide_selected,
                option_list,
                trigger_string,
                trigger_string_when_all_selected,
            } => Self::PlayOptionTalk {
                hide_button_auto,
                hide_selected,
                option_list: option_list
                    .into_iter()
                    .map(|option| OptionTalk::from_model(game, option))
                    .collect(),
                trigger_string,
                trigger_string_when_all_selected,
            },
            Task::SetAsRogueDialogue => Self::SetAsRogueDialogue,
            Task::ShowRogueTalkUI { show } => Self::ShowRogueTalkUI { show },
            Task::ShowRogueTalkBg { talk_bg_id } => Self::ShowRogueTalkBg { talk_bg_id },
            Task::TriggerCustomString { custom_string } => Self::TriggerCustomString {
                custom_string: custom_string.value,
            },
            Task::TriggerDialogueEvent { dialogue_event_id } => {
                Self::TriggerDialogueEvent { dialogue_event_id }
            }
            Task::TriggerCustomSound {
                emitter_type,
                group_id,
                id,
                sound_name,
                target_type,
                task_enabled,
                unique_name,
            } => Self::TriggerCustomSound {
                emitter_type,
                group_id: group_id.map(NonZero::get).unwrap_or_default(),
                id: id.map(NonZero::get).unwrap_or_default(),
                sound_name: sound_name.value,
                target_type,
                task_enabled,
                unique_name,
            },
            Task::TutorialTaskUnlock { trigger_param } => Self::TutorialTaskUnlock {
                trigger_param: trigger_param.value,
            },
            Task::WaitCustomString {
                custom_string,
                go_next_immediately,
                reset_when_task_begin,
                wait_owner_only,
            } => Self::WaitCustomString {
                custom_string: custom_string.value,
                go_next_immediately,
                reset_when_task_begin,
                wait_owner_only,
            },
            Task::WaitPerformanceEnd => Self::WaitPerformanceEnd,
            Task::FinishLevelGraph {
                make_owner_entity_die,
            } => Self::FinishLevelGraph {
                make_owner_entity_die,
            },
        }
    }
}
