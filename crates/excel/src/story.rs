use std::num::NonZero;

use crate::ExcelOutput;

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
pub struct RogueSimpleTalk<'a> {
    pub bg_id: u8,
    pub sentence: crate::talk::TalkSentenceConfig<'a>,
    pub text_speed: Option<u8>,
}

impl<'a> RogueSimpleTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(
        game: &'a Data,
        model: model::story::RogueSimpleTalk,
    ) -> Self {
        Self {
            bg_id: model.talk_bg_id.map(NonZero::get).unwrap_or_default(),
            sentence: game.talk_sentence_config(model.talk_sentence_id).unwrap(),
            text_speed: model.text_speed,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RogueOptionTalk<'a> {
    pub sentence: Option<crate::talk::TalkSentenceConfig<'a>>,
    pub option: &'a str,
    pub option_icon_type: Option<model::story::OptionIconType>,
    pub rogue_option_id: u32,
    pub trigger_custom_string: String,
    pub trigger_custom_string_talk: Option<crate::talk::TalkSentenceConfig<'a>>,
}

impl<'a> RogueOptionTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(
        game: &'a Data,
        model: model::story::RogueOptionTalk,
    ) -> Self {
        Self {
            sentence: model
                .talk_sentence_id
                .map(NonZero::get)
                .map(|id| game.talk_sentence_config(id))
                .map(Option::unwrap),
            option: model
                .option_textmap_id
                .map(NonZero::get)
                .map(|hash| model::Text { hash })
                .map(|text| game.text(text))
                .unwrap_or_default(),
            option_icon_type: model.option_icon_type,
            rogue_option_id: model.rogue_option_id.map(NonZero::get).unwrap_or_default(),
            trigger_custom_string_talk: if model.trigger_custom_string.starts_with("TalkSentence_")
            {
                let id = model
                    .trigger_custom_string
                    .split('_')
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                Some(game.talk_sentence_config(id).unwrap())
            } else {
                None
            },
            trigger_custom_string: model.trigger_custom_string,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Task<'a> {
    PlayAndWaitRogueSimpleTalk {
        simple_talk_list: Vec<RogueSimpleTalk<'a>>,
    },
    PlayRogueOptionTalk {
        option_list: Vec<RogueOptionTalk<'a>>,
    },
    SetAsRogueDialogue,
    ShowRogueTalkUI {
        show: bool,
    },
    ShowRogueTalkBg {
        talk_bg_id: u32,
    },
    TriggerCustomString {
        custom_string: String,
    },
    TriggerDialogueEvent {
        dialogue_event_id: u8,
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
    FinishLevelGraph {
        make_owner_entity_die: bool,
    },
}

impl<'a> Task<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::Task) -> Self {
        use model::story::Task;
        match model {
            Task::PlayAndWaitRogueSimpleTalk { simple_talk_list } => {
                Self::PlayAndWaitRogueSimpleTalk {
                    simple_talk_list: simple_talk_list
                        .into_iter()
                        .map(|talk| RogueSimpleTalk::from_model(game, talk))
                        .collect(),
                }
            }
            Task::PlayRogueOptionTalk { option_list } => Self::PlayRogueOptionTalk {
                option_list: option_list
                    .into_iter()
                    .map(|option| RogueOptionTalk::from_model(game, option))
                    .collect(),
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
