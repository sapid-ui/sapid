# Sapid 多维代码评审与后续改进报告

评审日期：2026 年 11 月 9 日。对象：Sapid 0.7.1，提交 `679127fe2b8b9d0ba5e882b5556589a4955c2877`。

## 修复结果与验证记录

F01–F11 的修复与回归均已完成，独立 agent 复核未发现剩余阻断。代码和本完成记录统一通过 [PR 14](https://github.com/sapid-ui/sapid/pull/14) 提交，实现提交为 `e27cb269ae4e41b8d3eb3a7ab6c47a5c321a11e5`；后续报告提交没有改动组件或发布实现。合并条件是最终提交的 [PR CI](https://github.com/sapid-ui/sapid/pull/14/checks) 全部通过且评审问题清零，实际合并与 CI 状态以 PR 为准。下方初始评审保留历史事实，源码链接固定到原始基线提交，不能将其缺陷描述视作仍未修复。

| 项目 | 完成状态 | 修复及回归证据 |
| --- | --- | --- |
| F01 | 修复完成 | [ColorInput](../src/color_input.rs)：请求代次、当前许可与值版本守卫；28 项取色测试覆盖延迟结果、同步卸载、Future 析构重入和同值更新 |
| F02 | 修复完成 | [Tree](../src/tree.rs) 与 [延迟拖放测试](../tests/tree_drag_drop.rs)：原请求 Scope 取消、模型/节点身份复核、allow_drop 重入，3 项回归 |
| F03 | 修复完成 | TOC 随文档 revision 更新；真实连续导航及前进/后退逐项验证当前锚点，目录不可用空集合假通过 |
| F04 | 修复完成 | 首次 Tab 可见的 skip link 将实际焦点送入 main；CSR/hydration 均验收 |
| F05 | 修复完成 | 开发 main 与产品 pin 分开；审计三份实际 workspace 锁和实际归档的完整 registry 图，绑定源码、锁及数据库摘要 |
| F06 | 修复完成 | 发布/部署失败关闭；上传从隔离 clone 的已审 registry 锁执行；核验真实 publish 临时归档的完整文件集合与哈希，覆盖历史包恢复 |
| F07 | 修复完成 | [Sapid 自有矩阵](../benchmarks/wasm-size/README.md)：17/17 测量与 17/17 浏览器验证完成，旧证据、错哈希和不兼容基线拒绝测试通过 |
| F08 | 修复完成 | 共享非模态 Layer 提供 Escape/外点关闭；保留查询；完整 tick 后协调焦点，输入与结果之间转移不会误卸载 |
| F09 | 修复完成 | 四主题 × 四类错误输入的真实 Tab 显示独立轮廓，错误边框保留；[样式验收](../examples/web/style-acceptance.js) 共 21 场景通过 |
| F10 | 修复完成 | 移动组件菜单可直接访问 Theme Studio 等主站入口；390px 直达路径通过 |
| F11 | 修复完成 | 文档视图按钮公开 pressed 状态，保留自然 Tab；浏览器等待实际渲染状态后断言，避免并发时序假失败 |

### 最终验证

- `check.ps1 -Web -Native`、格式检查和严格 Clippy 通过。
- 组件浏览器 **110 套件、1157 条检查通过，errors=[]**；Showcase quick 并发矩阵 **38/38 套件通过**，涵盖所有行为 suite 和两个响应式宽度，区别于每日 full 页面/配色穷举。
- Showcase Rust 25 项、路由 3 项、库内示例 31 项通过；当前 Release 的桌面/移动首页、文档及 Theme Studio 共 5 张截图已视觉复核。
- 审计证据 9 项、归档边界 9 项、发布工具 39 项、真实 Cargo 1.98 负/正对照 1 项、真实 RustSec 恶意/干净 fixture 1 项、actionlint 均通过。
- 当前实现提交的真实 Sapid 归档共 **636 文件**，registry 图审计与归一化发布 dry-run 通过，原工作区状态未变；本轮没有上传 crate。
- 本地三锁扫描在数据库 `899077d9d471b9fd3dbf4b2faea8ef26e83692e8` 下均无已知漏洞，Native 的 4 条 unmaintained 通知保留。最终固定生产组合的结果见上方 CI，不能用本地 sibling 结果替代。

### 发布重打包保护

独立评审用 Cargo 1.98 复现：原 path workspace 的根锁保持不变时，`cargo publish --locked` 仍可因 registry 更新生成不同的包内锁。修复后的 [归一化发布流程](../scripts/normalized_publication.py) 在隔离 clone 中使用已审完整 registry 锁，并检查真正的 `target/package/tmp-crate`。非元数据文件的路径集合与内容必须逐一相同；manifest、锁和 `Cargo.toml.orig` 只允许已证明的转换，VCS 仍绑定冻结产品 SHA。

隔离转换可能诚实地产生 `dirty=true`。该许可仅存在于专用候选验证路径，初始源归档仍拒绝 dirty。转换记录另行绑定原审计摘要，不改写可信审计证据；历史 clean 包与已归一化包的恢复都经过回归测试。原 checkout、Cargo home、target 和 TEMP/TMP 均不因该流程被全局重定向。

### 冻结 WASM 证据

测量使用五个干净的 detached sibling worktree：Sapid `0caa8de358d324fbf8324943e8e83ed534c1429a`，生产 Glory pin `f7875f1252d2419c414741f0b861195ea9bd0d56`，Apace `e7c2f483ed454d62183efbfcb9b60e8328effaed`，Takia `c7dfdfa0ca7fc94cad5c57e2f5e93b1d91b3de03`，Wingy `d6613df82dbdf2409d9bfa8762ef431639e42fdc`。之后的提交只补发布/验证工具，组件源码及该矩阵没有再改动。

Rust 1.99.1 release、wasm-bindgen 0.2.130，无额外 Binaryen pass，Chromium 156.0.8078.4。以下为字节数，CSS 单独服务，未混入 WASM/JS：

| 场景 | WASM 原始 | WASM Brotli | JS Brotli |
| --- | ---: | ---: | ---: |
| Glory text | 1178619 | 310167 | 12916 |
| Sapid Button | 1292958 | 335406 | 12909 |
| Chart 实际使用 | 1590098 | 384110 | 12896 |
| Code 实际使用 | 1373348 | 348201 | 12908 |
| Files 实际使用 | 1283139 | 329514 | 12911 |
| Editor 实际使用 | 1444397 | 363122 | 12916 |
| JSON 实际使用 | 1448120 | 364847 | 12901 |
| Dates 实际使用 | 1681033 | 391663 | 12909 |

Sapid 未使用、六类 feature 各自未使用及 all-unused 的 WASM 与 Glory text **哈希相同**。该结论只覆盖这套冻结配置；不得与下方历史 Binaryen 数值直接比较，也不代表帧耗时或内存占用。

本地证据在 `D:/Works/glory-rs/sapid-review-verify-20261109/sapid/benchmarks/wasm-size/target/verification/sapid-wasm-size/runs/frozen-review`。`report.json` SHA-256 为 `8363d1eca2482e8d5965f8fb82db5be06a8c194d0175d819d34b7740886a3f05`，`browser.json` 为 `93c7e436b4d6635733aba09524914371f4197a79b73dda76edd7ee89269bce2b`。复现入口读取所属 workspace 的 cargo metadata，不使用已停用的全局缓存。

### 后续范围

未编号的运行时性能测量、CSP、完整国际化、系统 IME/读屏及更广泛视觉改进仍是后续建议，不因 F01–F11 完成而自动视为已经验收。代码评审已由独立 agent 执行并记录于 PR；`@codex review` 已请求，未收到 bot 回复，不冒称 bot 审批。

## 初始评审结论


当前最值得优先投入的是异步结果的状态复核、展示站连续导航，以及发布审计与实际产物的绑定。组件库已有较完整的输入预算、受控状态、Owner 生命周期和跨宿主语义设计；本轮没有确认 P0/P1 漏洞或线程数据竞争。确认了两处异步操作在控件状态变化后仍可能提交旧结果，以及展示站目录随内部导航失去同步的问题。

本轮由三个独立 agent 分别审查安全与供应链、并发与性能、界面与可访问性，主 agent 负责工程质量、复核、运行验证和汇总。只交付改进报告，没有修改生产代码、发布或部署。

## 评审基线与证据强度

起始 Sapid 工作区干净。本地 Rust 为 `1.99.1`。本轮 Rust 检查使用当前 sibling 源码组合：

| 仓库 | 提交 |
| --- | --- |
| Glory | `d20a1392e358afaf3fe6cc17481bee38cb4a0fc0` |
| Apace | `e7c2f483ed454d62183efbfcb9b60e8328effaed` |
| Takia | `c7dfdfa0ca7fc94cad5c57e2f5e93b1d91b3de03` |
| Wingy | `d6613df82dbdf2409d9bfa8762ef431639e42fdc` |

[生产依赖配置](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/deploy/dependencies.json) 固定的 Glory 为 `f7875f1252d2419c414741f0b861195ea9bd0d56`，与本地版本不同。因此，本地通过不等于重新验证生产固定组合、Rust 1.95 MSRV、原生宿主或 registry-only 消费端。

证据标记：**运行复现**表示本轮实际执行；**源码确认**表示调用链或配置已确定；**改进机会**表示尚未证明构成实际故障或性能瓶颈。P1 为高影响紧急问题，P2 为应安排修复的问题，P3 为后续增强。严重度与实施顺序分别表达。

界面检查以当前源码为基准。本地 8795 未运行，没有把 11 月 6 日的历史构建当作当前页面截图。本轮额外做了 Chromium 的输入焦点最小实验；完整页面美观度、实际遮挡面积和响应式几何仍需用当前构建复核。

## 正交维度覆盖

| 维度 | 检查重点 | 当前判断与后续方向 |
| --- | --- | --- |
| 输入安全与隐私 | URL、JSON、代码、富文本、文件 token、主题序列化 | 有实际验证与预算；未确认可直接利用的注入或任意文件访问 |
| 供应链与发布 | 源码 pin、依赖审计、工具安装、provenance | 审计对象与发布对象、审计结果与发布门禁仍有脱节，见 F05/F06 |
| 逻辑并发 | 延迟 Future、几何查询、重入、同 tick 事件 | 重点是异步结果的当前有效性，见 F01/F02 |
| 生命周期与资源 | WeakScope、Owner 清理、取消、回调捕获 | 公共设施较完整；不应将 Rc/RefCell 本身当成线程竞争缺陷 |
| 状态正确性 | controlled 输入、原子快照、无效更新、导航 revision | 基础契约明确；展示站目录存在局部陈旧状态，见 F03 |
| 算法与分配效率 | 数据表筛选、树挂载、日程展开、图表派生 | 历史热点已修；当前应先测实际搜索与大展开树，再决定优化 |
| 启动与运行性能 | WASM 体积、输入延迟、帧耗时、内存 | 需要恢复可复现的 Sapid 体积入口，并补少量运行时基线 |
| 代码质量与维护性 | 模块边界、共享语义、错误接口、测试脚本 | 格式和严格 Clippy 通过；应复用已有组件实现展示站交互 |
| 可访问性 | Tab 路径、选中状态、焦点、landmark | 缺少跳过长侧栏的路径；视图选择和错误输入焦点可增强 |
| 交互与信息架构 | 搜索关闭、连续导航、移动入口 | F03/F04/F08/F10 对应具体用户路径 |
| 视觉与元素细节 | 字号、图标、触控区域、状态表达 | 有统一主题 token；移动文字与图标一致性需要当前截图定稿 |
| 布局与响应式 | 窄屏、局部滚动、长内容、缩放 | 已有响应式规则；还应覆盖 320px、200% 字号和长文本 |
| 国际化与宿主可移植性 | 显式 Locale/RTL、SSR、hydration、Native | 边界有文档；应按公开承诺逐项验证，不由 catalog 完成率推导全平台等价 |
| 文档与可复现性 | 清单生成、构建入口、历史证据 | 清单检查通过；WASM 文档仍指向已停用路径与变化后的 sibling 矩阵 |

## 优先处理清单

| 编号 | 优先级 | 发现 | 证据 | 建议顺序 |
| --- | --- | --- | --- | --- |
| F01 | P2 | 异步取色在只读、禁用或新值更新后仍提交旧结果 | 运行复现 | 第一批 |
| F02 | P2 | Tree 延迟拖放查询缺少当前状态与版本复核 | 源码确认 | 第一批 |
| F03 | P2 | 同主题内部切页后目录保留首次页面的链接 | 源码确认 | 第一批 |
| F04 | P2 | 普通键盘用户缺少跳过完整组件侧栏的路径 | 源码确认 | 第一批 |
| F05 | P2 | 质量审计使用 Glory main，无法直接证明固定生产组合 | 源码确认，验证范围缺口 | 第二批，与 F06 同做 |
| F06 | P2 | 发布和部署未在工作流内等待精确依赖审计通过 | 源码确认，验证链缺口 | 第二批 |
| F07 | P3 | WASM 体积文档的复现入口已过时 | 源码确认 | 第二批 |
| F08 | P3 | 搜索浮层缺少保留查询的独立关闭路径 | 源码确认 | 第三批 |
| F09 | P3 | 错误输入缺少与正常输入一致的焦点轮廓强调 | CSS 最小运行复现 | 第三批 |
| F10 | P3 | 移动文档菜单缺少主站入口 | 源码确认 | 第三批 |
| F11 | P3 | 文档视图按钮未公开当前选中状态 | 源码确认 | 第三批 |

### F01 异步取色结果绕过当前可操作状态

[color_input.rs 第 414 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/color_input.rs#L414) 定义 `picker_enabled = enabled && !readonly && !composing`，但 [第 948 行的完成路径](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/color_input.rs#L948) 只检查 WeakScope 是否仍存活，随后执行 `editing.accept` 并发出 change、commit。控件改为只读或禁用不会自动销毁 Owner。

**本轮复现：** 采用与 [现有取色测试](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/tests/color_pickers.rs#L531) 相同的 Executor，在激活后、执行任务前，分别设置 readonly、disabled 或更新 controlled 源，再执行任务。应用回调接受提议，三个场景都得到：

| 任务执行前的变化 | 执行后业务值 | change 次数 | commit 次数 |
| --- | --- | --- | --- |
| readonly 变为 true | `#12b886` | 1 | 1 |
| enabled 变为 false | `#12b886` | 1 | 1 |
| 业务值先变为 `#ffffff` | `#12b886` | 1 | 1 |

这说明延迟结果仍可改变本地值或提议覆盖较新业务值。不是卸载后回调问题；Owner 清理仍会取消任务。

**改进：** 捕获请求代次和启动时值版本，完成时复核当前许可、代次和版本。禁用、只读、组合输入或新编辑使旧操作失效时，应丢弃旧结果。controlled 回调继续只提议更新，不能改为直接写源。

**验收：** 可控延迟 Future 覆盖 readonly、disabled、新值、第二次请求取代、卸载；失效结果不发 change/commit，正常完成恰好一次。探针保存在本地 `target/verification/sapid/review-2026-11-09/eyedropper_probe.rs` 与同目录日志中；诊断断言记录的是当前缺陷行为，不是期望回归行为。

### F02 Tree 拖放查询没有重新验证当前状态

[tree.rs 第 1328 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/tree.rs#L1328) 发出 Bounds 查询，完成回调通过当前模型的结构校验后调用 `on_drag_drop`，未复核整棵树 enabled、源和目标 disabled、请求代次或模型版本。普通选择路径使用的 [usable](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/tree.rs#L342) 没有用于这里。[move_tree_node](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/tree_model.rs#L387) 验证树结构与循环，不负责可操作状态。

触发序列为 Drop 请求发出、树被禁用但行 Owner 保留、旧查询响应到达。正确的请求 watermark 只能证明响应对应原查询，不能证明当前业务状态仍允许移动。宿主的 Owner 限制也不能排除 Owner 仍存活的情况。这一项已完成调用链核验，尚未执行延迟 Render 的运行复现。

**改进：** 参考 [资源日程拖动](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/schedule/resources/drag.rs#L78) 的 ready 与版本复核。捕获模型版本和请求代次；结果到达后重新检查源、目标和整树当前可用性，再执行 allow_drop 与应用回调。

**验收：** 测试 Render 延迟 Bounds，发出 Drop 后分别禁用树、禁用节点、同键替换模型、卸载行、发出新请求；用原 token 与合法 watermark 回复，过期请求不得触发 drop callback。保持合法拖放行为。

### F03 文档目录没有跟随站内切页更新

[showcase.rs 第 700 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L700) 把初始 selection 存入 `toc_selection`，[目录生成](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L731) 始终使用它；正文则按 revision 读取当前 route。[browser.rs 第 130 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase_host/browser.rs#L130) 为同主题的文档间导航保留 shell，目录所在的真值 When 分支也因此保留。

例如从 Button 内部切到 TextInput，正文更新，而目录仍可能含 `#button-variants` 等旧锚点。完整页面重新加载不会暴露这个路径。

**改进：** 目录与正文一起按文档 revision 更新，从当前 route 读取组件。顺便修正 [Usage 永久 is-active](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L728)：明确采用当前 fragment 高亮，或移除会误导阅读位置的固定高亮。

**验收：** Button → TextInput → Slider 连续内部导航、浏览器前进/后退；逐个断言目录 href 在当前文档中有对应标题，并验证真实跳转。

### F04 缺少跳过长侧栏的键盘入口

[正文前的 sidebar](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L711) 包含完整组件目录，其 [链接生成](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L106) 使用普通可聚焦 Anchor；[站点测试](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/tests/showcase_site.rs#L60) 记录了 181 个导航链接。当前 shell 没有 skip link。

Main/Navigation landmark 有利于屏幕阅读器定位，但只使用 Tab 的用户仍要穿过长导航才能到正文交互区。

**改进：** 增加首次 Tab 可见的“跳至主内容”，复用 [AppShell 已有能力](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/app_shell.rs#L937) 的语义和焦点处理思路。

**验收：** 页面首次 Tab 出现入口，Enter 后实际焦点进入主内容，下一次 Tab 到正文控件；验证直达、内部切页和 SSR/hydration。

### F05 开发质量审计不能替代固定生产组合的审计

[quality.yml 第 48 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/quality.yml#L48) 和 [第 106 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/quality.yml#L106) 读取固定 Glory revision，实际 checkout 却使用 main；[产品检查](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/ci.yml#L142) 使用固定 revision。[开发版本说明](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/docs/development-versions.md#L23) 明确允许开发 CI 跟踪 sibling main，因此不能简单断言所有 main 检查都是配置错误。

真正的缺口是：对 main 的 MSRV/RustSec 结果无法直接证明固定生产版本及其传递依赖通过相同验证。Glory main 移动后，同一 Sapid 提交重跑也可能得到不同输入。

**改进：** 保留明确标识的 upstream-main 兼容性检查，同时增加生产固定组合与 registry 消费组合的精确审计。报告记录 product SHA、Glory SHA、实际三个 lockfile 哈希和 advisory DB revision，避免将不同组合的绿色结果合并解释。

**验收：** 令 main 与固定 revision 不同，确认生产审计选择固定组合；main 移动不改变冻结产品的审计输入；开发兼容性失败与产品审计失败分别可识别。

### F06 发布部署没有依赖精确审计结果

[release.yml 第 45 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/release.yml#L45) 仅调用 ci.yml，再允许 publish；RustSec 在独立 [quality.yml](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/quality.yml#L153) 执行，触发条件不含 tag push。[站点部署的 needs](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/.github/workflows/ci.yml#L255) 也未包含质量审计结果。

因此，从仓库工作流看，独立审计失败或尚未完成，并不必然阻止本次发布/部署。没有检查远端分支保护或组织策略，也没有据此断言产品目前含已知漏洞。

**改进：** 将针对本次实际 lockfile 的审计做成可复用必需任务，绑定冻结 product、依赖组合与产物 provenance。发布/部署核对报告摘要和 lock 哈希。与 F05 配套实施，不能只等待另一份依赖组合的审计。

**验收：** 注入漏洞 fixture、审计失败、报告缺失、lock 哈希不一致，均应在 publish/deploy 前停止；精确匹配的成功报告才能放行。

### F07 WASM 体积基线文档无法按当前入口重现

[wasm-size.md](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/docs/wasm-size.md#L3) 仍把 Sapid 的 36 应用矩阵描述为 sibling Glory 的当前入口，[第 35 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/docs/wasm-size.md#L35) 的命令和第 39 行仍指向已停用的共享缓存目录。当前本地 Glory 的 `benchmarks/wasm-size/README.md` 已声明活跃矩阵只含 Glory；其 cases.json 也没有 Sapid case。

这不会推翻历史测量数值，但读者不能把当前入口当作 Sapid 0.7.1 的可执行性能门禁。该发现针对当前 sibling 组合，不推断所有历史发布版本的 runner 行为。

**改进：** 明确历史基线的版本边界，在 Sapid 自己的仓库维护消费者矩阵，或指定仍包含 Sapid 的冻结复现来源。入口从所属 workspace 的 `cargo metadata --no-deps --format-version 1` 读取 target_directory，继承调用者 Cargo home、TEMP/TMP 和正常项目配置。

**验收：** 新检出环境可运行 empty、text、button 与六类实际使用/未使用 feature 的代表 case；记录工具版本、源码与锁文件哈希、WASM/JS/CSS 原始及压缩体积。文档命令不得重新创建已禁用目录。

### F08 搜索面板缺少独立关闭状态

[has_query](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L194) 只取决于查询是否非空，[搜索框与结果分支](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L277) 没有 Escape、外部关闭或独立 opened 状态。结果面板采用浮动定位，移动端采用 fixed；清空或导航之外缺少保留查询的关闭路径。

**改进：** 分离 query 与 opened，使用共享非模态 Layer/Popover 处理关闭与焦点。不要在展示站手写第二套层行为。

**验收：** 输入后 Escape、点击正文、Tab 离开、重新聚焦，验证面板关闭/重开且查询保留、结果链接仍可键盘访问。当前完整站点上的实际遮挡面积尚未测量。

### F09 错误输入的焦点强调与正常输入不一致

[sapid.css 第 34 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/styles/sapid.css#L34) 去掉 TextField 的 outline/shadow，[第 38 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/styles/sapid.css#L38) 仅为非 invalid 输入提供焦点边框变化；[TextField 默认边框](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/text_field.rs#L333) 在 invalid 时固定为 danger。完整样式检索未找到补偿规则。

Chromium 最小用例中，Tab 聚焦 invalid 输入时 `:focus-visible=true`，但 outline 为 none/0px、shadow 为 none，聚焦前后边框均为 `rgb(201,42,42)`；正常输入对照组聚焦后边框变为 `rgb(34,139,230)`。

**改进：** 给 focus-visible 增加独立轮廓或等价持续强调，与错误边框叠加；检查 combobox/tags/pills 的同形规则。文本插入光标仍可能显示，所以本项不直接判定为“完全没有可见焦点”或整站 WCAG 不合规。W3C 对焦点可见性的说明允许文本插入光标作为一种指示。[W3C Focus Visible](https://www.w3.org/WAI/WCAG22/Understanding/focus-visible.html)

**验收：** 默认/invalid、深浅主题、TextField/Textarea/组合输入的真实 Tab 和失焦，覆盖聚焦时才变 invalid 的状态切换。

### F10 移动文档菜单的信息架构不完整

[site.css 第 210 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/site.css#L210) 在窄屏隐藏 topnav，文档页菜单展示组件侧栏；包含 Theme Studio/Glory 的 [mobile nav](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L784) 只在首页或 Theme Studio 生成。

**改进：** 统一移动导航，使组件目录和主站入口都可直接访问，减少从组件文档进入 Theme Studio 的回首页绕路。

**验收：** 390px 下直接进入组件文档，打开菜单即可到 Theme Studio、组件目录和项目入口；焦点、滚动及关闭状态一致。

### F11 文档视图按钮没有公开当前选中状态

[showcase.rs 第 569 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L569) 的 Documentation、Properties、Styles API、Source、Capabilities 是普通 Button，当前项通过 is-active 样式表达，没有 pressed 或 tab selected/panel 关系。

**改进：** 优先复用 Sapid Tabs。若产品选择普通切换按钮组，则公开当前按压状态并清楚关联内容，不要只增加角色而遗漏对应键盘行为。

**验收：** Chromium AX 能识别当前项与内容；验证切换后焦点、自然 Tab 或完整 Tabs 方向键行为，以及 SSR/hydration 一致性。

## 性能与可维护性改进机会

### 先补代表性测量，再决定索引或虚拟化

DataTable [实际筛选路径](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/data_table/model.rs#L239) 会在过滤变化时对访问到的 cell 做 to_lowercase。现有 [派生 benchmark](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/data_table_derivation_bench.rs#L82) 重点是翻页与隐藏列。建议增加连续筛选、ASCII/Unicode、无匹配和末列匹配的耗时、分配量与浏览器输入延迟；只有收益明确时才增加受总内存预算约束的规范化索引。

Tree [each(rows)](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/tree.rs#L1121) 挂载当前展开的全部行，高度只形成滚动容器。[Tree 文档](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/docs/tree.md#L88) 已说明有限树和虚拟化边界，因此这不是缺失已承诺功能。建议测 512/2048 行、复杂标签、展开/收起循环的节点、Owner、内存与输入到绘制耗时，再决定是否提供可选窗口化；不能以破坏 Tree 语义和键盘定位为代价。

补一组小而稳定的运行时基线：资源周日程、DataTable 搜索、Tree 大展开、富文本历史和反复挂载卸载。记录 p50/p95、负载大小、宿主、源码哈希与测量环境；先建立历史分布，再定义回归阈值。传输体积、纯模型 CPU 时间和浏览器交互延迟应分开记录。

### 将异步有效性检查变成可复用的测试模式

F01/F02 的共性是 Owner 存活不等于操作仍被允许。先修具体路径，再提炼“请求代次、来源版本、当前许可”的测试夹具，覆盖延迟结果、状态变更、析构重入和新请求取代。避免先引入一个很大的通用抽象；现有 Form、CopyButton 和 Schedule 的实现可以作为对照。

展示站应优先消费库里已有 Tabs、AppShell 与 Layer 行为，以减少组件库自身有完善契约、展示站却绕开契约的维护分叉。

## 视觉设计与元素细节建议

以下是基于静态样式的设计机会，不作为已经截图证实的布局故障，也不赋予伪精确的美观分数。

| 方向 | 当前证据 | 建议与验收 |
| --- | --- | --- |
| 移动端可读性 | [site.css 第 260 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/site.css#L260) 一带将 mosaic eyebrow、提示、checkbox 缩至 8–11px | 优先减少装饰卡片或改布局，保持 live 表单文字可读；检查 320/360/390px、200% 字号和长文本 |
| 图标一致性 | [showcase.rs 第 294 行](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/src/showcase.rs#L294) 一带使用太阳、月亮、菜单等 Unicode 字符 | 统一矢量图标的描边、光学尺寸和基线，保留已有 accessible name；对照 Windows 字体与不同浏览器 |
| 触控密度 | [palette 24px](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/examples/web/site.css#L336)、主题/menu 控件 32px，与主 CTA 44–46px 并存 | 可以保持小圆形视觉并扩大实际命中区域；检查相邻点击区、焦点轮廓和窄屏排布，不仅凭尺寸判断合规 |
| 状态表达 | 目录 Usage 固定高亮、文档视图按钮仅样式选中 | 统一 hover、focus、selected、invalid、disabled 的视觉层次，并让语义状态与视觉一致 |
| 响应式信息层级 | 文档侧栏、目录、搜索与主导航在窄屏采用不同隐藏策略 | 从“组件文档直达”测试完整任务，而不是仅检查页面无横向溢出；确保重要入口仍可发现 |

建议建立 4 个主题组合 × 3 个宽度的代表页面截图集：首页、输入错误表单、长文档、表格、嵌套弹层和 Theme Studio。人工审查字重、留白、对齐、长标题截断、状态区分，再用稳定截图做回归。

## 安全增强与国际化边界

[deploy/_headers](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/deploy/_headers#L3) 定义 nosniff 与 Referrer-Policy，仓库未定义可验证的 CSP/嵌入策略。未检查 Cloudflare 外部规则，不能声称线上响应缺失。可先以 Report-Only 验证脚本与样式来源，再按实际需求配置 frame-ancestors；WASM bootloader、动态主题样式、SSR seed、文件 blob 预览均需纳入兼容性验证。相关机制见 [W3C CSP Level 3 草案](https://www.w3.org/TR/CSP3/)。这是纵深增强，非本轮确认漏洞。

[显式 UI 环境](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/docs/environment.md#L3) 已明确 Locale、RTL 和主题为应用显式消费的配置，不是自动修改所有控件的全局 Provider。因此不把“挂 Provider 后所有已有控件没有自动翻译”列为缺陷。后续可提供完整中文/阿拉伯文表单与弹层示例，检查内部提示词、日期数字、长文本和 RTL 几何，明确哪些字段必须由应用传入。OS IME 与真实屏幕阅读器应单列人工验收，不由 Chromium AX 或内存测试代替。

## 已有防护与历史问题复核

以下设计值得保留，不能作为缺失能力重新立项：

- Rust unsafe 禁止；JSON 在解析前做字节/深度限制；代码高亮使用文本范围；富文本是有限结构化文档，编辑历史有数量和字节预算。
- 文件 token、当前 controller 成员、读取范围与权限校验；Theme 名称校验和数值颜色序列化；当前 URL 经过 Glory 的共享 scheme 策略。
- Dirty 在完整 host tick 后协调 controlled 状态；WeakScope、OwnerDelay、Collection 对生命周期和清理有明确处理。
- 安装工具已经有哈希、锁文件、文件清单及无部署凭证安装阶段；发布已冻结产品 SHA，不能重报旧的移动 tag 检查绕过问题。

| 历史事项 | 当前实现证据 | 本轮判断 |
| --- | --- | --- |
| 资源日程重复展开 | [resources/snapshot.rs](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/schedule/resources/snapshot.rs#L26) 一次 expand，传给 expanded 布局 | 已修复，不重报历史倍数和耗时 |
| DataTable 翻页/隐藏列全量重新匹配 | [local.rs](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/data_table/local.rs#L48) 按数据 Rc/filter/sort 缓存 Matches，并有派生次数测试 | 已修复 |
| Plot 关闭表格仍格式化 | [plot_widget.rs](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/plot_widget.rs#L270) 仅在 show_table 时构造 tables | 已修复 |
| 公共错误缺少标准接口 | [DataTableError](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/data_table/model.rs#L120)、[TreeError](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/src/tree_model.rs#L112) 已有 Display/Error | 不重报 |
| 独立示例格式检查缺失或失败 | [format.ps1](https://github.com/sapid-ui/sapid/blob/679127fe2b8b9d0ba5e882b5556589a4955c2877/scripts/format.ps1) 覆盖三个 workspace；本轮通过 | 当前检查通过 |

## 本轮验证结果

| 检查 | 结果 |
| --- | --- |
| cargo metadata --no-deps --format-version 1 | target_directory 为本项目 `D:\Works\glory-rs\sapid\target`；构建继承正常环境，没有重定向全局缓存 |
| scripts/format.ps1 -Check | 通过，含主库及 Web/Native 独立 workspace |
| cargo test --locked --all-features | 通过，含单元、集成与两个文档示例 |
| cargo clippy --locked --all-features --all-targets -- -D warnings | 通过 |
| widget-plan.py --check | 通过，242 源条目、137 任务、13 阶段 |
| showcase-catalog.py --check | 通过，181 条目及编译源码目录校验 |
| capability-gallery.py --check | 通过，137 显式能力条目 |
| check_doc_links.py --base 679127fe2b8b9d0ba5e882b5556589a4955c2877 | 基线检查通过；报告交付前另做新增链接校验 |
| test_ci_scope.py / test_tools.py / test_library_lock.py / test_release.py | 分别 11 / 10 / 5 / 35 项通过 |
| test_format.py / test_dsl_paths.py | 分别 1 / 4 项通过 |
| node --test test_showcase_acceptance.cjs test_browser_fixture.cjs | 17 项通过；这些是 runner 回归，不是完整页面验收 |
| ColorInput 专用诊断探针 | 三种失效场景均复现旧取色结果提交，见 F01 |
| Chromium 输入焦点最小实验 | 复现 invalid 输入无额外边框/轮廓强调，见 F09 |
| test_audit.py | 未完成：要求 cargo-audit 0.22.2，本机 0.22.0，被版本门禁拒绝；没有得出当前依赖无漏洞结论 |

本轮没有运行完整 check.ps1 -Web -Native、完整展示站浏览器矩阵、MSRV 编译、registry-only 发布演练、OS IME 或屏幕阅读器人工验收。Rust 全 feature 通过与严格 Clippy 通过不覆盖这些边界。

## 建议实施计划

| 批次 | 工作范围 | 完成标准 |
| --- | --- | --- |
| 第一批，正确性与基础访问 | F01/F02 异步有效性；F03 目录；F04 skip link | 延迟操作回归、连续导航回归、真实键盘路径通过；库和对应浏览器检查通过 |
| 第二批，发布证据与可复现性 | F05/F06 精确组合审计与发布绑定；F07 体积入口 | 冻结源码和 lock 哈希可核对；模拟审计失败阻断发布；新环境可复现体积结果 |
| 第三批，交互与视觉 | F08–F11；移动文字、图标和命中区域 | 当前构建的深浅主题与桌面/移动任务验收；AX 状态和截图人工复核 |
| 第四批，按测量优化 | DataTable 搜索、Tree 展开、运行时性能与资源回收基线 | 同负载记录优化前后时间与分配/节点数据，证明收益且不破坏语义和内存预算 |

每项修复应在后续提交中更新本报告状态与对应证据；只有实际通过的组合才能写入能力清单。审美建议由当前构建对照图决定，性能优化由测量决定，避免将建议直接升级为已经证明的瓶颈或发布阻断项。
