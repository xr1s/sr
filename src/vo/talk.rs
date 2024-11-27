use crate::po::talk::VoiceType;

#[derive(Clone, Debug)]
pub struct TalkSentenceConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub text: &'a str,
    pub voice: Option<VoiceConfig>,
}

#[derive(Clone, Debug)]
pub struct VoiceConfig {
    pub id: u32,
    pub is_player_involved: bool,
    pub r#type: Option<VoiceType>,
}
