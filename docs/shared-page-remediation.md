# 共同页面修复进度

依据 [2026-11-04 逐页审查](shared-page-review-2026-11-04.md)，先完成 27 个 P1
页面，再推进其余页面。每项以实现、实际公开示例、分段文档和对应验收为完成条件。
机器记录见 [完整队列](shared-page-remediation.json)。原始审查保留为修复前证据。

## 已验证：8 / 27 个 P1 页面

| 页面 | 已完成的审查改进 |
| --- | --- |
| Pagination | 紧凑数字/箭头、完整可访问名称、正确文档顺序、可配置前后/边缘按钮、受控窗口和未知总数示例 |
| Breadcrumbs | 反应式分隔符、可配置间距、紧凑折叠动作、长路径及受控展开文档 |
| NavLink | 专属公开示例、描述/左右内容槽、三种活动变体、嵌套和受控展开示例、保留子树与编辑器身份 |
| Anchor | 专属链接示例、被动内容组合、应用接管导航请求、真实源码及边界说明 |
| Burger | 专属状态示例、原生按钮图标、尺寸/禁用/受控状态说明和实际操作验证 |
| Accordion | 四种变体、全宽标题/展开图标、静态与 keyed 一致呈现，多开/受控/内容回收/动态键值的四段独立示例 |
| Tabs | 三种变体、选中标记、纵向双栏布局、增长配置，方向/手动选择/内容保留/键值重排的五段独立示例 |
| Timeline | 默认节点/连接线、尺寸配置、进度更新、标记/状态语义的三段独立示例 |

验证记录：

- `tests/navigation.rs`：18 项通过，包含有限窗口、受控拒绝、事件顺序、分隔符更新和富链接身份。
- `examples/native/tests/navigation.rs`：3 项通过，覆盖真实引擎布局、富链接内容和子树回收。
- `scripts/showcase-remediation-navigation.cjs`：5 页 × 4 主题 × 2 宽度，40 组通过。
  检查翻页、名称/顺序、折叠、保持编辑器、导航接管和按钮操作；错误记录为空。
- 展示站 release 门禁通过，包含目录同步、14 个站点测试、3 个路由测试、29 个模块测试、
  host/wasm Clippy 以及发布资源尺寸/指纹验证。
- 原始记录与截图：`D:/BuildCache/sapid/remediation`、
  `D:/BuildCache/sapid/remediation-navigation-{build,native,browser}.log`。

第二批验证：46 项库回归、4 项原生引擎回归通过；3 页 × 4 主题 × 2 宽度的
24 组浏览器报告通过，错误记录为空。原生检查包含纵向面板位置和连接线/节点的实际几何。
日志为 `D:/BuildCache/sapid/remediation-composites-{unit,native,clippy,build,browser}.log`，
浏览器报告与截图位于 `D:/BuildCache/sapid/remediation/composite-remediation*`。

## 当前批次

PinInput、Scroller、RichEditor、Affix、BackgroundImage、TimeGrid：修复几何约束，
补对应的公开用法、行为配置与功能验证。PinInput 需要共享字距与字体相对长度算术，
以保持一个真实编辑器的值、原生光标、选区、粘贴和 IME。其余 P1 页面仍在队列中。
