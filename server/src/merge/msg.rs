#![allow(non_snake_case)]

use crate::database::{MessageType, WaContact, WaImgInfo, WaMessage, WaUserInfo};
use rbatis::crud::CRUD;

#[crud_table(table_name:userinfo)]
#[derive(Clone, Debug)]
pub struct MsgUserInfo {
    pub id: i32,
    pub value: String,
}
impl_field_name_method!(MsgUserInfo { id, value });

impl MsgUserInfo {
    pub const ID_TYPE_USERNAME: i32 = 2;
    pub const ID_TYPE_ALIAS: i32 = 42;
    pub const ID_TYPE_NICKNAME: i32 = 4;
}

impl WaUserInfo {
    pub fn from_msg(msg_user_info_vec: Vec<MsgUserInfo>) -> Self {
        let mut username: String = "".to_string();
        let mut alias: Option<String> = None;
        let mut nickname: Option<String> = None;
        for info in msg_user_info_vec.iter() {
            match info.id {
                MsgUserInfo::ID_TYPE_USERNAME => {
                    username = info.value.clone();
                }
                MsgUserInfo::ID_TYPE_ALIAS => {
                    alias = Some(info.value.clone());
                }
                MsgUserInfo::ID_TYPE_NICKNAME => {
                    nickname = Some(info.value.clone());
                }
                _ => {}
            }
        }
        WaUserInfo {
            username,
            alias,
            nickname,
        }
    }
}

#[crud_table(table_name:rcontact)]
#[derive(Clone, Debug)]
pub struct MsgRContact {
    pub username: String,
    pub alias: Option<String>,
    pub conRemark: Option<String>,
    pub domainList: Option<String>,
    pub nickname: Option<String>,
    pub pyInitial: Option<String>,
    pub quanPin: Option<String>,
    pub showHead: Option<i32>,
    pub r#type: Option<i32>,
    pub weiboFlag: Option<i32>,
    pub weiboNickname: Option<String>,
    pub conRemarkPYFull: Option<String>,
    pub conRemarkPYShort: Option<String>,
    pub lvbuff: Option<rbatis::Bytes>,
    pub verifyFlag: Option<i32>,
    pub encryptUsername: Option<String>,
    pub chatroomFlag: Option<i32>,
    pub deleteFlag: Option<i32>,
    pub contactLabelIds: Option<String>,
    pub descWordingId: Option<String>,
    pub openImAppid: Option<String>,
    pub sourceExtInfo: Option<String>,
    pub ticket: Option<String>,
    pub usernameFlag: Option<i64>,
}

impl WaContact {
    pub fn from_msg(owner: &str, rcontact: &MsgRContact) -> Self {
        WaContact {
            wa_owner: owner.to_string(),
            username: rcontact.username.clone(),
            alias: rcontact.alias.clone(),
            con_remark: rcontact.conRemark.clone(),
            domain_list: rcontact.domainList.clone(),
            nickname: rcontact.nickname.clone(),
            py_initial: rcontact.pyInitial.clone(),
            quan_pin: rcontact.quanPin.clone(),
            show_head: rcontact.showHead.clone(),
            r#type: rcontact.r#type.clone(),
            weibo_flag: rcontact.weiboFlag.clone(),
            weibo_nickname: rcontact.weiboNickname.clone(),
            con_remark_py_full: rcontact.conRemarkPYFull.clone(),
            con_remark_py_short: rcontact.conRemarkPYShort.clone(),
            lvbuff: rcontact.lvbuff.clone(),
            verify_flag: rcontact.verifyFlag.clone(),
            encrypt_username: rcontact.encryptUsername.clone(),
            chatroom_flag: rcontact.chatroomFlag.clone(),
            delete_flag: rcontact.deleteFlag.clone(),
            contact_label_ids: rcontact.contactLabelIds.clone(),
            desc_wording_id: rcontact.descWordingId.clone(),
            open_im_appid: rcontact.openImAppid.clone(),
            source_ext_info: rcontact.sourceExtInfo.clone(),
            ticket: rcontact.ticket.clone(),
            username_flag: rcontact.usernameFlag.clone(),
        }
    }
}

#[crud_table(table_name:message)]
#[derive(Clone, Debug)]
pub struct MsgMessage {
    pub msgId: i32,
    pub msgSvrId: u64,
    pub r#type: MessageType,
    pub status: Option<i32>,
    pub isSend: i32,
    pub isShowTimer: Option<i32>,
    pub createTime: u64,
    pub talker: String,
    pub content: Option<String>,
    pub imgPath: Option<String>,
    pub reserved: Option<String>,
    pub lvbuffer: Option<rbatis::Bytes>,
    pub transContent: Option<String>,
    pub transBrandWording: Option<String>,
    pub talkerId: Option<i32>,
    pub bizClientMsgId: Option<String>,
    pub bizChatId: Option<i32>,
    pub bizChatUserId: Option<String>,
    pub msgSeq: Option<i32>,
    pub flag: Option<i32>,
    pub solitaireFoldInfo: Option<rbatis::Bytes>,
    pub historyId: Option<String>,
}
impl_field_name_method!(MsgMessage { msgSvrId });

impl WaMessage {
    pub fn from_msg(owner: &str, message: &MsgMessage) -> Self {
        WaMessage {
            wa_owner: owner.to_string(),
            id: None,
            msg_svr_id: message.msgSvrId,
            r#type: message.r#type,
            status: message.status.clone(),
            is_send: message.isSend,
            is_show_timer: message.isShowTimer,
            create_time: message.createTime,
            talker: message.talker.clone(),
            content: message.content.clone(),
            img_path: message.imgPath.clone(),
            reserved: message.reserved.clone(),
            lvbuffer: message.lvbuffer.clone(),
            trans_content: message.transContent.clone(),
            trans_brand_wording: message.transBrandWording.clone(),
            talker_id: message.talkerId,
            biz_client_msg_id: message.bizClientMsgId.clone(),
            biz_chat_id: message.bizChatId,
            biz_chat_user_id: message.bizChatUserId.clone(),
            msg_seq: message.msgSeq,
            flag: message.flag,
            solitaire_fold_info: message.solitaireFoldInfo.clone(),
            history_id: message.historyId.clone(),
        }
    }
}

#[crud_table(table_name:ImgInfo2)]
#[derive(Clone, Debug)]
pub struct MsgImgInfo {
    pub id: u32,
    pub msgSvrId: u64,
    pub offset: Option<i32>,
    pub totalLen: Option<i32>,
    pub bigImgPath: String,
    pub thumbImgPath: Option<String>,
    pub createtime: Option<i32>,
    pub msglocalid: Option<i32>,
    pub status: Option<i32>,
    pub nettimes: Option<i32>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<String>,
    pub reserved4: Option<String>,
    pub hashdthumb: Option<i32>,
    pub iscomplete: Option<i32>,
    pub origImgMD5: Option<String>,
    pub compressType: Option<i32>,
    pub midImgPath: Option<String>,
    pub forwardType: Option<i32>,
    pub hevcPath: Option<String>,
    pub sendImgType: Option<i32>,
}
impl_field_name_method!(MsgImgInfo { msgSvrId });

impl WaImgInfo {
    pub fn from_msg(img_info: &MsgImgInfo) -> Self {
        WaImgInfo {
            id: None,
            msg_svr_id: img_info.msgSvrId,
            offset: img_info.offset.clone(),
            total_len: img_info.totalLen.clone(),
            big_img_path: img_info.bigImgPath.clone(),
            thumb_img_path: img_info.thumbImgPath.clone(),
            create_time: img_info.createtime.clone(),
            msglocalid: img_info.msglocalid.clone(),
            status: img_info.status.clone(),
            nettimes: img_info.nettimes.clone(),
            reserved1: img_info.reserved1.clone(),
            reserved2: img_info.reserved2.clone(),
            reserved3: img_info.reserved3.clone(),
            reserved4: img_info.reserved4.clone(),
            hashdthumb: img_info.hashdthumb.clone(),
            iscomplete: img_info.iscomplete.clone(),
            orig_img_md5: img_info.origImgMD5.clone(),
            compress_type: img_info.compressType.clone(),
            mid_img_path: img_info.midImgPath.clone(),
            forward_type: img_info.forwardType.clone(),
            hevc_path: img_info.hevcPath.clone(),
            send_img_type: img_info.sendImgType.clone(),
        }
    }
}
