# taskflow

**taskflow** is a professional CLI tool designed to manage the development lifecycle of agents. It ensures that every code change is tracked, documented in a CHANGELOG, and committed to Git with a standardized format and automated branching.

[English](#english) | [中文](#中文)

---

<a name="english"></a>
## English

### Core Features
- **Automated Branching**: `open` command handles branching from `dev` and immediate changelog "locking".
- **Remote Synchronization**: `close` and `release` commands automatically push changes to remote repositories.
- **GitHub-Integrated Releases**: Automated CI/CD (GitHub Actions) for cross-platform binary distribution on Tag creation.
- **Standardized Commits**: Enforces Conventional Commits with Task IDs integrated into the message.

### Usage
Use the absolute path to the binary appropriate for your operating system (e.g., `.agent/skills/taskflow/bin/taskflow`).

#### 1. Open Task
```bash
taskflow open "description" [slug]
```
#### 2. Close Task
```bash
taskflow close <type> <scope> <subject> <body> [footer]
```
#### 3. Release Version
```bash
taskflow release <version>
```

---

<a name="中文"></a>
## 中文

### 核心功能
- **自动分支管理**：`open` 命令自动从 `dev` 分支拉取并创建任务分支，同时“锁定” Changelog。
- **远程自动同步**：`close` 和 `release` 命令会自动将变更推送到远程仓库。
- **集成 GitHub Actions**：通过标签触发自动化 CI/CD，实现全平台二进制文件的分发。
- **标准化提交**：强制执行约定式提交（Conventional Commits），并在消息中集成任务 ID。

### 使用方法
请根据您的操作系统使用对应的二进制文件绝对路径（例如：`.agent/skills/taskflow/bin/taskflow`）。

#### 1. 开启任务
```bash
taskflow open "任务描述" [分支缩写]
```
#### 2. 关闭任务
```bash
taskflow close <类型> <范围> <主题> <详细内容> [页脚]
```
#### 3. 发布版本
```bash
taskflow release <版本号>
```

---

## Installation / 编译安装
Build requirements: Rust toolchain.

**Recommended (with packaging / 推荐使用脚本自动化打包):**
- macOS/Linux: `bash scripts/build.sh`
- Windows: `scripts\build.bat`

**Manual (Binary only / 仅编译二进制文件):**
```bash
cargo build --release
```
Packaged results will be in the `dist/` directory.
