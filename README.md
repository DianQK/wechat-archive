# 微信归档

> 目前是很潦草的开发工程，不建议线上直接使用。

## 使用说明

### 准备数据

- 准备好 `MicroMsg` 文件夹
- 搞定 `MicroMsg` 中的 `EnMicroMsg.db` 密码
- 添加对应 db 路径的密码到 `wechat-archive.toml` 文件

### 导入数据

> 注意：这里被设计为支持增量导入，换句话说你可以导入数据之后，清空微信，过段时间导入新的数据。同时支持多用户场景（未测试）。
> 建议：因为目前有大量工作没有完成，不建议直接使用，不过你可以使用  [Neo Backup](https://f-droid.org/en/packages/com.machiav3lli.backup) 完整备份 App，将数据始终留下来，随时可以重复/全新导入。

启动个 MySQL，参见 `server/docker/docker-compose.yaml`。

`server` 目录下执行：

```
cargo build --release
./target/release/wechat-archive merge /path/MicroMsg
```

### 启动服务

编译 frontend：

```
trunk build --public-url /static/
```

在 `server` 目录执行：

```
./target/release/wechat-archive --port 8080
```

## 开发说明

参考：[A Rust web server / frontend setup like it's 2022 (with axum and yew)](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/) 。

## 补充

- [Android 逆向分析实例(三)-解密微信 EnMicroMsg.db 数据库](https://www.cnblogs.com/lxh2cwl/p/14842479.html)

我的测试环境：LineageOS 18.1 + WeChat 8.0.18。
