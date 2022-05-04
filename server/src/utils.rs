pub fn get_avatar_path(username: &str) -> String {
    let username_md5 = format!("{:x}", md5::compute(&username));
    let avatar = format!(
        "/assets/avatar/{}/{}/user_{}.png",
        &username_md5[0..2],
        &username_md5[2..4],
        username_md5
    );
    avatar
}
