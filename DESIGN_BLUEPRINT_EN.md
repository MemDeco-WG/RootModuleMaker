# RootModuleMaker (RMM) Design Blueprint

> Analysis Date: 2025-11-07
> Version: v0.4.0

## 1. Project Overview

### 1.1 Project Positioning
RootModuleMaker (RMM) is a lightweight module development toolkit designed to provide an end-to-end workflow for Android Root modules (Magisk/KernelSU/APatch) from creation to building, testing, and publishing.

### 1.2 Technology Stack
- **Core Language**: Rust (high-performance core) + Python (extended features)
- **Build Tools**: Cargo, Maturin (Python-Rust bindings)
- **Package Management**: uv (Python), Cargo (Rust)
- **CLI Framework**: clap (Rust), click (Python)

### 1.3 Architecture Pattern
Hybrid Architecture:
- **Rust Core**: Provides high-performance binary tools and optional Python extensions
- **Python Layer**: Provides AI features, MCP server, and advanced CLI functions
- **Dual Operation Modes**:
  1. Pure Rust Binary (no Python dependencies, fast deployment)
  2. Python Extension Mode (full feature set)

## 2. Project Structure Analysis

### 2.1 Directory Structure
```
RootModuleMaker/
├── rust/                      # Rust core implementation
│   ├── src/
│   │   ├── main.rs           # Binary entry point
│   │   ├── lib.rs            # Library entry point (Python extension)
│   │   ├── cmds/             # Command implementation modules
│   │   │   ├── build.rs      # Build command
│   │   │   ├── init.rs       # Initialize command
│   │   │   ├── publish.rs    # Publish command
│   │   │   ├── sync/         # Sync-related commands
│   │   │   ├── check.rs      # Check command
│   │   │   ├── install.rs    # Install command
│   │   │   ├── run.rs        # Run script command
│   │   │   └── version.rs    # Version management
│   │   └── core/             # Core functionality modules
│   │       ├── types.rs      # Core data types
│   │       ├── env.rs        # Environment configuration
│   │       ├── deps.rs       # Dependency management
│   │       ├── git.rs        # Git integration
│   │       ├── runtime.rs    # Runtime environment detection
│   │       ├── features.rs   # Feature management
│   │       ├── packaging.rs  # Packaging functionality
│   │       └── author.rs     # Author information management
│   └── Cargo.toml            # Rust project configuration
├── src/pyrmm/                # Python extension source
│   ├── cli/                  # CLI implementation
│   ├── ai/                   # AI features (MCP server)
│   └── utils/                # Utility functions
├── docs/                     # Documentation directory
├── assets/                   # Asset files (rootAVD, etc.)
├── pyproject.toml            # Python project configuration
└── rmmproject.toml           # RMM project metadata
```

### 2.2 Module Dependency Graph

Based on `modules.tree` analysis:
```
crate rmmcore
├── mod cmds: pub             # Command modules (public API)
│   ├── build                 # Build system
│   ├── init                  # Project initialization
│   ├── publish               # Release management
│   ├── sync                  # Sync system
│   ├── check                 # Code checking
│   ├── install               # Install to device
│   ├── run                   # Script execution
│   └── version               # Version management
├── mod core: pub             # Core functionality (public API)
│   ├── types                 # Type definitions
│   ├── env                   # Environment management
│   ├── deps                  # Dependency system
│   ├── git                   # Git integration
│   ├── runtime               # Runtime
│   ├── features              # Feature management
│   ├── packaging             # Packaging system
│   └── author                # Author management
└── fn init: pub              # Initialization function
```

## 3. Core Functionality Design

### 3.1 Command System (cmds/)

#### 3.1.1 build - Build System
**Function**: Build Magisk/KernelSU/APatch modules
**Key Features**:
- Support for prebuild/postbuild script hooks
- ShellCheck static syntax checking
- Line ending normalization
- Exclude/include file management
- Multi-format packaging (ZIP, TAR.GZ)

**Data Flow**:
```
rmmproject.toml → BuildArgs → prebuild → File collection → 
ShellCheck → Packaging → postbuild → Output ZIP
```

#### 3.1.2 init - Project Initialization
**Function**: Create new projects from templates
**Supported Templates**:
- Basic template
- Custom template system

**Implementation Points**:
- Recursive template file copying
- Project metadata generation
- Git repository initialization

#### 3.1.3 publish - Release Management
**Function**: Publish modules to GitHub Releases
**Features**:
- GitHub API integration
- Release notes support
- Proxy acceleration URL generation
- Artifact packaging

**Workflow**:
```
Prepare Artifact → Read release notes → 
Parse repo info → GitHub API publish → 
Optional proxy acceleration link
```

#### 3.1.4 sync - Sync System
**Submodules**:
- `discovery`: Project discovery and addition
- `version`: Version information sync
- `versioning`: Version management strategies
- `author`: Author information sync
- `registry`: Project registry

**Core Functions**:
- Multi-project version sync
- Smart version incrementing
- update.json sync
- module.prop sync

#### 3.1.5 check - Static Checking
**Check Items**:
- Shell script syntax (ShellCheck)
- module.prop validation
- Dependency integrity
- Project structure validation

#### 3.1.6 install - Device Installation
**Supported Targets**:
```rust
pub enum InstallTarget {
    Magisk,        // Magisk Manager
    Kernelsu,      // KernelSU Manager
    Apatch,        // APatch Manager
    Avd,           // Android Virtual Device
    Physical,      // Physical device (ADB)
}
```

**Features**:
- ADB integration
- Multiple manager support
- Permission validation
- Dry-run mode

#### 3.1.7 run - Script Execution
**Function**: Execute project-defined scripts
**Source**: `[project.scripts]` in `rmmproject.toml`

### 3.2 Core System (core/)

#### 3.2.1 types - Type System
**Core Data Structures**:

```rust
// Project configuration (rmmproject.toml)
pub struct RmmProject {
    pub project: ProjectTable,      // PEP 621-style project metadata
    pub tool: Option<ToolTable>,    // tool.rmm configuration
}

// Module properties (module.prop)
pub struct ModuleProp {
    pub id: String,                 // Module ID (required)
    pub name: Option<String>,
    pub version: Option<String>,
    pub version_code: Option<u64>,
    pub author: Option<Author>,
    pub description: Option<String>,
    pub extra: BTreeMap<String, String>,
}

// Update metadata (update.json)
pub struct UpdateJson {
    // Magisk update specification fields
    // (see types.rs line 280+)
}

// Dependency specification
pub struct DependencySpec {
    pub source: String,             // github:user/repo
    pub source_type: Option<SourceType>,
    pub version: Option<String>,
    pub asset: Option<String>,
    pub alias: Option<String>,
    pub meta: Option<JsonValue>,
}

// Resolved dependency
pub struct ResolvedDependency {
    pub spec: DependencySpec,
    pub resolved_version: String,
    pub download_url: String,
    pub sha256: Option<String>,
    pub size: Option<u64>,
    pub source: String,
    pub meta: Option<BTreeMap<String, JsonValue>>,
}

// Lock file (rmm.lock)
pub struct RmmLock {
    pub lock_version: String,
    pub generated_at: String,
    pub entries: BTreeMap<String, DependencyLockEntry>,
}
```

#### 3.2.2 deps - Dependency Management System
**Design Pattern**: Resolver + Fetcher architecture

**Supported Dependency Sources**:
```rust
pub enum SourceType {
    Github,    // GitHub Releases
    Git,       // Git repositories
    Http,      // HTTP/HTTPS URLs
    Local,     // Local paths
    Other(String),
}
```

**Key Interfaces**:
```rust
trait DependencyResolver {
    fn resolve(&self, spec: &DependencySpec) -> Result<ResolvedDependency>;
    fn resolve_many(&self, specs: &[DependencySpec]) -> Result<Vec<ResolvedDependency>>;
}

trait ArtifactFetcher {
    fn fetch(&self, resolved: &ResolvedDependency, cache_dir: &Path) -> Result<PathBuf>;
}
```

**Dependency Manager**:
```rust
pub struct DependencyManager;
impl DependencyManager {
    // Add dependency
    pub fn add_dependency(...) -> Result<ResolvedDependency>;
    // Remove dependency
    pub fn remove_dependency(...) -> Result<()>;
    // List dependencies
    pub fn list_dependencies(...) -> Result<Vec<ResolvedDependency>>;
    // Install from lockfile
    pub fn install_from_lock(...) -> Result<()>;
    // Update lock entry
    pub fn update_lock_entry(...) -> Result<()>;
    // Ensure consistency
    pub fn ensure_consistency(...) -> Result<()>;
}
```

#### 3.2.3 runtime - Runtime Environment
**Supported Target Runtimes**:
```rust
pub enum RuntimeTarget {
    Magisk,
    Kernelsu,
    Apatch,
    Unknown,
}
```

**Key Functions**:
- Runtime environment detection
- Module validation
- Target rendering (generate files for specific runtimes)
- Packaging strategies

**Feature Detection**:
```rust
pub fn detect_runtime_environment(env: &HashMap<String, String>) -> RuntimeTarget;
pub fn validate_module_for_runtime(...) -> RuntimeResult<()>;
```

#### 3.2.4 env - Environment Configuration
**Environment Variable Management**:
```rust
pub struct EnvConfig {
    pub github_token: Option<String>,     // GitHub API token
    pub proxy_urls: Vec<String>,          // Proxy URL list
    pub adb_path: Option<String>,         // ADB path
    pub shellcheck_path: Option<String>,  // ShellCheck path
    // ... other configurations
}
```

**Proxy Manager**:
```rust
pub struct ProxyManager {
    proxies: Vec<String>,
    current_index: usize,
}
impl ProxyManager {
    pub fn new(proxies: Vec<String>) -> Self;
    pub fn current(&self) -> Option<&str>;
    pub fn advance(&mut self) -> Option<&str>;  // Failover
}
```

#### 3.2.5 git - Git Integration
**Functions**:
- Git repository detection
- Author information extraction
- Repository root path finding

```rust
pub struct GitAnalyzer;
impl GitAnalyzer {
    pub fn analyze_git_info(path: &Path) -> Result<Option<GitInfo>>;
    pub fn find_git_root(path: &Path) -> Result<Option<PathBuf>>;
    pub fn read_user_config(repo_root: &Path) -> Result<(String, String)>;
}
```

#### 3.2.6 features - Feature Management
**Design Purpose**: Manage optional features and experimental capabilities

```rust
pub enum Feature {
    WebUI,              // WebUI support
    ZygiskSupport,      // Zygisk module support
    ServiceD,           // service.d script support
    AutoVersionBump,    // Automatic version incrementing
    AiOptimization,     // AI optimization
    // ... more features
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

#### 3.2.7 packaging - Packaging System
**Supported Formats**:
- ZIP (Magisk/KernelSU standard format)
- TAR.GZ (source code packaging)

**Key Functions**:
```rust
// Create ZIP archive
pub fn create_zip_archive(source_dir: &Path, output: W) -> Result<()>;

// Create TAR.GZ archive
pub fn create_tar_gz_archive(source_dir: &Path, output_path: &Path) -> Result<()>;

// Execute build hooks
pub fn execute_prebuild(project_path: &Path) -> Result<()>;
pub fn execute_postbuild(project_path: &Path) -> Result<()>;

// Line ending normalization
pub fn copy_file_with_line_ending_normalization(src: &Path, dst: &Path) -> Result<()>;

// Apply exclusion rules
pub fn apply_exclusions_and_collect_paths(project_path: &Path) -> Result<Vec<PathBuf>>;
```

#### 3.2.8 author - Author Management
**Author Information Merge Strategy**:
Priority: primary > fallback > fallback2

**Sources**:
1. module.prop
2. Git configuration
3. MetaConfig (global default)

```rust
pub fn merge_authors(
    primary: Option<&Author>, 
    fallback: Option<&Author>, 
    fallback2: Option<&Author>
) -> Author;
```

## 4. Configuration File Specifications

### 4.1 rmmproject.toml
**Inspired by**: Python's `pyproject.toml` (PEP 621)

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
**Standard Magisk/KernelSU format**:
```properties
id=module_id
name=Module Name
version=1.0.0
versionCode=1
author=Author Name
description=Module Description
```

**Validation Rules**:
- `id`: Required, matches regex `^[a-zA-Z][a-zA-Z0-9._-]+$`
- `versionCode`: Required, non-negative integer

### 4.3 update.json
**Magisk update specification**:
```json
{
  "version": "1.0.0",
  "versionCode": 1,
  "zipUrl": "https://example.com/module.zip",
  "changelog": "https://example.com/changelog.md"
}
```

### 4.4 rmm.lock
**Dependency lockfile**:
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

## 5. Workflow Design

### 5.1 Typical Development Flow
```
1. Initialize project
   rmm init --template basic my-module
   ↓
2. Edit project files
   - Modify rmmproject.toml
   - Write scripts and code
   ↓
3. Add dependencies (optional)
   rmm add github:user/dependency@1.0.0
   ↓
4. Check code
   rmm check --shellcheck
   ↓
5. Build module
   rmm build
   ↓
6. Test (AVD or physical device)
   rmm install --target avd
   or
   rmm install --target physical
   ↓
7. Publish
   rmm publish --owner user --repo my-module
```

### 5.2 Build Flow Details
```
Start
 ↓
Load rmmproject.toml
 ↓
Execute prebuild scripts
 ↓
Collect source files
 ├─ Apply exclude rules
 ├─ Apply include rules
 └─ Line ending normalization
 ↓
Static checking (ShellCheck)
 ↓
Generate module.prop / update.json
 ↓
Create ZIP archive
 ↓
Execute postbuild scripts
 ↓
Output build artifacts
 ↓
End
```

### 5.3 Dependency Resolution Flow
```
Dependency declaration (DependencySpec)
 ↓
Parse source type
 ├─ GitHub → GitHubResolver
 ├─ Git → GitResolver
 ├─ HTTP → HttpResolver
 └─ Local → LocalResolver
 ↓
Resolve specific version
 ↓
Generate ResolvedDependency
 ↓
Download Artifact (ArtifactFetcher)
 ↓
Verify SHA256 (if provided)
 ↓
Install to project
 ↓
Update rmm.lock
```

## 6. Extended Features (Python Layer)

### 6.1 AI Features (src/pyrmm/ai/)
**MCP Server**:
- stdio mode
- SSE mode

**Functions**:
- Code auditing
- Module optimization suggestions
- Automatic fixes

### 6.2 CLI Extensions (src/pyrmm/cli/)
**Enhanced Commands**:
- `rmm complete`: Shell completion generation
- `rmmcp`: MCP server entry point

### 6.3 Proxy Management (src/pyrmm/utils/proxy.py)
**Functions**:
- GitHub acceleration
- Release download acceleration
- Failover

## 7. Testing Strategy

### 7.1 Unit Tests
**Covered Modules**:
- `test_version.rs`: Version parsing and comparison
- `test_build.rs`: Build system
- `test_init.rs`: Initialization functionality
- `test_run.rs`: Script execution
- `test_sync.rs`: Sync system

### 7.2 Integration Tests
**Test Scenarios**:
1. Complete build workflow
2. Dependency installation and resolution
3. Multi-project sync
4. AVD test environment

### 7.3 Virtual Machine Testing
**Function**: Test modules using AVD emulation
**Dependencies**: rootAVD (assets/rootAVD)

## 8. External Dependencies

### 8.1 Required Dependencies
- **Rust toolchain**: rustc, cargo
- **Python**: >= 3.11
- **uv**: Python package manager
- **maturin**: Rust-Python bindings

### 8.2 Optional Dependencies
- **ShellCheck**: Shell script static analysis
- **ADB**: Android Debug Bridge
- **Git**: Version control
- **rootAVD**: AVD root tool

### 8.3 Rust Dependencies (Key)
```toml
clap = "4.5.40"           # CLI parsing
anyhow = "1.0.100"        # Error handling
serde = "1.0.228"         # Serialization
toml = "0.9.7"            # TOML parsing
reqwest = "0.12.23"       # HTTP client
tokio = "1.48.0"          # Async runtime
git2 = "0.20.2"           # Git bindings
zip = "6.0.0"             # ZIP archiving
walkdir = "2.5.0"         # Directory traversal
pyo3 = "0.27.1"           # Python bindings (optional)
```

## 9. Future Plans

### 9.1 In Development
- [ ] Multi-project merged builds
- [ ] Dependency management (partial implementation)
- [ ] Module repository
- [ ] Quick install to physical device
- [ ] Notification system (Telegram/Discord/QQ/Coolapk)

### 9.2 Experimental Features
- [ ] Virtual machine module testing (experimental)
- [ ] WebUI support
- [ ] Zygisk module support

### 9.3 Planned Support
- [ ] GPG signing
- [ ] Module template marketplace
- [ ] Enhanced CI/CD integration
- [ ] Incremental update support

## 10. Performance Optimization Considerations

### 10.1 Why Choose Rust
1. **High Performance**: Build and packaging operations are intensive; Rust provides native performance
2. **Memory Safety**: No GC pauses, predictable resource usage
3. **Concurrency Support**: Tokio async runtime, efficient network IO
4. **Cross-platform**: Single binary, no runtime dependencies

### 10.2 Hybrid Architecture Advantages
1. **Flexible Deployment**: 
   - Pure Rust binary: Fast startup, no Python dependencies
   - Python extension: Full features, AI integration
2. **Development Efficiency**: Python for rapid prototyping, Rust for performance-critical paths
3. **Ecosystem**: Leverage Rust performance + Python rich libraries

### 10.3 Caching Strategy
```rust
pub struct CacheItem<T> {
    data: T,
    expires_at: Instant,
}
```
- Dependency resolution result caching
- GitHub API response caching
- Module metadata caching

## 11. Security Considerations

### 11.1 Dependency Security
- SHA256 verification
- Dependency lockfile
- Source verification

### 11.2 Sensitive Information
- GitHub Token read from environment variables
- No key storage in configuration files
- .env file support (via dotenvy)

### 11.3 Code Auditing
- ShellCheck static analysis
- Module prop validation
- ID regex validation

## 12. Error Handling

### 12.1 Error Types
```rust
pub enum CoreError {
    Io(std::io::Error),
    Parse(String),
    Validation(String),
    Network(String),
    Git(String),
    // ... more error types
}
```

### 12.2 Error Propagation
- Use `anyhow::Result` at application layer
- Use `CoreError` in core library
- Provide detailed error context

## 13. Summary

### 13.1 Project Advantages
1. ✅ **Hybrid Architecture**: Rust performance + Python flexibility
2. ✅ **Complete Toolchain**: Full workflow support from init to publish
3. ✅ **Multi-runtime Support**: Magisk/KernelSU/APatch
4. ✅ **Modern Design**: PEP 621-like configuration, dependency management
5. ✅ **Developer-friendly**: Shell completion, detailed error messages

### 13.2 Design Principles
1. **Minimal Dependencies**: Pure Rust core, optional Python extensions
2. **Convention over Configuration**: Sensible defaults, flexible overrides
3. **Security First**: Validation, verification, auditing
4. **User Experience**: Clear output, helpful error messages
5. **Extensibility**: Plugin system, template support

### 13.3 Core Value Proposition
> "Lightweight module development toolkit — end-to-end workflow from creation to building, testing, and publishing"

RMM is not just a build tool, but a complete module development ecosystem designed to let module developers focus on module logic itself, rather than tedious build, test, and release processes.

---

**Document Version**: 1.0  
**Generation Tool**: Structured analysis based on modules.tree and symbols.txt  
**Maintainer**: RMM Development Team
