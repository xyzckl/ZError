<div align="center">
  <h2>ZError Web (Linux 独立分支)</h2>
  <span>专为无头服务器打造的全天候远程Web控制AI题库软件</span>
</div>

## 项目简介

ZError Web 是一款专为OCS网课助手设计的AI题库软件。

**重要声明**：本项目分支具有绝对的独立性，专为 Linux 服务器环境提供全天候在线的远程题库服务。这是一个与 Windows 版本完全不同的服务方向，专为无头（Headless）服务器量身打造，并通过 Web 可视化面板进行全面控制与管理。

软件采用 Rust 后端和 Vue 前端架构，可以方便地为OCS网课助手搭建本地AI题库，并支持AI平台以及模型的自定义。

## 主要功能

- **全天候运行**：专为无头Linux服务器设计，支持后台持续稳定运行。
- **纯Web端控制**：从服务器配置到题目管理，完全通过浏览器进行操作。
- OCS题库配置
- 自定义AI供应商以及AI模型
- 本地题库管理与缓存（支持题目编辑，文件夹分类）
- 支持一键部署自动化脚本

## 独立性强调

这个版本（Linux Web版）彻底移除了原有 Windows 平台的相关介绍和界面依赖。通过 Web 端管理取代了传统的桌面端 UI，赋予项目全新的服务器端定位，旨在打造一款轻量、高效、全天候响应的AI题库后端。

原 Windows 版本项目地址：[https://github.com/Miaozeqiu/ZError](https://github.com/Miaozeqiu/ZError)

## 系统要求

- Linux 无头服务器环境（支持 Systemd）
- Node.js环境 (用于前端构建)
- Rust环境 (用于后端构建)

## 安装方法

这份指南专为 **Linux Headless（无图形界面）服务器**量身定制。教程摒弃了固定的版本标签，采用 GitHub 官方的动态重定向机制，确保每次部署或更新时都能**直接抓取到项目仓库的最新发布版本（Latest Release）**。

---

## 📑 部署前置准备

在纯命令行服务器上运行该程序，需要先补齐底层图形链接库，并清理网络通道。

### 1. 安装基础运行时依赖

即使程序以无窗口模式运行，其底层框架依然依赖以下轻量级动态库，请先在服务器执行安装：

```bash
sudo apt update && sudo apt install -y libgtk-3-0 libwebkit2gtk-4.1-0 psmisc

```

> *(注：`psmisc` 工具包中包含 `fuser` 命令，用于后续一键清理端口占用)*

### 2. 云服务器防火墙放行

请前往你的云服务商控制台，在防火墙入站规则中放行以下两个端口：

* **`8080` 端口** (TCP)：用于访问 Web UI 网页前端。
* **`80` 端口** (TCP)：用于后端 API 服务器通信。

---

## 🚀 核心部署步骤

### 第一步：一键获取最新版二进制文件

创建专属工作目录，并利用 GitHub 提供的 `releases/latest/download` 动态接口，直接下载最新发布的免安装文件：

```bash
# 1. 创建并进入干净的工作目录
mkdir -p ~/zerror_app && cd ~/zerror_app

# 2. 拉取最新发布的 zerror 核心文件
wget https://github.com/xyzckl/ZError/releases/latest/download/zerror

# 3. 赋予该文件最高可执行权限
chmod +x zerror

```

### 第二步：一键强制释放冲突端口

为了防止旧进程或其他服务（如 Nginx、旧版服务）强占 `8080` 和 `80` 端口导致程序初始化崩溃，在启动前执行“物理清理”：

```bash
# 强行终止任何正在占用 8080 和 80 端口的进程
sudo fuser -k 8080/tcp 80/tcp 2>/dev/null

```

### 第三步：配置 Systemd 守护进程（长期后台运行）

在 1GB 内存的轻量级服务器上，使用系统自带的 `systemd` 来托管程序是最省资源、最稳定的生产环境方案。

运行以下命令创建服务配置文件：

```bash
sudo nano /etc/systemd/system/zerror.service

```

将以下标准配置粘贴进去（**请根据你的实际用户名修改其中的路径**，当前默认用户名为 `xyzck`）：

```ini
[Unit]
Description=ZError Web Service (Automated Release)
After=network.target

[Service]
Type=simple
# 必须使用 root 用户启动以允许绑定特权端口 80
User=root
WorkingDirectory=/home/xyzck/zerror_app
ExecStart=/home/xyzck/zerror_app/zerror --web-port 8080
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target

```

*粘贴完成后，按 `Ctrl + O` 然后回车保存，再按 `Ctrl + X` 退出编辑器。*

---

## 🛠️ 服务启动与维护

配置文件就绪后，通过以下组合拳让程序正式挂载并在后台永久燃烧：

```bash
# 1. 刷新系统服务列表
sudo systemctl daemon-reload

# 2. 启动服务并将其设置为开机自动启动
sudo systemctl start zerror
sudo systemctl enable zerror

```

### 🔍 运行状态与日志监控

* **检查程序是否成功存活**：
```bash
sudo systemctl status zerror

```


如果看到绿色的 **`active (running)`**，说明 AI 提交的代码已成功在你的服务器上安家落地。
* **实时跟踪程序后端日志（排错神器）**：
```bash
journalctl -u zerror.service -f -n 50

```



---

## 🔄 日常维护：如何更新到最新版？

未来如果再次提交了新代码并发布了新版本，你**不需要**重写配置文件，只需在服务器上执行以下三行命令，即可秒级完成平滑更新：

```bash
cd ~/zerror_app
# 1. 停止当前运行的服务
sudo systemctl stop zerror
# 2. 重新拉取最新覆盖下载，并补上执行权限
wget -O zerror https://github.com/xyzckl/ZError/releases/latest/download/zerror && chmod +x zerror
# 3. 重新启动服务
sudo systemctl start zerror

```

## 许可证

本项目采用 MIT 许可证
