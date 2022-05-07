CREATE TABLE IF NOT EXISTS `wa_user_info`(
   `username` VARCHAR(255) NOT NULL UNIQUE COMMENT '微信内部唯一 id',
   `alias` VARCHAR(255) DEFAULT NULL COMMENT '用户可以设置的微信号',
   `nickname` VARCHAR(255) DEFAULT NULL COMMENT '微信昵称',
   PRIMARY KEY ( `username` ),
   INDEX(`alias`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `wa_contact`( -- rcontact
   `wa_owner` VARCHAR(255) NOT NULL COMMENT '属于谁的好友',
   `username` VARCHAR(255) NOT NULL COMMENT '微信内部唯一 id',
   `alias` VARCHAR(255) DEFAULT NULL COMMENT '用户可以设置的微信号',
   `con_remark` VARCHAR(255) DEFAULT NULL,
   `domain_list` VARCHAR(255) DEFAULT NULL,
   `nickname` VARCHAR(255) DEFAULT NULL COMMENT '微信昵称',
   `py_initial` VARCHAR(255) DEFAULT NULL,
   `quan_pin` VARCHAR(255) DEFAULT NULL,
   `show_head` INTEGER DEFAULT 0,
   `type` INTEGER DEFAULT 0,
   `weibo_flag` INTEGER DEFAULT 0,
   `weibo_nickname` VARCHAR(255) DEFAULT NULL,
   `con_remark_py_full` VARCHAR(255) DEFAULT NULL,
   `con_remark_py_short` VARCHAR(255) DEFAULT NULL,
   `lvbuff` BLOB DEFAULT NULL,
   `verify_flag` INTEGER DEFAULT 0,
   `encrypt_username` TEXT DEFAULT NULL,
   `chatroom_flag` INTEGER DEFAULT NULL,
   `delete_flag` INTEGER DEFAULT 0,
   `contact_label_ids` VARCHAR(255) DEFAULT NULL,
   `desc_wording_id` VARCHAR(255) DEFAULT NULL,
   `open_im_appid` VARCHAR(255),
   `source_ext_info` VARCHAR(255),
   `ticket` VARCHAR(255) DEFAULT NULL,
   `username_flag` BIGINT DEFAULT 0,
   PRIMARY KEY ( `username` ),
   UNIQUE KEY `uniq_id` (`wa_owner`,`username`),
   INDEX(`wa_owner`, `alias`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `wa_message`( -- message
   `wa_owner` VARCHAR(255) NOT NULL COMMENT '属于谁的消息',
   `id` INT UNSIGNED AUTO_INCREMENT,
   -- msgId INTEGER, 微信本地数据内部记录 id？
   `msg_svr_id` BIGINT UNSIGNED NOT NULL UNIQUE COMMENT '服务端唯一 ID',
   `type` INT NOT NULL,
   `status` INT DEFAULT NULL,
   `is_send` INT NOT NULL,
   `is_show_timer` INTEGER DEFAULT NULL,
   `create_time` BIGINT UNSIGNED NOT NULL,
   `talker` VARCHAR(255) NOT NULL,
   `content` MEDIUMTEXT DEFAULT NULL,
   `img_path` TEXT DEFAULT NULL,
   `reserved` MEDIUMTEXT DEFAULT NULL, -- BLOB ???
   `lvbuffer` BLOB DEFAULT NULL,
   `trans_content` TEXT DEFAULT NULL,
   `trans_brand_wording` TEXT DEFAULT NULL,
   `talker_id` INTEGER DEFAULT NULL,
   `biz_client_msg_id` TEXT DEFAULT NULL,
   `biz_chat_id` INTEGER DEFAULT -1,
   `biz_chat_user_id` TEXT DEFAULT NULL,
   `msg_seq` INTEGER DEFAULT NULL,
   `flag` INT DEFAULT NULL,
   `solitaire_fold_info` BLOB DEFAULT NULL,
   `history_id` TEXT DEFAULT NULL,
   PRIMARY KEY ( `id` ),
   INDEX(`wa_owner`, `msg_svr_id`, `create_time`, `talker`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `wa_img_info`( -- ImgInfo2
   `id` INT UNSIGNED AUTO_INCREMENT,
   `msg_svr_id` BIGINT UNSIGNED NOT NULL UNIQUE, 
   `offset` INT DEFAULT NULL,
   `total_len` INT DEFAULT NULL, 
   `big_img_path` VARCHAR(255) NOT NULL, 
   `thumb_img_path` VARCHAR(255) DEFAULT NULL, 
   `create_time` INT DEFAULT NULL, 
   `msglocalid` INT DEFAULT NULL, 
   `status` INT DEFAULT NULL,
   `nettimes` INT DEFAULT NULL, 
   `reserved1` INT DEFAULT NULL, 
   `reserved2` INT DEFAULT NULL,
   `reserved3` TEXT DEFAULT NULL,
   `reserved4` TEXT DEFAULT NULL,
   `hashdthumb` INT DEFAULT 0, 
   `iscomplete` INT DEFAULT 1, 
   `orig_img_md5` VARCHAR(255) DEFAULT NULL, 
   `compress_type` INT DEFAULT 0, 
   `mid_img_path` TEXT DEFAULT NULL, 
   `forward_type` INT DEFAULT 0, 
   `hevc_path` TEXT DEFAULT NULL,
   `send_img_type` INT DEFAULT 0,
   PRIMARY KEY ( `id` ),
   INDEX(`msg_svr_id`, `create_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS `wa_conversation`(
   `id` INT UNSIGNED AUTO_INCREMENT,
   `owner` VARCHAR(255) NOT NULL,
   `username` VARCHAR(255) NOT NULL,
   `msg_count` INT UNSIGNED DEFAULT 0,
   `last_time` BIGINT UNSIGNED NOT NULL,
   `content` MEDIUMTEXT DEFAULT NULL,
   `msg_type` INT NOT NULL,
   `digest` VARCHAR(255) DEFAULT '',
   `digest_user` TEXT DEFAULT NULL,
   PRIMARY KEY ( `id` ),
   UNIQUE KEY `uniq_id` (`owner`,`username`),
   INDEX(`owner`, `username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
