# tiktok-matrix

tiktok matrix 是一个可以本地部署的，基于adb和android无障碍服务的tiktok自动化运营工具，可以实现自动注册账号，自动发布视频，自动点赞，自动关注，自动收藏，自动浏览视频等功能。
> tiktok matrix is a tiktok automation tool based on adb and android accessibility services that can be deployed locally. It can automatically register accounts, automatically publish videos, automatically like, automatically follow, automatically collect, and automatically browse videos.

## get started

[官方网站](https://niostack.com/tiktok.html)

[中文文档](https://github.com/niostack/tiktok-matrix/wiki/%E4%B8%AD%E6%96%87%E6%96%87%E6%A1%A3)

[English Document](https://github.com/niostack/tiktok-matrix/wiki/EnglishDoc)

## telegram group

[Join Niostack telegram group](https://t.me/+iGhozoBfAbI5YmE1)

## dev

```shell
npm install --global @tauri-apps/cli
npm install --global shx
npm install
npm run tauri dev
npm run tauri build
```

## 感谢下面开源项目

* https://github.com/niostack/atx-agent
* https://github.com/niostack/android-uiautomator-server
* https://github.com/DeviceFarmer/minicap
* https://github.com/DeviceFarmer/minitouch

## release

### 0.0.4

- [x] save settings to local file
- [x] add wifi config, auto connect wifi

### 0.0.3

- [x] optimize the stability of auto publish video
- [x] add account info(like follower count,earning)

### 0.0.2

- [x] add target country select
- [x] Optimize the stability of video releases

### 0.0.1

- [x] manage devices
- [x] manage groups
- [x] manage accounts
- [x] manage materials
- [x] auto register tiktok account
- [x] auto upload video to tiktok with product link (ai video or upload material)
- [x] auto train account by auto swipe to view For You videos ,auto click like , follow,collection
