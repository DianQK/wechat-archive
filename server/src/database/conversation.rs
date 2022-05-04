#[crud_table]
#[derive(Clone, Debug)]
pub struct WaConversation {
    pub id: Option<u32>,
    pub owner: String,
    pub username: String,
    pub msg_count: u64,
    pub last_time: u64,
    pub content: Option<String>,
    pub msg_type: i32,
    pub digest: String,
    pub digest_user: Option<String>,
}
impl_field_name_method!(WaConversation {
    id,
    owner,
    username
});
