# Sapid / Mantine 共同页面审查（2026-11-04）

Sapid 的共同页面均已逐页检查。需要改进的不只是文档篇幅：已确认若干控件宽度/默认皮肤、错误的公开示例，以及 MiniCalendar、Heatmap、Transition、Menu 内容组合等真实能力差异。本轮新增工作是审查与可复现证据，尚未批量实现这些后续修正；PasswordInput/AppShell/Menubar 前一轮改动作为当前基线保留。

## 范围与证据

- 共同页面181个，按当前 Sapid 目录与官方同名页面一一映射；逐页重新抓取并浏览对应 Mantine 官方文档，不把2026-10-25的冻结 API 审计当作最新事实。
- Mantine：181份当前页面 HTML、181次实际浏览器访问；176页定位到通用首例截图，AppShell/Box/Portal/VisuallyHidden/Events data 未被该选择器定位到标准内联预览，仍审阅其文档，不据此判定无示例。
- Sapid：当前8795发布资源预览，asset assets-9ef400f5ee0331815cc0；181页各扫描1280×1000浅色、390×844浅色、1280×1000深色，共543次；主题为blue，Chromium 156.0.8078.4。
- 全部首例截图已按桌面对照、窄屏、深色联系表人工检查；原始完整截图同时保留。另做181页行内标题复核和15页针对性布局/交互量测，重要能力差异结合当前源码核查。
- 自动扫描：0个页面/控制台运行错误，0个空首例，0次整个文档横向溢出；4个窄屏首例存在内部溢出。以上不等于所有交互或全部颜色/变体已验收。
- 文档组织：22页有独立 detail 段落，159页没有；后者部分已有丰富行内场景，不能直接算作159页缺示例。

优先级表示后续工作的顺序：P1 为已复现的布局问题、错用示例或已核实的核心功能差异；P2 为文档、呈现与功能展示改进；P3 为本轮未发现急迫问题、仍建议补充的说明。它不是平台支持等级。

逐页记录：P1 27 页，P2 131 页，P3 23 页。一个页面可同时涉及多个问题；下列问题按根因归并。

## 应优先处理的问题

### F01 · PinInput 的可见格宽错误

1280px 桌面下，首个预览的六个 .sapid-pin-cell 宽度均为 7.6875px。源码按每格 1ch、容器 length ch 布局；这使验证码格子挤成一条。公开主例还显示 length/changes/commits/ordered 等验收状态。

改进：把可见单元格设计成有足够宽度和 gap 的输入格，统一字号/边框/焦点；保留单一原生输入的编辑、选区、粘贴和组合输入模型。主例只保留验证码与简洁反馈。

验收：桌面/390px 各格间距清晰，合法输入/粘贴/删除/键盘连续工作；显隐和重绘不更换编辑器身份，不显示原始协议计数。

位置：[PinInput 布局](../src/pin_input.rs#L379)。

### F02 · 窄屏的文字按钮和横向收缩失控

390px：Pagination 导航按钮高约274px、页码按钮高约122px，预览内部横向溢出54px；RichEditor 工具按钮高约99px，文字逐字竖排。桌面 Scroller tile 宽32px，文字区约13px，Item 文字变成竖排。Scroller 的 Next 操作仍能滚动132px，Pagination 点 Page 3 仍能切页，说明不能把布局失效写成控制器失效。

改进：Pagination 可见内容使用箭头/数字，完整名称放可访问属性；Toolbar 使用紧凑图标和合理分组/换行；Scroller 子项给出非收缩的合理宽度。排查共享 control 的 padding/min-width/flex-shrink 与长文本规则。

验收：390px 下按钮标签不逐字竖排，图标命中区合理；页码/工具栏/横滚键盘操作继续工作；页面与局部预览的溢出分别断言。

位置：[Pagination 标签](../src/navigation.rs#L252)；[Scroller 公开示例](../examples/web/src/focused_demos/mantine_pages.rs#L1838)。

### F03 · 默认预览绑定了其他组件或验收夹具

NavLink 页测得 .sapid-nav-link 为0而 .sapid-navigation-menu 为1，目录实际绑定 navigation_menu.rs。Anchor 绑定整个 typography.rs，Burger 绑定 actions.rs，Transition 绑定 environment.rs。FloatingWindow 初始显示两个覆盖文档的浮窗。Carousel、Dropzone 等主例被 Owner/拒绝/并发/生命周期测试操作占据。

改进：每个页面首例绑定自己的最小真实组件；把验收程序继续留在测试入口，把有教学价值的高级场景改写成独立说明和源码。浮窗/对话框默认由明确按钮打开。

验收：页面名称、首例组件、源码面板三者一致；默认阅读不被浮窗遮挡；公开 Usage 不出现裸露协议计数或无解释的 Before/After/Owner 控件。

位置：[Anchor/Burger/NavLink 目录](../examples/web/src/showcase_catalog.rs#L72)；[Transition 目录](../examples/web/src/showcase_catalog.rs#L139)。

### F04 · MiniCalendar 和 Heatmap 是组件定义差异

MiniCalendar 的构造函数返回 Calendar，仅设置 ExtraSmall/mini，实测仍有42个日期格；官方对应组件是短横向日期条，默认显示七天。Heatmap 与 Matrix 共用 MatrixData(row/column/cells)；官方 Heatmap 按日期组织周列与贡献值，一般矩阵另有 MatrixChart。对应事实参见 [MiniCalendar](https://mantine.dev/dates/mini-calendar/) 与 [Heatmap](https://mantine.dev/charts/heatmap/)。

改进：给 MiniCalendar 设计可见天数与滚动日期窗口，给 Heatmap 增加日期数据/范围/周列适配；保留现有完整月历和通用 MatrixChart 各自职责。必须修改模型与行为，不能只换皮肤。

验收：MiniCalendar 按配置天数呈现连续日期并可导航；Heatmap 用日期键数据正确定位周/日及范围，MatrixChart 仍正确处理一般二维矩阵。

位置：[mini_calendar 构造](../src/calendar.rs#L366)；[Heatmap/Matrix 数据默认值](../src/plot_widget.rs#L45)；[MatrixData](../src/plot_model.rs#L65)。

### F05 · Transition 不能作为已经交付的通用组件展示

目录描述明确写 general Transition builder is not yet available；页面实际展示 environment/RTL/token 夹具。Collapse 的专用高度动画不等于通用进入/退出过渡契约。

改进：将页面明确标注为能力边界，或实现受控挂载、进入/退出阶段、完成/取消和 Reduce 的通用有限契约；在真实过渡示例完成前，不以 environment 的平台状态替代。

验收：公开说明与实现一致；若实现通用过渡，验证初始态、反向切换、内容退休、零时长与 Reduce，分别记录宿主证据。

位置：[Transition 目录说明](../examples/web/src/showcase_catalog.rs#L139)。

### F06 · Menu / Menubar 仍缺部分内容组合能力

MenuEntry 仅有 Item/RadioGroup/Submenu/Separator；MenuItem 提供 label/kind/checked 等，没有通用搜索编辑器、独立组标签或左右自定义内容槽。RadioGroup 自身有 label，不能据此说所有标签均缺失。打开默认 Tools 菜单后未找到搜索输入；源类型检查才是功能差异的主要依据。官方内容能力可对照 [Menu](https://mantine.dev/core/menu/) 和 [Menubar](https://mantine.dev/core/menubar/)。

改进：按 Widget/Children 组合模型扩展有限搜索、独立组说明、图标/快捷键等展示区，复用当前菜单控制器和焦点/祖先策略；将内容能力分段演示。Menubar 本轮已有 hover/position/width 和七段介绍，后续需补共享内容能力。

验收：混合项、搜索、checkbox/radio、子菜单的键盘/焦点与受控关闭保持一致；禁用项不激活，定制内容不引入竞争控制器；原生能力另按已支持范围验证。

位置：[MenuItem](../src/menu/mod.rs#L33)；[MenuEntry](../src/menu/mod.rs#L258)。

### F07 · Accordion / Tabs / Timeline / Breadcrumbs 缺默认控件视觉

Accordion 以普通按钮为主，Tabs 是一排普通填色按钮，Timeline 是无连接线的编号列表，Breadcrumbs 缺可辨认分隔。Accordion 实测可开合、Tabs 实测可选中 Messages；现有 multiple/collapsible、trigger_content、Timeline marker 和 Breadcrumbs 展开能力都应保留。

改进：先完善共享默认样式：整行 disclosure/箭头、tab strip/选中标记、时间线节点与连接、面包屑分隔，再补变体和方向示例。不要为视觉修正重写现有选择控制器。

验收：默认示例无需自定义 CSS 即可辨认组件类型；移动/深色/焦点/禁用状态清晰；原有展开/选择/自定义内容不退化。

位置：[Accordion trigger](../src/accordion/mod.rs#L202)；[Tabs trigger](../src/tabs/mod.rs#L255)；[Timeline 列表](../src/timeline.rs#L213)；[Breadcrumbs](../src/navigation.rs#L401)。

### F08 · 选择器和日期输入的动作区应回到输入框内

Select 首例以整行文本按钮显示当前项，Clear selection 另占一行；DatePickerInput、DateTimePicker、MonthPickerInput、YearPickerInput 的 Choose… 文本触发器与值输入分离，窄屏更冗长。Select 现有 combobox 角色和打开选择逻辑存在。

改进：统一输入外壳、当前值、内嵌展开/日历/清除动作与 disabled/loading 占位；复用 PasswordInput 本轮紧凑动作的思路，保留各控件独立语义和修订/焦点规则。FileInput 可采用同样的文件摘要外壳。

验收：桌面/390px 动作不另占大行，长值合理截断/换行，清除与打开互不误触；输入身份、选区、表单值、可访问名称和弹层焦点不退化。

### F09 · 局部 overflow:auto 正在掩盖组件溢出

543 次渲染未发现整个文档横向溢出，但390px下四个首例内部仍溢出：Pagination 54px、Affix 158px、BackgroundImage 18px、TimeGrid 24px。通用预览容器允许横滚，因此“页面无横滚”不能证明控件宽度正确。BackgroundImage 的深色文字和测试背景对比也不足。

改进：逐项修正实际宽度/定位/换行；仅对有意支持横滚的网格声明滚动策略。用适合阅读的本地图片/遮罩演示背景，不用双色测试图承载主例。

验收：在390px同时检测 document 与 preview 的 scrollWidth；非滚动控件内部溢出归零，有意横滚的组件给出清楚入口且内容可达。

### F10 · 文档结构不足，但不能按标题数量判功能缺失

本轮181页只有22页使用独立 .site-component-detail；其余159页没有该包装，但 Button、Splitter、TreeSelect、Table、Calendar、四种基础图表、Schedule 和 RichEditor 等在 Usage 内已展示多类场景。官方抓取共2209个二级标题；这只是组织形式指标，不能相减得到“缺失功能数”。

改进：先把已有行内场景拆成“解释→独立 demo→对应源码”，再按能力逐项确认确实缺少的演示/API；简单布局/文本组件保持简洁，复杂选择器、菜单、日期和调度组件优先。

验收：每个分段可独立理解、源码可直接对应运行结果；API 表与实际 builder 一致，示例覆盖与平台能力分开标识。

### F11 · 图表与调度页面需要重新安排主次

22个图表普遍把小图放进窄卡片，Hide/Previous/Next/Clear 等大块控制占主要面积；Sparkline 尤其带七个 Hide Wn 按钮。13个调度页面很多场景堆在 Usage，日期/工具栏/网格的层次弱。部分数据导航承担非指针访问用途，不应直接删除。

改进：图表扩大图面，图例和数据导航紧凑分组，保留键盘/辅助读数；调度按视图、日期/资源窗口、全天/重叠、受控提案和复发数据分节，统一标题、间距和窄屏滚动。

验收：首屏能看清主图或主视图；各辅助操作可键盘访问，窄屏有明确滚动；未实际操作的拖动/创建/复发组合继续标为待补测。

### F12 · 相似组件不意味着承诺 React 插件或所有宿主等价

RichEditor 基于 Sapid 的有限 Rust 文档模型，与上游基于 Tiptap 的可扩展模型存在产品边界；Box/Portal 等底层容器的 API 也不应按 React 属性名逐字对齐。浏览器审查不证明原生 OS 输入、系统菜单、文件对话框、屏幕阅读器或移动设备通过。

改进：将本轮确认的布局/示例问题直接修复；对真实缺失功能写明确有限契约与验收；对技术栈/宿主边界说明差异，不为了同名 API 引入不受控兼容层。

验收：每项完成声明都有具体目标与证据；保留 Cage/Bond/Vary、Widget 和现有显式服务边界，不用编译成功替代实际平台验收。

## 181 页逐页清单

每行记录本轮实际看到的问题或建议，功能未充分操作验证的项目使用“展示/说明/补测”，不据此判定 API 缺失。上游链接是本轮实际访问的对应官方页面。Sapid 对应 Rust 示例路径、页面结构计数及截图路径记录在同名 JSON 中。

### Layout

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 1 | [AppShell](https://mantine.dev/core/app-shell/) | P2 · 文档/布局 | 本轮已修正主框架、折叠导航与移动抽屉；继续把 section 滚动、尺寸/偏移、边框及受控配置拆成独立说明，不能据三个示例宣称全量对齐。 |
| 2 | [AspectRatio](https://mantine.dev/core/aspect-ratio/) | P2 · 示例 | 已有基础与 flex 场景；使用能看清裁切的本地图片，独立解释比例、子内容适配与受限容器，避免双色测试图承担主要展示。 |
| 3 | [Center](https://mantine.dev/core/center/) | P3 · 文档 | 主示例可辨认；补充水平/垂直居中、inline 内容和容器高度的对照说明，无需为简单组件机械复制分段数量。 |
| 4 | [Container](https://mantine.dev/core/container/) | P3 · 文档 | 基础容器正常；增加不同最大宽度、流式宽度与嵌套边距的并排说明，让尺寸差异可见。 |
| 5 | [Flex](https://mantine.dev/core/flex/) | P2 · 文档 | 当前基础排列不足以解释布局选择；分开展示方向、换行、对齐、间距及窄屏变化，并说明 Rust 侧显式样式契约。 |
| 6 | [Grid](https://mantine.dev/core/grid/) | P2 · 文档 | 补充分栏跨度、偏移、响应宽度与不等高内容的独立示例；不要仅用等宽色块证明布局能力。 |
| 7 | [Group](https://mantine.dev/core/group/) | P2 · 文档 | 基础横排可见；补充不换行/换行、等宽增长和长文本子项场景，特别说明收缩约束，避免重现 Scroller 的竖排文字。 |
| 8 | [SimpleGrid](https://mantine.dev/core/simple-grid/) | P2 · 文档 | 将列数、间距和窄屏列变化拆开说明；以不等内容长度展示单元格的实际宽度与换行。 |
| 9 | [Space](https://mantine.dev/core/space/) | P3 · 文档 | 基础占位可辨认；分别说明水平与垂直间距，并与父容器 gap 的使用场景区分。 |
| 10 | [Splitter](https://mantine.dev/core/splitter/) | P2 · 文档/补测 | 已有嵌套、固定侧栏、多面板、受控尺寸等行内场景；整理成说明/示例/源码分段，补测键盘、最小尺寸和窄容器拖动，不把缺少 detail 包装误判为缺功能。 |
| 11 | [Stack](https://mantine.dev/core/stack/) | P3 · 文档 | 基础纵排可辨认；补充 align/justify/gap 的直观对照和长内容，说明容器高度对分布的影响。 |

### Inputs

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 12 | [AlphaSlider](https://mantine.dev/core/alpha-slider/) | P2 · 外观/文档 | 透明度轨道和读数的语义不够直观；增加透明棋盘背景及不同底色对照，独立说明受控值与禁用状态。 |
| 13 | [AngleSlider](https://mantine.dev/core/angle-slider/) | P2 · 示例 | 公开预览仍包含验收型输出；先给角度控件和简洁读数，再分段展示步长、方向、受控状态与键盘，移走协议计数。 |
| 14 | [Checkbox](https://mantine.dev/core/checkbox/) | P2 · 文档 | 基础勾选可辨认；分别展示不确定状态、组、标签位置、禁用/只读与错误反馈，补测标签点击和键盘。 |
| 15 | [Chip](https://mantine.dev/core/chip/) | P2 · 文档 | 基础选中样式可见；补充单选/多选组、取消选择、禁用与长标签，说明它与 Button/Checkbox 的选择语义。 |
| 16 | [ColorInput](https://mantine.dev/core/color-input/) | P2 · 文档/复核 | 已有 16 个独立细节段落，结构较完整；优先核对输入框动作、弹层尺寸、颜色预览及移动触控，精简重复说明，不按段落数重复补建。 |
| 17 | [ColorPicker](https://mantine.dev/core/color-picker/) | P2 · 外观/文档 | 已有四个细节示例；补透明棋盘、可读的通道值及不同格式/色板说明，检查窄屏轨道与输入区对齐。 |
| 18 | [Fieldset](https://mantine.dev/core/fieldset/) | P3 · 文档 | 基础 legend/内容结构可见；补充禁用 fieldset 与内部控件、嵌套布局的说明及边框间距对照。 |
| 19 | [FileInput](https://mantine.dev/core/file-input/) | P2 · 外观/示例 | 主预览更接近独立 Choose files 按钮和状态文字；补输入框式文件摘要、清除/多文件/长文件名展示，保留显式文件授权与取消语义。 |
| 20 | [HueSlider](https://mantine.dev/core/hue-slider/) | P2 · 外观/文档 | 彩虹轨道可见，但缺直观标签与当前值解释；补键盘、步长、禁用及不同尺寸示例，确保无需只靠颜色判断。 |
| 21 | [Input](https://mantine.dev/core/input/) | P2 · 文档 | 应作为输入外壳的系统说明：分开展示 label/description/error、必填、左右动作区、尺寸与禁用，供其他输入页共用约定。 |
| 22 | [JsonInput](https://mantine.dev/core/json-input/) | P2 · 示例/文档 | 公开例子混有提交/验证计数；改为 JSON 编辑、格式化、错误位置与失焦行为的独立讲解，调试输出留在验收页。 |
| 23 | [MaskInput](https://mantine.dev/core/mask-input/) | P2 · 示例/文档 | 当前偏验收工作台；给电话/日期等清晰示例，分别说明占位、原始值/显示值、编辑删除与组合输入边界。 |
| 24 | [NativeSelect](https://mantine.dev/core/native-select/) | P2 · 示例 | 保留真正原生 select；去掉 Before/After 和混杂的多选夹具，单独展示选项组、禁用、尺寸与单/多选差异。 |
| 25 | [NumberInput](https://mantine.dev/core/number-input/) | P2 · 外观/文档 | 补数字格式、精度、上下界、步长、失焦钳制和控制按钮的分段对照；检查框内动作大小与窄屏对齐。 |
| 26 | [PasswordInput](https://mantine.dev/core/password-input/) | P2 · 文档/回归 | 眼睛切换已改为输入框内图标；继续补受控显隐、左右区与 loading 的独立说明，回归输入身份、值、选区和可访问名称。 |
| 27 | [PinInput](https://mantine.dev/core/pin-input/) | P1 · 布局/示例 | 六个格子实测各仅 7.69px，挤成约 46px 的窄条，且有验收计数；重做格宽/间距/焦点表现，保留单一原生编辑器与 IME/选区模型。 |
| 28 | [Radio](https://mantine.dev/core/radio/) | P2 · 文档 | 补组、横纵排、描述、禁用和受控选择的独立展示；键盘检查以真实组语义为准，不从默认截图推断缺功能。 |
| 29 | [RangeSlider](https://mantine.dev/core/range-slider/) | P2 · 文档/补测 | 已有七个细节段落；重点补测双手柄交叉约束、键盘、标记标签与窄屏命中区域，减少大按钮对滑轨的干扰。 |
| 30 | [Rating](https://mantine.dev/core/rating/) | P2 · 文档 | 补半星/分数显示、只读、清空、尺寸和自定义符号的能力说明；若未支持相应能力，明确有限契约后再列实现任务。 |
| 31 | [SegmentedControl](https://mantine.dev/core/segmented-control/) | P2 · 文档/外观 | 补等宽/内容宽度、纵向、禁用项、长文本和受控值对照；检查选中轨道、焦点边框与紧凑尺寸。 |
| 32 | [Slider](https://mantine.dev/core/slider/) | P2 · 文档/补测 | 已有四个细节段落；继续核对 label/marks、步长、禁用和键盘，并在窄容器确认读数不会遮挡手柄。 |
| 33 | [Switch](https://mantine.dev/core/switch/) | P2 · 文档 | 补尺寸、标签位置、组和内部标记的展示；检查轨道/拇指比例与焦点样式，不把文字按钮当开关呈现。 |
| 34 | [Textarea](https://mantine.dev/core/textarea/) | P2 · 文档 | 区分固定行数与自动增高能力，展示 min/max rows、错误/禁用和长文本；明确各宿主支持边界，不能仅从编译推断原生自适应高度。 |
| 35 | [TextInput](https://mantine.dev/core/text-input/) | P2 · 文档/外观 | 补 label/description/error、左右区、变体、受控输入和长占位符；将清除/动作统一放入合理的输入外壳。 |

### Combobox

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 36 | [Autocomplete](https://mantine.dev/core/autocomplete/) | P2 · 文档/补测 | 已有六个细节段落；核对筛选、空结果、分组、选项渲染与移动弹层尺寸，保留组合输入和当前编辑器身份。 |
| 37 | [Cascader](https://mantine.dev/core/cascader/) | P2 · 文档/外观 | 已有禁用/只读等行内例子；分开解释层级展开、选中路径、搜索/清空能力与弹层宽度，检查窄屏多列可达性。 |
| 38 | [Combobox](https://mantine.dev/core/combobox/) | P2 · 文档 | 应讲清低层组合契约、触发器/目标/选项/搜索的关系，给最小例子后再给自定义组合，避免只展示一个成品选择器。 |
| 39 | [ComboboxPopover](https://mantine.dev/core/combobox-popover/) | P2 · 文档 | 把建议列表从普通 Popover 用法中讲清；补锚点、宽度、空结果、受控打开与键盘焦点示例。 |
| 40 | [MultiSelect](https://mantine.dev/core/multi-select/) | P2 · 外观/文档 | 已有三个独立细节示例；强化紧凑 token、框内清除/展开、长标签和空结果展示，检查窄屏多行输入高度。 |
| 41 | [Pill](https://mantine.dev/core/pill/) | P3 · 文档 | 基础 token 可辨认；补移除动作、禁用、尺寸、长文本及与输入框组合的简短示例。 |
| 42 | [PillsInput](https://mantine.dev/core/pills-input/) | P2 · 文档/外观 | 已有三个细节段落；继续补多行 token、光标位置、删除与 placeholder 关系，检查动作区与最后一行编辑器对齐。 |
| 43 | [Select](https://mantine.dev/core/select/) | P1 · 外观/组合 | 主触发器像普通整行文本按钮，Clear selection 另占一行；改为含当前值、内嵌箭头/清除的输入框外观，保留现有 combobox 角色与选择控制器。 |
| 44 | [TagsInput](https://mantine.dev/core/tags-input/) | P2 · 文档/补测 | 已有五个细节段落；突出分隔符、重复项、最大数量和粘贴的原子提案，补测空输入删除与组合输入，移走协议型输出。 |
| 45 | [TreeSelect](https://mantine.dev/core/tree-select/) | P2 · 文档/补测 | 已有九类行内情景，包括级联、搜索、渲染与限额；拆成清楚的解释/源码段落，补测窄屏、焦点及受控拒绝后的状态。 |

### Buttons

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 46 | [ActionIcon](https://mantine.dev/core/action-icon/) | P2 · 文档/外观 | 核对默认图标命中区、尺寸/变体与 accessible name，补 loading/disabled/group 示例；避免用长文字撑开图标按钮。 |
| 47 | [Button](https://mantine.dev/core/button/) | P2 · 文档 | 已有 full-width、左右区、compact、loading 和 group 等行内例子；整理成分段说明和对应源码，补窄屏长标签的宽度规则。 |
| 48 | [CloseButton](https://mantine.dev/core/close-button/) | P3 · 文档/回归 | 基础关闭动作可辨认；补尺寸/禁用和名称示例，检查图标居中与键盘焦点，不增加多余文字按钮。 |
| 49 | [CopyButton](https://mantine.dev/core/copy-button/) | P2 · 文档/边界 | 分开说明复制前后反馈、失败和超时；示例应以复制动作/反馈为中心，保持显式剪贴板服务与宿主支持范围。 |
| 50 | [FileButton](https://mantine.dev/core/file-button/) | P2 · 文档/边界 | 展示自定义触发器、多文件和取消选择；说明文件授权/读取分离，避免将原生文件能力当成普通字符串 value。 |
| 51 | [Toggle](https://mantine.dev/core/toggle/) | P2 · 示例 | 公开页仍带验收状态；换为可理解的按下/未按下示例，再说明受控拒绝、禁用和 group，回归 aria-pressed。 |
| 52 | [UnstyledButton](https://mantine.dev/core/unstyled-button/) | P3 · 文档 | 补自定义视觉但保留按钮语义的例子，明确键盘/禁用行为和链接场景的区别。 |

### Navigation

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 53 | [Anchor](https://mantine.dev/core/anchor/) | P1 · 错用示例 | 目录绑定整页 typography 夹具，含无关段落和 requests/ordered 输出；改为 Anchor 专属的链接、下划线、禁用/导航目标说明。 |
| 54 | [Breadcrumbs](https://mantine.dev/core/breadcrumbs/) | P1 · 外观 | 主例只见连续导航词，没有清晰分隔符；补默认/自定义分隔、溢出折叠和当前项样式，保留现有展开控制。 |
| 55 | [Burger](https://mantine.dev/core/burger/) | P1 · 错用示例 | 目录绑定 actions 工作台，小图标混在一组无关按钮中；建立独立开/关、尺寸、受控状态示例，检查图标与命中区比例。 |
| 56 | [NavLink](https://mantine.dev/core/nav-link/) | P1 · 错用示例/功能 | 页内实际为 NavigationMenu，零个 sapid-nav-link；先接真正 NavLink，再补嵌套/当前项/展开与展示区说明，现有 builder 缺描述和左右内容槽。 |
| 57 | [Pagination](https://mantine.dev/core/pagination/) | P1 · 布局 | 移动端导航文字逐字竖排，按钮高约 274px、页码高约 122px且预览溢出54px；显示箭头与紧凑数字，完整说明保留在可访问名称中。 |
| 58 | [Stepper](https://mantine.dev/core/stepper/) | P2 · 外观/文档 | 已有纵向例子；增强步骤圆点/连线、当前/完成/错误的层次，分别解释内容区、交互跳转和禁用。 |
| 59 | [TableOfContents](https://mantine.dev/core/table-of-contents/) | P2 · 示例/文档 | 主预览仍偏滚动验收夹具；换为有层级的真实文章，说明 heading 来源、活动项、滚动定位与嵌套显示。 |
| 60 | [Tabs](https://mantine.dev/core/tabs/) | P1 · 外观/文档 | 标签仍像普通填色按钮，缺少连贯 tab strip、基线和选中标记；保留已能工作的选择/自定义触发内容，补方向、变体与内容保留说明。 |
| 61 | [Tree](https://mantine.dev/core/tree/) | P2 · 示例/复核 | 已有八个独立细节段落，但 Usage 仍是拒绝提案/切换 Owner 的工作台；换成文件树主例，保留键盘、选择与加载的细节说明。 |

### Feedback

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 62 | [Alert](https://mantine.dev/core/alert/) | P2 · 文档/外观 | 基础标题可见；补颜色/变体、图标与关闭动作，以及无标题和长内容，检查标题/正文/图标的间距。 |
| 63 | [EmptyState](https://mantine.dev/core/empty-state/) | P2 · 示例 | 补图标/标题/说明/主次动作的真实空状态，分别展示无数据、无搜索结果和受限空间，避免只给一段空提示。 |
| 64 | [Loader](https://mantine.dev/core/loader/) | P3 · 文档 | 基础加载可辨认；补不同类型/尺寸/颜色、内联对齐和无障碍描述，说明自定义 loader 的有限方式。 |
| 65 | [Notification](https://mantine.dev/core/notification/) | P2 · 文档/外观 | 补图标、关闭、loading、长标题/正文等状态；与 notifications-system 的队列生命周期说明分开。 |
| 66 | [Progress](https://mantine.dev/core/progress/) | P2 · 复核 | 已有九个细节段落；重点检查分段标签、溢出、动画/Reduce 和读数对比，不重复建设已有用法。 |
| 67 | [RingProgress](https://mantine.dev/core/ring-progress/) | P2 · 复核 | 已有九个细节段落；核对分段圆环、中心内容、圆角、大小和读数的可读性，窄屏保持图形为主。 |
| 68 | [SemiCircleProgress](https://mantine.dev/core/semi-circle-progress/) | P2 · 复核 | 已有九个细节段落；核对半圆起止、中心读数、厚度与缩放，补与完整圆环的选择说明。 |
| 69 | [Skeleton](https://mantine.dev/core/skeleton/) | P3 · 文档 | 基础占位可见；补圆形、圆角、文本行及带 children 的加载切换，说明动画/Reduce 行为。 |

### Overlays

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 70 | [ActionBar](https://mantine.dev/core/action-bar/) | P2 · 示例/文档 | 给批量选择后的真实操作条，分开演示固定位置、关闭、禁用及窄屏排列，检查动作按钮不会挤压成竖排。 |
| 71 | [Affix](https://mantine.dev/core/affix/) | P1 · 布局/示例 | 390px 预览内部横向溢出158px；重新约束演示容器/固定位置和长内容，解释 viewport 固定与局部预览的关系。 |
| 72 | [Dialog](https://mantine.dev/core/dialog/) | P2 · 文档 | 补非模态对话框的位置/关闭与焦点说明，明确与 Modal 的区别；主示例避免遮挡阅读区域。 |
| 73 | [Drawer](https://mantine.dev/core/drawer/) | P2 · 文档/补测 | 补方向、尺寸、标题/关闭区、滚动、受控打开和嵌套焦点，核对移动端内容高度与关闭后恢复焦点。 |
| 74 | [FloatingIndicator](https://mantine.dev/core/floating-indicator/) | P2 · 示例 | 当前偏指标/控制夹具；给随活动项移动的独立主例，再说明目标测量、过渡、尺寸变化与 RTL。 |
| 75 | [FloatingWindow](https://mantine.dev/core/floating-window/) | P1 · 示例/布局 | 初始就有两个固定浮窗盖住文档和测试控件；默认关闭，点击后在明确区域演示，几何/模态/生命周期分开讲解。 |
| 76 | [HoverCard](https://mantine.dev/core/hover-card/) | P2 · 文档/补测 | 补延时、内容交互、位置、箭头与触屏替代入口；核对从触发器到卡片的指针移动不会误关。 |
| 77 | [LoadingOverlay](https://mantine.dev/core/loading-overlay/) | P2 · 示例/文档 | 当前夹具把多个独立 dialog/region 混在主例；改为一个局部 loading 容器，说明定位上下文、遮罩和焦点/禁用行为。 |
| 78 | [Menu](https://mantine.dev/core/menu/) | P1 · 功能/示例 | 主例带验收输出；源码 MenuEntry 无搜索编辑器/独立组标签/任意左右展示槽，先补菜单组合能力及独立示例，再与上游细节逐项对照。 |
| 79 | [Menubar](https://mantine.dev/core/menubar/) | P1 · 功能/后续 | 本轮已有七段独立说明和 hover/position/width 支持；仍继承 Menu 的搜索/标签/内容槽差异，需继续完善内容组合和子菜单展示，不能标成完全对齐。 |
| 80 | [Modal](https://mantine.dev/core/modal/) | P2 · 文档/补测 | 已有两个独立细节示例；继续补标题/关闭、长内容滚动、尺寸、嵌套与受控焦点，保留已有宿主有限支持声明。 |
| 81 | [Overlay](https://mantine.dev/core/overlay/) | P2 · 文档 | 分开说明透明度、背景/模糊能力、局部与全屏定位及叠层；使用能看清遮罩效果的背景，明确支持与未支持样式。 |
| 82 | [Popover](https://mantine.dev/core/popover/) | P2 · 文档/补测 | 补位置/箭头、宽度、关闭策略、受控状态与键盘焦点；检查窄屏碰撞和长内容，而非只用标题加按钮。 |
| 83 | [Tooltip](https://mantine.dev/core/tooltip/) | P2 · 文档/补测 | 补位置、延时、多行、禁用触发器与移动端替代说明；保留可访问名称，不让 tooltip 成为唯一标签。 |
| 84 | [Tour](https://mantine.dev/core/tour/) | P2 · 示例/文档 | 当前偏步骤验收；用真实目标区域展示步骤、前后导航、跳过、目标缺失和恢复焦点，并把状态计数移出主例。 |

### Data display

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 85 | [Accordion](https://mantine.dev/core/accordion/) | P1 · 外观/文档 | 目前像数个普通边框按钮，缺整行标题/展开箭头/分隔与变体；点击开合已工作，保留 multiple/collapsible/自定义 trigger 内容，改共享皮肤并分段说明。 |
| 86 | [Avatar](https://mantine.dev/core/avatar/) | P2 · 示例/文档 | 已有 Avatar.Group；替换双色测试图，展示照片、姓名回退、图标回退、重叠和尺寸，补加载失败的含义。 |
| 87 | [BackgroundImage](https://mantine.dev/core/background-image/) | P1 · 布局/可读性 | 移动预览内部溢出18px，双色测试背景上的深色文案对比不足；约束尺寸，换合适资源，展示 cover/定位与可读的前景层。 |
| 88 | [Badge](https://mantine.dev/core/badge/) | P2 · 文档 | 补尺寸/变体、左右展示区、圆点和长文本；保持徽章紧凑，说明何时需要实际交互控件。 |
| 89 | [Card](https://mantine.dev/core/card/) | P2 · 示例/文档 | 布局可辨认但测试图削弱示范；换真实比例本地资源，独立展示分区、边框/阴影、交互和不同内容长度。 |
| 90 | [ColorSwatch](https://mantine.dev/core/color-swatch/) | P2 · 示例/外观 | 清理验收型输出，补透明棋盘、圆角/尺寸、复制或选择动作，当前颜色须有文字替代。 |
| 91 | [DataList](https://mantine.dev/core/data-list/) | P2 · 文档/外观 | 补横纵向、标签宽度、长值换行及紧凑空间对照；让标签/值的视觉层次明显，不只是一列文字。 |
| 92 | [Image](https://mantine.dev/core/image/) | P2 · 示例 | 用能识别裁切的本地图片替换双色测试图，分开展示 fit、尺寸、圆角、失败回退和可访问替代文字。 |
| 93 | [Indicator](https://mantine.dev/core/indicator/) | P2 · 文档 | 补各方位、偏移、数值/文本、自定义大小与禁用状态；检查父内容和指示器的遮挡关系。 |
| 94 | [Kbd](https://mantine.dev/core/kbd/) | P3 · 文档 | 基础键帽可辨认；补组合快捷键和尺寸/暗色对照，保证它只负责展示而不暗示已注册键盘命令。 |
| 95 | [NumberFormatter](https://mantine.dev/core/number-formatter/) | P3 · 文档 | 基础格式输出可见；补千分位、小数位、前后缀和负数的对照，明确采用的 locale/格式契约。 |
| 96 | [OverflowList](https://mantine.dev/core/overflow-list/) | P2 · 示例/文档 | 主例偏尺寸/状态测试；给真实导航或标签列表，分段说明溢出摘要、可用宽度、受控展开和键盘到达隐藏项。 |
| 97 | [RollingNumber](https://mantine.dev/core/rolling-number/) | P2 · 文档/补测 | 补增减、格式化、位数变化及 Reduce 模式，检查最终语义读数与视觉过渡，避免只用一次数字变化证明动效。 |
| 98 | [Spoiler](https://mantine.dev/core/spoiler/) | P2 · 示例/文档 | 去掉拒绝/生命周期型状态，展示长文章收起/展开、阈值、标签与内容变化，补焦点和无动画模式说明。 |
| 99 | [ThemeIcon](https://mantine.dev/core/theme-icon/) | P3 · 文档/外观 | 补尺寸、圆角、颜色/变体和图标缩放对照；明确其为展示容器，交互场景使用 ActionIcon。 |
| 100 | [Timeline](https://mantine.dev/core/timeline/) | P1 · 外观 | 主例仍为普通编号列表，缺时间线连接线和默认节点；先做默认皮肤及活动段，再展示现有 marker 内容槽、对齐和长正文。 |

### Typography

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 101 | [Blockquote](https://mantine.dev/core/blockquote/) | P3 · 文档 | 补来源/署名、图标、颜色和长引用排版，检查正文与边线/引号的间距。 |
| 102 | [Code](https://mantine.dev/core/code/) | P3 · 文档 | 补 inline 与 block、长行换行/滚动、颜色对照；把语法高亮能力引导到 CodeHighlight。 |
| 103 | [Highlight](https://mantine.dev/core/highlight/) | P3 · 文档 | 补多个匹配、不区分大小写/转义规则等实际支持说明，以及无匹配和长文本示例。 |
| 104 | [List](https://mantine.dev/core/list/) | P3 · 文档 | 补有序/无序、嵌套、图标和间距，检查 marker 对齐，不为纯展示列表引入选择语义。 |
| 105 | [Mark](https://mantine.dev/core/mark/) | P3 · 文档 | 补不同背景、长行换行及与 Text/Highlight 的组合说明，检查暗色下前景对比。 |
| 106 | [Table](https://mantine.dev/core/table/) | P2 · 文档/补测 | 已有 caption/tfoot/sticky、滚动、间距、边框、纵向和选择行等场景；拆成独立解释/源码，补移动横滚与表头关联检查。 |
| 107 | [Text](https://mantine.dev/core/text/) | P2 · 文档 | 补字号/字重/颜色、截断/多行限制、继承和 inline 的对照；明确文本渲染与容器布局的边界。 |
| 108 | [Title](https://mantine.dev/core/title/) | P3 · 文档 | 已有多个级别和行数限制；补字号与语义级别独立配置的说明，检查截断后完整可访问名称。 |
| 109 | [Typography](https://mantine.dev/core/typography/) | P2 · 文档 | 展示真实富文本文章中的标题/段落/列表/链接/表格，说明作用范围与默认样式，不混入导航/请求验收状态。 |

### Miscellaneous

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 110 | [Box](https://mantine.dev/core/box/) | P3 · 文档/边界 | 这是通用容器，应讲清 Widget/显式样式与 HTML 层的关系；不把 React polymorphic/component 属性照搬为缺失 API。 |
| 111 | [Collapse](https://mantine.dev/core/collapse/) | P2 · 文档/补测 | 补受控开合、内容高度变化、时长/Reduce 及完成回调说明；回归逻辑隐藏、焦点和原生有限支持，不以首帧截图证明动画等价。 |
| 112 | [Divider](https://mantine.dev/core/divider/) | P3 · 文档 | 补水平/垂直、标签位置、虚线/点线及 flex 容器约束；检查长标签不会压坏分隔线。 |
| 113 | [FocusTrap](https://mantine.dev/core/focus-trap/) | P2 · 文档/补测 | 以打开后的真实表单说明初始焦点、Tab 循环、恢复和动态禁用；本轮未逐一验证所有键盘组合，应补专门操作记录。 |
| 114 | [Marquee](https://mantine.dev/core/marquee/) | P2 · 文档/补测 | 补方向/速度、暂停、重复内容和 Reduce 策略；检查静止替代及不可读的持续运动，避免只给自动滚动首帧。 |
| 115 | [Paper](https://mantine.dev/core/paper/) | P3 · 文档 | 补圆角、边框、阴影和 padding 对照，说明它与 Card 的组合职责。 |
| 116 | [Portal](https://mantine.dev/core/portal/) | P2 · 文档/边界 | 上游没有被通用选择器找到内联预览；应以层级/目标容器例子说明 Sapid 的实际宿主能力，不能由截图缺失判定功能缺失。 |
| 117 | [ScrollArea](https://mantine.dev/core/scroll-area/) | P2 · 文档/补测 | 阅读区域主例可用；补横/纵滚动、滚动条策略、尺寸、滚动定位和嵌套，明确浏览器/原生滚动的支持边界。 |
| 118 | [Scroller](https://mantine.dev/core/scroller/) | P1 · 布局 | 16 个 tile 被压成32px宽，文字区约13px、每个字母竖排；为项目设置合理非收缩宽度和 gap，下一项滚动本身已验证可用（132px）。 |
| 119 | [Toolbar](https://mantine.dev/core/toolbar/) | P2 · 外观/文档 | 补紧凑图标组、分隔、溢出和 roving focus 的操作示例；检查长标签在窄屏不会被压成 RichEditor 式竖排。 |
| 120 | [Transition](https://mantine.dev/core/transition/) | P1 · 功能/错用示例 | 目录明确写 general Transition builder 尚未提供，实际预览是 environment/RTL/tokens 夹具；需单独设计进入/退出契约，或清晰标为能力边界，不能当已实现的同名组件展示。 |
| 121 | [VisuallyHidden](https://mantine.dev/core/visually-hidden/) | P2 · 示例/文档 | 清理 Before/After 等验收内容，以图标按钮/表单标签说明视觉隐藏和可访问名称；保留专门焦点可见说明。 |

### Dates

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 122 | [DatePicker](https://mantine.dev/dates/date-picker/) | P2 · 文档/补测 | 已有六个细节段落；补测单/多/范围、禁用日期、locale/周起始和键盘，收紧日期单元格/头部间距。 |
| 123 | [DatePickerInput](https://mantine.dev/dates/date-picker-input/) | P1 · 外观/组合 | 输入值与 Choose dates 文本触发器分行呈现；合成输入框内的日历/清除动作，保留现有六段选择示例及焦点/值同步。 |
| 124 | [DateTimePicker](https://mantine.dev/dates/date-time-picker/) | P1 · 外观/组合 | 有十段内容，但日期/时间与大文本动作缺少统一输入框结构；整理内嵌动作、弹层布局和移动宽度，受控/边界说明分段保留。 |
| 125 | [InlineDateTimePicker](https://mantine.dev/dates/inline-date-time-picker/) | P2 · 文档/外观 | 已有十段细节示例；优化月历与时间区的对齐、间距和窄屏堆叠，说明它与弹层式 DateTimePicker 的选择场景。 |
| 126 | [DateInput](https://mantine.dev/dates/date-input/) | P2 · 示例/文档 | 当前偏可编辑日期验收夹具；把输入解析、格式、无效值、失焦和日历选择拆成主例/细节，去掉原始计数。 |
| 127 | [MonthPicker](https://mantine.dev/dates/month-picker/) | P2 · 外观/文档 | 补单/多/范围、年份导航、禁用月份和 locale，调整网格/选中范围表现，不能只凭默认月份判定能力。 |
| 128 | [MonthPickerInput](https://mantine.dev/dates/month-picker-input/) | P1 · 外观/组合 | 月份值和 Choose months 独立文本按钮使布局冗长；用共用输入动作外壳，分开说明显示格式、范围和清空。 |
| 129 | [YearPicker](https://mantine.dev/dates/year-picker/) | P2 · 外观/文档 | 补年份区间导航、单/多/范围与禁用年，核对网格间距、当前年和选中范围区别。 |
| 130 | [YearPickerInput](https://mantine.dev/dates/year-picker-input/) | P1 · 外观/组合 | 年份值和 Choose years 分离；统一输入框、内嵌展开/清除和弹层宽度，再补格式与受控值说明。 |
| 131 | [TimeInput](https://mantine.dev/dates/time-input/) | P2 · 文档/补测 | 已有五个细节段落；补测时分秒、12/24小时、上下界、键盘编辑和空值，核对输入区与动作尺寸。 |
| 132 | [TimePicker](https://mantine.dev/dates/time-picker/) | P2 · 文档/复核 | 已有十五个细节段落，避免重复堆例子；核对滚动列、时间格式、禁用时间、popover 尺寸和移动命中区。 |
| 133 | [TimeGrid](https://mantine.dev/dates/time-grid/) | P1 · 布局/文档 | 390px 预览内部溢出24px；收紧网格列/按钮最小宽度或说明横滚策略，再分段展示间隔、上下界、禁用和受控选择。 |
| 134 | [TimeValue](https://mantine.dev/dates/time-value/) | P3 · 文档 | 补12/24小时、秒、空值和格式化输出，说明它是显示组件而不是时间编辑器。 |
| 135 | [MiniCalendar](https://mantine.dev/dates/mini-calendar/) | P1 · 功能定义 | Sapid 是42天月历的 ExtraSmall 版本；Mantine 是默认七天的横向日期条。需要专门的可见天数/移动窗口与导航模型，不能只改 CSS。 |
| 136 | [Calendar](https://mantine.dev/dates/calendar/) | P2 · 文档/复核 | 已有自定义日期选择、全宽、静态日期、周数等行内场景；拆成分段解释，说明低层 Calendar 与 DatePicker 的职责。 |

### Charts

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 137 | [AreaChart](https://mantine.dev/charts/area-chart/) | P2 · 外观/文档 | 已有堆叠、参考线和 brush 场景，但卡片/大控制按钮抢占主图；扩大图面、紧凑图例，保留可键盘访问的数据导航。 |
| 138 | [BarChart](https://mantine.dev/charts/bar-chart/) | P2 · 外观/文档 | 已有堆叠、参考线和 brush；改善图面比例、轴标签和图例，分开说明方向、序列与数值格式的实际支持。 |
| 139 | [LineChart](https://mantine.dev/charts/line-chart/) | P2 · 外观/文档 | 已有图例、参考线和 brush；突出折线与轴，弱化大块控制按钮，补曲线/缺失值等能力说明与对照，而非从截图推断缺失。 |
| 140 | [CompositeChart](https://mantine.dev/charts/composite-chart/) | P2 · 外观/文档 | 已有图例/参考/brush；用不同系列类型的清晰对照说明组合关系、轴与提示内容，控制区改为紧凑辅助区。 |
| 141 | [CandlestickChart](https://mantine.dev/charts/candlestick-chart/) | P2 · 外观/文档 | 主图偏小、控制区偏重；补 OHLC 数据含义、涨跌色、时间轴/范围和 tooltip 的独立说明，保持可访问读数。 |
| 142 | [DonutChart](https://mantine.dev/charts/donut-chart/) | P2 · 外观/文档 | 扩大圆环相对卡片比例，改紧凑图例，补中心内容、厚度、标签和选中读数说明。 |
| 143 | [PieChart](https://mantine.dev/charts/pie-chart/) | P2 · 外观/文档 | 将图形与图例作为主体，补标签/角度/数值格式与空数据，数据导航保留为辅助操作。 |
| 144 | [FunnelChart](https://mantine.dev/charts/funnel-chart/) | P2 · 外观/文档 | 补阶段名称、数值和颜色映射、标签/tooltip；避免只有几何图和一列大按钮。 |
| 145 | [RadarChart](https://mantine.dev/charts/radar-chart/) | P2 · 外观/文档 | 补网格/轴标签、不同序列和量纲说明，扩大雷达图，紧凑化 Hide/Previous/Next 控制。 |
| 146 | [ScatterChart](https://mantine.dev/charts/scatter-chart/) | P2 · 外观/文档 | 补坐标轴/单位、系列图例、点大小与 tooltip 的说明，增加可读刻度并调整图面和控制区比例。 |
| 147 | [BubbleChart](https://mantine.dev/charts/bubble-chart/) | P2 · 外观/文档 | 明确 x/y/size 三个维度，补大小范围、系列颜色与读数；改善坐标说明和窄屏图面。 |
| 148 | [RadialBarChart](https://mantine.dev/charts/radial-bar-chart/) | P2 · 外观/文档 | 补环轨、起止角、数值归一和标签；让环图为主体，系列开关改紧凑图例。 |
| 149 | [Sparkline](https://mantine.dev/charts/sparkline/) | P2 · 示例/外观 | 用于紧凑趋势的组件却带七个 Hide W1…W7 大按钮；先放进指标卡/表格真实场景，辅助读数与操作另列细节。 |
| 150 | [Heatmap](https://mantine.dev/charts/heatmap/) | P1 · 功能定义 | 当前与 MatrixChart 共用 row/column MatrixData，显示通用矩阵；Mantine Heatmap 是日期贡献图。需补日期数据/周列/范围模型，矩阵用途继续由 MatrixChart 承担。 |
| 151 | [BarsList](https://mantine.dev/charts/bars-list/) | P2 · 外观/文档 | 补排名标签、数值对齐、正负值、颜色和排序示例；减少辅助控制对列表本身的挤占。 |
| 152 | [BulletChart](https://mantine.dev/charts/bullet-chart/) | P2 · 外观/文档 | 补实际值、目标值和区间含义及格式，改善轴/标记可读性与图面比例。 |
| 153 | [GaugeChart](https://mantine.dev/charts/gauge-chart/) | P2 · 外观/文档 | 补范围、阈值/分段、读数/单位和尺寸；把仪表主体与辅助数据导航分开。 |
| 154 | [WaffleChart](https://mantine.dev/charts/waffle-chart/) | P2 · 外观/文档 | 补单元格代表值、行列/间距、图例和舍入说明，扩大格子图并压缩系列按钮。 |
| 155 | [Treemap](https://mantine.dev/charts/treemap/) | P2 · 外观/文档 | 补层级数据、节点标签/颜色、父子导航和小区域回退，保持可访问导航但避免按钮压过图面。 |
| 156 | [SunburstChart](https://mantine.dev/charts/sunburst-chart/) | P2 · 外观/文档 | 补层级、扇区标签/颜色与下钻说明，改紧凑辅助导航，检查小扇区可达与读数。 |
| 157 | [SankeyChart](https://mantine.dev/charts/sankey-chart/) | P2 · 外观/文档 | 补源/目标/流量数据、节点标签与颜色，增加图形可读宽度，保留对应的可访问数据路径。 |
| 158 | [MatrixChart](https://mantine.dev/charts/matrix-chart/) | P2 · 文档/外观 | 保留一般行列矩阵定位，补行列标签、色阶/legend、空单元和负值；与日期型 Heatmap 的用途明确区分。 |

### Schedule

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 159 | [Schedule](https://mantine.dev/schedule/schedule/) | P2 · 文档/外观 | 已有视图、受控窗口、应用窗口及复发数据行内例子；拆清工具栏、视图切换与事件提案说明，改善日期层级和网格间距。 |
| 160 | [DayView](https://mantine.dev/schedule/day-view/) | P2 · 文档/外观 | 已有重叠/全天/呈现选项；突出单日时间轴、事件层次与滚动，补操作记录，不把未点击的拖动/创建直接列为缺失。 |
| 161 | [MonthView](https://mantine.dev/schedule/month-view/) | P2 · 文档/外观 | 已有 disclosure/locale/周起始；优化格子留白、事件摘要/更多入口与日期标题，将有限控制契约单独说明。 |
| 162 | [WeekView](https://mantine.dev/schedule/week-view/) | P2 · 文档/外观 | 已有重叠/全天场景；提高时间/日期表头层次和事件可读性，补窄屏滚动、定位当前时间与键盘操作说明。 |
| 163 | [YearView](https://mantine.dev/schedule/year-view/) | P2 · 文档/外观 | 已有日历呈现/locale 场景；改善年度网格密度和事件摘要，分开展示月份导航与事件展开。 |
| 164 | [MobileMonthView](https://mantine.dev/schedule/mobile-month-view/) | P2 · 文档/外观 | 已有多种月份呈现；重点检查390px下日期网格、下方日程和选中态，说明触控路径与普通 MonthView 的差别。 |
| 165 | [AgendaView](https://mantine.dev/schedule/agenda-view/) | P2 · 文档/外观 | 多日期长列表全部堆在 Usage；分开空日期、全天/时间事件、locale 和受控选择，增强日期分组层次。 |
| 166 | [ResourcesDayView](https://mantine.dev/schedule/resources-day-view/) | P2 · 文档/外观 | 独立说明资源列、全天/重叠、日期窗口及资源窗口；改善表头和窄屏横滚，不把默认两资源当能力上限。 |
| 167 | [ResourcesWeekView](https://mantine.dev/schedule/resources-week-view/) | P2 · 文档/外观 | 补资源/日期双层表头与窗口导航的清晰例子，改善多列密度，补测冻结表头和键盘可达性是否在支持契约内。 |
| 168 | [ResourcesMonthView](https://mantine.dev/schedule/resources-month-view/) | P2 · 文档/外观 | 多资源多日期表格信息密度高；补资源名称、事件溢出/展开和横滚说明，提供宽窄容器对照。 |
| 169 | [ResourcesSchedule](https://mantine.dev/schedule/resources-schedule/) | P2 · 文档/复核 | 已有受控状态、移动/缩放提案和有限窗口；拆独立细节段落并补实际操作记录，说明组件提案与应用接受的分工。 |
| 170 | [Recurring events](https://mantine.dev/schedule/recurring-events/) | P2 · 文档 | 当前主要是频率/间隔/结束三项摘要；补规则数据、展开窗口、例外/修改等已支持边界与实际日历例子，未核实能力不标完成。 |
| 171 | [Events data](https://mantine.dev/schedule/events-data/) | P2 · 文档/边界 | 本身偏数据契约，不强求视觉 demo；补完整类型样例、时间/资源标识、窗口和验证错误，与实际视图串联说明。 |

### Extensions

| # | 页面 / 官方参考 | 优先级 · 类型 | 观察与下一步 |
| --- | --- | --- | --- |
| 172 | [CodeHighlight](https://mantine.dev/code-highlight/code-highlight/) | P2 · 文档/示例 | 已有复制、语言切换和展开控件；分开主题、语言范围、长行、行号与多文件说明，将剪贴板状态放在动作反馈处。 |
| 173 | [JsonViewer](https://mantine.dev/code-highlight/json-viewer/) | P2 · 示例/文档 | 主例有 Toggle acceptance/Reorder/proposals 等验收按钮；改为真实 JSON 树，分段说明展开、路径、复制和数据更新。 |
| 174 | [Notifications system](https://mantine.dev/x/notifications/) | P2 · 文档 | 只有 Show notification 不足以讲清系统；补位置、堆叠、更新/关闭、loading/超时和应用局部状态，说明与单个 Notification 的关系。 |
| 175 | [Spotlight](https://mantine.dev/x/spotlight/) | P2 · 示例/文档 | 主例混有 Before/After、异步/大量命令夹具；换真实命令面板，分开搜索、分组、空结果、快捷键及受控打开。 |
| 176 | [Carousel](https://mantine.dev/x/carousel/) | P1 · 示例 | 主例被百万页虚拟化、编辑器草稿、拒绝/生命周期计数占据；换媒体/卡片轮播，虚拟化/RTL/自动播放/受控拒绝另作有解释的细节场景。 |
| 177 | [Lightbox](https://mantine.dev/x/lightbox/) | P2 · 示例/文档 | 先显示小型缩略图画廊和打开后的图片导航，再介绍缩放/元数据/受控关闭；移走接受提案/Owner 等测试控件。 |
| 178 | [Dropzone](https://mantine.dev/x/dropzone/) | P1 · 示例 | 主例混文件选择器、Unstyle/Disabled、并发读取、清空等验收工具；建立清晰拖放区/文件反馈，限制、拒绝、读取和取消另分段保留。 |
| 179 | [NavigationProgress](https://mantine.dev/x/nprogress/) | P2 · 示例/文档 | 主例是 start/report/replace/publish 工作台；给真实导航进度，再分开说明任务替换、取消、完成和并发策略。 |
| 180 | [Modals manager](https://mantine.dev/x/modals/) | P2 · 示例/文档 | 主例以队列/更新/全部关闭按钮及 Pending 计数为主；给确认/表单场景，再分段说明队列、更新、关闭和焦点恢复。 |
| 181 | [Rich text editor](https://mantine.dev/x/tiptap/) | P1 · 布局/边界 | 390px 下文字工具按钮高约99px，标签逐字竖排；改紧凑动作与分组换行策略。已有多种行内示例，应拆分；Rust 有限文档模型与任意 Tiptap 插件兼容是不同范围。 |

## 建议执行顺序

1. 先修共享尺寸/收缩规则：PinInput、Pagination、Scroller、RichEditor 及四个窄屏内部溢出；为这些真实故障保留有意义的浏览器几何断言。
2. 纠正目录与首例：Anchor/Burger/NavLink/Transition；清理 FloatingWindow、Carousel、Dropzone 等公开主例中的验收夹具，保留独立测试入口。
3. 完善选择器输入外壳和 Accordion/Tabs/Timeline/Breadcrumbs 默认皮肤，以一组共享视觉规则修复多个消费者。
4. 分别实现并验收 MiniCalendar 日期条、日期型 Heatmap、通用 Transition，以及 Menu/Menubar 内容组合；先写清有限契约与宿主边界。
5. 整理复杂页面的分段说明/源码，再优化图表、调度和媒体示例；对本轮标为补测的交互逐项补证据，不把文档渲染正常当作功能完成。

## 复核与局限

- 这是完整页面范围的审查，不是全部 API 组合、全部弹层、全部键盘路径的等价性证明。15个针对性复现仅覆盖记录中的操作；本轮未运行全库编译/单测，因为新增内容为审计脚本与报告。
- 官方页面在不同包/缓存中可能显示不同版本，本报告以2026-11-04实际抓取内容和每页 SHA256 为依据，不统一声称某一版本。历史 API 冻结文件保持不变。
- 联系表裁切方便比对，完整元素截图仍保留。超高元素截图中可能出现浏览器固定站点页头覆盖，这是截图捕获方式，未作为组件重复页头缺陷。
- 默认预览无法证明尚未展开的搜索/过滤/拖动/创建等能力缺失；只有已复现现象或当前源码类型明确不提供的能力列为确定差异，其余写为展示/说明/补测。
- 浏览器发现不能直接推广为原生平台失败/通过；任何原生改动仍须遵守当前宿主的显式支持和真实输入验收边界。

## 复现

原始 HTML、完整截图、联系表和量测 JSON 均保存在 `D:/BuildCache/sapid/shared-page-audit`，不进入运行时依赖或发布资源。

```powershell
$env:SAPID_BUILD_ROOT = 'D:/BuildCache/sapid'
$env:SAPID_BROWSER_OUTPUT = 'D:/BuildCache/sapid/shared-page-audit'
$env:SAPID_PLAYWRIGHT = 'D:/BuildCache/sapid/node-tools/node_modules/playwright'
$env:PLAYWRIGHT_BROWSERS_PATH = 'D:/BuildCache/sapid/browsers'
$env:TEMP = 'D:/BuildCache/sapid/tmp'
$env:TMP = $env:TEMP
$env:PYTHONPYCACHEPREFIX = 'D:/BuildCache/sapid/pycache'
# 先启动本轮发布资源的 8795 预览服务器，再执行：
python scripts/audit-shared-pages.py
node scripts/audit-shared-pages.cjs
node scripts/audit-shared-pages.cjs --headings-only
node scripts/audit-upstream-previews.cjs
node scripts/audit-shared-page-probes.cjs
python scripts/audit-page-contact-sheets.py
# 人工逐页审查并更新 JSON 后，重新渲染报告：
python scripts/render-shared-page-review.py
python scripts/render-shared-page-review.py --check
```

抓取器会复用本轮缓存目录中已有 HTML；需要复核未来版本时应通过 SAPID_BROWSER_OUTPUT 使用新的日期目录并新建报告，不把缓存命中当作新抓取。截图脚本只采集证据，不会自动给出功能等价结论。
