### 顺丰速运APP积分中心
- [x] 签到
- [x] 自动完成积分任务
- [ ] ......

### dokcer 运行

- 执行一次: `docker run -it --rm -e SF_EXPRESS_SESSION_ID="nodeqqeeewww1221212211" classmatelin/sf-express sf-express`

- 内置定时任务: `5 0 * * *`, `docker run -itd -e SF_EXPRESS_SESSION_ID="nodeqqeeewww1221212211" --name sf-express classmatelin/sf-express`
### 安装

点击[Release](https://github.com/ClassmateLin/cf-express/releases), 下载对应平台的压缩包解压即可。

### 编译

执行:
```
git clone https://github.com/ClassmateLin/cf-express.git
cd cf-express
cargo build --release
```
输出文件: `./target/release/sf_express`


### 使用

- 对顺丰速运APP进行抓包, 在cookie中找到sessionId, 如`nodeqqeeewww1221212211`。
- 在cf_express同级(或上级)目录下创建一个.env文件, 填入sessionId(如有多个账号,则使用;间隔填入):
```
SF_EXPRESS_SESSION_ID="node0xnbyl6urn2v3he875xxhpnu2993255"
```
- `./sf-express`执行即可。


### 其他脚本

- [东东农场](https://github.com/ClassmateLin/jd-farm)

- [签到领京豆](https://github.com/ClassmateLin/jd-take-bean)

- [丘大叔签到](https://github.com/ClassmateLin/uncle-qiu-sign-in)

- [又拍云云海探宝](https://github.com/ClassmateLin/upyun)
