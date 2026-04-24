# Shutdown Remote

一个基于 Web 的远程关机/重启控制应用。通过安全的 HTTPS 连接，你可以在任何设备上通过浏览器远程控制电脑的关机和重启操作。

## ✨ 特性

- 🔒 **安全加密**：使用自签名证书提供 HTTPS 加密连接
- 📱 **跨平台支持**：支持 Windows、Linux 系统
- 🌐 **响应式界面**：现代化的 Web 界面，支持桌面和移动端访问
- 📲 **PWA 支持**：可安装为 Progressive Web App，支持离线访问
- ⚡ **轻量高效**：后端采用 Rust 编写，性能优异，资源占用低
- 🔄 **开机自启**：支持系统启动时自动运行服务

## 📁 项目结构

```
.
├── client/                 # Web 前端项目
│   ├── apps/web/           # React + Vite + TanStack Router 应用
│   └── packages/ui/        # 共享 UI 组件库
├── server/                 # Rust 后端服务
│   ├── src/                # 源代码
│   ├── config.toml         # 配置文件模板
│   ├── cert.pem            # SSL 证书
│   └── key.pem             # SSL 私钥
├── dist/                   # 构建后的分发目录
│   ├── shutdown-auto       # Linux 后端程序
│   ├── shutdown-auto.exe   # Windows 后端程序
│   ├── web/                # 前端静态文件
│   ├── config.toml         # 配置文件
│   ├── install.sh          # Linux 安装脚本
│   └── install.ps1         # Windows 安装脚本
├── build.sh                # Linux 构建脚本
├── build.ps1               # Windows 构建脚本
├── install.sh              # Linux 安装脚本（根目录）
└── install.ps1             # Windows 安装脚本（根目录）
```

## 🚀 快速开始

### 前置要求

- **Linux**: systemd 初始化系统（大多数现代 Linux 发行版）
- **Windows**: Windows 10/11，PowerShell
- **网络**: 确保客户端设备可以访问服务器所在机器的 IP 地址和端口

### 从源码构建

#### Linux / macOS

```bash
# 赋予构建脚本执行权限
chmod +x build.sh

# 执行构建
./build.sh
```

#### Windows

```powershell
# 以管理员身份运行 PowerShell
.\build.ps1
```

构建完成后，所有必要文件将打包到 `dist` 目录中，并生成 `shutdown-remote.zip` 压缩包。

### 安装部署

#### 🐧 Linux 系统安装

1. **解压安装包**（如果使用的是 zip 包）：

   ```bash
   unzip shutdown-remote.zip -d shutdown-remote
   cd shutdown-remote
   ```

2. **运行安装脚本**：

   ```bash
   chmod +x install.sh
   sudo ./install.sh
   ```

3. **安装过程会自动完成**：
   - 复制文件到 `/usr/local/share/shutdown-remote`
   - 创建 systemd 服务 `shutdown-remote.service`
   - 设置开机自启动
   - 立即启动服务

4. **查看服务状态**：

   ```bash
   systemctl status shutdown-remote
   ```

5. **查看实时日志**：

   ```bash
   journalctl -u shutdown-remote -f
   ```

#### 🪟 Windows 系统安装

1. **解压安装包**：
   - 右键点击 `shutdown-remote.zip`，选择"全部提取"

2. **以管理员身份运行 PowerShell**：
   - 在开始菜单搜索 "PowerShell"
   - 右键点击，选择"以管理员身份运行"

3. **进入解压目录并执行安装**：

   ```powershell
   # 切换到解压目录
   cd C:\Users\你的用户名\Downloads\shutdown-remote
   
   # 如果提示执行策略限制，先运行以下命令
   Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
   
   # 执行安装脚本
   .\install.ps1
   ```

4. **安装过程会自动完成**：
   - 复制文件到 `%USERPROFILE%\shutdown-remote`
   - 创建计划任务 `ShutdownRemoteAutoStart`
   - 设置开机自启动

### 配置说明

服务的配置文件位于：

- **Linux**: `/usr/local/share/shutdown-remote/config.toml`
- **Windows**: `%USERPROFILE%\shutdown-remote\config.toml`

默认配置示例：

```toml
[server]
port = 8443          # 服务端口
https = true         # 是否启用 HTTPS

[security]
shutdown_key = "your-secret-key-here"  # 关机操作的安全密钥
```

**修改配置后需要重启服务**：

```bash
# Linux
sudo systemctl restart shutdown-remote

# Windows
# 在任务计划程序中找到并重新运行任务，或重启电脑
```

### 访问应用

安装完成后，你可以通过以下方式访问：

1. **本地访问**：

   ```
   https://localhost:8443
   ```

2. **局域网访问**：

   ```
   https://<服务器IP地址>:8443
   ```

3. **首次访问**：
   - 由于使用自签名证书，浏览器会显示安全警告
   - 点击"高级" → "继续访问"（Chrome）或"接受风险并继续"（Firefox）

## 📱 PWA 移动端的安装

本应用支持 Progressive Web App (PWA)，可以在手机上获得类似原生应用的体验。

### Android (Chrome)

1. 使用 Chrome 浏览器访问应用地址
2. 点击浏览器菜单（三个点）→ **"添加到主屏幕"**
3. 或者点击地址栏右侧的 **"安装"** 图标（如果显示）
4. **重要**：建议通过浏览器弹出的"安装应用"提示进行安装，这样可以获得更好的离线支持和无地址栏体验

### iOS (Safari)

1. 使用 Safari 浏览器访问应用地址
2. 点击底部分享按钮（方框带箭头图标）
3. 选择 **"添加到主屏幕"**
4. 点击右上角的"添加"

### PWA 注意事项

- ✅ **必须使用 HTTPS**：PWA 的 Service Worker 只能在 HTTPS 环境下工作（localhost 除外）
- ✅ **首次加载需联网**：确保首次访问时网络正常，以便缓存必要资源
- ✅ **清除缓存重装**：如果遇到问题，可以尝试清除站点数据后重新安装

## 🛠️ 使用方法

### 基本操作

1. **打开应用**：在浏览器中访问 `https://<IP地址>:8443`
2. **查看设备信息**：首页会显示当前设备的系统信息、CPU、内存等
3. **执行关机**：点击"关机"按钮，输入配置的安全密钥确认
4. **执行重启**：点击"重启"按钮，输入配置的安全密钥确认

### API 接口

你也可以通过 HTTP API 直接控制：

```bash
# 获取设备状态
curl -X POST https://<IP>:8443/getStatus \
  -H "Content-Type: application/json" \
  -d '{"key": "your-secret-key"}'

# 执行关机
curl -X POST https://<IP>:8443/shutdown \
  -H "Content-Type: application/json" \
  -d '{"key": "your-secret-key"}'

# 执行重启
curl -X POST https://<IP>:8443/reboot \
  -H "Content-Type: application/json" \
  -d '{"key": "your-secret-key"}'
```

## 🗑️ 卸载方法

### Linux 卸载

```bash
# 1. 停止服务
sudo systemctl stop shutdown-remote

# 2. 禁用开机自启
sudo systemctl disable shutdown-remote

# 3. 删除服务文件
sudo rm /etc/systemd/system/shutdown-remote.service

# 4. 重载 systemd 配置
sudo systemctl daemon-reload

# 5. 删除应用文件
sudo rm -rf /usr/local/share/shutdown-remote

# 6. （可选）清理日志
sudo journalctl --vacuum-time=1s
```

### Windows 卸载

1. **删除计划任务**：
   - 按 `Win + R`，输入 `taskschd.msc`，回车
   - 在左侧找到"任务计划程序库"
   - 找到 `ShutdownRemoteAutoStart` 任务
   - 右键点击，选择"删除"

2. **删除应用文件**：
   - 打开文件资源管理器
   - 进入 `C:\Users\你的用户名\`
   - 删除 `shutdown-remote` 文件夹

3. **或使用 PowerShell 一键卸载**：

   ```powershell
   # 以管理员身份运行
   Unregister-ScheduledTask -TaskName "ShutdownRemoteAutoStart" -Confirm:$false
   Remove-Item -Path "$env:USERPROFILE\shutdown-remote" -Recurse -Force
   ```

## ❓ 常见问题

### 1. 无法访问服务

**问题**：浏览器显示"无法连接"或"连接被拒绝"

**解决方案**：

- 检查服务是否正在运行：

  ```bash
  # Linux
  systemctl status shutdown-remote
  
  # Windows
  # 查看任务计划程序中的任务状态
  ```

- 检查防火墙设置，确保端口（默认 8443）已开放
- 确认服务器和客户端在同一网络中

### 2. 浏览器显示安全警告

**问题**：访问时浏览器提示"您的连接不是私密连接"

**解决方案**：

- 这是正常现象，因为使用的是自签名证书
- 点击"高级" → "继续访问"即可
- 如果需要正式证书，可以替换 `cert.pem` 和 `key.pem` 文件

### 3. PWA 离线功能不可用

**问题**：添加到主屏幕后，离线无法访问

**解决方案**：

- 确认使用 HTTPS 访问（localhost 除外）
- 首次访问时确保网络正常，让 Service Worker 完成缓存
- 尝试清除浏览器缓存后重新安装 PWA
- 在 Chrome 中访问 `chrome://serviceworker-internals/` 检查 SW 状态

### 4. 关机/重启无响应

**问题**：点击按钮后没有反应

**解决方案**：

- 检查 `config.toml` 中的 `shutdown_key` 是否正确
- 查看服务日志确认是否有错误：

  ```bash
  # Linux
  journalctl -u shutdown-remote -e
  
  # Windows
  # 查看事件查看器或程序输出
  ```

- 确认后端程序有执行关机/重启的系统权限

### 5. Linux 服务启动失败

**问题**：`systemctl start shutdown-remote` 失败

**解决方案**：

- 查看详细错误信息：

  ```bash
  journalctl -u shutdown-remote -e
  ```

- 常见原因：
  - 端口被占用：修改 `config.toml` 中的端口号
  - 证书文件缺失：确认证书文件存在且可读
  - 权限问题：确保文件所有权正确

## 🔧 开发指南

### 技术栈

- **前端**：React 18, Vite, TanStack Router, TypeScript, TailwindCSS
- **UI 组件**：shadcn/ui
- **PWA**：vite-plugin-pwa, Workbox
- **后端**：Rust, Axum, tokio, rustls
- **状态管理**：Zustand

### 本地开发

```bash
# 1. 启动后端（开发模式）
cd server
cargo run

# 2. 启动前端（新终端）
cd client/apps/web
pnpm dev
```

前端开发服务器运行在 `http://localhost:5173`，会自动代理 API 请求到后端。

### 构建生产版本

参见上方的"从源码构建"部分。

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 支持

如有问题，请提交 Issue 或通过以下方式联系：

- 提交 GitHub Issue
- 查看项目文档

---

**享受便捷的远程控制体验！** 🎉
