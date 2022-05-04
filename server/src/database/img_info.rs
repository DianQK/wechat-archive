#[crud_table]
#[derive(Clone, Debug)]

pub struct WaImgInfo {
    pub id: Option<u32>,
    pub msg_svr_id: u64,
    pub offset: Option<i32>,
    pub total_len: Option<i32>,
    pub big_img_path: String,
    pub thumb_img_path: Option<String>,
    pub create_time: Option<i32>,
    pub msglocalid: Option<i32>,
    pub status: Option<i32>,
    pub nettimes: Option<i32>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<String>,
    pub reserved4: Option<String>,
    pub hashdthumb: Option<i32>,
    pub iscomplete: Option<i32>,
    pub orig_img_md5: Option<String>,
    pub compress_type: Option<i32>,
    pub mid_img_path: Option<String>,
    pub forward_type: Option<i32>,
    pub hevc_path: Option<String>,
    pub send_img_type: Option<i32>,
}
impl_field_name_method!(WaImgInfo { msg_svr_id });
