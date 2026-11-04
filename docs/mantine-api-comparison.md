# Mantine 全组件 API 与功能差异核实

核实日期：2026-10-25；官方文档版本：9.7.4。
逐项覆盖 **181 个 Mantine 条目**：121 Core、15 Dates、22 Charts、13 Schedule、8 官方扩展、2 Code。
官方来源：[Core 目录](https://mantine.dev/core/package/)、[官方扩展](https://mantine.dev/x/extensions/)、
[完整文档（包含功能示例和 Props 表）](https://mantine.dev/llms-full.txt)。

## 结论与核实边界

旧 inventory 的 done 表示有限功能批次已交付，**不表示 Mantine API 等价**。本报告逐项比对官方文档、
本地公开 Rust 入口、实现和能力矩阵；没有把文件存在、同名方法、近似拼写或 SSR HTML 当成功能验收。
Props 表包括同页子部件的公开属性；继承的原生 HTML 属性和所有 React 类型重载不一定在表内。
功能表是经实现核实的差异与设计决策，附录是完整文档表面的可追溯索引；未映射属性是核实候选，不能直接解释为缺失。
快照记录 4,659 个按文档条目计数的 Props 字段与其类型；不同条目的同名字段重复计数，不是同等数量的缺失功能。
历史宿主证据沿用 capability-matrix，新增功能的当前运行证据单列，未逐个重验所有上游功能组合。

Rust 的 Widget、Cage/Bond/Vary、Owner 和有限宿主协议保持不变；不移植 React hooks、任意 DOM/ARIA 注入、
polymorphic component、任意 Recharts/Tiptap/Embla 插件或全局 provider。通用尺寸、颜色、圆角等可通过 typed Style/Theme 组合，
不因缺少同名便捷 prop 就重复建立状态。原生支持、浏览器协议和实际 OS/读屏验收分别记账。

部分 theme/size/variant、声明预算与组合策略属于挂载期配置，并非所有 Mantine prop 都可直接传 Vary。
响应式值/禁用/只读等按各 builder 明确暴露的绑定工作；Style 组合不等于已提供上游同名动态 API，也不以重建节点冒充原位更新。

## 按报告执行的批次

| 批次 | 优先级 | 要补齐的行为 | 归属与状态 |
| --- | --- | --- | --- |
| A1 | P0 | MultiSelect max_values / hide_picked_options / clearable，含受控拒绝、IME、空组与游标恢复 | 已实现并验收：29 项选择控件测试、浏览器四主题 × 两尺寸 × CSR/SSR；受控拒绝/IME/空组/游标/焦点均有证据 |
| A2 | P0 | 真实 input/textarea placeholder：Glory 共享语义/HTML/SSR/stream + Sapid 输入族 | 已实现并验收：render/HTML 差分/SSR stream、9 类 Sapid 输入；Apace 明确拒绝并保留成功前缀，未声称原生绘制支持 |
| A3 | P1 | Button Light / Outline / Subtle 外观，保留 loading/disabled/自定义 Style | 已实现并验收：新增三种外观；四主题 loading/disabled/显式样式与真实浏览器恢复均通过 |
| A4 | P1 | dropdown 配置、option slot、lineClamp、slider marks、分段 progress、日期 presets、日程统一编辑 | 已实现并验收：Select/TreeSelect 有界 dropdown 与 option content、Typography lineClamp、Slider/RangeSlider marks、Line/Ring Progress sections、DatePicker/Input typed presets，以及 ResourcesSchedule 统一 Day/Week/Month 的权限校验 move/resize 提议；任意 React renderer、浏览器全局拖放和应用冲突/持久化仍保持显式边界 |
| A5 | P1 | Mantine 9.7.4 展示结构、真实 Rust/Mantine API 参考、Button 多场景与 compact | 展示站提供 Documentation/Properties/Styles API，181 个官方指南按 Mantine 分类与顺序独立展示；Button 支持五档尺寸、compact、颜色、圆角及独立场景；其他组件保持已验证的有限契约 |

## 逐组件核对

| Mantine / 官方文档 | Sapid 现有对应 | 经核实的差异与借鉴决策 | 优先级 |
| --- | --- | --- | --- |
| [AppShell](https://mantine.dev/core/app-shell/) | AppShell：header/sidebar/footer、breakpoint、drawer/rail | 不是所有 Mantine aside/section/offset/hiddenFrom 组合；需补区域配置与独立滚动证据，不能 CSS 编译通过即视为所有布局成立。 | P2 |
| [AspectRatio](https://mantine.dev/core/aspect-ratio/) | AspectRatio：反应比例、独立媒体框、居中绝对内容和尺寸约束 | polymorphic 根元素不移植；媒体裁切/替换内容由 Image fit 负责，不把 ratio 当全套图像组件。 | 边界 |
| [Center](https://mantine.dev/core/center/) | Center/Container/Space：居中、fluid/max_width、响应式阈值 | Mantine 命名容器尺寸和 strategy 不是逐项兼容；已能 typed Style 组合，优先补文档示例而非同义参数。 | P2 |
| [Container](https://mantine.dev/core/container/) | Center/Container/Space：居中、fluid/max_width、响应式阈值 | Mantine 命名容器尺寸和 strategy 不是逐项兼容；已能 typed Style 组合，优先补文档示例而非同义参数。 | P2 |
| [Flex](https://mantine.dev/core/flex/) | Box/Flex/Stack/Group：布局、gap、对齐、换行、宽高 | Group grow/preventGrowOverflow 的便捷策略尚缺；响应式值通过 ResponsiveRoot 显式组合。Box 的任意 component/ref API 不移植。 | P2 |
| [Grid](https://mantine.dev/core/grid/) | Grid/SimpleGrid：有限轨道、span/offset、响应式测量 | 与 Mantine 的 12 列/自动填充/容器查询模型不同；auto-fit/minColWidth 等策略需新增有限布局算法，不能仅重命名 columns。 | P2 |
| [Group](https://mantine.dev/core/group/) | Box/Flex/Stack/Group：布局、gap、对齐、换行、宽高 | Group grow/preventGrowOverflow 的便捷策略尚缺；响应式值通过 ResponsiveRoot 显式组合。Box 的任意 component/ref API 不移植。 | P2 |
| [SimpleGrid](https://mantine.dev/core/simple-grid/) | Grid/SimpleGrid：有限轨道、span/offset、响应式测量 | 与 Mantine 的 12 列/自动填充/容器查询模型不同；auto-fit/minColWidth 等策略需新增有限布局算法，不能仅重命名 columns。 | P2 |
| [Space](https://mantine.dev/core/space/) | Center/Container/Space：居中、fluid/max_width、响应式阈值 | Mantine 命名容器尺寸和 strategy 不是逐项兼容；已能 typed Style 组合，优先补文档示例而非同义参数。 | P2 |
| [Splitter](https://mantine.dev/core/splitter/) | Splitter：保留两面板、反应横纵方向、范围/step、commit、引用和受控折叠/恢复 | 不支持任意多 pane 树、collapsed/expand API、复杂 snap；需有界 panel model 与共享 drag/keyboard 协议后再增加。 | P2 |
| [Stack](https://mantine.dev/core/stack/) | Box/Flex/Stack/Group：布局、gap、对齐、换行、宽高 | Group grow/preventGrowOverflow 的便捷策略尚缺；响应式值通过 ResponsiveRoot 显式组合。Box 的任意 component/ref API 不移植。 | P2 |
| [AlphaSlider](https://mantine.dev/core/alpha-slider/) | HueSlider/AlphaSlider：有限颜色范围、键盘/commit | 缺 onScrubStart/End 便捷回调和任意 root props；可在通用 drag 会话语义上增加，不能以值改变猜测手势边界。 | P2 |
| [AngleSlider](https://mantine.dev/core/angle-slider/) | AngleSlider：圆周映射、键盘、有限值 | 缺 marks/自定义 label/自动 snap；保持角度域和端点策略显式，复用共享 Slider 的能力而非复制输入驱动。 | P2 |
| [Checkbox](https://mantine.dev/core/checkbox/) | Checkbox：原生 Check、受控值、Field；MixedCheckbox 独立 | indeterminate 已由 MixedCheckbox 表达；自定义 icon/labelPosition/多色变体未形成直接 API，先完善视觉插槽，避免改变原生 Check。 | P2 |
| [Chip](https://mantine.dev/core/chip/) | Chip/ChipGroup：单/多选、受控值、真实输入、Field | 缺 checked icon、icon position 和完整 variant；整组的受控默认值/禁用已有，不应重复引入另一套状态。 | P2 |
| [ColorInput](https://mantine.dev/core/color-input/) | ColorInput 已覆盖 hex(a)/rgb(a)/hsl(a)、受控草稿/commit、preserve invalid、禁用自由输入、连续色面与完整 swatches、点击色块关闭、隐藏弹层、loading/左右 section、error/success/disabled/read-only；EyeDropper 由应用显式注入异步服务并支持自定义图标。ColorPicker 已覆盖连续色面、swatches、withPicker=false、fullWidth、可配置通道标签和实时格式化表单代理。 | 独立 ColorPicker 继续以无损 Hsva 作为业务值，只在真实隐藏 successful control 中按所选格式发布表单快照；EyeDropper 不调用隐式浏览器全局 API，实际浏览器或 OS 取色授权由应用提供的服务负责。 | P2 |
| [ColorPicker](https://mantine.dev/core/color-picker/) | ColorInput 已覆盖 hex(a)/rgb(a)/hsl(a)、受控草稿/commit、preserve invalid、禁用自由输入、连续色面与完整 swatches、点击色块关闭、隐藏弹层、loading/左右 section、error/success/disabled/read-only；EyeDropper 由应用显式注入异步服务并支持自定义图标。ColorPicker 已覆盖连续色面、swatches、withPicker=false、fullWidth、可配置通道标签和实时格式化表单代理。 | 独立 ColorPicker 继续以无损 Hsva 作为业务值，只在真实隐藏 successful control 中按所选格式发布表单快照；EyeDropper 不调用隐式浏览器全局 API，实际浏览器或 OS 取色授权由应用提供的服务负责。 | P2 |
| [Fieldset](https://mantine.dev/core/fieldset/) | Fieldset/Legend：原生分组、禁用传播、主题尺寸 | variant/legendPosition 等展示配置可用 Style，但需补充带 legend 的视觉示例；不要把禁用透明度当实际禁用。 | P2 |
| [FileInput](https://mantine.dev/core/file-input/) | FileInput/FileButton：共享 File 协议、Picker、控制器/结果 | 缺目录、多种 value renderer 等完整 Mantine props 组合需依共享配置判断；native picker facet 仍拒绝，不能把 Web 文件测试当本地OS支持。 | P2 |
| [HueSlider](https://mantine.dev/core/hue-slider/) | HueSlider/AlphaSlider：有限颜色范围、键盘/commit | 缺 onScrubStart/End 便捷回调和任意 root props；可在通用 drag 会话语义上增加，不能以值改变猜测手势边界。 | P2 |
| [Input](https://mantine.dev/core/input/) | Field/Label/Description/Error：真实关联、required/invalid/success、四部分顺序；TextField共享loading与左右sections/有限宽度 | 已补完整FieldSlot排列校验、Field/控件success合并且error优先，以及TextField/PasswordInput/Textarea共用的响应式loading、左右内容和20..=200px section宽度；showcase提供实际切换。任意inputContainer/labelElement及子部件props继续用显式Widget/Style组合，不移植React元素注入。 | P2 |
| [JsonInput](https://mantine.dev/core/json-input/) | JsonInput：有界解析/format、错误、草稿/只读、真实 placeholder | 本轮补 placeholder；autosize/formatOnBlur 等便捷行为仍需定义组合和失焦次序。 | P2 |
| [MaskInput](https://mantine.dev/core/mask-input/) | MaskInput：有限 mask、raw/display 索引、paste/组合保护 | 不兼容任意第三方 mask 对象、动态 mask 引擎；保留 typed finite schema，并补 display 格式示例。 | 边界 |
| [NativeSelect](https://mantine.dev/core/native-select/) | NativeSelect：真实 select、分组、多选、成功值 | NativeSelect 的左右 sections 需 InputGroup 组合；Apace select facet 拒绝。不能将浏览器 select 打开状态作为可控 popup。 | P2 |
| [NumberInput](https://mantine.dev/core/number-input/) | NumberInput：Decimal、locale、clamp、step、草稿与拒绝、紧凑上下步进器、真实 placeholder | 已补共享有界响应式 placeholder，并改进上下步进器的尺寸、分隔线、图标和禁用态；缺完整千分位/前后缀/字符串数值/hold interval API，不能用 f64 替换当前精确小数，也不导入任意 formatter。 | P2 |
| [PasswordInput](https://mantine.dev/core/password-input/) | PasswordInput：显隐/组合保护、Field、真实 placeholder | 本轮补 placeholder；显隐自定义图标/部位仍待补。不能重建输入实现；native 密码值投影与 hint 支持是不同承诺。 | P2 |
| [PinInput](https://mantine.dev/core/pin-input/) | PinInput：长度/字母表、mask、autofill、complete、原子编辑 | 缺一组可单独引用的 input 单元等 UI 细项；现有是共享单 Input 投影，不假装独立多输入。onComplete 已有不能误报。 | P2 |
| [Radio](https://mantine.dev/core/radio/) | RadioGroup：键盘、方向、单选值、required/表单代理 | 缺 Radio.Card/Indicator 展示子部件、图标与标签位置；可以现有组和 slot 组合，需单独验收点击区域与焦点。 | P2 |
| [RangeSlider](https://mantine.dev/core/range-slider/) | RangeSlider：原子双 thumb、min/max gap、push/hold/clamp、marks-only、value labels、domain 与具名引用 | Mantine 的 minRange 对应 minimum_gap，maxRange 对应 maximum_gap，主要交互示例已覆盖；仍缺任意 thumb renderer，固定 Lower/Upper 身份不会因视觉交叉互换。 | P2 |
| [Rating](https://mantine.dev/core/rating/) | Rating：精度、display_only、clearable、键盘/Field | 缺任意 empty/full symbol renderer、hover preview 回调；图标表现与真实 radio 选择分离，避免 hover 修改受控值。 | P2 |
| [SegmentedControl](https://mantine.dev/core/segmented-control/) | SegmentedControl：真实 radios、方向、受控值 | 缺独立指示器动画、fullWidth/equal-width 的成套配置；不得以动画代替选中语义，布局可用 Style。 | P2 |
| [Slider](https://mantine.dev/core/slider/) | Slider：有限 range、方向/步长、commit、marks、marks-only、浮动/常显 value label 与值文本 | Mantine 主要数值、marks、label 示例已覆盖；仍缺任意 thumb/label renderer，保持有限语义节点而不注入自由 DOM tooltip。 | P2 |
| [Switch](https://mantine.dev/core/switch/) | Switch：真实 CheckMode::Switch、受控值、Field | 缺 on/offLabel、thumbIcon、labelPosition、颜色/尺寸的完整视觉组合；保留真实 checkbox 键盘行为。 | P2 |
| [Textarea](https://mantine.dev/core/textarea/) | Textarea：固定行数、min/max rows、浏览器 autosize、真实 placeholder | 本轮补多行 placeholder；autosize 与 placeholder 在 native 各有拒绝边界，固定无 hint 行数仍支持。resize 不是 intrinsic sizing。 | P2 |
| [TextInput](https://mantine.dev/core/text-input/) | TextField：value/commit、Purpose、Field、readonly/invalid、真实 placeholder | 本轮补有界响应式 placeholder，独立于名称/值/版本；native 对该可选 hint 明确拒绝。任意原生属性转发不移植。 | P2 |
| [Autocomplete](https://mantine.dev/core/autocomplete/) | Autocomplete：自由值/建议 key 分离、commit、placeholder、limit、首项高亮、autoSelectOnBlur、clearable、loading、分组与显式 filter | 主要 Mantine 行为示例已覆盖；仍缺任意 renderOption 节点和 JS 风格 filter 回调。保留 NativeOption 的有界文本/分组模型，不引入宿主任意节点。 | P2 |
| [Cascader](https://mantine.dev/core/cascader/) | Cascader：完整路径、分列、懒加载、clearable | 缺 changeOnSelect/showSearch/multiple 等组合；并非所有 Select props 可直接转发，需保持路径/叶选择策略与原子受控提议。 | P2 |
| [Combobox](https://mantine.dev/core/combobox/) | Combobox：Input/listbox、Contains/Prefix/None 过滤、结果 limit、响应式 loading、分组/禁用项、键盘/IME、真实 placeholder | 已补并公开有限 filter、最多1024项声明内的可见结果 limit、左右 loading 与真实 placeholder；showcase 覆盖受控搜索、prefix+limit+loading、分组和禁用项。任意回调 renderer/filter 与万项虚拟化不以宿主外 DOM 回调伪装。 | P2 |
| [ComboboxPopover](https://mantine.dev/core/combobox-popover/) | Popover：锚点定位、碰撞处理、焦点/关闭策略、Target/Auto/Px 宽度、offset 与碰撞方向箭头 | 已由 Glory 宿主测量真实 target 宽度并暴露碰撞后的箭头方向；Sapid 用一个 typed builder 和 trigger/content/close factories 代替 React 组合子部件与任意 middleware。overlay、过渡和任意 DOM ref 仍保持有限宿主边界。 | P2 |
| [MultiSelect](https://mantine.dev/core/multi-select/) | MultiSelect：独立搜索/受控值、max_values、隐藏已选、clearable、placeholder、limit、loading 与分组 | 主要选择/搜索/状态示例已覆盖；仍缺任意 renderOption/renderPill 与拖拽重排。有限 NativeOption/Pill 语义不接受任意宿主节点。 | P2 |
| [Pill](https://mantine.dev/core/pill/) | Badge/Pill：三种 badge variant、受控移除、紧凑标签 | 上一轮修复尺寸与 close action；Badge 的 dot/左右 section/gradient、Pill 的拖拽排序仍未完整提供。先补展示 slot，再考虑有界排序。 | P2 |
| [PillsInput](https://mantine.dev/core/pills-input/) | TagsInput/PillsInput：原子 state、重复/分隔/粘贴/移除、max_tags、placeholder、clearable、loading 与 acceptValueOnBlur | 主要输入、清空、失焦提交和解析示例已覆盖；仍缺组合建议列表与拖拽重排。max_tags 已有，不能误报 maxTags 缺失。 | P2 |
| [Select](https://mantine.dev/core/select/) | Select：受控/default/clearable/loading、分组、有限 option content、勾选位置、dropdown 高度/锚点与 Field；searchable 由显式 Combobox 组合提供 | 已对齐基础选择、受控清空、loading、分组/disabled、有限 option content、check 位置、滚动高度与共享锚点，并以五段 showcase 和真实浏览器点击验收。search/filter/limit/empty 仍显式组合 Combobox，不制造同名别名；任意宿主节点 renderOption 与无限虚拟结果不进入有限声明契约。 | P2 |
| [TagsInput](https://mantine.dev/core/tags-input/) | TagsInput/PillsInput：原子 state、重复/分隔/粘贴/移除、max_tags、placeholder、clearable、loading 与 acceptValueOnBlur | 主要输入、清空、失焦提交和解析示例已覆盖；仍缺组合建议列表与拖拽重排。max_tags 已有，不能误报 maxTags 缺失。 | P2 |
| [TreeSelect](https://mantine.dev/core/tree-select/) | TreeSelect：完整路径身份、single/multiple/级联或独立勾选、受控值/展开/搜索、hide_picked、max_values、typed node/value renderer 与有限下拉配置 | 已补真实 editable combobox 搜索、保留祖先过滤/空结果、隐藏已选、选择上限、连接线、节点和值渲染、下拉宽高/锚点及8组展示。checkedStrategy 由 TreeChoicePolicy 和完整 TreeChoices 显式表达；render_value 是有限 typed widget 工厂，不接受任意 DOM，也不隐式复制业务树。 | P2 |
| [ActionIcon](https://mantine.dev/core/action-icon/) | ActionIcon/CloseButton：五种 variant、尺寸、自定义 icon、禁用 | 上一轮已补主要外观与 close icon；loading/custom-loader 仍缺。任意 component、gradient 和全局 variant registry 不移植。 | P2 |
| [Button](https://mantine.dev/core/button/) | Button：六种外观、五档尺寸与 compact、受控禁用/loading、图文插槽和按钮组 | 已补 Light/Outline/Subtle 与 compact；展示页覆盖 fullWidth/justify、颜色、圆角和独立场景。gradient/任意 variant resolver 不移植，loading 自定义显示仍可后续完善。 | P2 |
| [CloseButton](https://mantine.dev/core/close-button/) | ActionIcon/CloseButton：五种 variant、尺寸、自定义 icon、禁用 | 上一轮已补主要外观与 close icon；loading/custom-loader 仍缺。任意 component、gradient 和全局 variant registry 不移植。 | P2 |
| [CopyButton](https://mantine.dev/core/copy-button/) | CopyButton：真实 Clipboard action、结果/feedback/labels | 不把未授权环境/SSR当复制成功；任意 render-prop 可由 Widget 组合替代，多个复制控件的状态仍分开。 | 边界 |
| [FileButton](https://mantine.dev/core/file-button/) | FileInput/FileButton：共享 File 协议、Picker、控制器/结果 | 缺目录、多种 value renderer 等完整 Mantine props 组合需依共享配置判断；native picker facet 仍拒绝，不能把 Web 文件测试当本地OS支持。 | P2 |
| [Toggle](https://mantine.dev/core/toggle/) | Toggle/ToggleGroup：单/多选、pressed、方向、键盘 | 缺完整外观/图标位置配置。Mantine Toggle 与 checkbox 语义不等同；使用真正 pressed Button，不用 Check 伪装。 | P2 |
| [UnstyledButton](https://mantine.dev/core/unstyled-button/) | Button：六种外观、五档尺寸与 compact、受控禁用/loading、图文插槽和按钮组 | 已补 Light/Outline/Subtle 与 compact；展示页覆盖 fullWidth/justify、颜色、圆角和独立场景。gradient/任意 variant resolver 不移植，loading 自定义显示仍可后续完善。 | P2 |
| [Anchor](https://mantine.dev/core/anchor/) | Anchor：真实 href、current、宿主导航和目标上下文 | 缺 underline 策略便捷 API；Style/CSS 可组合。new context 原生明确拒绝，非路由/下载协议不隐式加入。 | P2 |
| [Breadcrumbs](https://mantine.dev/core/breadcrumbs/) | Breadcrumbs：真实 links/current、有限条目 | 缺自定义 separator/图标 slot 等便捷 API；自动折叠应复用 OverflowList，不把静态省略符当可导航隐藏内容。 | P2 |
| [Burger](https://mantine.dev/core/burger/) | Burger：展开、名称、controls、有限动画和时长 | 缺自定义线粗/颜色便捷参数；typed Style 已可覆盖颜色。真实展开与动画状态已独立，不需要复制 DOM click 驱动。 | P2 |
| [NavLink](https://mantine.dev/core/nav-link/) | NavLink：href/current、展开 children；NavigationMenu 独立 | 缺 right/left section、autoContrast、description 的统一便捷参数；禁止通过 label 或 role 变更重置节点输入状态。 | P2 |
| [Pagination](https://mantine.dev/core/pagination/) | Pagination：边界/兄弟页、受控值、未知下一页 | 缺完整 control 图标/edge slots 与响应式策略；现有数据窗口不等价远程数据缓存，分页请求应由应用管理。 | P2 |
| [Stepper](https://mantine.dev/core/stepper/) | Stepper：active/complete、可跳步、可选步骤、内容策略 | 缺 loading/自定义 completed/progress icon 和 iconPosition；控件选择与业务校验分离，不自动把 step click 当验证成功。 | P2 |
| [TableOfContents](https://mantine.dev/core/table-of-contents/) | TableOfContents：显式 heading 注册、current/reveal/offset | 不扫描任意 DOM selector；auto headings/文字自动取样与样式 slots 有差异。显式上下文更适合多宿主与虚拟列表。 | 边界 |
| [Tabs](https://mantine.dev/core/tabs/) | Tabs：受控/默认值、方向、自动/手动激活、动态页和保留策略 | 缺 inverted/variant/颜色/独立 tab 图标部位等便捷配置；allowTabDeactivation/reorder 需先定义空选中与键盘行为。 | P2 |
| [Tree](https://mantine.dev/core/tree/) | Tree：有界节点、懒加载、check/多选、键盘、render_label、批量展开、过滤、连接线和真实拖放 | 已补受控/批量展开、级联与独立勾选、异步加载、保留祖先过滤、Before/Inside/After 拖放及 allow_drop、连接线和文件树展示。render_label 使用有限 typed content 而非任意 DOM；虚拟化仍由应用以有限数据源组合，绝不扫描底层大列表来导出 AX 树。 | P2 |
| [Alert](https://mantine.dev/core/alert/) | Alert：severity、announcement、可关闭与内容 slot | 缺多种 variant/icon/close 部位便捷 API；announcement 与静态 alert 需区分，不让每条说明自动打断读屏。 | P2 |
| [EmptyState](https://mantine.dev/core/empty-state/) | EmptyState：title/description/illustration/actions | 缺尺寸预设/多种 media 布局；插槽足以组合，优先改示例与默认留白，避免引入业务空状态。 | P2 |
| [Loader](https://mantine.dev/core/loader/) | Loader：有限 Graphic、尺寸/线宽、可停动画 | Mantine 多种 loader 类型/自定义 registry 未提供；自定义 Graphic 可组合，仍需尊重 reduced motion 和静态 fallback。 | P2 |
| [Notification](https://mantine.dev/core/notification/) | Notification/系统：有界队列、更新/关闭、计时与 hover 暂停 | 缺分区域 placement、任意 transition 和完整 loader/icon slots；队列有界与 app-local context 优于无界全局管理器。 | P2 |
| [Progress](https://mantine.dev/core/progress/) | Progress/Ring/SemiCircle：真实 range/progress 语义、纵向线条、最多16个有序彩色 sections、ring gap/start/caps 与有限文本 label | Mantine 的 compound line/ring 和主要几何示例已覆盖；仍缺任意 Widget label slot 与动态 section 声明，静态 sections 不冒充响应式业务列表。 | P2 |
| [RingProgress](https://mantine.dev/core/ring-progress/) | Progress/Ring/SemiCircle：真实 range/progress 语义、纵向线条、最多16个有序彩色 sections、ring gap/start/caps 与有限文本 label | Mantine 的 compound line/ring 和主要几何示例已覆盖；仍缺任意 Widget label slot 与动态 section 声明，静态 sections 不冒充响应式业务列表。 | P2 |
| [SemiCircleProgress](https://mantine.dev/core/semi-circle-progress/) | Progress/Ring/SemiCircle：真实 range/progress 语义、纵向线条、最多16个有序彩色 sections、ring gap/start/caps 与有限文本 label | Mantine 的 compound line/ring 和主要几何示例已覆盖；仍缺任意 Widget label slot 与动态 section 声明，静态 sections 不冒充响应式业务列表。 | P2 |
| [Skeleton](https://mantine.dev/core/skeleton/) | Skeleton：loading、圆/矩形、有限宽高 | 缺默认 shimmer/animate 配置与内容保留的完整组合；应优先复用 MotionPolicy，而非插入独立无限计时器。 | P2 |
| [ActionBar](https://mantine.dev/core/action-bar/) | Toolbar/ActionBar：有界组、roving、selected_count/clear | Mantine ActionBar placement/transition/portal 配置不同；不得把横排 Toolbar 当完整底部浮条。可用 shared Layer 进一步补齐。 | P2 |
| [Affix](https://mantine.dev/core/affix/) | Affix：显式 viewport/container、offset、sticky | Native typed Sticky 仍拒绝；任意 zIndex/portalTarget 不自动移植。遵守已测试的视口定位与原生不同能力。 | 边界 |
| [Dialog](https://mantine.dev/core/dialog/) | FloatingDialog 对应 Mantine Dialog：非模态提示 | 非 Modal：不隐式焦点圈闭/外点关闭。placement/close/transition 细项仍不同，使用共享 Layer 不另造驱动。 | P2 |
| [Drawer](https://mantine.dev/core/drawer/) | Drawer：侧边 Layer、内容策略；Sheet 提供手势和 snap | 缺位置相关的自动尺寸、完整 close/overlay 插槽和过渡配置；snap 已有有限语义，不能宣称任意 CSS 动画。 | P2 |
| [FloatingIndicator](https://mantine.dev/core/floating-indicator/) | FloatingIndicator：活 target/geometry、有限运动 | 缺自由 transform transition 与多个 targets 的任意 ref API；使用非 owning 节点引用，测量之后仍不宣称已显示帧。 | P2 |
| [FloatingWindow](https://mantine.dev/core/floating-window/) | FloatingWindow：真实 bounds/raise、拖拽约束、键盘 | 缺 minimize/maximize/persist storage 和完整标题栏 slot；尺寸与 viewport 约束已存在，不能只用 absolute div 代表窗口协议。 | P2 |
| [HoverCard](https://mantine.dev/core/hover-card/) | HoverCard：共享 Hint/Layer 与延迟、保留策略 | 缺 HoverCard.Group 协调延迟和更细 focus 交互配置；复杂内容仍须共享 Layer 栈处理 Escape/外点。 | P2 |
| [LoadingOverlay](https://mantine.dev/core/loading-overlay/) | Overlay/LoadingOverlay：显式遮罩、透明度、内容状态 | 缺 blur、loaderProps/自定义 loader slot 和 zIndex 便利项；遮罩与真实 inert/焦点阻断必须分开验收。 | P2 |
| [Menu](https://mantine.dev/core/menu/) | Menu：真实菜单、子菜单、radio/check、roving/typeahead | 缺所有 Mantine trigger/transition/arrow 配置；现有 HoverDelay/placement 不等于完整 compound API。native 子菜单等仍按矩阵拒绝。 | P2 |
| [Menubar](https://mantine.dev/core/menubar/) | Menubar：多菜单、方向、键盘与菜单状态 | 桌面 native Menubar facet 仍拒绝；Web 的触发/过渡细项与 Mantine 不完全一致，不写为跨平台已完成。 | 边界 |
| [Modal](https://mantine.dev/core/modal/) | Dialog 对应 Mantine Modal：共享居中 Layer、viewport 滚动、焦点返回、栈、关闭策略与 fullScreen | centered 是默认宿主布局，fullScreen 有明确 builder；仍缺过渡时序回调和独立 body scroll-area 配置。不要把 Mantine Dialog 当模态框。 | P2 |
| [Overlay](https://mantine.dev/core/overlay/) | Overlay/LoadingOverlay：显式遮罩、透明度、内容状态 | 缺 blur、loaderProps/自定义 loader slot 和 zIndex 便利项；遮罩与真实 inert/焦点阻断必须分开验收。 | P2 |
| [Popover](https://mantine.dev/core/popover/) | Popover：锚点定位、碰撞处理、焦点/关闭策略、Target/Auto/Px 宽度、offset 与碰撞方向箭头 | 已由 Glory 宿主测量真实 target 宽度并暴露碰撞后的箭头方向；Sapid 用一个 typed builder 和 trigger/content/close factories 代替 React 组合子部件与任意 middleware。overlay、过渡和任意 DOM ref 仍保持有限宿主边界。 | P2 |
| [Tooltip](https://mantine.dev/core/tooltip/) | Tooltip：hover/focus/touch、有界延迟、placement | 缺 Group 延迟共享/完整 arrow/多行布局选项；不能用 title 属性当同一功能，native 浮层仍遵守其拒绝边界。 | P2 |
| [Tour](https://mantine.dev/core/tour/) | Tour：显式目标、步骤、导航/完成/取消、失效恢复 | 缺自由 renderStep/spotlight 视觉配置、完整 overlay 点击策略；目标必须是活的语义节点，不扫描虚拟列表源数据。 | P2 |
| [Accordion](https://mantine.dev/core/accordion/) | Accordion：多开/不可全关、方向、键盘、heading_level、动态项 | 缺完整 chevron/旋转/图标位置和外观变体配置；可先用 trigger_content 组合，动画时长应复用 Collapse 协议。 | P2 |
| [Avatar](https://mantine.dev/core/avatar/) | Avatar/AvatarGroup：fallback、max_visible、重叠、名称 | 缺自动姓名取色/initials 算法和完整 group overflow slot；不是所有异步图片配置都等价，需沿用 request revision 与资源预算。 | P2 |
| [BackgroundImage](https://mantine.dev/core/background-image/) | Image/BackgroundImage：fit、fallback、load/error、资源身份、有限 srcset/sizes/loading | 已补 width-descriptor 候选源、viewport/pixel sizes 与 auto/eager/lazy；Web/SSR 使用真实 img 属性，native 在资源工作前显式拒绝 responsive/lazy。Next.js polymorphic root 不移植。 | P2 |
| [Badge](https://mantine.dev/core/badge/) | Badge/Pill：三种 badge variant、受控移除、紧凑标签 | 上一轮修复尺寸与 close action；Badge 的 dot/左右 section/gradient、Pill 的拖拽排序仍未完整提供。先补展示 slot，再考虑有界排序。 | P2 |
| [Card](https://mantine.dev/core/card/) | Card/Paper：header/body/footer、border、有限阴影 | Card.Section 的负边距和继承 padding 规则不是任意容器已有；优先明确 section 边界。颜色/圆角已可 Style 覆盖。 | P2 |
| [ColorSwatch](https://mantine.dev/core/color-swatch/) | ColorSwatch：静态颜色或真实交互、checked、尺寸 | 缺自动 contrast/check 图标/叠加 slot；色彩透明棋盘与可访问名称已是不同责任，不把纯色 div 当交互。 | P2 |
| [DataList](https://mantine.dev/core/data-list/) | DataList：term/description 与 Item 媒体/动作 | Mantine 的横/纵布局和列对齐配置仍有差异；现有语义与布局可组合，需补长标签/窄屏验收。 | P2 |
| [Image](https://mantine.dev/core/image/) | Image/BackgroundImage：fit、fallback、load/error、资源身份、有限 srcset/sizes/loading | 已补 width-descriptor 候选源、viewport/pixel sizes 与 auto/eager/lazy；Web/SSR 使用真实 img 属性，native 在资源工作前显式拒绝 responsive/lazy。Next.js polymorphic root 不移植。 | P2 |
| [Indicator](https://mantine.dev/core/indicator/) | Indicator/ThemeIcon：位置、颜色、标签、尺寸、有限 variant | 缺 Indicator processing 动画/offset 和完整 ThemeIcon gradient/自动对比；先补静态布局，再复用有限运动。 | P2 |
| [Kbd](https://mantine.dev/core/kbd/) | Kbd：键组合、平台差异、文档语义 | 不支持任意 component；自定义快捷键显示/尺寸可组合，显示快捷键不等于全局键盘注册。 | 边界 |
| [NumberFormatter](https://mantine.dev/core/number-formatter/) | NumberFormatter：显式 locale、货币/百分比与精度 | 不是完整 Intl/NumberFormat：缺 compact/engineering/unit、任意前后缀与自定义 grouping；保留有限支持并拒绝未知格式。 | P2 |
| [OverflowList](https://mantine.dev/core/overflow-list/) | OverflowList：真实尺寸/有界动态项、overflow slot | 缺任意 DOM children 测量与 collapseFrom 的完整策略；复用当前窗口/注册节点，禁止保留访问过的业务项。 | P2 |
| [RollingNumber](https://mantine.dev/core/rolling-number/) | Marquee/RollingNumber：有限 translation、暂停、格式化 | 缺 hover/reduced-motion 组合和完整 glyph 翻滚动画；数值文本过渡不是完整逐字符动画，需如实区分。 | P2 |
| [Spoiler](https://mantine.dev/core/spoiler/) | Spoiler：测量高度、展开、标签、引用 | 缺完整过渡参数/尺寸同步配置；需要沿用真实测量与宿主帧，不应定时猜测溢出。 | P2 |
| [ThemeIcon](https://mantine.dev/core/theme-icon/) | Indicator/ThemeIcon：位置、颜色、标签、尺寸、有限 variant | 缺 Indicator processing 动画/offset 和完整 ThemeIcon gradient/自动对比；先补静态布局，再复用有限运动。 | P2 |
| [Timeline](https://mantine.dev/core/timeline/) | Timeline：active、marker、标题/描述和进度 | 缺 reverseActive、lineVariant、bullet 对齐等完整策略；用显式数据顺序，不能反转显示却改变访问顺序。 | P2 |
| [Blockquote](https://mantine.dev/core/blockquote/) | Blockquote：引用与 citation 的有限语义 | 缺 icon/iconSize 和完整色彩变体；可组合显示 slot，引用来源语义不应变为任意 HTML。 | P2 |
| [Code](https://mantine.dev/core/code/) | Code/Mark/Highlight：有限文档部位、查询高亮、转义 | 缺多关键字/重叠匹配/autoContrast 等便捷策略；Code 的高亮语言属于可选 CodeHighlight，不把 plain Code 标为完整语法高亮。 | P2 |
| [Highlight](https://mantine.dev/core/highlight/) | Code/Mark/Highlight：有限文档部位、查询高亮、转义 | 缺多关键字/重叠匹配/autoContrast 等便捷策略；Code 的高亮语言属于可选 CodeHighlight，不把 plain Code 标为完整语法高亮。 | P2 |
| [List](https://mantine.dev/core/list/) | List：真实有序/无序列表和条目 | 缺 list icon、withPadding/center 等便捷 API；图标列表需保留原生列表语义，不能用普通 Row 模拟。 | P2 |
| [Mark](https://mantine.dev/core/mark/) | Code/Mark/Highlight：有限文档部位、查询高亮、转义 | 缺多关键字/重叠匹配/autoContrast 等便捷策略；Code 的高亮语言属于可选 CodeHighlight，不把 plain Code 标为完整语法高亮。 | P2 |
| [Table](https://mantine.dev/core/table/) | Table：真实表格、span、captionSide/head/body/foot、stripe/hover、独立边框、sticky header、间距、tabular nums、滚动组合与行选择示例 | 已补 sticky/captionSide/spacing/hover/borders/tabular 配置与复杂示例；排序、筛选、列管理和虚拟化仍属 DataTable，native sticky 能力单独验收。 | P2 |
| [Text](https://mantine.dev/core/text/) | Text/Title/Typography：语义级别、单行截断、有限多行 lineClamp、gradient、字重/行高与主题 | lineClamp 和 text gradient 已扩展为 Glory 的独立有限样式字段，Web/SSR 实际降级并验证；Apace 在真实布局/paint 支持前明确 preflight 拒绝。任意 HTML 排版容器和字符串标记不进入组件契约。 | P2 |
| [Title](https://mantine.dev/core/title/) | Text/Title/Typography：语义级别、单行截断、有限多行 lineClamp、gradient、字重/行高与主题 | lineClamp 和 text gradient 已扩展为 Glory 的独立有限样式字段，Web/SSR 实际降级并验证；Apace 在真实布局/paint 支持前明确 preflight 拒绝。任意 HTML 排版容器和字符串标记不进入组件契约。 | P2 |
| [Typography](https://mantine.dev/core/typography/) | Text/Title/Typography：语义级别、单行截断、有限多行 lineClamp、gradient、字重/行高与主题 | lineClamp 和 text gradient 已扩展为 Glory 的独立有限样式字段，Web/SSR 实际降级并验证；Apace 在真实布局/paint 支持前明确 preflight 拒绝。任意 HTML 排版容器和字符串标记不进入组件契约。 | P2 |
| [Box](https://mantine.dev/core/box/) | Box/Flex/Stack/Group：布局、gap、对齐、换行、宽高 | Group grow/preventGrowOverflow 的便捷策略尚缺；响应式值通过 ResponsiveRoot 显式组合。Box 的任意 component/ref API 不移植。 | P2 |
| [Collapse](https://mantine.dev/core/collapse/) | Collapsible：展开、内容保留、有限高度过渡 | 与 Collapse 共享逻辑；任意 transition 名称/回调、水平折叠仍不是完整契约，不将只设置 CSS 算宿主动画支持。 | P2 |
| [Divider](https://mantine.dev/core/divider/) | Separator：水平/垂直、可访问名称、长度与粗细 | 缺可见 label/标签位置与 dashed/dotted 便捷参数；名称不是可见标签。边框样式可组合，标签分隔须保留分隔语义。 | P2 |
| [FocusTrap](https://mantine.dev/core/focus-trap/) | Portal/FocusScope：显式目标、失效回调、焦点范围与生命周期 | 对应 FocusScope 的 auto_focus/loop_tab/contain/return_focus；不采用 DOM selector 或 React child ref。需分别看 Web 与 native 的实际焦点证据。 | P2 |
| [Marquee](https://mantine.dev/core/marquee/) | Marquee/RollingNumber：有限 translation、暂停、格式化 | 缺 hover/reduced-motion 组合和完整 glyph 翻滚动画；数值文本过渡不是完整逐字符动画，需如实区分。 | P2 |
| [Paper](https://mantine.dev/core/paper/) | Card/Paper：header/body/footer、border、有限阴影 | Card.Section 的负边距和继承 padding 规则不是任意容器已有；优先明确 section 边界。颜色/圆角已可 Style 覆盖。 | P2 |
| [Portal](https://mantine.dev/core/portal/) | Portal/FocusScope：显式目标、失效回调、焦点范围与生命周期 | 不采用任意 DOM selector、ref 和 React child-clone API。FocusTrap 对应 FocusScope；任意 transition 样式注册仍待设计。 | P2 |
| [ScrollArea](https://mantine.dev/core/scroll-area/) | ScrollArea/Scroller：真实 scroll/query、反应轴、五种滚动条类型、尺寸/偏移/隐藏延迟、overscroll 样式和按实际范围定位 | 保留宿主真实滚动条与原生滚动/AX；浏览器样式由 Sapid CSS 提供，不引入 Radix DOM/第二套拖拽控制器。Hover/Scroll 非零延迟需显式宿主 timer；非 WebKit 引擎使用 thin 尺寸回退，原生平台外观仍独立验收。 | P2 |
| [Scroller](https://mantine.dev/core/scroller/) | ScrollArea/Scroller：真实 scroll/query、反应轴、五种滚动条类型、尺寸/偏移/隐藏延迟、overscroll 样式和按实际范围定位 | 保留宿主真实滚动条与原生滚动/AX；浏览器样式由 Sapid CSS 提供，不引入 Radix DOM/第二套拖拽控制器。Hover/Scroll 非零延迟需显式宿主 timer；非 WebKit 引擎使用 thin 尺寸回退，原生平台外观仍独立验收。 | P2 |
| [Toolbar](https://mantine.dev/core/toolbar/) | Toolbar/ActionBar：有界组、roving、selected_count/clear | Mantine ActionBar placement/transition/portal 配置不同；不得把横排 Toolbar 当完整底部浮条。可用 shared Layer 进一步补齐。 | P2 |
| [Transition](https://mantine.dev/core/transition/) | Theme/locale/direction/motion：app-local 主题与 typed Style | 不移植全局 MantineProvider/React hooks 或默认浏览器 locale；保留显式 app/request 边界。默认 props registry/任意样式 resolver 暂不采纳。 | 边界 |
| [VisuallyHidden](https://mantine.dev/core/visually-hidden/) | VisuallyHidden：语义文字、固定隐藏样式和引用 | 不支持 polymorphic component；可访问名称/描述与真正视觉隐藏保留，不应通过 display:none 替代。 | 边界 |
| [DatePicker](https://mantine.dev/dates/date-picker/) | DatePicker/Input：单/多/范围、弹层、排除日、受控、有限 typed presets、最多四列连续月份、columnsToScroll、week numbers/hide weekdays、month/year/decade levels、ISO/日月年/月日年显示格式 | 已补 single/multiple/range 共用的完整 DateSelection presets、共享选择/月锚点的 numberOfColumns/columnsToScroll、withWeekNumbers/hideWeekdays、受控 levels，以及不改变 typed value 的三种有限 display format；showcase 与浏览器验收覆盖单值、多值和范围。任意 locale/date-fns 格式串及任意 formatter 闭包仍不移植。保留 CivilDate、四列、32 项 preset 限额和 range 不完整态独立模型。 | P2 |
| [DatePickerInput](https://mantine.dev/dates/date-picker-input/) | DatePicker/Input：单/多/范围、弹层、排除日、受控、有限 typed presets、最多四列连续月份、columnsToScroll、week numbers/hide weekdays、month/year/decade levels、ISO/日月年/月日年显示格式 | 已补 single/multiple/range 共用的完整 DateSelection presets、共享选择/月锚点的 numberOfColumns/columnsToScroll、withWeekNumbers/hideWeekdays、受控 levels，以及不改变 typed value 的三种有限 display format；showcase 与浏览器验收覆盖单值、多值和范围。任意 locale/date-fns 格式串及任意 formatter 闭包仍不移植。保留 CivilDate、四列、32 项 preset 限额和 range 不完整态独立模型。 | P2 |
| [DateTimePicker](https://mantine.dev/dates/date-time-picker/) | DateTimePicker/Inline：真实 input dialog、日期+时间草稿、显式 zone resolver、presets/default time、秒精度、有限 display format、clearable、TimePicker controls、DatePicker levels/columns、原子 range 双端值 | 单值与原子 range 的 popup/inline 均已转发受控 month/level、defaultLevel/maxLevel/onLevelChange、最多四列、columnsToScroll、week numbers/weekday visibility；十段 showcase 覆盖这些能力。时区转换保持显式 provider，不引入隐式环境数据库。 | P2 |
| [InlineDateTimePicker](https://mantine.dev/dates/inline-date-time-picker/) | DateTimePicker/Inline：真实 input dialog、日期+时间草稿、显式 zone resolver、presets/default time、秒精度、有限 display format、clearable、TimePicker controls、DatePicker levels/columns、原子 range 双端值 | 单值与原子 range 的 popup/inline 均已转发受控 month/level、defaultLevel/maxLevel/onLevelChange、最多四列、columnsToScroll、week numbers/weekday visibility；十段 showcase 覆盖这些能力。时区转换保持显式 provider，不引入隐式环境数据库。 | P2 |
| [DateInput](https://mantine.dev/dates/date-input/) | DateInput：ISO/日月年/月日年有限格式、响应式 placeholder、自定义 parser、picker、reject/commit | 已补三种严格 finite display/parser 格式和共享宿主 placeholder；仍不接受任意 locale/date-fns 格式串或模糊日期猜测，保持 CivilDate、typed parser 和明确错误恢复。showcase 展示三种格式、placeholder 与 custom parser。 | P2 |
| [MonthPicker](https://mantine.dev/dates/month-picker/) | MonthPicker/Input：YearMonth、单/多/范围、排除值、Grid | 缺月份文本自定义、多列年份跨度、presets/标签格式；不把 picker 的 locale 参数转发视为所有日历格式。 | P2 |
| [MonthPickerInput](https://mantine.dev/dates/month-picker-input/) | MonthPicker/Input：YearMonth、单/多/范围、排除值、Grid | 缺月份文本自定义、多列年份跨度、presets/标签格式；不把 picker 的 locale 参数转发视为所有日历格式。 | P2 |
| [YearPicker](https://mantine.dev/dates/year-picker/) | YearPicker/Input：CivilYear、有限十年、范围/多选、Grid | 缺多年份面板/decadeLabelFormat/预设值；同族 Month API 不意味着全部 Year API 同名。 | P2 |
| [YearPickerInput](https://mantine.dev/dates/year-picker-input/) | YearPicker/Input：CivilYear、有限十年、范围/多选、Grid | 缺多年份面板/decadeLabelFormat/预设值；同族 Month API 不意味着全部 Year API 同名。 | P2 |
| [TimeInput](https://mantine.dev/dates/time-input/) | TimeInput/Value：LocalTime、12/24h、分/秒精度、英中有限 locale、响应式真实 placeholder、草稿/边界 | 已补共享宿主 placeholder，并在 showcase 同时展示 24h、英文/中文 12h、秒精度、初值与状态；仍不支持任意 dayjs format、环境 locale 或自动时区，本地墙上时间和瞬时时刻继续分开。 | P2 |
| [TimePicker](https://mantine.dev/dates/time-picker/) | TimePicker/Grid：分段 SpinButton、step/排除、12/24h、草稿、受控 clearable、分段 placeholder、typed duration/minHoursDigits、presets、完整 controls dropdown、pasteSplit、受控锚定弹层、labels/focus、typed placement/width | 已补独立 TimeDuration、动态小时宽度、真实 Popover presets/groups 与小时/分钟/秒/period 控件列、反向列表/有限滚动、32-byte post-edit pasteSplit、受控 open 和十五段 showcase；任意 JS popoverProps 由有限 typed placement/width 代替。 | P2 |
| [TimeGrid](https://mantine.dev/dates/time-grid/) | TimePicker/Grid：分段 SpinButton、step/排除、12/24h、草稿、受控 clearable、分段 placeholder、typed duration/minHoursDigits、presets、完整 controls dropdown、pasteSplit、受控锚定弹层、labels/focus、typed placement/width | 已补独立 TimeDuration、动态小时宽度、真实 Popover presets/groups 与小时/分钟/秒/period 控件列、反向列表/有限滚动、32-byte post-edit pasteSplit、受控 open 和十五段 showcase；任意 JS popoverProps 由有限 typed placement/width 代替。 | P2 |
| [TimeValue](https://mantine.dev/dates/time-value/) | TimeInput/Value：LocalTime、12/24h、分/秒精度、英中有限 locale、响应式真实 placeholder、草稿/边界 | 已补共享宿主 placeholder，并在 showcase 同时展示 24h、英文/中文 12h、秒精度、初值与状态；仍不支持任意 dayjs format、环境 locale 或自动时区，本地墙上时间和瞬时时刻继续分开。 | P2 |
| [MiniCalendar](https://mantine.dev/dates/mini-calendar/) | Calendar/MiniCalendar：真实42格Grid、CivilDate、受控月份/选择、排除/周起点、week numbers、typed day renderer、full width 与 passive static | 已补公开 DateSelection 投影和激活提议、有限 inline day renderer、全宽百分比网格、无隐藏按钮的静态模式及5组展示；仍不隐式生成无限年份 native select、Calendar 自身多月或任意 locale formatter。DatePicker 已提供最多四列月份；Calendar 保持真实 Grid/TableMetadata 与 roving day controls。 | P2 |
| [Calendar](https://mantine.dev/dates/calendar/) | Calendar/MiniCalendar：真实42格Grid、CivilDate、受控月份/选择、排除/周起点、week numbers、typed day renderer、full width 与 passive static | 已补公开 DateSelection 投影和激活提议、有限 inline day renderer、全宽百分比网格、无隐藏按钮的静态模式及5组展示；仍不隐式生成无限年份 native select、Calendar 自身多月或任意 locale formatter。DatePicker 已提供最多四列月份；Calendar 保持真实 Grid/TableMetadata 与 roving day controls。 | P2 |
| [AreaChart](https://mantine.dev/charts/area-chart/) | Area/Line/Bar/Composite：typed 数据/轴/domain/legend/selection、inclusive brush，以及有界 reference area/line/dot 与标签 | 不暴露 Recharts arbitrary props；复杂 tooltip renderer、动画、多轴全部组合和任意 formatter 保留为有限协议边界。reference 几何由 Web/Native 共用，不是页面 SVG 叠加。 | P2 |
| [BarChart](https://mantine.dev/charts/bar-chart/) | Area/Line/Bar/Composite：typed 数据/轴/domain/legend/selection、inclusive brush，以及有界 reference area/line/dot 与标签 | 不暴露 Recharts arbitrary props；复杂 tooltip renderer、动画、多轴全部组合和任意 formatter 保留为有限协议边界。reference 几何由 Web/Native 共用，不是页面 SVG 叠加。 | P2 |
| [LineChart](https://mantine.dev/charts/line-chart/) | Area/Line/Bar/Composite：typed 数据/轴/domain/legend/selection、inclusive brush，以及有界 reference area/line/dot 与标签 | 不暴露 Recharts arbitrary props；复杂 tooltip renderer、动画、多轴全部组合和任意 formatter 保留为有限协议边界。reference 几何由 Web/Native 共用，不是页面 SVG 叠加。 | P2 |
| [CompositeChart](https://mantine.dev/charts/composite-chart/) | Area/Line/Bar/Composite：typed 数据/轴/domain/legend/selection、inclusive brush，以及有界 reference area/line/dot 与标签 | 不暴露 Recharts arbitrary props；复杂 tooltip renderer、动画、多轴全部组合和任意 formatter 保留为有限协议边界。reference 几何由 Web/Native 共用，不是页面 SVG 叠加。 | P2 |
| [CandlestickChart](https://mantine.dev/charts/candlestick-chart/) | Scatter/Bubble/Candlestick：有界数据、采样、选择 | 缺 brush/多轴/自定义 symbol 与完整 tooltip formatting；采样必须报告源数量，不能默默把全量数据当已绘制。 | P2 |
| [DonutChart](https://mantine.dev/charts/donut-chart/) | Pie/Donut/Radial：有限 Plot、legend/selection、可访问数据 | 缺任意分段 label renderer/动画/复杂 ring/tooltip；静态图形、数据表与真实选择各有证据，不能靠图形相似推断 API 等价。 | P2 |
| [PieChart](https://mantine.dev/charts/pie-chart/) | Pie/Donut/Radial：有限 Plot、legend/selection、可访问数据 | 缺任意分段 label renderer/动画/复杂 ring/tooltip；静态图形、数据表与真实选择各有证据，不能靠图形相似推断 API 等价。 | P2 |
| [FunnelChart](https://mantine.dev/charts/funnel-chart/) | Radar/Funnel/Sparkline：有限 Plot/options 与语义数据 | 缺多 radar 叠加和各类插值/label/tooltip 完整配置；Sparkline 的有限模型不同于任意 Recharts 属性。 | P2 |
| [RadarChart](https://mantine.dev/charts/radar-chart/) | Radar/Funnel/Sparkline：有限 Plot/options 与语义数据 | 缺多 radar 叠加和各类插值/label/tooltip 完整配置；Sparkline 的有限模型不同于任意 Recharts 属性。 | P2 |
| [ScatterChart](https://mantine.dev/charts/scatter-chart/) | Scatter/Bubble/Candlestick：有界数据、采样、选择 | 缺 brush/多轴/自定义 symbol 与完整 tooltip formatting；采样必须报告源数量，不能默默把全量数据当已绘制。 | P2 |
| [BubbleChart](https://mantine.dev/charts/bubble-chart/) | Scatter/Bubble/Candlestick：有界数据、采样、选择 | 缺 brush/多轴/自定义 symbol 与完整 tooltip formatting；采样必须报告源数量，不能默默把全量数据当已绘制。 | P2 |
| [RadialBarChart](https://mantine.dev/charts/radial-bar-chart/) | Pie/Donut/Radial：有限 Plot、legend/selection、可访问数据 | 缺任意分段 label renderer/动画/复杂 ring/tooltip；静态图形、数据表与真实选择各有证据，不能靠图形相似推断 API 等价。 | P2 |
| [Sparkline](https://mantine.dev/charts/sparkline/) | Radar/Funnel/Sparkline：有限 Plot/options 与语义数据 | 缺多 radar 叠加和各类插值/label/tooltip 完整配置；Sparkline 的有限模型不同于任意 Recharts 属性。 | P2 |
| [Heatmap](https://mantine.dev/charts/heatmap/) | Heatmap/Matrix/Waffle：离散数据、色阶与可访问表 | 缺完整日期热图布局、hover renderer、多色 cell shape 和任意动画；需明确 missing value 和有限颜色映射。 | P2 |
| [BarsList](https://mantine.dev/charts/bars-list/) | BarsList/Bullet/Gauge：有限形状与数据标签 | 缺目标/区间段/动态格式的完整组合；优先 typed thresholds/labels，不能把 progress 换名当 gauge/bullet。 | P2 |
| [BulletChart](https://mantine.dev/charts/bullet-chart/) | BarsList/Bullet/Gauge：有限形状与数据标签 | 缺目标/区间段/动态格式的完整组合；优先 typed thresholds/labels，不能把 progress 换名当 gauge/bullet。 | P2 |
| [GaugeChart](https://mantine.dev/charts/gauge-chart/) | BarsList/Bullet/Gauge：有限形状与数据标签 | 缺目标/区间段/动态格式的完整组合；优先 typed thresholds/labels，不能把 progress 换名当 gauge/bullet。 | P2 |
| [WaffleChart](https://mantine.dev/charts/waffle-chart/) | Heatmap/Matrix/Waffle：离散数据、色阶与可访问表 | 缺完整日期热图布局、hover renderer、多色 cell shape 和任意动画；需明确 missing value 和有限颜色映射。 | P2 |
| [Treemap](https://mantine.dev/charts/treemap/) | Treemap/Sunburst/Sankey：有界层级/流、导航和数据表 | 缺任意布局算法/节点链接 render/动态图过渡；需先校验层级/环/预算，不能导入任意图形引擎配置。 | P2 |
| [SunburstChart](https://mantine.dev/charts/sunburst-chart/) | Treemap/Sunburst/Sankey：有界层级/流、导航和数据表 | 缺任意布局算法/节点链接 render/动态图过渡；需先校验层级/环/预算，不能导入任意图形引擎配置。 | P2 |
| [SankeyChart](https://mantine.dev/charts/sankey-chart/) | Treemap/Sunburst/Sankey：有界层级/流、导航和数据表 | 缺任意布局算法/节点链接 render/动态图过渡；需先校验层级/环/预算，不能导入任意图形引擎配置。 | P2 |
| [MatrixChart](https://mantine.dev/charts/matrix-chart/) | Heatmap/Matrix/Waffle：离散数据、色阶与可访问表 | 缺完整日期热图布局、hover renderer、多色 cell shape 和任意动画；需明确 missing value 和有限颜色映射。 | P2 |
| [Schedule](https://mantine.dev/schedule/schedule/) | Schedule/事件/recurrence：有限模型、显式 zone/provider、窗口，以及受控 Day/Week/Month/Year 等统一视图头 | 不暴露任意外部事件/overlap renderer；通用 DnD/resize 属于各实际视图的有限编辑协议，不由统一容器伪造。应用在切换时原子接受 view 与匹配当前窗口。 | P2 |
| [DayView](https://mantine.dev/schedule/day-view/) | Day/Week：有界时间窗口、真实 Grid、重叠 lane、全天区、精确选择，以及可配 slot height/header/all-day 展示 | 已补与官方示例相对应的时间范围、interval、slot height、全天区、重叠、受控窗口、静态/readonly 和逐页 showcase。任意 React slot/event renderer、浏览器全局外拖与非资源 event resize 不移植；编辑使用资源视图已有的 typed 权限/目标/冲突协议。 | P2 |
| [MonthView](https://mantine.dev/schedule/month-view/) | Month/Year/MobileMonth：真实有限 Grid、选择/窗口、事件 disclosure，以及 weekday/outside/today/max-events/header 展示选项 | 已补星期标题、月外日期显隐、显式 today、每日预览上限、无 header、week start/locale/RTL 和三个独立 showcase。任意 React 日期/event renderer 与浏览器全局拖拽不移植；日期跳转仍是应用接受精确窗口提议。 | P2 |
| [WeekView](https://mantine.dev/schedule/week-view/) | Day/Week：有界时间窗口、真实 Grid、重叠 lane、全天区、精确选择，以及可配 slot height/header/all-day 展示 | 已补与官方示例相对应的时间范围、interval、slot height、全天区、重叠、受控窗口、静态/readonly 和逐页 showcase。任意 React slot/event renderer、浏览器全局外拖与非资源 event resize 不移植；编辑使用资源视图已有的 typed 权限/目标/冲突协议。 | P2 |
| [YearView](https://mantine.dev/schedule/year-view/) | Month/Year/MobileMonth：真实有限 Grid、选择/窗口、事件 disclosure，以及 weekday/outside/today/max-events/header 展示选项 | 已补星期标题、月外日期显隐、显式 today、每日预览上限、无 header、week start/locale/RTL 和三个独立 showcase。任意 React 日期/event renderer 与浏览器全局拖拽不移植；日期跳转仍是应用接受精确窗口提议。 | P2 |
| [MobileMonthView](https://mantine.dev/schedule/mobile-month-view/) | Month/Year/MobileMonth：真实有限 Grid、选择/窗口、事件 disclosure，以及 weekday/outside/today/max-events/header 展示选项 | 已补星期标题、月外日期显隐、显式 today、每日预览上限、无 header、week start/locale/RTL 和三个独立 showcase。任意 React 日期/event renderer 与浏览器全局拖拽不移植；日期跳转仍是应用接受精确窗口提议。 | P2 |
| [AgendaView](https://mantine.dev/schedule/agenda-view/) | AgendaView：有界窗口事件展示/选择、locale | 缺分组 header/空日策略/完整事件 body renderer；应用负责数据加载，不建立访问日期的永久事件缓存。 | P2 |
| [ResourcesDayView](https://mantine.dev/schedule/resources-day-view/) | Resource views：统一受控 Day/Week/Month、资源窗口/分页、有界 Grid，以及 move/resize 精确编辑提议 | 不接收任意 React event/resource renderer 或浏览器全局外拖对象；typed 编辑、权限、stale commit 和应用接受窗口保持有限协议。外部拖入业务对象需由应用先转换成已验证 ScheduleEdit。 | P2 |
| [ResourcesWeekView](https://mantine.dev/schedule/resources-week-view/) | Resource views：统一受控 Day/Week/Month、资源窗口/分页、有界 Grid，以及 move/resize 精确编辑提议 | 不接收任意 React event/resource renderer 或浏览器全局外拖对象；typed 编辑、权限、stale commit 和应用接受窗口保持有限协议。外部拖入业务对象需由应用先转换成已验证 ScheduleEdit。 | P2 |
| [ResourcesMonthView](https://mantine.dev/schedule/resources-month-view/) | Resource views：统一受控 Day/Week/Month、资源窗口/分页、有界 Grid，以及 move/resize 精确编辑提议 | 不接收任意 React event/resource renderer 或浏览器全局外拖对象；typed 编辑、权限、stale commit 和应用接受窗口保持有限协议。外部拖入业务对象需由应用先转换成已验证 ScheduleEdit。 | P2 |
| [ResourcesSchedule](https://mantine.dev/schedule/resources-schedule/) | Resource views：统一受控 Day/Week/Month、资源窗口/分页、有界 Grid，以及 move/resize 精确编辑提议 | 不接收任意 React event/resource renderer 或浏览器全局外拖对象；typed 编辑、权限、stale commit 和应用接受窗口保持有限协议。外部拖入业务对象需由应用先转换成已验证 ScheduleEdit。 | P2 |
| [Recurring events](https://mantine.dev/schedule/recurring-events/) | Schedule/事件/recurrence：有限模型、显式 zone/provider、窗口，以及受控 Day/Week/Month/Year 等统一视图头 | 不暴露任意外部事件/overlap renderer；通用 DnD/resize 属于各实际视图的有限编辑协议，不由统一容器伪造。应用在切换时原子接受 view 与匹配当前窗口。 | P2 |
| [Events data](https://mantine.dev/schedule/events-data/) | Schedule/事件/recurrence：有限模型、显式 zone/provider、窗口，以及受控 Day/Week/Month/Year 等统一视图头 | 不暴露任意外部事件/overlap renderer；通用 DnD/resize 属于各实际视图的有限编辑协议，不由统一容器伪造。应用在切换时原子接受 view 与匹配当前窗口。 | P2 |
| [CodeHighlight](https://mantine.dev/code-highlight/code-highlight/) | CodeHighlight：有限语言/token、行号/高亮、复制 | 不兼容 Shiki/highlight.js 任意 grammar/themes、多文件 tabs 等；新语言须增加有界 lexer 和错误/大文本预算。 | P2 |
| [JsonViewer](https://mantine.dev/code-highlight/json-viewer/) | JsonViewer：有限 tree、搜索/展开/复制、键盘 | 缺任意 JS object/循环引用/custom renderer；JSON 与 runtime object inspector 不同，保留深度/节点/字节限制。 | 边界 |
| [Notifications system](https://mantine.dev/x/notifications/) | Notification/系统：有界队列、更新/关闭、计时与 hover 暂停 | 缺分区域 placement、任意 transition 和完整 loader/icon slots；队列有界与 app-local context 优于无界全局管理器。 | P2 |
| [Spotlight](https://mantine.dev/x/spotlight/) | Command/Spotlight：搜索、分组、异步结果、显式 shortcut 和 handle | 缺自由渲染结果/完整 actions 分部 API；任意 fuzzy highlight 和 loading slot 待设计。现有命令与快捷键已受 Owner 保护。 | P2 |
| [Carousel](https://mantine.dev/x/carousel/) | Carousel：有界 window/overscan、current、swipe、autoplay | 不是 Embla 全部 options/plugins；缺多 slide 排列/dragFree/细粒度 controls，优先共享窗口逻辑而非引入 JS 引擎。 | P2 |
| [Lightbox](https://mantine.dev/x/lightbox/) | Lightbox：真实图片/标题、有限 view/zoom、Layer | 缺 thumbnails、视频支持和完整 gestures；transform/缩放需要有限语义与预算，不让原图资源永久留在访问缓存。 | P2 |
| [Dropzone](https://mantine.dev/x/dropzone/) | Dropzone：有限接收/预算/拒绝、读取/预览、取消 | 缺全局 window dropzone、更多 reject slots/拖动文案；全局文件访问/读取不隐式加入，保留权限和资源预算。 | P2 |
| [NavigationProgress](https://mantine.dev/x/nprogress/) | NavigationProgress：显式 start/set/finish/cancel handle | 缺自动增量等动画策略；不能让 loader 定时器逃逸 Owner，关闭后不应重新启动。 | P2 |
| [Modals manager](https://mantine.dev/x/modals/) | ModalsManager：app-local 有界队列、confirm/cancel、结果/更新 | 缺任意 React context modal 注册；有限 typed request 可以扩展，不采纳全局 registry/任意组件注入。 | P2 |
| [Rich text editor](https://mantine.dev/x/tiptap/) | RichTextEditor：有限 RichDocument、格式/selection/history、controlled、placeholder、subtle 与无工具栏/只读/禁用状态 | 不是 Tiptap plugin/HTML 编辑器；任意 extension、表格、任务、图片上传、源码模式、语法高亮和协同编辑属于显式架构边界，必须先扩展 Glory 的有限文档/编辑协议，不能用字符串 HTML 或页面脚本伪装。 | 边界 |

完整 Props、功能章节与本地入口见 [API 表面索引](mantine-api-surface.md)；机器事实见 [快照](mantine-api-audit.json)。

## 本轮验证与复现

- 官方快照：181 条组件/工具目录记录、4,659 个按条目计数的 Props 字段（含同页子部件）及其类型；刷新器会拒绝未知新增目录页或无法解析的 Props 行，排除示例代码里的伪章节。
- `scripts/check.ps1` 完整通过：415 组、2,649 次测试执行及默认/all-features 严格 Clippy（测试次数包含多 feature 重复运行）。日志：`D:/BuildCache/sapid/mantine-library-gate.log`。
- 最终变更另补跑 29 项选择控件、3 项输入 hint、10 项 Button 增强测试，以及 all-features/all-targets 严格 Clippy。日志：`mantine-final-choice.log`、`mantine-final-input-tests.log`、`mantine-final-sapid-clippy.log`。
- Glory：共享 hint 验证/回滚与 HTML prepared-field/full-shape 差分通过；独立 SSR stream 的 hint patch 通过；Apace descriptor-local preflight/direct apply 拒绝与前缀保留通过，native tests 严格 Clippy 通过。日志：`mantine-glory-placeholder.log`、`mantine-html-placeholder.log`、`mantine-stream-placeholder.log`、`mantine-native-placeholder.log`、`mantine-native-clippy.log`。
- 引擎来源检查与 fork 自有测试全部通过：保留两个 full SHA、六个上游包名及锁文件来源。修正检查器过期的十包断言，改为核对当前包含 opt-in i18n 的完整十一包集合。未修改 fork 代码或依赖 revision。日志：`mantine-engine-final.log`。
- 展示站 14 项站点测试、3 项路由测试、28 项库测试、host/wasm32 严格 Clippy、release WASM 与 48 个代表性 SSR 文档通过；最终资产 `assets-b4eaa6272aa978d9378f`。
- Chromium 148.0.7778.96 的统一矩阵 33/33 通过，`complete=true`、`errors=[]`；覆盖全部 181 个真实 slug 路由、Theme Studio/调色顺序/导航/缓存，以及 Calendar、Input、Popover、Typography、Image、日期时间、选择、Tree、ColorPicker、Modal、RichEditor、Charts 和全部 Schedule 专项。报告：`D:/BuildCache/sapid/showcase-acceptance/results.json`。
- 新增浏览器专项同时验证真实 Input 的 value setter 未在 hint 更新时调用、节点/UTF-16 选区/预编辑保留，四主题、1440/360px、CSR/真实 SSR 接管均通过。CDP IME 是协议证据，不等于 OS IME 或读屏语音验收。
- A4 已由当前有限 API、专项组件测试与真实 showcase 验收完成；A5 已完成 Mantine 9.7.4 展示结构、真实 API 参考和 Button 多场景/compact。逐项差异表仍明确记录任意 React renderer、隐式 I/O、全局拖放和未验证 OS/AT 能力边界，不把目录覆盖写成无限制 API 全兼容。

复现：`python scripts/mantine-api-audit.py`；离线检查：`python scripts/mantine-api-audit.py --check`。
上游事实仅在显式 `--refresh` 时更新；完整文档缓存在 D 盘，仓库只保存属性/章节事实和 SHA256，未复制第三方实现。
文档快照 SHA256：`f8deb873cd919202efed530cd243b7e4474bc8d1621a2899ec2b03b58c9c4d8a`。
