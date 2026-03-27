# AutoHarness 一键安装

本目录提供跨平台的一键安装脚本。

## 当前可用的二进制

| 操作系统 | 架构 | 状态 |
|-----------|------|------|
| macOS | Intel (x86_64) | ✅ 可用 |
| macOS | Apple Silicon (ARM) | ⬅️ 当前系统，使用 x86_64 兼容版 |
| Linux | x86_64 | 🔨 需自行编译 |
| Windows | x86_64 | 🔨 需自行编译 |

> **注意**: 当前只编译了 macOS x86_64 二进制。对于其他平台，请参考下方的"跨平台编译"部分。

## 快速开始

### Linux / macOS

```bash
cd install
chmod +x install.sh
./install.sh
```

### Windows

```powershell
cd install
.\install.bat
```

## 使用方式

```bash
# 安装
./install.sh

# 卸载
./install.sh uninstall

# 查看帮助
./install.sh --help
```

## 安装位置

- 默认安装到: `~/.local/bin/autoharness`
- 如果需要，手动添加到 PATH:
  ```bash
  export PATH="$HOME/.local/bin:$PATH"
  ```

## 验证安装

```bash
autoharness --version
autoharness --help
```

## 快速命令示例

```bash
# 合成代码
autoharness synthesize --code "fn test() {}"

# 运行基准测试
autoharness benchmark

# 查看配置
autoharness config show
```

## 跨平台编译

如需为其他平台编译二进制：

```bash
# Linux x86_64 (需要 Linux 系统或 cross 工具)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows x86_64 (需要交叉编译)
cargo build --release --target x86_64-pc-windows-gnu
```

## 问题排查

1. **command not found**: 确保 `~/.local/bin` 在 PATH 中
2. **权限错误**: 检查安装目录权限
3. **二进制不匹配**: 确认下载了对应平台的版本

---
Version: 0.1.0 | License: MIT
