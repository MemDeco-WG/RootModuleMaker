# RootModuleMaker (RMM) 设计蓝图

> 项目分析日期: 2025-11-07
> 基于版本: v0.4.0

## 1. 项目概述

### 1.1 项目定位
RootModuleMaker (RMM) 是一个轻量级的模块开发工具集，旨在为 Android Root 模块（Magisk/KernelSU/APatch）提供从创建到构建、测试到发布的一站式工作流。

### 1.2 技术栈
- **核心语言**: Rust (高性能核心) + Python (扩展功能)
- **构建工具**: Cargo, Maturin (Python-Rust 绑定)
- **包管理**: uv (Python), Cargo (Rust)
- **CLI框架**: clap (Rust), click (Python)

### 1.3 架构模式
混合架构（Hybrid Architecture）:
- **Rust核心**: 提供高性能的二进制工具和可选的Python扩展
- **Python层**: 提供AI功能、MCP服务器和高级CLI功能
- **双模式运行**:
  1. 纯Rust二进制 (无Python依赖，快速部署)
  2. Python扩展模式 (完整功能集)

## 2. 项目结构分析

### 2.1 目录结构
```
RootModuleMaker/
├── rust/                      # Rust核心实现
│   ├── src/
│   │   ├── main.rs           # 二进制入口点
│   │   ├── lib.rs            # 库入口点 (Python扩展)
│   │   ├── cmds/             # 命令实现模块
│   │   │   ├── build.rs      # 构建命令
│   │   │   ├── init.rs       # 初始化命令
│   │   │   ├── publish.rs    # 发布命令
│   │   │   ├── sync/         # 同步相关命令
│   │   │   ├── check.rs      # 检查命令
│   │   │   ├── install.rs    # 安装命令
│   │   │   ├── run.rs        # 运行脚本命令
│   │   │   └── version.rs    # 版本管理
│   │   └── core/             # 核心功能模块
│   │       ├── types.rs      # 核心数据类型
│   │       ├── env.rs        # 环境配置
│   │       ├── deps.rs       # 依赖管理
│   │       ├── git.rs        # Git集成
│   │       ├── runtime.rs    # 运行时环境检测
│   │       ├── features.rs   # 功能管理
│   │       ├── packaging.rs  # 打包功能
│   │       └── author.rs     # 作者信息管理
│   └── Cargo.toml            # Rust项目配置
├── src/pyrmm/                # Python扩展源码
│   ├── cli/                  # CLI实现
│   ├── ai/                   # AI功能 (MCP服务器)
│   └── utils/                # 工具函数
├── docs/                     # 文档目录
├── assets/                   # 资源文件 (rootAVD等)
├── pyproject.toml            # Python项目配置
└── rmmproject.toml           # RMM项目元数据
```

### 2.2 模块依赖关系图

基于 `modules.tree` 的分析:
```
crate rmmcore
├── mod cmds: pub             # 命令模块 (公开API)
│   ├── build                 # 构建系统
│   ├── init                  # 项目初始化
│   ├── publish               # 发布管理
│   ├── sync                  # 同步系统
│   ├── check                 # 代码检查
│   ├── install               # 安装到设备
│   ├── run                   # 脚本运行
│   └── version               # 版本管理
├── mod core: pub             # 核心功能 (公开API)
│   ├── types                 # 数据类型定义
│   ├── env                   # 环境管理
│   ├── deps                  # 依赖系统
│   ├── git                   # Git集成
│   ├── runtime               # 运行时
│   ├── features              # 特性管理
│   ├── packaging             # 打包系统
│   └── author                # 作者管理
└── fn init: pub              # 初始化函数
```

## 3. 核心功能设计

### 3.1 命令系统 (cmds/)

#### 3.1.1 build - 构建系统
**功能**: 构建Magisk/KernelSU/APatch模块
**关键特性**:
- 支持 prebuild/postbuild 脚本钩子
- ShellCheck静态语法检查
- 行尾标准化处理
- 排除/包含文件管理
- 多格式打包 (ZIP, TAR.GZ)

**数据流**:
```
rmmproject.toml → BuildArgs → prebuild → 文件收集 → 
ShellCheck → 打包 → postbuild → 输出ZIP
```

#### 3.1.2 init - 项目初始化
**功能**: 从模板创建新项目
**支持的模板**:
- 基础模板
- 自定义模板系统

**实现要点**:
- 模板文件递归复制
- 项目元数据生成
- Git仓库初始化

#### 3.1.3 publish - 发布管理
**功能**: 发布模块到GitHub Releases
**特性**:
- GitHub API集成
- Release notes支持
- 代理加速URL生成
- Artifact打包

**流程**:
```
准备Artifact → 读取release notes → 
解析repo信息 → GitHub API发布 → 
可选代理加速链接
```

#### 3.1.4 sync - 同步系统
**子模块**:
- `discovery`: 项目发现和添加
- `version`: 版本信息同步
- `versioning`: 版本管理策略
- `author`: 作者信息同步
- `registry`: 项目注册表

**核心功能**:
- 多项目版本同步
- 智能版本递增
- update.json同步
- module.prop同步

#### 3.1.5 check - 静态检查
**检查项**:
- Shell脚本语法 (ShellCheck)
- module.prop验证
- 依赖完整性
- 项目结构验证

#### 3.1.6 install - 设备安装
**支持的目标**:
```rust
pub enum InstallTarget {
    Magisk,        // Magisk Manager
    Kernelsu,      // KernelSU Manager
    Apatch,        // APatch Manager
    Avd,           // Android Virtual Device
    Physical,      // 物理设备 (ADB)
}
```

**特性**:
- ADB集成
- 多种管理器支持
- 权限验证
- Dry-run模式

#### 3.1.7 run - 脚本运行
**功能**: 执行项目定义的脚本
**来源**: `rmmproject.toml` 中的 `[project.scripts]`

### 3.2 核心系统 (core/)

#### 3.2.1 types - 类型系统
**核心数据结构**:

```rust
// 项目配置 (rmmproject.toml)
pub struct RmmProject {
    pub project: ProjectTable,      // PEP 621风格项目元数据
    pub tool: Option<ToolTable>,    // tool.rmm配置
}

// 模块属性 (module.prop)
pub struct ModuleProp {
    pub id: String,                 // 模块ID (必需)
    pub name: Option<String>,
    pub version: Option<String>,
    pub version_code: Option<u64>,
    pub author: Option<Author>,
    pub description: Option<String>,
    pub extra: BTreeMap<String, String>,
}

// 更新元数据 (update.json)
pub struct UpdateJson {
    // Magisk更新规范字段
    // (详见types.rs 280行+)
}

// 依赖规范
pub struct DependencySpec {
    pub source: String,             // github:user/repo
    pub source_type: Option<SourceType>,
    pub version: Option<String>,
    pub asset: Option<String>,
    pub alias: Option<String>,
    pub meta: Option<JsonValue>,
}

// 已解析依赖
pub struct ResolvedDependency {
    pub spec: DependencySpec,
    pub resolved_version: String,
    pub download_url: String,
    pub sha256: Option<String>,
    pub size: Option<u64>,
    pub source: String,
    pub meta: Option<BTreeMap<String, JsonValue>>,
}

// 锁文件 (rmm.lock)
pub struct RmmLock {
    pub lock_version: String,
    pub generated_at: String,
    pub entries: BTreeMap<String, DependencyLockEntry>,
}
```

#### 3.2.2 deps - 依赖管理系统
**设计模式**: Resolver + Fetcher架构

**支持的依赖源**:
```rust
pub enum SourceType {
    Github,    // GitHub Releases
    Git,       // Git仓库
    Http,      // HTTP/HTTPS URL
    Local,     // 本地路径
    Other(String),
}
```

**关键接口**:
```rust
trait DependencyResolver {
    fn resolve(&self, spec: &DependencySpec) -> Result<ResolvedDependency>;
    fn resolve_many(&self, specs: &[DependencySpec]) -> Result<Vec<ResolvedDependency>>;
}

trait ArtifactFetcher {
    fn fetch(&self, resolved: &ResolvedDependency, cache_dir: &Path) -> Result<PathBuf>;
}
```

**依赖管理器**:
```rust
pub struct DependencyManager;
impl DependencyManager {
    // 添加依赖
    pub fn add_dependency(...) -> Result<ResolvedDependency>;
    // 移除依赖
    pub fn remove_dependency(...) -> Result<()>;
    // 列出依赖
    pub fn list_dependencies(...) -> Result<Vec<ResolvedDependency>>;
    // 从锁文件安装
    pub fn install_from_lock(...) -> Result<()>;
    // 更新锁文件
    pub fn update_lock_entry(...) -> Result<()>;
    // 确保一致性
    pub fn ensure_consistency(...) -> Result<()>;
}
```

#### 3.2.3 runtime - 运行时环境
**支持的目标运行时**:
```rust
pub enum RuntimeTarget {
    Magisk,
    Kernelsu,
    Apatch,
    Unknown,
}
```

**关键功能**:
- 运行时环境检测
- 模块验证
- 目标渲染 (为特定运行时生成文件)
- 打包策略

**特性检测**:
```rust
pub fn detect_runtime_environment(env: &HashMap<String, String>) -> RuntimeTarget;
pub fn validate_module_for_runtime(...) -> RuntimeResult<()>;
```

#### 3.2.4 env - 环境配置
**环境变量管理**:
```rust
pub struct EnvConfig {
    pub github_token: Option<String>,     // GitHub API令牌
    pub proxy_urls: Vec<String>,          // 代理URL列表
    pub adb_path: Option<String>,         // ADB路径
    pub shellcheck_path: Option<String>,  // ShellCheck路径
    // ... 其他配置
}
```

**代理管理器**:
```rust
pub struct ProxyManager {
    proxies: Vec<String>,
    current_index: usize,
}
impl ProxyManager {
    pub fn new(proxies: Vec<String>) -> Self;
    pub fn current(&self) -> Option<&str>;
    pub fn advance(&mut self) -> Option<&str>;  // 故障转移
}
```

#### 3.2.5 git - Git集成
**功能**:
- Git仓库检测
- 作者信息提取
- 仓库根路径查找

```rust
pub struct GitAnalyzer;
impl GitAnalyzer {
    pub fn analyze_git_info(path: &Path) -> Result<Option<GitInfo>>;
    pub fn find_git_root(path: &Path) -> Result<Option<PathBuf>>;
    pub fn read_user_config(repo_root: &Path) -> Result<(String, String)>;
}
```

#### 3.2.6 features - 功能管理
**设计目的**: 管理可选功能和实验性特性

```rust
pub enum Feature {
    WebUI,              // WebUI支持
    ZygiskSupport,      // Zygisk模块支持
    ServiceD,           // service.d脚本支持
    AutoVersionBump,    // 自动版本递增
    AiOptimization,     // AI优化
    // ... 更多特性
}

pub enum SupportLevel {
    Stable,
    Experimental,
    Deprecated,
}

pub struct FeaturesManager;
impl FeaturesManager {
    pub fn list_features(&self) -> Vec<FeatureSpec>;
    pub fn is_supported(&self, feature: Feature) -> bool;
    pub fn enable_feature(&self, feature: Feature) -> Result<()>;
    pub fn disable_feature(&self, feature: Feature) -> Result<()>;
}
```

#### 3.2.7 packaging - 打包系统
**支持的格式**:
- ZIP (Magisk/KernelSU标准格式)
- TAR.GZ (源码打包)

**关键功能**:
```rust
// 创建ZIP归档
pub fn create_zip_archive(source_dir: &Path, output: W) -> Result<()>;

// 创建TAR.GZ归档
pub fn create_tar_gz_archive(source_dir: &Path, output_path: &Path) -> Result<()>;

// 执行构建钩子
pub fn execute_prebuild(project_path: &Path) -> Result<()>;
pub fn execute_postbuild(project_path: &Path) -> Result<()>;

// 行尾标准化
pub fn copy_file_with_line_ending_normalization(src: &Path, dst: &Path) -> Result<()>;

// 排除规则应用
pub fn apply_exclusions_and_collect_paths(project_path: &Path) -> Result<Vec<PathBuf>>;
```

#### 3.2.8 author - 作者管理
**作者信息合并策略**:
优先级: primary > fallback > fallback2

**来源**:
1. module.prop
2. Git配置
3. MetaConfig (全局默认)

```rust
pub fn merge_authors(
    primary: Option<&Author>, 
    fallback: Option<&Author>, 
    fallback2: Option<&Author>
) -> Author;
```

## 4. 配置文件规范

### 4.1 rmmproject.toml
**灵感来源**: Python的 `pyproject.toml` (PEP 621)

```toml
[project]
id = "RootModuleMaker"
description = "A Rmm project: RootModuleMaker"
readme = "README.md"
changelog = "CHANGELOG.md"
license = "LICENSE"
dependencies = []

[project.scripts]
install = "rmm device install"
test = "rmm test"
build = "rmm build"

[[authors]]
name = "LIghtJUNction"
email = "LIghtJUNction.me@gmail.com"

[urls]
github = "https://github.com/MemDeco-WG/RootModuleMaker"

[build-system]
requires = ["rmm>=0.3.0"]
build-backend = "rmm"

[tool.rmm.build]
build = ["rmm"]
exclude = [".git", ".rmmp", "*.tmp", "*.log"]
include = []
postbuild = ["echo 'Build completed'"]
prebuild = ["echo 'Starting build'"]

[tool.rmm.build.scripts]
clean = '''...'''

[tool.rmm.build.src]
exclude = [".git", "*.tmp", "*.log", "node_modules"]
include = []
```

### 4.2 module.prop
**标准Magisk/KernelSU格式**:
```properties
id=module_id
name=Module Name
version=1.0.0
versionCode=1
author=Author Name
description=Module Description
```

**验证规则**:
- `id`: 必需，匹配正则 `^[a-zA-Z][a-zA-Z0-9._-]+$`
- `versionCode`: 必需，非负整数

### 4.3 update.json
**Magisk更新规范**:
```json
{
  "version": "1.0.0",
  "versionCode": 1,
  "zipUrl": "https://example.com/module.zip",
  "changelog": "https://example.com/changelog.md"
}
```

### 4.4 rmm.lock
**依赖锁文件**:
```toml
lock_version = "1.0"
generated_at = "2025-11-07T19:39:13Z"

[entries]
[entries.dependency_id]
id = "dependency_id"
resolved_version = "1.0.0"
download_url = "https://..."
sha256 = "..."
source = "github:user/repo"
installed_path = "modules/dependency"
```

## 5. 工作流程设计

### 5.1 典型开发流程
```
1. 初始化项目
   rmm init --template basic my-module
   ↓
2. 编辑项目文件
   - 修改 rmmproject.toml
   - 编写脚本和代码
   ↓
3. 添加依赖 (可选)
   rmm add github:user/dependency@1.0.0
   ↓
4. 检查代码
   rmm check --shellcheck
   ↓
5. 构建模块
   rmm build
   ↓
6. 测试 (AVD或物理设备)
   rmm install --target avd
   或
   rmm install --target physical
   ↓
7. 发布
   rmm publish --owner user --repo my-module
```

### 5.2 构建流程详解
```
开始
 ↓
加载 rmmproject.toml
 ↓
执行 prebuild 脚本
 ↓
收集源文件
 ├─ 应用 exclude 规则
 ├─ 应用 include 规则
 └─ 行尾标准化
 ↓
静态检查 (ShellCheck)
 ↓
生成 module.prop / update.json
 ↓
创建 ZIP 归档
 ↓
执行 postbuild 脚本
 ↓
输出构建产物
 ↓
结束
```

### 5.3 依赖解析流程
```
依赖声明 (DependencySpec)
 ↓
解析源类型
 ├─ GitHub → GitHubResolver
 ├─ Git → GitResolver
 ├─ HTTP → HttpResolver
 └─ Local → LocalResolver
 ↓
解析具体版本
 ↓
生成 ResolvedDependency
 ↓
下载 Artifact (ArtifactFetcher)
 ↓
验证 SHA256 (如果提供)
 ↓
安装到项目
 ↓
更新 rmm.lock
```

## 6. 扩展功能 (Python层)

### 6.1 AI功能 (src/pyrmm/ai/)
**MCP服务器**:
- stdio模式
- SSE模式

**功能**:
- 代码审计
- 模块优化建议
- 自动修复

### 6.2 CLI扩展 (src/pyrmm/cli/)
**增强命令**:
- `rmm complete`: Shell补全生成
- `rmmcp`: MCP服务器入口

### 6.3 代理管理 (src/pyrmm/utils/proxy.py)
**功能**:
- GitHub加速
- Release下载加速
- 故障转移

## 7. 测试策略

### 7.1 单元测试
**覆盖模块**:
- `test_version.rs`: 版本解析和比较
- `test_build.rs`: 构建系统
- `test_init.rs`: 初始化功能
- `test_run.rs`: 脚本运行
- `test_sync.rs`: 同步系统

### 7.2 集成测试
**测试场景**:
1. 完整构建流程
2. 依赖安装和解析
3. 多项目同步
4. AVD测试环境

### 7.3 虚拟机测试
**功能**: 使用AVD仿真测试模块
**依赖**: rootAVD (assets/rootAVD)

## 8. 外部依赖

### 8.1 必需依赖
- **Rust工具链**: rustc, cargo
- **Python**: >= 3.11
- **uv**: Python包管理器
- **maturin**: Rust-Python绑定

### 8.2 可选依赖
- **ShellCheck**: Shell脚本静态分析
- **ADB**: Android调试桥
- **Git**: 版本控制
- **rootAVD**: AVD Root工具

### 8.3 Rust依赖 (关键)
```toml
clap = "4.5.40"           # CLI解析
anyhow = "1.0.100"        # 错误处理
serde = "1.0.228"         # 序列化
toml = "0.9.7"            # TOML解析
reqwest = "0.12.23"       # HTTP客户端
tokio = "1.48.0"          # 异步运行时
git2 = "0.20.2"           # Git绑定
zip = "6.0.0"             # ZIP归档
walkdir = "2.5.0"         # 目录遍历
pyo3 = "0.27.1"           # Python绑定 (可选)
```

## 9. 未来规划

### 9.1 开发中功能
- [ ] 多项目合并构建
- [ ] 依赖管理 (部分实现)
- [ ] 模块仓库
- [ ] 快捷安装至物理机
- [ ] 通知系统 (Telegram/Discord/QQ/酷安)

### 9.2 实验性功能
- [ ] 虚拟机仿真模块测试 (实验中)
- [ ] WebUI支持
- [ ] Zygisk模块支持

### 9.3 计划支持
- [ ] GPG签名
- [ ] 模块模板市场
- [ ] CI/CD集成增强
- [ ] 增量更新支持

## 10. 性能优化考虑

### 10.1 为什么选择Rust
1. **高性能**: 构建和打包操作密集，Rust提供原生性能
2. **内存安全**: 无GC停顿，可预测的资源使用
3. **并发支持**: Tokio异步运行时，高效网络IO
4. **跨平台**: 单一二进制，无运行时依赖

### 10.2 混合架构优势
1. **灵活部署**: 
   - 纯Rust二进制: 快速启动，无Python依赖
   - Python扩展: 完整功能，AI集成
2. **开发效率**: Python快速原型，Rust性能关键路径
3. **生态系统**: 利用Rust性能 + Python丰富库

### 10.3 缓存策略
```rust
pub struct CacheItem<T> {
    data: T,
    expires_at: Instant,
}
```
- 依赖解析结果缓存
- GitHub API响应缓存
- 模块元数据缓存

## 11. 安全考虑

### 11.1 依赖安全
- SHA256校验
- 依赖锁文件
- 来源验证

### 11.2 敏感信息
- GitHub Token从环境变量读取
- 不在配置文件中存储密钥
- .env文件支持 (通过dotenvy)

### 11.3 代码审计
- ShellCheck静态分析
- 模块prop验证
- ID正则验证

## 12. 错误处理

### 12.1 错误类型
```rust
pub enum CoreError {
    Io(std::io::Error),
    Parse(String),
    Validation(String),
    Network(String),
    Git(String),
    // ... 更多错误类型
}
```

### 12.2 错误传播
- 使用 `anyhow::Result` 在应用层
- 使用 `CoreError` 在核心库
- 提供详细错误上下文

## 13. 总结

### 13.1 项目优势
1. ✅ **混合架构**: Rust性能 + Python灵活性
2. ✅ **完整工具链**: 从init到publish全流程支持
3. ✅ **多运行时支持**: Magisk/KernelSU/APatch
4. ✅ **现代化设计**: 类PEP 621配置，依赖管理
5. ✅ **开发者友好**: Shell补全，详细错误信息

### 13.2 设计原则
1. **最小依赖**: 核心功能纯Rust，可选Python扩展
2. **约定优于配置**: 合理默认值，灵活覆盖
3. **安全第一**: 验证、校验、审计
4. **用户体验**: 清晰输出，有用的错误信息
5. **可扩展性**: 插件系统，模板支持

### 13.3 核心价值主张
> "轻量模块开发工具集 — 从创建到构建、测试到发布的一站式工作流"

RMM不仅仅是一个构建工具，而是一个完整的模块开发生态系统，旨在让模块开发者专注于模块逻辑本身，而不是繁琐的构建、测试和发布流程。

---

**文档版本**: 1.0  
**生成工具**: 基于modules.tree和symbols.txt的结构化分析  
**维护者**: RMM开发团队
