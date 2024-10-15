#[derive(Clone, Debug)]
pub struct ExtraEffect<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
    pub r#type: u8,
}
