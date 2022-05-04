#[crud_table]
#[derive(Clone, Debug)]
pub struct WaUserInfo {
    pub username: String,
    pub alias: Option<String>,
    pub nickname: Option<String>,
}
impl_field_name_method!(WaUserInfo { username });
