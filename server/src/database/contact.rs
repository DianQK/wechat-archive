#[crud_table]
#[derive(Clone, Debug)]
pub struct WaContact {
    pub id: Option<u32>,
    pub wa_owner: String,
    pub username: String,
    pub alias: Option<String>,      // username alias
    pub con_remark: Option<String>, // 备注
    pub domain_list: Option<String>,
    pub nickname: Option<String>, // 昵称
    pub wa_display_name: String,  // 展示的名称
    pub py_initial: Option<String>,
    pub quan_pin: Option<String>,
    pub show_head: Option<i32>,
    pub r#type: Option<i32>,
    pub weibo_flag: Option<i32>,
    pub weibo_nickname: Option<String>,
    pub con_remark_py_full: Option<String>,
    pub con_remark_py_short: Option<String>,
    pub lvbuff: Option<rbatis::Bytes>,
    pub verify_flag: Option<i32>,
    pub encrypt_username: Option<String>,
    pub chatroom_flag: Option<i32>,
    pub delete_flag: Option<i32>,
    pub contact_label_ids: Option<String>,
    pub desc_wording_id: Option<String>,
    pub open_im_appid: Option<String>,
    pub source_ext_info: Option<String>,
    pub ticket: Option<String>,
    pub username_flag: Option<i64>,
}

impl_field_name_method!(WaContact {
    id,
    wa_owner
    username,
    wa_display_name
});
