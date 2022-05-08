use crate::database::WaUserInfo;
use crate::{
    database::{WaContact, WaConversation, WaImgInfo, WaMessage},
    merge::msg::MsgImgInfo,
    RB,
};
use anyhow::Result;
use log::info;
use rbatis::{rbatis::Rbatis, Page, PageRequest};
use std::collections::HashMap;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use rbatis::crud::CRUD;

mod msg;
use msg::{MsgMessage, MsgRContact, MsgUserInfo};

pub struct MergeMicroMsg {
    source: PathBuf,
    dest: PathBuf,
    password_map: HashMap<String, String>,
}

impl MergeMicroMsg {
    pub fn new(source: &str, dest: &str, password_map: &HashMap<String, String>) -> Self {
        let source = Path::new(source).to_path_buf();
        let dest = Path::new(dest).to_path_buf();
        let password_map = password_map.clone();
        MergeMicroMsg {
            source,
            dest,
            password_map,
        }
    }

    pub async fn merge(&self) -> Result<()> {
        self.merge_emoji();
        self.merge_users_history().await
    }

    fn merge_emoji(&self) {
        let source_emoji = self.source.join("emoji");
        self.rsync_to_dest(source_emoji.as_path());
    }

    fn rsync_to_dest(&self, source: &Path) {
        info!("{} -> {}", source.display(), self.dest.display());
        Command::new("rsync")
            .arg("-r")
            .arg(source)
            .arg(self.dest.as_path())
            .arg("-v")
            .output()
            .expect(&format!("同步 {} 失败", source.display()));
    }

    async fn merge_users_history(&self) -> Result<()> {
        for entry in fs::read_dir(self.source.as_path())? {
            let entry = entry?;
            let path = entry.path();
            let msg_db_path = path.join("EnMicroMsg.db");
            let video_path = path.join("video"); // TODO: 如何处理 Android 外部数据？有哪些需要保存
            if msg_db_path.exists() {
                info!("path: {}", path.display());
                self.merge_user_database(msg_db_path.as_path()).await?;
                self.merge_user_files(path.as_path())?;
            } else if video_path.exists() {
                info!("path: {}", path.display());
                self.merge_user_files(path.as_path())?;
            }
        }
        Ok(())
    }

    async fn decrypt_db(&self, db_path: &Path) -> Result<Rbatis> {
        // 解密数据库
        let parent_path = db_path.parent().unwrap();
        let decrypted_db_path = parent_path.join("EnMicroMsg.decrypted.db");
        info!("db path: {}", decrypted_db_path.display());
        if decrypted_db_path.exists() {
            fs::remove_file(decrypted_db_path.as_path()).expect("删除历史解密 db 失败");
        }
        let md5_path_name = parent_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let password = self
            .password_map
            .get(&md5_path_name)
            .expect(&format!("没有设置 {} 路径的密码", &md5_path_name));
        let output = Command::new("sqlcipher")
            .arg(db_path)
            .arg(&format!("PRAGMA key = '{}';", password))
            .arg("PRAGMA cipher_use_hmac = OFF; PRAGMA cipher_page_size = 1024; PRAGMA kdf_iter = 4000; PRAGMA cipher_compatibility = 1;")
            .arg(&format!("ATTACH DATABASE '{}' AS db KEY ''; SELECT sqlcipher_export('db'); DETACH DATABASE db;", decrypted_db_path.display()))
            .output()
            .expect(&format!("同步用户数据失败"));
        info!("output: {:?}", output);
        let msg_rb = Rbatis::new();
        msg_rb
            .link(&format!("sqlite://{}", decrypted_db_path.display()))
            .await
            .unwrap();
        Ok(msg_rb)
    }

    async fn merge_contacts(&self, msg_rb: &Rbatis, owner: &str) -> Result<()> {
        // 保存联系人
        let msg_contacts: Vec<MsgRContact> = msg_rb.fetch_list().await.unwrap();
        for msg_contact in msg_contacts.iter() {
            let mut wa_contact = WaContact::from_msg(&owner, &msg_contact);
            let select_wrapper = RB
                .new_wrapper()
                .eq(WaContact::wa_owner(), owner)
                .eq(WaContact::username(), &wa_contact.username);
            let old_wa_contact: Option<WaContact> =
                RB.fetch_by_wrapper(select_wrapper).await.unwrap();
            match old_wa_contact {
                Some(old_wa_contact) => {
                    wa_contact.id = old_wa_contact.id;
                    RB.update_by_column(WaContact::id(), &wa_contact).await?;
                }
                None => {
                    RB.save(&wa_contact, &[]).await?;
                }
            }
        }
        Ok(())
    }

    async fn merge_messages(&self, msg_rb: &Rbatis, owner: &str) -> Result<()> {
        // 保存聊天消息
        // let msg_messages: Vec<MsgMessage> = msg_rb.fetch_list().await.unwrap();
        let mut page_request = PageRequest::new(1, 1000); //分页请求，页码，条数
                                                          // 忽略 msgSvrId 为 NULL 的数据，非正式聊天内容
        let sql_wrapper = msg_rb.new_wrapper().is_not_null(MsgMessage::msgSvrId());
        let mut msg_messages: Page<MsgMessage> = msg_rb
            .fetch_page_by_wrapper(sql_wrapper.clone(), &page_request)
            .await?;
        while !msg_messages.records.is_empty() {
            let svr_ids: Vec<u64> = msg_messages
                .records
                .iter()
                .map(|message| message.msgSvrId)
                .collect();
            let exist_messages: Vec<WaMessage> = RB
                .fetch_list_by_column(WaMessage::msg_svr_id(), &svr_ids)
                .await?;
            let exist_svr_ids: Vec<u64> = exist_messages.iter().map(|m| m.msg_svr_id).collect();

            let mut records = msg_messages.records;
            records.sort_by_key(|r| r.msgSvrId);
            records.dedup_by(|a, b| a.msgSvrId == b.msgSvrId); // 为什么会出现相同的 svg id？
            let messages: Vec<WaMessage> = records
                .iter()
                .filter(|message| !exist_svr_ids.contains(&message.msgSvrId))
                .map(|message| WaMessage::from_msg(&owner, &message))
                .collect();
            RB.save_batch(&messages, &[]).await?;
            page_request.page_no += 1;
            msg_messages = msg_rb
                .fetch_page_by_wrapper(sql_wrapper.clone(), &page_request)
                .await?;
        }
        Ok(())
    }

    async fn merge_img_infos(&self, msg_rb: &Rbatis, owner: &str) -> Result<()> {
        // 保存图片信息
        let mut page_request = PageRequest::new(1, 1000); //分页请求，页码，条数
                                                          // 不清楚 msgSvrId 为 NULL 的情况是什么图片
        let sql_wrapper = msg_rb.new_wrapper().is_not_null(MsgImgInfo::msgSvrId());
        let mut msg_img_infos: Page<MsgImgInfo> = msg_rb
            .fetch_page_by_wrapper(sql_wrapper.clone(), &page_request)
            .await?;
        while !msg_img_infos.records.is_empty() {
            let svr_ids: Vec<u64> = msg_img_infos
                .records
                .iter()
                .map(|img_info| img_info.msgSvrId)
                .collect();
            let exist_img_infos: Vec<WaImgInfo> = RB
                .fetch_list_by_column(WaImgInfo::msg_svr_id(), &svr_ids)
                .await?;
            let exist_svr_ids: Vec<u64> = exist_img_infos.iter().map(|m| m.msg_svr_id).collect();

            let mut records = msg_img_infos.records;
            records.sort_by_key(|r| r.msgSvrId);
            records.dedup_by(|a, b| a.msgSvrId == b.msgSvrId); // 为什么会出现相同的 svg id？
            let img_infos: Vec<WaImgInfo> = records
                .iter()
                .filter(|img_info| !exist_svr_ids.contains(&img_info.msgSvrId))
                .map(|img_info| WaImgInfo::from_msg(&img_info))
                .collect();
            RB.save_batch(&img_infos, &[]).await?;
            page_request.page_no += 1;
            msg_img_infos = msg_rb
                .fetch_page_by_wrapper(sql_wrapper.clone(), &page_request)
                .await?;
        }
        Ok(())
    }

    async fn merge_user_database(&self, db_path: &Path) -> Result<()> {
        let msg_rb = self.decrypt_db(db_path).await?;
        let result: Vec<MsgUserInfo> = msg_rb
            .fetch_list_by_column(
                MsgUserInfo::id(),
                &[
                    MsgUserInfo::ID_TYPE_USERNAME,
                    MsgUserInfo::ID_TYPE_ALIAS,
                    MsgUserInfo::ID_TYPE_NICKNAME,
                ],
            )
            .await
            .unwrap();
        let user_info = WaUserInfo::from_msg(result);
        let old_user_info: Option<WaUserInfo> = RB
            .fetch_by_column(WaUserInfo::username(), &user_info.username)
            .await
            .unwrap();
        info!("old_user_info {:?}", old_user_info);
        match old_user_info {
            Some(_) => {
                RB.update_by_column(WaUserInfo::username(), &user_info)
                    .await?;
            }
            None => {
                RB.save(&user_info, &[]).await?;
            }
        }
        let owner = user_info.username;
        self.merge_contacts(&msg_rb, &owner).await?;
        self.merge_messages(&msg_rb, &owner).await?;
        self.merge_img_infos(&msg_rb, &owner).await?;

        // 计算会话列表
        let latest_messages = WaMessage::select_latest_messages(&RB, &owner).await?;
        for message in latest_messages.iter() {
            let sql_wrapper = msg_rb
                .new_wrapper()
                .eq(WaConversation::owner(), &owner)
                .eq(WaConversation::username(), &message.talker);
            let conversation: Option<WaConversation> =
                RB.fetch_by_wrapper(sql_wrapper.clone()).await?;
            let sql_wrapper = msg_rb
                .new_wrapper()
                .eq(WaMessage::wa_owner(), &owner)
                .eq(WaMessage::talker(), &message.talker);
            let msg_count = RB.fetch_count_by_wrapper::<WaMessage>(sql_wrapper).await?;
            let digest_user = match message.is_owner_send() {
                true => "".to_string(),
                false => message.talker.to_string(),
            };

            match conversation {
                Some(conversation) => {
                    let mut conversation = conversation;
                    conversation.msg_count = msg_count;
                    conversation.last_time = message.create_time;
                    conversation.content = message.content.clone();
                    // conversation.msg_type = message.r#type;
                    conversation.digest = message.get_digest();
                    conversation.digest_user = Some(digest_user);
                    RB.update_by_column(WaConversation::id(), &conversation)
                        .await?;
                }
                None => {
                    let conversation = WaConversation {
                        id: None,
                        owner: owner.to_string(),
                        username: message.talker.clone(),
                        msg_count: msg_count,
                        last_time: message.create_time,
                        content: message.content.clone(),
                        // msg_type: message.r#type,
                        digest: message.get_digest(),
                        digest_user: Some(digest_user),
                    };
                    RB.save(&conversation, &[]).await?;
                }
            };
        }

        Ok(())
    }

    fn merge_user_files(&self, path: &Path) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let exclude_dirs = [OsStr::new("appbrand"), OsStr::new("oly")];
            if let Some(file_name) = path.file_name() {
                if exclude_dirs.contains(&file_name) {
                    continue;
                }
            }
            self.rsync_to_dest(path.as_path());
        }
        Ok(())
    }
}
