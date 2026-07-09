<div align="center">
  <h2>ZError Web (Linux 独立分支)</h2>
  <span>专为无头服务器打造的全天候远程Web控制AI题库软件</span>
</div>

## 项目简介

ZError Web 是一款专为OCS网课助手设计的AI题库软件。

本项目分支具有绝对的独立性，专为 Linux 服务器环境提供全天候在线的远程题库服务。这是一个与 Windows 版本完全不同的服务方向，专为无头（Headless）服务器量身打造，并通过 Web 可视化面板进行全面控制与管理。

软件采用 Rust 后端和 Vue 前端架构，可以方便地为OCS网课助手搭建本地AI题库，并支持AI平台以及模型的自定义。

## 主要功能

* **全天候运行**：专为无头Linux服务器设计，支持后台持续稳定运行。
* **纯Web端控制**：从服务器配置到题目管理，完全通过浏览器进行操作。
* **OCS题库配置**：支持自定义OCS网课助手的本地AI题库。
* **自定义AI平台**：自由添加AI供应商、自定义模型名称与请求端点。
* **本地缓存管理**：提供题目编辑、分类文件夹整理与模糊搜索功能。

## 独立性说明

这个版本彻底移除了原有 Windows 平台的相关介绍和界面依赖。通过 Web 端管理取代了传统的桌面端 UI，赋予项目全新的服务器端定位，旨在打造一款轻量、高效、全天候响应的AI题库后端。

原 Windows 版本项目地址：[https://github.com/Miaozeqiu/ZError](https://github.com/Miaozeqiu/ZError)

## 系统要求

* Linux 无头服务器环境（支持 Systemd 进程托管）
* 网络畅通，且服务器的相关特权/普通端口未被占用

---

## 📑 部署前置准备

在纯命令行服务器上运行该程序，需要先补齐底层图形链接库并清理网络通道。

### 1. 安装基础运行时依赖

即使程序以无窗口的 Web 模式运行，其底层框架依然依赖一些轻量级动态链接库。请先在服务器上执行以下命令完成预装：

```bash
sudo apt update && sudo apt install -y libgtk-3-0 libwebkit2gtk-4.1-0 psmisc

```

*(注：`psmisc` 工具包中包含 `fuser` 命令，用于后续一键清理残留的端口占用)*

### 2. 云服务器防火墙放行

请前往您的云服务商控制台，在安全组入站规则中放行以下两个端口：

* **`8080` 端口** (TCP)：用于浏览器访问前端控制面板 Web UI。
* **`3000` 端口** (TCP)：用于后端 API 服务器通信（请确保此端口未被其他程序如代理工具占用）。

---

## 🚀 核心部署步骤

### 第一步：获取并解压最新完整运行包

我们在流水线中加入了自动打包机制。现在通过下面的命令直接拉取包含前端静态网页（`dist` 目录）和后端主程序的最新版压缩包，在服务器就地解压：

```bash
# 1. 创建并进入干净的工作目录
mkdir -p ~/zerror_app && cd ~/zerror_app

# 2. 拉取最新发布的完整运行包
wget https://github.com/xyzckl/ZError/releases/latest/download/zerror-linux-x64.tar.gz

# 3. 解压压缩包里的全部内容并清理多余临时文件
tar -xzf zerror-linux-x64.tar.gz && rm zerror-linux-x64.tar.gz

# 4. 赋予后端主程序最高可执行权限
chmod +x zerror

```

解压完成后，您的工作目录中应该包含可执行程序 `zerror` 以及完整的前端静态文件目录 `dist`。

### 第二步：一键强制释放冲突端口

为了防止旧进程、之前异常退出的服务或其他代理程序抢占 `8080` 或 `3000` 端口，在启动前可以执行物理清理：

```bash
# 强行终止任何正在占用 8080 和 3000 端口的进程
sudo fuser -k 8080/tcp 3000/tcp 2>/dev/null

```

### 第三步：配置 Systemd 守护进程
### (或者单次启动：sudo ./zerror)

在轻量级服务器上，使用系统自带的 `systemd` 托管程序是最节省系统资源且最稳定的生产环境方案。

运行以下命令创建服务配置文件：

```bash
sudo nano /etc/systemd/system/zerror.service

```

将以下标准配置粘贴进去（**请根据您的实际用户名修改 WorkingDirectory 与 ExecStart 中的路径**，当前默认用户名为 `xyzck`）：

```ini
[Unit]
Description=ZError Web Service (Automated Release Bundle)
After=network.target

[Service]
Type=simple
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

配置文件就绪后，通过以下服务命令让程序正式挂载并在后台持续运行：

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

如果看到绿色的 **`active (running)`**，说明程序已成功在您的服务器上稳定安家。

* **实时跟踪程序后端日志**：

```bash
journalctl -u zerror.service -f -n 50

```

---

## 🔄 日常维护：如何更新到最新版？

未来当您提交新代码并发布新版本时，您**不需要**重新编写配置文件。只需在服务器上依次执行以下四行命令，即可实现无损平滑更新（该操作会妥善保留已生成的 `config/` 目录和本地数据库）：

```bash
cd ~/zerror_app

# 1. 停止当前运行的旧服务
sudo systemctl stop zerror

# 2. 重新拉取最新的运行包
wget https://github.com/xyzckl/ZError/releases/latest/download/zerror-linux-x64.tar.gz

# 3. 覆盖解压并清理压缩包
tar -xzf zerror-linux-x64.tar.gz && rm zerror-linux-x64.tar.gz

# 4. 重新补齐可执行权限并启动服务
chmod +x zerror && sudo systemctl start zerror

```

## 警告

ZError Web 目前处于开发状态，在该警告删除前请不要尝试作为正式服务使用。
### 当然您也可以尝试创建新分支发起合并请求，帮助完成该项目。

## 许可证

本项目采用 MIT 许可证

```

```
