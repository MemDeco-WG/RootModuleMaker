
<table>
  <tr>
    <td width="140" valign="middle">
      <a href="https://github.com/LIghtJUNction/RootManage-Module-Model/">
        <img src="assets/logo.png" alt="RMM logo" width="120" style="border-radius:8px;" />
      </a>
    </td>
    <td valign="middle">
      <h1 style="margin:0;">RMM (Root Module Maker)</h1>
      <p style="margin-top:6px; margin-bottom:6px;">轻量模块开发工具集 — 从创建到构建、测试到发布的一站式工作流</p>
      <!-- <p style="margin:6px 0 0 0;"><a href="https://github.com/APMMPDEVS/RootManage-Module-Model/"><img src="https://repobeats.axiom.co/api/embed/4dbcdf8b2d24156dcf08cef7cc801d9adb317cae.svg" alt="RepoBeats" /></a></p> -->
    </td>
  </tr>
</table>

---

## RMM 模式对比

> 下表把“传统模式”和“新模式”的功能并列展示，方便快速对比。

| 功能 / Feature | 传统模式 | 新模式 |
|---|:---:|:---:|
| 运行 Action Workflow | ✅ | ✅ |
| 完整构建流程 | ✅ | ✅ |
| 在任意地方运行无需新建 GitHub 仓库 | ❌ | ☑️ |
| prebuild script（编译前脚本） | ❌ 不支持 | ☑️ 支持 |
| postbuild script（编译后脚本） | ❌ 不支持 | ☑️ 支持 |
| 分模板初始化功能 | ❌ 不支持 | ☑️ 支持（不完善） |
| 多项目合并构建 | ❌ 不支持 | ☑️ 开发中 |
| 依赖管理 | ❌ 不支持 | ☑️ 开发中 |
| 多模块合并 | ❌ 不支持 | ☑️ 支持 |
| 模块仓库 | ❌ 不支持 | ☑️ 开发中 |
| AI 测试 / 审计 / 优化 / 修复 | ❌ 不支持 | ☑️ 支持 |
| 通知 / 模块推送（Telegram / Discord / QQ / 酷安） | ❌ 不支持 | ☑️ 开发中 |
| 代理加速 | ❌ 不支持 | ☑️ 支持 |
| 虚拟机仿真模块测试 | ❌ 不支持 | ☑️ 支持(实验中) |
| 模块构建日志 | ❌ 不支持 | ☑️ 支持 |
| 快捷安装至物理机 | ❌ 不支持 | ☑️ 开发中 |
| GPG 签名 | ❌ 不支持 | ☑️ 计划支持 |

> 注：表中“开发中 / developing”表示该功能正在实现中，状态可能会随着版本更新而变化。

## 快速介绍

RMM (模块开发工具集) v0.1.7 之前 由纯python实现
RMM v0.2.0 至今 由 Rust 混合 Python 实现速度大幅度提升

缺点：安装难度变大（我会优先编译python3.13-win64）：

- maturin
- 需要安装 Rust
- 需要安装配置cmake

核心特点：

> 支持shellcheck静态sh语法检查 ，在build阶段发现错误.
> 语法错误包含详细的错误信息和修复建议
> 全模块开发环节支持
> 从新建模块 到 构建模块 到测试模块 到发布模块
> 甚至 ，发布模块时可以选择在release note选择添加代理加速下载链接

> 不想下载？这样安装到手机太慢了！
> 我们还支持直接通过adb连接AVD测试机虚拟仿真与直接安装到真机！

> 如果你是kernelsu用户，还支持不重启手机直接测试模块（因为ksud有这个功能）

> 支持命令补全功能
> 使用rmm complete 命令生成补全脚本!

> 模块开发mcp服务器
>
> 现已支持mcp服务器 stdio 和 sse 模式

avd你可以参考下面的教程，本项目拷贝了rootAVD几个关键文件位于assets/rootAVD目录。

参考[rootAVD教程](https://gitlab.com/newbit/rootAVD)对你的AVD进行root.

**Magisk 下载**：请访问 [Magisk GitHub 仓库](https://github.com/topjohnwu/Magisk) 获取最新版本或访问 [Releases 页面](https://github.com/topjohnwu/Magisk/releases)下载所需版本。

## 使用方法

### 方式一：下载预编译二进制文件（推荐）

从 [Releases](https://github.com/MemDeco-WG/RootModuleMaker/releases) 页面下载适合您系统的预编译二进制文件：

- **Linux x86_64**: `rmm-linux-x86_64`
- **Linux ARM64**: `rmm-linux-aarch64`
- **macOS x86_64**: `rmm-macos-x86_64`
- **macOS ARM64 (Apple Silicon)**: `rmm-macos-aarch64`
- **Windows x86_64**: `rmm-windows-x86_64.exe`

下载后，给予执行权限（Linux/macOS）：

```bash
chmod +x rmm-*
```

然后直接运行：

```bash
./rmm-linux-x86_64 --help
```

**注意**：预编译的二进制文件是纯 Rust 实现，不支持 Python 扩展命令。如需使用 Python 扩展功能，请使用方式二或方式三安装。

### 方式二：安装 uv (Python 包)

> 从pypi安装

```bash
uv tool install pyrmm 
```
> 使用rmm命令

```bash
rmm
```

> 卸载

```
uv tool uninstall pyrmm
```

### 方式三：从源码安装（开发者）

开发者快速开始（可复制粘贴执行）——在本仓库根目录运行以下命令：

1. 克隆仓库并进入目录
2. 同步依赖并构建
3. 使用 maturin 进行本地开发（编译 Python 扩展）
4. 将工具以可编辑模式安装到本地 Python 环境

```bash
git clone https://github.com/LIghtJUNction/RootManageModuleModel.git
cd RootManageModuleModel
uv sync -U
uv build && maturin develop
uv tool install -e . --force
```

开发者可以使用

```
.venv/bin/rmm
```

方便调试

说明：
- 在 macOS / Linux 下请使用 zsh 或 bash；在 Windows 下可使用 PowerShell 或 Git Bash。
- 如果遇到权限或环境问题，请先确保已安装 Rust、maturin、以及项目所需的构建工具（例如 cmake）。

### 方式四：从源码构建二进制文件

如果你只想使用独立的二进制文件（不需要 Python 扩展），可以直接编译 Rust 二进制：

```bash
git clone https://github.com/MemDeco-WG/RootModuleMaker.git
cd RootModuleMaker/rust
cargo build --release --bin rmm
```

编译后的二进制文件位于 `target/release/rmm` (或 Windows 上的 `rmm.exe`)。

**交叉编译示例**：

```bash
# 安装目标平台
rustup target add aarch64-unknown-linux-gnu

# 编译 ARM64 Linux 版本
cargo build --release --bin rmm --target aarch64-unknown-linux-gnu
```

## 用户手册

### rootAVD:

致谢：[rootAVD](https://gitlab.com/newbit/rootAVD)
示例命令：
.\rootAVD.bat "system-images\android-36\google_apis\x86_64\ramdisk.img"

WIN + R 输入以下命令

%LOCALAPPDATA%\Android\Sdk\system-images

system-images\android-36\google_apis\x86_64\ramdisk.img 需要替换为实际路径

#### 模块仓库

开发中 计划兼容现有模块仓库

#### Magick模块MCP服务器

> 现已支持mcp服务器 stdio 和 sse 模式
> 但是功能并不完善


## License

MIT License
Copyright (c) 2025 APMMPDEVS

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

> 如果你使用rmm构建模块，请勿移除rmm自动生成的license文件
> 请在license顶部添加您的license
> 本项目采用MIT License
> 因此你可以将你开发的模块进行闭源处理
> 并且允许将模块进行商业化处理
> 唯一的要求是包含一份RMM MIT License的副本

## 声明

本开源项目旨在促进模块生态系统的发展和创新。
拥抱AI技术，提升模块开发效率和质量。

> 前提

- 具备一定的Python编程基础
- 熟悉基本的命令行操作
- 了解模块化开发的基本概念
- 开启静态类型检查，等级为strict

## 贡献

我们欢迎任何形式的贡献，包括但不限于：

- 提交问题和建议
- 提交代码和文档的改进
- 参与讨论和社区活动
  请遵循以下步骤进行贡献：

1. Fork 本仓库
2. 创建一个新的分支

> PYTHOON 贡献指南
先删掉uv.lock
执行uv sync

打包测试（实际上是调用的maturin打包的）：
uv build

本地安装测试:
uv tool install -e .

感谢你的支持与贡献！

## 外部依赖

- uv
- maturin 用来编译Rust python 扩展模块 基于pyo3
- shellcheck 用来检查shell脚本语法
- adb 用来连接AVD或物理机
- rootAVD 用来root AVD -- 可选 如果有测试需求

## 环境变量

- GITHUB_ACCESS_TOKEN: 用于访问GitHub API的令牌 如果未设置 无法使用发布release功能

## 致谢名单

> Credits
> Kernel-Assisted Superuser: The KernelSU idea.
> Magisk: The powerful root tool.
> genuine: APK v2 signature validation.
> Diamorphine: Some rootkit skills.
> KernelSU: The kernel based root solution.
> APATCH : The kernel based root solution.
> RootAVD: The AVD root script.
> ShellCheck: The shell script static analysis tool.
