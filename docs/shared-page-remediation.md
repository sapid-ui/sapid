# 共同页面修复进度

依据 [2026-11-04 逐页审查](shared-page-review-2026-11-04.md)，先完成 27 个 P1
页面，再推进其余页面。每项以实现、实际公开示例、分段文档和对应验收为完成条件。
机器记录见 [完整队列](shared-page-remediation.json)。原始审查保留为修复前证据。

当前已完成 **181 / 181**，其中 P1 **27 / 27**、其余 **154 / 154**。
最新测试与资源版本见本文末尾的“最终完成”记录。

## 已验证：27 / 27 个 P1 页面

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
| PinInput | 有间距的字体对齐格子、原生编辑余量、反应式掩码、真实焦点边框，验证码主例和三段独立文档 |
| Scroller | 非收缩卡片、紧凑边界箭头、固定步长/尺寸/图标/无控件配置、百分比宽度只解析一次，三段独立示例 |
| RichTextEditor | 紧凑工具栏、有限动作配置、H1–H6/选区状态/实际历史可用性，文章主例和四段独立文档 |
| Affix | 响应式局部主例，viewport 显隐、反应式偏移、sticky 的三段独立操作示例 |
| BackgroundImage | 响应式宽度、本地背景与可读前景层，cover/contain、反应式来源/失败回退的独立示例 |
| TimeGrid | 响应式等宽列、密集列局部横滚和键盘滚入视野、RTL 同排，间隔/边界/受控/状态的三段独立示例 |
| Select | 内嵌箭头/清空、起始侧标签/占位颜色、焦点返回和宽度占位，主例及六段独立源码文档 |
| DatePickerInput | 内嵌日历/清空、三模式清空提案、友好显示与 ISO 提交分离，主例及七段文档 |
| DateTimePicker | 内嵌操作、可本地化操作名称、显示/ISO 分离，保留真实分段/预设/范围，十一段独立文档 |
| MonthPickerInput | 月域本地化格式/内嵌清空、独立 ISO 提交，四段格式/受控/模式/状态示例 |
| YearPickerInput | 年域本地化格式/内嵌清空、独立 ISO 提交，四段格式/受控/模式/状态示例 |
| FloatingWindow | 默认关闭的笔记主例、紧凑标题栏/可披露移动缩放动作，几何拒绝、内容保留/回收、嵌套确认和弱句柄的四段独立示例 |
| Carousel | 卡片主例、内嵌方向箭头/尺寸和显隐、最多九个指示器，同任务连续命令/外部暂停修复，四段受控/几何/时钟/有限窗口文档 |
| Dropzone | 实际拖放/chooser 主例和可读文件反馈，原子拒绝、显式图片预览、4 KiB Owner 绑定读取、禁用重配置的四段独立文档 |

| Menu | 六段真实操作文档、组标题/被动图标与快捷键、受控显隐和定位、保留焦点的搜索及同一动作/复选/单选/子菜单协议 |
| Menubar | 文档/视图主例、九段独立文档，搜索/过滤/清空、根菜单切换、组标题/内容槽和同一菜单控制器 |

| MiniCalendar | 独立的1–31天日期条、受控范围/选择、排除/边界/中英文/RTL、实际键盘与稳定按钮，四段独立源码文档 |
| Heatmap | 独立日期贡献网格、缺失与零值区分、受控域/范围、locale/色阶/点击、单一提示、稳定日期和真实键盘滚入视野，四段独立源码文档 |
| Transition | 独立受控 presence、预设与自定义过渡、延迟/完成/反向切换、立即隐藏交互、内容保留与完成后回收，五段独立源码文档；浏览器实际变换及原生淡入像素验收 |

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

第三批验证：65 项组件回归、14 项原生引擎检查、库与原生相关 Clippy 通过；
6 页 × 4 主题 × 2 宽度的 48 组浏览器验收通过，错误记录为空。
额外检查了 TimeGrid 的密集列键盘滚动、Scroller 百分比宽度、PinInput 的实际字体间距与掩码身份，
以及富文本格式状态/撤销重做/受控拒绝/链接操作。独立文档均通过挂载/回收检查。
展示站 release 门禁通过；浏览器资源为 `assets-4a1201cb894baeb1edfb`，
WASM 为 25,403,338 字节，低于发布上限。
记录为 `D:/BuildCache/sapid/remediation-geometry-{unit,native,clippy,build,browser}-final.log`，
浏览器报告及截图位于 `D:/BuildCache/sapid/remediation/geometry-remediation*`。
RichTextEditor 仍明确拒绝原生编辑；这些结果不代表任意 Tiptap 扩展、OS IME 或读屏器验收。

第四批验证：61 项组件回归、另 11 项 Select 库回归通过；14 项原生引擎检查通过，
其中输入动作的 3 项在最后的标签布局修改后再次通过。5 页 × 4 主题 × 2 宽度的
40 组浏览器验收通过，错误记录为空；另 21 条既有选择/日期/时间公开站回归通过。
检查包括真实清空/选择、受控拒绝、焦点和节点身份、格式/locale 切换与实际 FormData。
五页共 32 段编译源码文档通过挂载/回收检查。截图复核后修正了 Select 的起始侧对齐
和占位颜色，并在最终矩阵中加入相应位置/颜色断言。

发布门禁通过，资源为 `assets-d421e3493e8d53d8a8f0`，WASM 为 25,658,208 字节。
DateTimePicker 子控件配置与挂载分离后，181 个主例的两套主题审计在默认主线程栈通过；
Windows 上完整调试文档的 SSR 生成改在固定 2 MiB 工作线程内构建和回收本地 UI 状态。
记录为 `D:/BuildCache/sapid/remediation-input-shell-*-final.log`，浏览器报告及截图位于
`D:/BuildCache/sapid/remediation/input-remediation*`。

本轮已执行 `scripts/check.ps1`，其中 2,857 项测试通过。全量严格检查暴露的两处
既有 Clippy 问题（Each 的多余借用、SSR 测试的无效 struct update）已修正，共享层
严格 Clippy 重跑通过。后续局部修改另做了相应回归；27 个 P1 完成后再次运行完整门禁。

第五批验证：29 项组件回归、9 项原生引擎检查通过；轮播颜色/悬停修正后，相关
3 项原生检查再次通过。三页共十二段编译文档通过挂载/回收检查，库、相关原生和
host/wasm 严格 Clippy 通过。3 页 × 4 主题 × 2 宽度的 24 组浏览器检查通过，错误为空；
覆盖真实标题拖动与选区、关闭重开/Owner 生命周期、几何拒绝、嵌套模态焦点返回、
同任务连续翻页、有限九指示器/编辑器保留、显式时钟和外部暂停、实际 chooser、可信拖放、
拒绝保留附件、显式图片解码/移除、UTF-8 有界读取与重配置。截图复核后改进深色选中圆点，
并在四主题中检查实际悬停背景下的圆点对比度。

发布门禁通过，资源为 `assets-628044d90a3f40c186d6`。记录为
`D:/BuildCache/sapid/remediation-media-{unit-final,native-final,build-final,browser-final}.log`，
浏览器报告及截图位于 `D:/BuildCache/sapid/remediation/media-remediation*`。
轮播仍采用整页切换；文件选择后的上传由应用负责，这些结果不代表任意插件或所有 OS 输入验收。

第六批验证：Menu/ContextMenu/Search 的20项组件回归、原生8种主题/组件组合、
共享渲染19项、HTML全形状差分2项、Apace菜单8项与AX当前关系1项、Wry显式拒绝1项通过。
共享层严格Clippy通过；公开站host/wasm Clippy、14个站点/3个路由/28个模块检查与release门禁通过。
2页 × 4主题 × 2宽度的16组公开页面检查通过，错误为空。重建同一代码的SSR/CSR/WASM后，
既有Menu 11、Menubar 10、ContextMenu 14条浏览器回归通过，包括受控拒绝、延迟hover取消、
被动内容命中、键盘层级、原生复选/单选、hydration身份与真实Shell/Patch/End。
新增搜索流式检查也通过，编辑器保持身份且在WASM前更新当前项关系。

截图复核移除了重复的CSS箭头；浏览器发现并修复了文本编辑器提前返回、菜单名称覆盖和
被动内容的hover事件归属。HTML流脚本校验同步到当前格式17，并新增起止版本守卫测试。
Windows验收SSR生成器采用固定2 MiB本地线程，响应式状态在同一线程内构建与回收。
发布资源为 `assets-4d18a9382aeb9d2ebdfd`，WASM为26,032,735字节，低于25 MiB上限；
公开构建省去四个旧画廊入口，默认验收构建仍保留它们。
记录为 `D:/BuildCache/sapid/remediation-menu-*` 与 `D:/BuildCache/sapid/remediation-mini-build.log`；
原生/AX协议证据不代表OS读屏器验收，Wry继续明确拒绝菜单搜索。

## 当前批次

当前累计已验证 80 / 181 页：27 / 27 个 P1，以及 53 / 154 个 P2/P3。
Inputs 分类的24个共同页面均已完成；继续其余120页，下一批为按钮/动作页面。

FileInput 最终实现输入式文件名摘要、内嵌矢量清除图标、独立操作区背景和焦点返回。
8项组件回归、3项四主题原生服务/几何/像素/焦点/身份/取消/读取/预览回归与严格Clippy
通过；配置变化仍静默撤销选择，用户清除则通知空列表。最终统一复验8个输入页面，
共64组四主题/双宽度浏览器检查全部通过，错误为空；包含真实磁盘夹具选择、多文件、
长名称、限制拒绝、清除及焦点返回。资源 `assets-acbb53d887807ee803d9`，冻结汇总
`examples/web/target/remediation/inputs-final.json`，构建 `inputs-focus-final-build.log`。
原生CPU/提供器及浏览器自动化仍不代表OS文件对话框、移动设备和读屏器验收。

NumberInput 已修复真实失焦钳制、当前范围/步长和可用性读取，并补齐三种钳制策略、
五段源码文档。宽屏输入伸缩填满外壳，步进区贴齐右边；14项原生相关回归覆盖数字、
时间和日期时间消费端。MaskInput 增加独立反应式 placeholder，修复当前 Field 状态
保护，四段源码区分字符模式、原始/显示值、编辑/粘贴和受控状态。NativeSelect 保留
真实 select/optgroup，补齐四段文档及当前 enabled 保护；四主题原生实际 caption、
多行选择和身份回归通过，平台弹层操作与读屏另计。77项相关组件回归和严格Clippy
通过；三页各8组浏览器操作通过。资源 `assets-1afb6cb1c297032a1923`，报告
`data-input-remediation.json`、`form-input-remediation.json`。FileInput 本轮8组功能
检查同样通过，取消由原生注入提供器 None 单独验证，不把浏览器设置空 FileList
当成 OS 对话框取消验收。

Input、TextInput、Textarea 已有三个专属首例与十一段完整源码，修复同 packet 更新
Field 可用性后仍交付输入回调的问题。20项组件回归、4项四主题原生回归与严格Clippy
通过；原生固定行高、控件身份和内嵌动作通过，自动增高明确拒绝。24组浏览器操作、
真实增高/缩回和主题/宽度检查通过，错误为空。JsonInput 新增真实失焦格式化，
与按钮共享有界路径，保留受控拒绝、最后一次输入、组合输入和当前可用性保护；
四段独立源码区分格式、错误位置/预算、失焦及状态。9项JSON组件检查和四主题原生
相关回归通过；8组JSON浏览器检查通过。统一资源 `assets-8560c6d31d55fb3702fb`，
报告 `text-remediation.json` 与 `data-input-remediation.json`。后者另有NumberInput的
4组宽屏布局失败，仍在修复，未据此标记数字页完成。

Switch 已完成真实轨道/拇指、五种尺寸、前后标签、内部被动标记和焦点/错误轮廓，
保留实际原生输入、Field 关系、受控拒绝与同 packet 可用性保护。四段独立编译源码
展示尺寸、标签反馈、状态和独立布尔分组。165 项相关单元/模型检查、8 项原生检查
及严格 Clippy 通过，含 40 个四主题/尺寸/标签位置夹具、真实原生指针激活与像素。
12 组浏览器检查（8 组主题/宽度及4组触屏）与32组相关页面回归通过，错误为空。
资源 `assets-11ca5203747044318312`，报告 `examples/web/target/remediation/switch-remediation.json`。

MiniCalendar 与 Heatmap 的四主题/双宽度各8组检查通过，错误为空；库与原生实际布局/焦点/像素检查通过。Heatmap 额外修正了按钮文字基线造成的最后一行裁切，并验证单一当前提示。当前正常 workspace 产物为 `examples/web/target/remediation/heatmap-remediation.json`、`mini-calendar-remediation.json`，对应 `assets-58a16a1731f8d3094156`，实际 Chromium 148.0.7778.96。

Transition 的最终8组浏览器检查（含自定义透明度/缩放/旋转/原点）、4项组件与4项原生检查通过；当前格式18的Shell/Patch/End及Wry显式拒绝通过。公开站release门禁通过，资源为 `assets-36bf4890c84df3eb193e`，WASM 14,180,765字节。原生还验证 authored CSS隐藏祖先时立即完成退出、恢复后不重放。

27个P1已完成专项验证。全量门禁首次暴露的原生profile旧版本断言已修正，正在按正常workspace重跑完整门禁；之后继续154个P2/P3页面，首先回归PasswordInput与AppShell。

最终统一浏览器复验：27页 × 4主题 × 2宽度，共216组通过，错误为空；所有批次使用 `assets-36bf4890c84df3eb193e`。汇总为 `examples/web/target/remediation/p1-final.json`。全量门禁另发现ColorInput关闭后焦点恢复导致重开的问题，已修复并通过17项组件/四主题原生回归；RangeSlider既有clamp验收现显式选择clamp策略，另验证当前默认push策略。其余旧验收已经检查，Stepper改为按实际可访问名称定位按钮。

`./scripts/check.ps1 -Native -Web` 已在正常的三个workspace产物目录完整通过，记录为 `target/remediation/full-gate-verified.log` 与 `full-gate-summary.json`。包括默认/全feature库测试、共享runtime、原生adapter/consumer、严格Clippy、Web/WASM和新的SSR/stream夹具；不会把忽略的OS验收算作通过。首批27项完成，剩余154项继续推进。

## 后续页面：53 / 154 已验证

PasswordInput 已完成专属首例、5段独立源码文档、同packet可用性保护、9项输入组件回归、四主题/五尺寸的原生眼睛像素与选区保留检查，以及8组浏览器操作/布局检查。最新发布资源 `assets-0f9a3514a18c301199c9`，浏览器记录 `examples/web/target/remediation/password-remediation.json`。ColorInput的Escape/外部点击/重新聚焦另有8组浏览器回归通过，仍保留在待完成的页面队列中继续审阅其文档。

AppShell 已完成 viewport 固定框架、主区与 ShellSection 独立滚动、共用有界键盘滚动命令、右侧栏，以及反应式分区尺寸/偏移/边框/显隐。6段独立源码文档和单独的全窗口示例通过公开站 release 门禁；12项组件回归、3项原生实际几何/滚动/身份回归、严格Clippy，以及16组浏览器操作/布局检查通过，错误为空。资源 `assets-9421a48bb4ddcb424174`，记录 `examples/web/target/remediation/shell-remediation.json`。移动原生模态Layer继续明确拒绝；不把桌面几何验证算作OS输入或读屏验收。

其余10个布局页已完成专属首例和17段独立源码文档：AspectRatio使用可辨认的本地图像；Center展示普通文流中的inline frame；Container/Space区分内容上限、内边距和两个间距轴；Flex/Group/Stack展示方向、换行、增长、长文本和高度分布；Grid/SimpleGrid用实际测量重排同一编辑器；Splitter拆分受控/键盘、嵌套和固定侧栏场景。32项组件回归、7项原生回归和严格Clippy通过。10页×4主题×2宽度共80组浏览器检查全部通过，错误为空，包含Splitter拖动到两端的约束。资源 `assets-175211fec743fb1c04f5`，记录 `examples/web/target/remediation/layout-remediation.json` 和 `layout-build-complete.log`。保留了inline示例修正前的失败证据，未将其记为通过。

Checkbox、Radio、Chip、Fieldset 的四个专属首例与十段完整源码文档已完成。23项相关组件回归与严格Clippy、3项四主题原生实际控件/不确定状态/禁用边界/身份回归通过；32组浏览器操作、布局与状态检查通过，错误为空，资源 `assets-58aa3aa6a61663088c67`。记录为 `examples/web/target/remediation/choice-remediation.json`。另保留同packet只读/禁用保护的修正前失败日志。颜色五页本轮44组检查也通过，但正在补齐继续核对发现的受控色板弹层与完整源码文档，暂不标记完成。

AlphaSlider、HueSlider、AngleSlider、ColorPicker、ColorInput 已完成最终验证。除五个专属首例外，颜色滑块/面板有十段完整源码文档，ColorInput有十七段独立编译源码；新增的色板弹层可受控拒绝显隐。真实失焦修正经变更提案再提交，遵守组合输入、只读/禁用和显式关闭修正的策略；关闭色板返回同一编辑器，内嵌预览可再次打开。色块按实际容器宽度分配等宽正方形网格，保持配置列数和节点身份。52项相关组件、12项相关原生检查与严格Clippy通过；44组浏览器矩阵及10项既有公开站回归通过，错误为空。最终资源 `assets-814f9b98b7bc3522d30f`，报告 `examples/web/target/remediation/color-remediation.json`。保留了失焦、受控色板、列数的修正前失败证据，没有把旧失败报告算作通过。

Slider、RangeSlider、Rating、SegmentedControl 已完成四个专属首例和十九段完整编译源码文档。RangeSlider修正同packet配置读取，同时保持尚未接受的外部pair与当前交互位置分离；三种碰撞策略都有实际示例。Rating支持有界静态空/实自定义图形，保留半星、受控、清空与真实radio；SegmentedControl支持反应式等宽全宽/内容宽度，切换不重建控件。58项相关组件、14项原生与严格Clippy通过。截图复核后另修正深色滑块拇指对比度，追加31项相关组件与15项原生复验，含四主题真实像素。最终40组浏览器检查通过（32组四主题/双宽度及8组CDP触屏），错误为空；资源 `assets-da3cff8b982335dc1cea`，报告 `examples/web/target/remediation/selection-remediation.json`。不把模拟触屏或原生CPU/协议结果算作实际移动设备、OS输入和读屏器验收。


ActionIcon、Button、CloseButton、CopyButton、FileButton、Toggle、UnstyledButton 已完成七个专属首例和二十五段完整编译源码文档。新增可见加载指示器及保留原图标的加载切换、默认矢量关闭图标和精确五档方形命中区；六类按钮修复同 packet 禁用后仍执行回调的问题。文件按钮限额示例提供实际拒绝反馈，复制示例有真实结果和显式可取消的浏览器定时器。49项相关组件、6项相关原生检查和严格Clippy通过；发布门禁含46项网站测试及WASM/SSR。56组浏览器检查全部通过，错误为空，包含真实剪贴板读回、当前浏览器上下文权限拒绝、定时重置、可信文件选择、清空与同文件重选。最终资源 `assets-70ac21f9e035d1ca724e`；冻结记录 `examples/web/target/remediation/buttons-final.json`，构建记录 `buttons-feedback-final-build.log`。原生提供者结果不等同于OS实际对话框、系统剪贴板或读屏验收。


## Combobox 九页与文档标题修订

Autocomplete、Combobox、ComboboxPopover、MultiSelect、Pill、PillsInput、TagsInput、
TreeSelect、Cascader 已完成：真实选择器组合、有界选项描述、统一换行编辑框、
紧凑加载/清空与焦点返回、文档顺序的成功值代理、级联父路径/紧凑布局，以及搜索结果祖先临时展开。
九个主例和 48 个独立源码分节均实际编译；受控拒绝不会改写业务源。

`examples/web/target/remediation/combobox-final.json` 固化 72/72 组浏览器验收，
四主题、1280/390px，错误记录为空，资源 `assets-78c3911462ca7e3fb88d`。
同时验证 Button 页的两种宽度，主标题和小节标题均为 400 字重；该规则适用于整个文档站。
原生八项相关检查与后续三项层级检查通过；搜索的红/绿回归保留在正常项目 target 中。
具体日志及平台边界见机器队列。当前总计 77/181 页，P1 27/27、其余 50/154。


## Navigation 剩余三页

Stepper、TableOfContents、Tree 已完成审查项。主例分别是可操作工作流、真实分层文章和项目文件树；
15 个独立源码分节实际编译。Stepper 新增真实连接线与五尺寸正圆标记，保留受控拒绝、
非法 active 的最后有效状态及内容生命周期；窄屏横轨道局部滚动，实际焦点请求可达。
目录说明显式注册、活动项、局部滚动/offset/reveal 与嵌套层级；文件树保留键盘、搜索、
加载、勾选和拖放的明确说明，主例不再显示验收工作台。

35 项相关库测试、8 项四主题原生测试和严格 Clippy 通过。
`examples/web/target/remediation/navigation-extra-final.json` 固化 24/24 组浏览器检查，
四主题、1280/390px，错误记录为空，资源 `assets-fb4848a41a890d09d9ea`。
原生文章测试需要推进 WindowApp 的任务执行器；最初只推进 document 的失败属于测试驱动问题，
未据此改写目录实现。当前累计 80/181（P1 27/27，其他 53/154），余下 101 页继续按清单推进。

## Feedback 八页

Alert、EmptyState、Loader、Notification、Progress、RingProgress、SemiCircleProgress、Skeleton 已完成。
八个独立首例及 26 个完整编译源码模块补上三种加载器、加载通知、受控关闭、真实内容占位、
可暂停的不确定进度、分段标签和实际数据动作。占位保留编辑器身份与输入；关闭使用紧凑矢量图标；
空状态操作区在同排和窄屏换行后均居中。所有文档页主标题和分节标题继续使用 400 字重。

974 项 Sapid 全目标测试、389 项共享 render/HTML/SSR 测试、相关原生像素/几何检查与严格 Clippy 通过。
最终发布门禁包含 49 项网站测试、WASM 和 SSR。`examples/web/target/remediation/feedback-final.json`
固化 64/64 组浏览器检查，四主题、1280/390px，错误为空，资源 `assets-56f459bdcd14dd1e6252`。
明暗手机截图已目视检查。宿主动画/CPU 像素及浏览器协议证据不表示 OS 读屏语音或系统动态偏好同步。
当前累计 88/181（P1 27/27、其他 61/154）；剩余 93 页继续推进。

## Overlay 剩余十一页

ActionBar、Dialog、Drawer、FloatingIndicator、HoverCard、LoadingOverlay、Modal、Overlay、Popover、Tooltip、Tour 已完成。
十一首例和 43 段独立完整源码补齐实际选择/编辑/导航流程、定位与宽度、局部滚动、
嵌套焦点、受控拒绝及恢复、内容保留、指针间隙、目标丢失恢复和真实局部 reveal。
共享标题区采用紧凑矢量关闭；操作条窄屏换行；Tour 分行显示正文与进度，采用主题导航按钮。
默认 Windows 主线程 DateTimePicker 构建的栈溢出已通过拆分构建函数修正，未改变栈/构建环境。

977 项 Sapid 全目标测试、相关库/原生检查和严格 Clippy 通过；发布门禁包括 50 项网站测试、
181 页双调色板主题审计、WASM 和 SSR。`examples/web/target/remediation/overlays-final.json`
固定 88/88 组四主题/双宽度浏览器验收，errors=[]，资源 `assets-8ffe525c155a11cf05bd`。
实际 Chrome AX 排除和焦点拒绝验证 inert；未用 Playwright 的 role 定位器数量推断 AX 暴露。
明暗手机打开状态、长标题及修订后 Tour 截图已目视检查。Drawer 边缘层与 Tooltip 原生层继续明确拒绝；
HoverCard 仅记录 WindowApp Dialog/Preserve 的已测子集，不推断 OS 指针/触屏或读屏。
当前累计 99/181（P1 27/27，其他 72/154），剩余 82 页继续推进。


## Data display 剩余十三页

Avatar、Badge、Card、ColorSwatch、DataList、Image、Indicator、Kbd、NumberFormatter、
OverflowList、RollingNumber、Spoiler、ThemeIcon 已完成：13 个真实首例和 42 段完整编译源码。
补齐头像矢量回退/请求恢复、徽章圆点和有限图标/文本槽、数据列表横纵布局及列宽/间距、
指示器偏移/颜色与覆盖命中、数字前后缀、主题图标独立尺寸。RollingNumber 修复大字裁切，
数字行框、位移和分隔符跟随字体/行高；OverflowList 修复 More 被压缩逐字换行并反馈错误宽度的问题。
图片采用本地真实配图，完整生成提示词与来源记在 `generated-showcase-media.json`。

986 项 Sapid 全量回归、相关原生几何/像素/命中检查和严格 Clippy 通过；发布门禁含
51 项网站测试、181 页主题审计、WASM 和 SSR。`examples/web/target/remediation/data-display-final.json`
固定 104/104 组四主题/双宽度验收，errors=[]，资源 `assets-849b55b8955d9c4fe7b6`。
实际验证照片失败/恢复、控件受控拒绝、覆盖点击、键盘、输入保留、格式与动效/减弱动态偏好；
明暗手机主例、裁切对照、打开后的编辑弹层截图已目视检查。有限协议/CPU/浏览器证据不等同 OS 输入或读屏。
当前累计 112/181（P1 27/27，其他 85/154），剩余 69 页继续推进。


## Typography 九页

Blockquote、Code、Highlight、List、Mark、Table、Text、Title、Typography 已完成九首例和 33 段独立完整源码。
真实文章包含标题/段落/列表/引用/链接/表格；文档说明字面区分大小写匹配、显式主题范围、
原始完整文字与视觉截断，以及浏览器/原生边界。文档站和标题示例均验证常规字重。
表格显式启用双向局部滚动，并修复 caption_side(Bottom) 只改 DOM 顺序的错误；
新的标准 caption-side 声明在 unstyled 也保留结构位置。浏览器 ellipsis 被误写为拒绝的能力说明已更正。

987 项 Sapid 回归、441 项共享样式/render/HTML/SSR 检查、相关原生几何/像素与严格 Clippy 通过。
共享支持掩码仍为单个 u64；HTML/Record 样式 profile 14、Apace 15，HTML shape/wire 仍为 19。
真实 CPU 测量说明上下切换/恢复和点虚线间隙，实际 Sapid 原生首例验证说明位于页脚下方。
发布门禁含 52 项网站测试、181 页主题审计、WASM 和 SSR。
`examples/web/target/remediation/typography-final.json` 固定 72/72 组四主题/双宽度检查，errors=[]，
资源 `assets-96c72a26edbf774df57f`。实际检查横向键盘滚动、sticky 表头、表头关联、页脚/说明几何、
完整 Chrome AX 标题名和跨 Mark 选择；明暗手机文章/表格/引用/截断截图已目视检查。
原生继续明确拒绝 ellipsis、clamp、glyph gradient 与 Sticky；不将协议/CPU 证据视为 OS 输入或读屏。
当前累计 121/181（P1 27/27，其他 94/154），剩余 60 页继续推进。

## Miscellaneous 剩余十页

Box、Collapse、Divider、FocusTrap、Marquee、Paper、Portal、ScrollArea、Toolbar、VisuallyHidden
已完成十个首例和 34 段独立完整源码。Divider 补齐线型与三种标签位置；Collapse 完成回调
过滤初始/旧/重复终态；工具栏采用有完整名称的 30px 图标按钮。真实表单验证初始焦点、Tab 循环、
嵌套恢复；Portal 验证移动/停放后保留输入与选择。滚动包含五策略、显式可取消时钟、
双轴、嵌套与当前位置命令。宽屏首例也能直接看到文本循环和横向滚动。

Sapid 全特性回归 1301 项（128 组，含 doctest）、16 项相关原生验收和严格 Clippy 通过。
发布门禁含 53 项网站测试、181 页主题审计、WASM 和 SSR。
`examples/web/target/remediation/misc-final.json` 固定 80/80 组四主题/双宽度验收，errors=[]，
资源 `assets-a0fe88ec082430b2ffc1`。检查真实键盘、隐藏/inert、保留草稿与选择、
动态偏好及恢复、独立滚动、有效 Chrome AX 说明和常规字重；明暗手机截图已目视检查。
原生样式/CPU/焦点协议与浏览器 AX 不表示 OS 输入或读屏语音验收。
当前累计 131/181（P1 27/27，其他 104/154），剩余 50 页继续推进。

## Dates 剩余九页

九个日期/时间首例、51 个新独立完整源码模块与七个保留的 DatePicker 段落，合计58段，
已完成并匹配目录。DateInput 改内嵌日历动作；时间操作按组换行，小时字段保留可编辑宽度；
日期时间确认/清空在同一底部操作行。修复真实 InputEdit 通道只处理整段粘贴、忽略普通数字
输入的问题：连续编辑小时/分钟后确认会提交最新完整值；组合输入终态不重复，重复粘贴仍重申
接受值，超限拒绝不调用解析器。日历自定义内容不再重复，未选日期边框透明，范围中段与端点不同，
焦点轮廓跟随实际焦点。文档明确 ISO/显示格式、无效草稿、受控拒绝、显式日期提供者与民用时间边界。

`examples/web/target/remediation/dates-final.json` 为72/72组四主题、桌面/手机验收，errors=[]，
资源 `assets-5c0ea8d23841bed497a0`。截图已检查明暗主题；既有两套时间公开站检查也通过。
全特性1314项/129组、当前18项日期原生检查与严格Clippy通过；发布门禁含55项网站检查、WASM及SSR。
首轮一次本地连接 ERR_NO_BUFFER_SPACE 保留在 dates-transport-initial.json；重跑完整72组无重试通过。
当前累计140/181（P1 27/27，其他113/154）。图表21页仍在处理图面行列标识，未提前标记完成；
之后还有日程13页、扩展7页。

## Charts 剩余21页

21个独立主例和90段完整源码已完成，图面与辅助导航分离，图例为紧凑可按压状态按钮，
Sparkline用于实际指标趋势且保留全部精确读数。补齐经共享几何验证的圆环厚度/中心文字、
饼图和径向图角度、Waffle行列/间距及气泡半径范围；新增XY刻度/网格。截图复核发现Matrix
缺行列名称后，补齐真实轴标识和有主题底色的数值，并验证缺失单元与零值的区别。新增本地
错误回调，拒绝更新仍保留同一批图形、标签和表格；无回调时保留Glory错误上报，初始非法声明仍失败。

`examples/web/target/remediation/charts-final.json` 168/168组、errors=[]，资源
`assets-767d7b8a291eaf7fbc09`。明暗手机截图已复核；原有参考线/区域/点、brush和图例检查也通过。
当前批次27项组件/呈现回归、10项几何测试、6项原生测试（含四主题×21实际首例和18类几何的循环）
及严格Clippy通过。新Matrix原生测试检查真实行列位置和标签底色像素；发布含55项网站检查、WASM/SSR。
文档逐类说明实际数据含义、边界和交互，没有将任意Recharts回调或OS读屏标记为已实现。
累计161/181（P1 27/27，其他134/154），余日程13页、扩展7页。

## 最终完成：181 / 181

27 个 P1 页面和其余 154 个共有页面的审查项均已完成。页面主标题与分节标题
统一使用常规字重 400；日程日期分组采用 16px 常规字重，全天事件保留真实列表
结构并移除默认圆点。Button 页的实际效果见
`examples/web/target/remediation/final-button-titles.png`。

日程 13 页提供 13 个独立主例与 62 段完整源码，涵盖受控选择/拒绝、窗口与资源
分页、真实时间滚动、资源编辑与冲突策略、复发/异常、半开区间和显式时区解析。
修复多实例命名冲突、移动日期激活不更新详情、Agenda 注册错误网格事件，以及
同窗口资源更新丢失编辑目标。滚动分节明确选择 Week，原生回归实际检查滚动偏移。
见 [日程说明](schedule-pages.md)。

扩展 7 页提供 7 个独立主例与 30 段完整源码，分别展示真实代码/JSON 内容、
相邻复制反馈、应用局部通知队列与显式时钟、项目命令面板、照片画廊、可取消
导航任务、确认与可保存表单。分节覆盖搜索/分组/空结果、受控拒绝、更新/关闭、
超时/暂停、缩放、队列及焦点恢复，明确队列通知与单个 Notification loading 的区别。
见 [扩展说明](extension-pages.md)。

最终验收记录：

- `target/remediation/final-canonical-check.log`：完整
  `./scripts/check.ps1 -Native -Web` 返回 0；3626 次测试通过、2 项既有忽略，
  835 个测试输出组，包含严格 Clippy、原生与 Web/WASM 构建。修正旧样式版本断言
  和图标按钮定位/命中断言后完整复跑通过；原始失败记录保留。
- `examples/web/target/remediation/final-showcase-release.log`：57 项网站/路由/源码
  检查、181 页主题审计、严格 host/WASM 检查、release WASM 与 SSR 均通过。
- `schedule-final.json`：104/104；`extensions-final.json`：56/56。均为四主题、
  桌面/手机交互验收，errors=[]，最终资源为 `assets-0a161dd6994d8483b89d`。
  四套既有日程公开站检查也已通过，保留真实几何、滚动和视图切换断言。
- `all-pages-current.json`：181 页 × 桌面明色/手机暗色，362/362；errors=[]，
  最终资源 `assets-0a161dd6994d8483b89d`，检查标题字重、实际预览挂载和页面宽度。
- `final-p1/p1-final-current.json`：27 页、216/216；errors=[]。PasswordInput 的
  8 组和 AppShell 的 16 组专项也通过，分别记录在 `final-extra`。这批复验资源为
  `assets-698b1550136c28cef930`；之后仅修正独立的 Schedule 滚动分节，运行库与
  P1 源码未变，最终资源另经上述全站复检。两批资源分别记录。

上述浏览器 JSON 与截图位于 `examples/web/target/remediation`。机器清单
[shared-page-remediation.json](shared-page-remediation.json)、组件清单及能力证据
同步更新；原始逐页审查保持不变。这里验证的是审查范围与有限宿主合同，
不宣称完整 Mantine API 兼容，也不将浏览器/CPU 检查推断为 OS 输入或读屏验收。
