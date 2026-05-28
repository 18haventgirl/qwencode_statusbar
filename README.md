# QCStatusBar

基于 Rust 的 Qwen Code 状态栏工具。

在 Qwen Code 底部状态栏显示模型名称、Git 分支、Token 用量等信息，并支持自定义"思考中"轮播短语。

## 效果预览

```
🤖 mimo-v2.5-pro | 📁 qwencode_statusbar | 🌿 main ● | ⚡ 28k tokens
```

## 安装

### 一键安装（推荐）

**Windows（PowerShell）：**

```powershell
irm https://raw.githubusercontent.com/18haventgirl/qwencode_statusbar/master/install.ps1 | iex
```

**cargo install（需要 Rust 环境）：**

```bash
cargo install --git https://github.com/18haventgirl/qwencode_statusbar
```

### 从源码编译

```bash
git clone https://github.com/18haventgirl/qwencode_statusbar.git
cd qwencode_statusbar
cargo install --path .
```

### 验证安装

```bash
qcstatusbar --version
```

## 配置 Qwen Code

编辑 `~/.qwen/settings.json`，在 `ui` 下添加 `statusLine`：

```json
{
  "ui": {
    "statusLine": {
      "type": "command",
      "command": "qcstatusbar"
    }
  }
}
```

重启 Qwen Code 后生效。

## 状态栏 Segment

| Segment | 图标 | 说明 | 默认 |
|---------|------|------|------|
| Model | 🤖 | 当前模型名称 | ✅ |
| Directory | 📁 | 工作目录 | ✅ |
| Git | 🌿 | 分支名、脏状态、ahead/behind | ✅ |
| ContextWindow | ⚡ | Token 用量 | ✅ |
| Session | 📝 | 代码行数增减 | ❌ |
| Metrics | 📊 | API 请求数、延迟、错误数 | ❌ |

### 启用/禁用 Segment

编辑 `~/.qwen/qcstatusbar/config.toml`，修改对应 segment 的 `enabled` 字段：

```toml
[[segments]]
id = "session"
enabled = true    # 改为 true 启用
```

### Git Segment 选项

```toml
[[segments]]
id = "git"
enabled = true

[segments.options]
show_sha = true   # 显示 commit SHA 前7位
```

## 主题

### 使用内置主题

```bash
qcstatusbar --theme minimal
```

内置主题：`default`、`minimal`、`nord`、`gruvbox`

### 自定义主题

在 `~/.qwen/qcstatusbar/themes/` 下创建 `.toml` 文件，参考已有主题文件格式。

## 自定义"思考中"短语

Qwen Code 在模型推理时会显示轮播短语。QCStatusBar 支持通过 `config.toml` 管理这些短语。

### 编辑短语

编辑 `~/.qwen/qcstatusbar/config.toml` 底部的 `phrases` 数组：

```toml
phrases = [
    "今天的你，比昨天又厉害了一点点 ☀️",
    "喝口水，伸个懒腰，我在帮你跑着呢",
    "别急，好东西值得等一等 🍰",
    # 添加更多...
]
```

### 同步到 Qwen Code

```bash
qcstatusbar --sync-phrases
```

该命令会将 `config.toml` 中的短语写入 `~/.qwen/settings.json` 的 `ui.customWittyPhrases`。

重启 Qwen Code 后生效。

> **注意**：短语是静态列表，Qwen Code 每 15 秒随机切换一条。不支持根据对话上下文动态生成。

可在 `~/.qwen/qcstatusbar/models.toml` 中自定义模型显示名称和上下文窗口大小。

## 配置文件说明

| 文件 | 用途 |
|------|------|
| `~/.qwen/qcstatusbar/config.toml` | 主配置（段落、样式、短语） |
| `~/.qwen/qcstatusbar/models.toml` | 模型显示名和上下文限制 |
| `~/.qwen/qcstatusbar/themes/*.toml` | 主题文件 |
| `~/.qwen/settings.json` | Qwen Code 配置（statusLine、customWittyPhrases） |

## CLI 参数

```
qcstatusbar [OPTIONS]

Options:
  -t, --theme <THEME>    使用指定主题
      --sync-phrases     同步短语到 ~/.qwen/settings.json
  -h, --help             显示帮助
  -V, --version          显示版本
```

## 工作原理

1. Qwen Code 调用 `qcstatusbar`，通过 stdin 传入 JSON 数据（模型、Git、上下文等）
2. QCStatusBar 解析 JSON，收集各 Segment 数据
3. 渲染为 ANSI 彩色文本输出到 stdout
4. Qwen Code 在底部 footer 显示输出

stdin JSON 示例：

```json
{
  "session_id": "abc123",
  "model": { "display_name": "mimo-v2.5-pro" },
  "context_window": { "current_usage": 28000, "used_percentage": 34.3 },
  "workspace": { "current_dir": "/path/to/project" },
  "git": { "branch": "main" },
  "metrics": { "models": { "mimo-v2.5-pro": { "api": { "total_requests": 10 } } } }
}
```

## 开发

```bash
# 编译
cargo build

# 编译 release
cargo build --release

# 安装到 ~/.cargo/bin
cargo install --path .

# 运行测试
cargo test
```

MIT
