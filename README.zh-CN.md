<h1 align="center">

```
░█▀▄░█▀█░█▀█░█▀█░█░█
░█░█░█░█░█▀▀░█▀█░▄▀▄
░▀▀░░▀▀▀░▀░░░▀░▀░▀░▀
```

````

</h1>

<p align="center">
  <strong>为 ADHD 大脑的真实工作方式调校的终端智能体。</strong><br>
  基于 <a href="https://github.com/openai/codex">openai/codex</a> 的 fork —— 换了名字、重写了提示词、重新接线，让开始、继续、完成一件事的摩擦更小。
</p>

<p align="center">
  <a href="#-为什么做-dopax">理念</a> ·
  <a href="#-与上游的差异">差异</a> ·
  <a href="#-安装--构建">安装</a> ·
  <a href="#️-配置">配置</a> ·
  <a href="#-上游同步">上游同步</a> ·
  <a href="#-许可证">许可</a>
</p>

<p align="center">
  <a href="README.zh-CN.md">简体中文</a> | <a href="README.md">English</a>
</p>

---

## 🧠 为什么做 Dopax

通用的编码助手默认用户是神经典型（neurotypical）的工作模式：上下文干净、节奏稳定、任务启动毫无阻力。Dopax 把相反的假设作为**静默的、不带病理化标签的默认值**——轻度执行功能障碍、启动摩擦高、时间感知偏差——并悄悄把脚手架搭进去：

- **不贴标签，不打扰式关怀。** 它绝不会把"ADHD"说回给你，也不会问"你今天心情如何"。关怀以*结构*的形式出现：更小的第一步、更清晰的选项、更低的启动能量。
- **按需微切块。** 任务显得沉重时，它会被拆成"两行代码就能赢一次"的小块，而不是一整块压过来的庞然大物。
- **内建的五分钟法则。** "只做最小的一小块，做 5 分钟，之后随时可以停"是默认动作，不是干预手段。
- **补偿时间盲。** 人为的迷你截止线、短程专注冲刺、可见的时间线，取代模糊的"以后再说"。
- **稳定的、不评判的存在。** 丢掉的会话、中途重启、半途而废——它不会因此不耐烦。回来，接上，继续。

底层它依然是一个完整的编码智能体——关键在于*关系层*和*工具层*都朝着"更低摩擦"倾斜。

## ✨ 与上游的差异

Dopax 紧跟 `openai/codex`（当前基线 **0.151.0**），只在其上叠加一层小而精准的增量：

| 领域 | 变更 |
|---|---|
| **系统提示词** | 完整的 ADHD 友好关系层：隐形支持默认、任务粉碎、五分钟法则、诱惑捆绑、时间盲策略。提供 base / Claude / Codex 三种提示词变体（`codex-rs/dopax_system_prompt*.md`）。 |
| **经历管理器** | 新工具 `dopax_experience_manager`：以日期区间追踪进行中的项目、里程碑与个人成长事件。自动向上下文注入 `<current_time>` 与 `<active_experiences>`；过期或已完成的经历在启动时自动清理。（`core/src/experiences.rs`） |
| **多选提问** | 新工具 `request_user_multi_select`：让智能体以一道结构化的多选题代替反复的自由文本追问。 |
| **独立主目录** | `DOPAX_HOME` 环境变量（回落到 `CODEX_HOME`），默认 `~/.dopax` —— Dopax 与上游 Codex 可以在同一台机器共存。 |
| **自定义供应商** | `dopax login --api-key` 提供交互选择：OpenAI 官方，或任意 OpenAI 兼容的 Responses 端点（中转 / 代理 / 本地服务），并写入 `config.toml` 的 `dopax-custom` 供应商。 |
| **宽容的模型列表** | `/models` 解析同时接受严格的 Codex 后端格式、宽松的中转格式，以及标准 OpenAI `{"object":"list","data":[...]}` 格式——第三方中转也能驱动模型选择器。 |
| **Codex 导入** | 从已有的 `~/.codex` 安装一键迁移（设置、历史、会话、记忆）到 Dopax。*（向新版源适配器架构的移植进行中。）* |
| **品牌** | TUI 会话头、`/app`、`/skills`、导入流程——全部是 Dopax。 |

其余一切均来自上游：沙箱、MCP、插件、钩子、智能体面板、计划模式、记忆。

## 📦 安装 / 构建

前置要求：Rust（较新的 stable 版本），不需要 Node。

```bash
git clone https://github.com/connectedGraph/dopax.git
cd dopax/codex-rs
cargo build --release -p codex-cli
````

产物为 `target/release/dopax`（由上游 `codex` CLI 入口改名而来——命令与参数完全一致）。

```bash
# 首次运行
dopax            # 交互式 TUI
dopax login      # ChatGPT 登录；或 `dopax login --api-key` 配置自定义供应商
```

> **Windows 提示：** 构建与测试已在 Windows 11 上验证。跑测试套件请使用 `RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib` —— 默认 1 MB 测试栈在 Windows 上会溢出（原版 codex 在纯 Windows 上行为相同）。

## ⚙️ 配置

Dopax 读取 `~/.dopax/config.toml`（与上游 Codex 格式一致）。要点：

```toml
# 指向任意 OpenAI 兼容端点
[model_providers.dopax-custom]
name = "My relay"
base_url = "https://my-relay.example.com/v1"
wire_api = "responses"

model_provider = "dopax-custom"
```

- `DOPAX_HOME` 覆盖主目录；`CODEX_HOME` 仍作为回落生效。
- 已是 Codex 用户：在 TUI 里运行 `/import` 即可迁入设置、历史与会话。
- 经历管理器的数据存于 `~/.dopax/experiences.json`，受 `current_time_reminder` 功能开关控制。

## 🔄 上游同步

Dopax 是一个**薄 fork**：全部产品增量只有少量文件，因此上游发版采取整体合并而非 cherry-pick。

- 当前基线：`openai/codex` **rust-v0.151.0**（2026-08-29）
- 同步节奏：每 1–2 个上游稳定版合并一次
- 合并历史保留在 `dopax/merge-*` 分支与提交信息中

## 🗺️ 路线图

- [ ] 将"从 Codex 导入"迁移移植到新的 `external-agent-migration` 源适配器架构
- [ ] TUI 内的经历时间线可视化
- [ ] 可配置的提示词风格切换（支持型 / 中性 / 专注型）
- [ ] 发布带签名的预编译二进制

## 📄 许可证

与上游一致：[Apache-2.0](LICENSE)。

---

<p align="center">
<sub>
<code>dopax</code> —— 因为最难的是第一次提交。
</sub>
</p>
