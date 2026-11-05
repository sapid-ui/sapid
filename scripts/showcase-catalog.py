"""Generate the Rust documentation catalogue and direct-widget family dispatch."""
import argparse
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "examples/web/src/showcase_catalog.rs"
EXAMPLES = ROOT / "examples/web/src/showcase_examples.rs"
WEB = ROOT / "examples/web"
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--check", action="store_true")
args = parser.parse_args()
inventory = json.loads((ROOT / "docs/widget-inventory.json").read_text(encoding="utf-8"))
ledger = json.loads((ROOT / "docs/capability-matrix.json").read_text(encoding="utf-8"))
tasks = inventory["tasks"]
records = {entry["id"]: entry for entry in ledger["tasks"]}
if len(tasks) != 137 or len(records) != 137 or {t["id"] for t in tasks} != set(records):
    raise SystemExit("catalogue requires all 137 canonical tasks and explicit ledger entries")

EXTRA_PAGES = {"P0-08": "drawer", "P2-09": "actions", "P10-07": "json-viewer"}
CORE = {f"P0-{i:02}": name for i, name in enumerate([
    "button", "text_field", "checkbox", "collapsible", "accordion", "tabs", "dialog", "drawer", "popover"
], 1)} | {"P0-14": "catalogue", "P2-08": "icon_actions", "P3-10": "pills", "P4-08": "multi_select", "P4-09": "tags"}
FOCUSED_GROUPS = {
    "layout_controls": ["P3-04", "P6-07", "P6-08"],
    "fields": ["P1-01", "P1-02", "P1-03", "P1-04", "P1-05", "P1-06"],
    "choices": ["P0-10", "P2-01", "P2-02", "P2-03", "P2-05", "P2-06"],
    "responsive": ["P3-02", "P3-03"],
    "document": ["P3-07", "P3-08", "P3-09", "P3-16", "P3-18"],
    "display": ["P3-11", "P3-17", "P3-20"],
    "alerts": ["P5-01", "P5-05", "P5-10", "P5-12"],
    "hints": ["P5-03", "P5-04"],
    "navigation": ["P6-01", "P6-02"],
    "timeline": ["P3-12", "P3-19"],
    "rating": ["P7-05"],
    "number_input": ["P7-01"],
    "measurement": ["P7-02", "P7-03", "P5-08"],
    "copy_button": ["P10-03"],
    "file_button": ["P10-01"],
    "rich_editor": ["P10-08"],
    "toolbar": ["P2-07", "P6-10"],
    "charts": ["P11-01", "P11-02", "P11-03", "P11-04", "P11-05", "P11-06", "P11-07", "P11-08"],
    "schedule": ["P12-01"],
}
FOCUSED = {identity: family for family, identities in FOCUSED_GROUPS.items() for identity in identities}
FOCUSED_NAMES = {
    "P3-04": "focused_aspect_ratio",
    "P6-07": "focused_scroll_area",
    "P6-08": "focused_splitter",
    "P0-10": "focused_collection",
    "P1-01": "focused_field",
    "P1-02": "focused_field_error",
    "P1-03": "focused_fieldset",
    "P1-04": "focused_textarea",
    "P1-05": "focused_password_input",
    "P1-06": "focused_input_group",
    "P2-01": "focused_checkbox_group",
    "P2-02": "focused_switch",
    "P2-03": "focused_radio_group",
    "P2-05": "focused_segmented_control",
    "P2-06": "focused_chip_group",
    "P2-07": "focused_button_group",
    "P3-02": "focused_center_container",
    "P3-03": "focused_grid",
    "P3-07": "focused_list",
    "P3-08": "focused_blockquote",
    "P3-09": "focused_inline_code",
    "P3-11": "focused_card",
    "P3-12": "focused_separator",
    "P3-16": "focused_kbd",
    "P3-17": "focused_empty_state",
    "P3-18": "focused_data_list",
    "P3-19": "focused_timeline",
    "P3-20": "focused_number_formatter",
    "P5-01": "focused_alert_dialog",
    "P5-03": "focused_tooltip",
    "P5-04": "focused_hover_card",
    "P5-05": "focused_alert",
    "P5-08": "focused_progress",
    "P5-10": "focused_skeleton",
    "P5-12": "focused_floating_dialog",
    "P6-01": "focused_breadcrumbs",
    "P6-02": "focused_pagination",
    "P6-10": "focused_toolbar",
    "P7-01": "focused_number_input",
    "P7-02": "focused_slider",
    "P7-03": "focused_range_slider",
    "P7-05": "focused_rating",
    "P10-01": "focused_file_button",
    "P10-03": "focused_copy_button",
    "P10-08": "focused_rich_editor",
    "P11-01": "focused_chart",
    "P11-02": "focused_cartesian_charts",
    "P11-03": "focused_polar_charts",
    "P11-04": "focused_point_charts",
    "P11-05": "focused_shape_charts",
    "P11-06": "focused_matrix_charts",
    "P11-07": "focused_range_charts",
    "P11-08": "focused_hierarchy_charts",
    "P12-01": "focused_schedule",
}
FOCUSED_DISPLAY = {
    "P12-05-resourcesschedule": ("focused_resources_schedule", "resources_schedule"),
    "P12-02": ("focused_dayview", "schedule_timelines"),
    "P12-02-weekview": ("focused_weekview", "schedule_timelines"),
    "P12-03": ("focused_monthview", "schedule_calendars"),
    "P12-03-yearview": ("focused_yearview", "schedule_calendars"),
    "P12-03-mobilemonthview": ("focused_mobilemonthview", "schedule_calendars"),
}
if set(FOCUSED_NAMES) != set(FOCUSED):
    raise SystemExit("every focused demo requires an explicit semantic module name")
SPECIAL = {
    "index": ("form", "form_themed", "scope, theme.clone()", True),
    "composites": ("composite_form", "form_themed", "scope, theme.clone()", True),
    "charts": ("charts_form", "charts_form", "theme, false", False),
    "cartesian": ("charts_form", "charts_form", "theme, true", False),
    "json-viewer": ("json_tree_example", "json_tree_example", "theme", False),
    "tree-select": ("tree_choices_form", "tree_choices_form", "theme, false", False),
    "cascader": ("tree_choices_form", "tree_choices_form", "theme, true", False),
}
for index, page in enumerate(["polar-charts", "point-charts", "shape-charts", "matrix-charts", "range-charts", "hierarchy-charts"]):
    SPECIAL[page] = ("plots_form", "plots_form", f"theme, {index}", False)

# Reader-facing introductions are separate from the canonical engineering evidence.
LEADS = {
    "P0": [
        "Trigger actions with accessible buttons, themed variants and loading states.",
        "Collect a single line of text with labels, descriptions and controlled values.",
        "Let users choose whether to enable an option or accept a condition.",
        "Reveal supporting content with a compact disclosure control.",
        "Organize related information into expandable sections.",
        "Switch between related views without leaving the current page.",
        "Focus attention on a task in a modal dialog.",
        "Open a side panel for navigation, settings or additional information.",
        "Place interactive content beside the control that opens it.",
        "Build consistent keyboard navigation for collections of interactive items.",
        "Compose focus boundaries and render content into an explicit destination.",
        "Configure colors, typography, spacing, language and reading direction.",
        "Provide accessible text that stays visually hidden.",
        "Explore every widget and compare its documented platform capabilities.",
    ],
    "P1": [
        "Connect a form control with its label, help text and validation state.",
        "Give controls clear labels, useful descriptions and actionable error messages.",
        "Group related controls under a shared legend.",
        "Collect multiline text with fixed rows or content-based sizing.",
        "Enter a password with an optional visibility control.",
        "Arrange an input with leading or trailing content and actions.",
        "Coordinate typed form values, validation and submission feedback.",
        "Submit or reset a form and read the values of its successful controls.",
    ],
    "P2": [
        "Manage multiple checkbox choices, including partially selected states.",
        "Turn a setting on or off with a switch.",
        "Choose one option from a named group of radio controls.",
        "Represent on-or-off actions with toggle buttons.",
        "Choose one value from a compact row of connected options.",
        "Present single or multiple choices as compact selectable chips.",
        "Arrange related buttons into one connected group.",
        "Offer compact icon actions and clearly named close controls.",
        "Toggle navigation with an animated menu button.",
    ],
    "P3": [
        "Build horizontal and vertical layouts with flexible spacing and alignment.",
        "Center content, constrain its width and add intentional space.",
        "Arrange content in responsive columns with spans and consistent gaps.",
        "Keep media or other content at a chosen width-to-height ratio.",
        "Create readable text, headings and consistent typography.",
        "Link to another page or location with an accessible text link.",
        "Present related items as ordered or unordered lists.",
        "Set quotations apart from surrounding text.",
        "Display inline code and draw attention to important words.",
        "Show compact labels, statuses and removable pills.",
        "Group content on themed surfaces with padding, borders and shadows.",
        "Separate content with horizontal or vertical dividers.",
        "Represent people with avatars, initials and grouped images.",
        "Display images with accessible alternatives or use an image as a backdrop.",
        "Add a small indicator or place an icon on a themed surface.",
        "Display keyboard keys and shortcuts in running text.",
        "Explain an empty view and offer a useful next action.",
        "Lay out labeled details and structured content rows.",
        "Show events or milestones in chronological order.",
        "Format numbers, currencies and units for readable display.",
        "Collapse long content behind a show-more control.",
    ],
    "P4": [
        "Organize actions in a keyboard-accessible menu with nested submenus.",
        "Open a menu for the item under the pointer or keyboard focus.",
        "Keep application menus together in a navigable menu bar.",
        "Choose values using the browser's native select control.",
        "Choose one value from a styled list of options.",
        "Combine text entry with a searchable list of known options.",
        "Suggest matching values while allowing users to enter their own text.",
        "Search for and select several values from a list of options.",
        "Enter, paste and remove free-form tags in a single input.",
        "Find commands with search and keyboard navigation in a command palette.",
    ],
    "P5": [
        "Ask users to confirm an important action before continuing.",
        "Reveal a drawer with interactive snap points and gesture dismissal.",
        "Show a short description when a control is hovered or focused.",
        "Preview richer supporting content beside a hovered or focused element.",
        "Call attention to important information, warnings or errors.",
        "Deliver temporary notifications with application-controlled actions.",
        "Indicate that an operation is still in progress.",
        "Show progress as a bar, ring or semicircular indicator.",
        "Show page or application activity with a compact navigation progress bar.",
        "Reserve space for content while it is loading.",
        "Place a visual or loading overlay over a region of the interface.",
        "Keep a nonmodal message visible while users continue working.",
        "Coordinate queued dialogs and confirmations from one place.",
        "Create a movable and resizable floating workspace.",
        "Guide users through a sequence of highlighted interface targets.",
    ],
    "P6": [
        "Show the path to the current page and link to its ancestors.",
        "Move between pages of results with compact pagination controls.",
        "Build nested navigation with links and expandable sections.",
        "Organize application destinations in a sidebar.",
        "Compose a responsive application frame with navigation and main content.",
        "Break a workflow into named steps and show the current stage.",
        "Keep overflowing content in an accessible scrolling region.",
        "Let users resize adjacent panels with a draggable splitter.",
        "Keep an action or supporting element at a chosen viewport position.",
        "Group related actions in a keyboard-navigable toolbar.",
        "Navigate a document through its registered headings.",
        "Move items that no longer fit into an anchored overflow panel.",
        "Animate an indicator between the currently selected elements.",
        "Display moving text and smoothly changing numeric values.",
    ],
    "P7": [
        "Enter numeric values with increment and decrement controls.",
        "Choose a value by moving a thumb along a track.",
        "Select the lower and upper ends of a numeric range.",
        "Choose an angle with a circular slider.",
        "Collect a rating or display a read-only score.",
        "Preview a color in a compact swatch.",
        "Adjust a color's hue or transparency along a slider.",
        "Choose and edit colors with a picker or text input.",
        "Enter a short verification code using connected input slots.",
        "Guide text entry with a structured input mask.",
        "Edit JSON text with explicit parsing and validation feedback.",
    ],
    "P8": [
        "Present structured information in rows, columns and named headers.",
        "Explore tabular data with sorting, selection and windowed rows.",
        "Browse hierarchical items with expandable branches.",
        "Choose a value from a searchable hierarchy.",
        "Choose a nested value by moving through successive columns.",
    ],
    "P9": [
        "Configure civil dates, calendar language and week conventions.",
        "Browse days in a calendar or a compact month view.",
        "Pick a date or date range from a calendar panel.",
        "Type a date with parsing feedback and an optional calendar.",
        "Choose a month with a compact year-based picker.",
        "Choose a year from a navigable grid.",
        "Enter a time and display it in the configured format.",
        "Choose a time from interactive clock fields and a time grid.",
        "Choose a date and time together in a panel or inline view.",
    ],
    "P10": [
        "Select local files through a named file input or action button.",
        "Drop files into a selection area and display their attachments.",
        "Copy explicit text to the clipboard with clear feedback.",
        "Browse slides with navigation controls and swipe gestures.",
        "Open images in an interactive viewer with zoom and navigation.",
        "Display source code with syntax highlighting and copy actions.",
        "Inspect JSON values in an expandable structured view.",
        "Edit a structured rich document with formatting controls.",
    ],
    "P11": [
        "Compose charts with coordinated colors, legends and accessible data.",
        "Compare trends and categories with area, line, bar and composite charts.",
        "Show proportions with pie, donut and radial bar charts.",
        "Explore relationships and ranges with scatter, bubble and candlestick charts.",
        "Compare dimensions, stages and compact trends with specialized charts.",
        "Show patterns and proportions in heatmap, matrix and waffle charts.",
        "Compare values and targets with ranked bars, bullets and gauges.",
        "Explore hierarchies and flows with treemap, sunburst and Sankey charts.",
    ],
    "P12": [
        "Describe scheduled events and their recurrence using civil dates and times.",
        "Arrange events along a daily or weekly timeline.",
        "Browse scheduled events in month, year and compact mobile views.",
        "Read upcoming events as a chronological agenda.",
        "Compare availability and events across multiple resources.",
    ],
}
DESCRIPTIONS = {
    f"{phase}-{number:02}": description
    for phase, descriptions in LEADS.items()
    for number, description in enumerate(descriptions, 1)
}
if set(DESCRIPTIONS) != set(records):
    raise SystemExit("reader introductions must describe every canonical work item")
DISPLAY_DESCRIPTIONS = {
    "MiniCalendar": "Choose a date from a compact strip with controlled ranges, bounds and localized labels.",
    "Heatmap": "Show dated contributions in week columns, with value domains, localized labels and current-date feedback.",
    "Center": "Center content horizontally and vertically, with an optional inline layout.",
    "Container": "Constrain content width and center it within the available space.",
    "Space": "Add explicit horizontal or vertical space between layout elements.",
    "Box": "Compose arbitrary content in a themed layout box with typed styles.",
    "Transition": "Animate controlled presence with fade, scale and slide effects, delayed lifecycle callbacks and explicit content ownership.",
}

def group(task):
    phase, number = task["id"].split("-")
    n = int(number)
    overrides = {
        "P0-04": "Miscellaneous", "P0-11": "Miscellaneous", "P0-13": "Miscellaneous",
        "P3-06": "Navigation", "P3-12": "Miscellaneous",
        "P6-07": "Miscellaneous", "P6-10": "Miscellaneous", "P6-12": "Data display",
        "P6-13": "Overlays", "P6-14": "Miscellaneous", "P7-06": "Data display",
        "P8-03": "Navigation", "P10-01": "Inputs", "P10-03": "Buttons",
    }
    if task["id"] in overrides:
        return overrides[task["id"]]
    if phase == "P0":
        return {1: "Buttons", 2: "Inputs", 3: "Inputs", 5: "Data display", 6: "Navigation", 7: "Overlays", 8: "Overlays", 9: "Overlays", 10: "Navigation", 12: "Extensions", 14: "Extensions"}[n]
    if phase == "P1": return "Inputs"
    if phase == "P2": return "Buttons" if n in (4, 7, 8, 9) else "Inputs"
    if phase == "P3":
        return "Layout" if n <= 4 else "Typography" if n in (5, 7, 8, 9, 16) else "Data display"
    if phase == "P4": return "Combobox" if 4 <= n <= 9 else "Navigation"
    if phase == "P5": return "Overlays" if n in (1, 2, 3, 4, 12, 13, 14, 15) else "Feedback"
    if phase == "P6": return "Layout" if n in (8, 9) else "Navigation"
    if phase == "P7": return "Inputs"
    if phase == "P8": return "Combobox" if n in (4, 5) else "Data display"
    if phase == "P10": return "Extensions"
    return {"P9": "Dates", "P11": "Charts", "P12": "Dates"}[phase]


def target(page):
    if page in SPECIAL:
        module, function, arguments, scoped = SPECIAL[page]
    else:
        module = page.replace("-", "_") + "_form"
        paths = [WEB / "src" / f"{module}.rs", WEB / f"{module}.rs"]
        path = next((p for p in paths if p.is_file()), None)
        if path is None: raise SystemExit(f"no Rust demo module for {page}")
        source = path.read_text(encoding="utf-8")
        scoped = bool(re.search(r"pub fn form\(\s*scope:\s*&Scope", source))
        function, arguments = ("form", "scope, theme.clone()") if scoped else (module, "theme")
    return module, function, arguments, scoped


def literal(text):
    # JSON strings are valid Rust UTF-8 string literals for these metadata values.
    return json.dumps(text, ensure_ascii=False)


# Each displayed source is an entire Rust module compiled into the preview.
# Keep adapter bootstraps and test-only drivers outside these widget modules.
def fixture_source(module):
    path = WEB / "src" / f"{module}.rs"
    text = path.read_text(encoding="utf-8")
    text = re.sub(r'(?ms)^pub fn (?:inert_stream_probe|sheet_stream_probe)\(\) -> impl Widget \{.*?\n\}\n', '', text)
    boundary = re.search(r'\n#\[cfg\((?:target_arch = "wasm32"|test)\)\]\nmod (?:browser|tests) \{', text)
    if boundary:
        text = text[:boundary.start()]
    # Shared plot data is included explicitly, rather than a file-path override.
    text = text.replace('#[path = "../../fixtures/plot_samples.rs"]\npub mod samples;',
                        'pub mod samples {\n' + (WEB.parent / "fixtures/plot_samples.rs").read_text(encoding="utf-8") + '\n}')
    # Media URLs are build inputs, but displayed demos must remain self-contained.
    if 'crate::assets::media(' in text:
        text = text.replace('crate::assets::media(', 'media(')
        helper = (WEB / 'src/assets.rs').read_text(encoding='utf-8')
        text += '\n' + '\n'.join(line for line in helper.splitlines() if not line.startswith('//!'))
    return text.strip()


def formatted(text):
    result = subprocess.run(["rustfmt", "--edition", "2024", "--config-path", str(ROOT / "rustfmt.toml"),
                             "--emit", "stdout"],
                            input=text, text=True, encoding="utf-8", capture_output=True, check=True)
    return result.stdout.replace("\r\n", "\n")


def timer_browser():
    browser = (WEB / "src/menu_form.rs").read_text(encoding="utf-8")
    browser = browser[browser.index('mod browser {'):browser.index('    thread_local!')]
    browser = browser.replace('    use super::*;\n', '').replace('    use glory::web::WebApp;\n', '')
    return '\n#[cfg(target_arch = "wasm32")]\n' + browser + '}\n'


def bundle(page):
    if page == "menubar":
        return formatted((WEB / "src/dedicated_demos/menubar.rs").read_text(encoding="utf-8"))
    if page == "shell":
        return formatted((WEB / f"src/showcase_demos/{page}.rs").read_text(encoding="utf-8"))
    module, function, arguments, scoped = target(page)
    body = fixture_source(module)
    dependencies = []
    # The only cross-fixture dependencies are a host timer and JSON demo.
    if module == "menu_form":
        body += timer_browser()
    if re.search(r'(?:crate|super)::menu_form::', body):
        timer = fixture_source("menu_form")
        timer = timer[timer.index('#[cfg(not(target_arch = "wasm32"))]'):]
        timer += timer_browser()
        dependencies.append('#[allow(dead_code)]\nmod menu_form {\n' + timer + '\n}')
        body = body.replace('crate::menu_form::', 'super::menu_form::')
    if 'crate::json_tree_example::' in body:
        dependencies.append('mod json_tree_example {\n' + fixture_source('json_tree_example') + '\n}')
        body = body.replace('crate::json_tree_example::', 'super::json_tree_example::')
    expression = f'fixture::{function}({arguments})'
    if scoped:
        expression = f'widget(move |scope| {expression}.0.show_in(scope))'
    return formatted('use glory::prelude::*;\n\npub fn demo(theme: sapid::Theme) -> impl Widget {\n    '
                     + expression + '\n}\n\n#[allow(dead_code, unused_imports)]\nmod fixture {\n'
                     + body + '\n}\n' + '\n'.join(dependencies))


def focused_bundle(identity, family):
    body = (WEB / f"src/focused_demos/{family}.rs").read_text(encoding="utf-8")
    controls = ''
    if 'super::controls' in body:
        controls = (WEB / "src/focused_demos/controls.rs").read_text(encoding="utf-8")
        if family == "layout_controls":
            # Fit the source panel's lexical budget by including the controls
            # the layout demos actually call.
            used = set(re.findall(r"super::controls::(\w+)\(", body))
            functions = re.findall(r"(?ms)^(pub fn (\w+)\(.*?^})", controls)
            imports = "use glory::prelude::*;\nuse glory::style::FlexWrap;\nuse sapid::Theme;\n\n"
            controls = imports + "\n\n".join(function for function, name in functions if name in used)
        controls = '\n\n#[allow(dead_code, unused_imports)]\nmod controls {\n' + controls + '\n}'
    return formatted('use glory::prelude::*;\n\npub fn demo(theme: sapid::Theme) -> impl Widget {\n    '
                     + f'focused::demo_for("{identity}", theme)\n'
                     + '}' + controls + '\n\n#[allow(dead_code, unused_imports)]\nmod focused {\n'
                     + body + '\n}\n')


MANTINE_FACTS = json.loads((ROOT / "docs/mantine-api-audit.json").read_text(encoding="utf-8"))
MANTINE_GROUPS = {
    "Layout": ["AppShell", "AspectRatio", "Center", "Container", "Flex", "Grid", "Group", "SimpleGrid", "Space", "Splitter", "Stack"],
    "Inputs": ["AlphaSlider", "AngleSlider", "Checkbox", "Chip", "ColorInput", "ColorPicker", "Fieldset", "FileInput", "HueSlider", "Input", "JsonInput", "MaskInput", "NativeSelect", "NumberInput", "PasswordInput", "PinInput", "Radio", "RangeSlider", "Rating", "SegmentedControl", "Slider", "Switch", "Textarea", "TextInput"],
    "Combobox": ["Autocomplete", "Cascader", "Combobox", "ComboboxPopover", "MultiSelect", "Pill", "PillsInput", "Select", "TagsInput", "TreeSelect"],
    "Buttons": ["ActionIcon", "Button", "CloseButton", "CopyButton", "FileButton", "Toggle", "UnstyledButton"],
    "Navigation": ["Anchor", "Breadcrumbs", "Burger", "NavLink", "Pagination", "Stepper", "TableOfContents", "Tabs", "Tree"],
    "Feedback": ["Alert", "EmptyState", "Loader", "Notification", "Progress", "RingProgress", "SemiCircleProgress", "Skeleton"],
    "Overlays": ["ActionBar", "Affix", "Dialog", "Drawer", "FloatingIndicator", "FloatingWindow", "HoverCard", "LoadingOverlay", "Menu", "Menubar", "Modal", "Overlay", "Popover", "Tooltip", "Tour"],
    "Data display": ["Accordion", "Avatar", "BackgroundImage", "Badge", "Card", "ColorSwatch", "DataList", "Image", "Indicator", "Kbd", "NumberFormatter", "OverflowList", "RollingNumber", "Spoiler", "ThemeIcon", "Timeline"],
    "Typography": ["Blockquote", "Code", "Highlight", "List", "Mark", "Table", "Text", "Title", "Typography"],
    "Miscellaneous": ["Box", "Collapse", "Divider", "FocusTrap", "Marquee", "Paper", "Portal", "ScrollArea", "Scroller", "Toolbar", "Transition", "VisuallyHidden"],
}
MANTINE_ORDER = {
    name: (group_index, item_index)
    for group_index, (group, names) in enumerate(MANTINE_GROUPS.items())
    for item_index, name in enumerate(names)
}
MANTINE_BY_TASK = {}
for item in MANTINE_FACTS["entries"]:
    MANTINE_BY_TASK.setdefault(item["task"], []).append(item)
NAMED_DEMO_TASKS = {task for task, items in MANTINE_BY_TASK.items() if len(items) > 1} | {
    "P3-07", "P3-13", "P4-05", "P4-06", "P4-07", "P6-06", "P8-01", "P8-04", "P8-05", "P10-06",
}
PRESERVE_PRIMARY = {"P0-01", "P2-08", "P3-10", "P4-09", "P5-11", "P6-07", "P12-01"}
SOURCE_GROUPS = {
    "mantine-charts": "Charts",
    "mantine-dates": "Dates",
    "mantine-schedule": "Dates",
    "mantine-code": "Extensions",
    "mantine-extensions": "Extensions",
}


def public_slug(name):
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1-\2", name)
    return re.sub(r"[^a-zA-Z0-9]+", "-", name).strip("-").lower()


def display_items(task):
    items = MANTINE_BY_TASK.get(task["id"], [])
    if not items:
        return []
    items = sorted(items, key=lambda item: MANTINE_ORDER.get(item["name"], (99, 99)))
    result = []
    for index, item in enumerate(items):
        legacy_suffix = re.sub(r"[^a-z0-9]+", "-", item["name"].lower()).strip("-")
        identity = task["id"] if index == 0 else f"{task['id']}-{legacy_suffix}"
        order = MANTINE_ORDER.get(item["name"], (99, 99))
        category = (list(MANTINE_GROUPS)[order[0]] if order[0] < len(MANTINE_GROUPS)
                    else SOURCE_GROUPS[item["source"]])
        result.append((identity, item["name"], category))
    return result


def named_source(body, name, family):
    chart = family == "charts"
    indent = 12 if chart else 8
    marker = "        return match id {" if chart else "    match name {"
    function_start = body.index("pub fn demo_for")
    start = body.index(marker, function_start)
    prefix = body[:function_start]
    pattern = re.compile(r"(?m)^" + " " * indent + r"\"[^\"]+\"(?:\s*\|\s*\"[^\"]+\")*\s*=>")
    arms = list(pattern.finditer(body, start))
    default = re.search(r"(?m)^" + " " * indent + r"_ => unreachable!", body[start:])
    if not arms or not default:
        raise SystemExit(f"cannot isolate named demo {name}")
    end = start + default.start()
    selected = []
    for index, arm in enumerate(arms):
        arm_end = arms[index + 1].start() if index + 1 < len(arms) else end
        labels = re.findall(r"\"([^\"]+)\"", arm[0].split("=>")[0])
        if name in labels:
            selected.append(body[arm.start():arm_end])
    if not selected:
        raise SystemExit(f"no named demo branch for {name}")
    selected_source = "".join(selected)
    declarations = list(re.finditer(
        r"(?m)^(?:pub\s+)?(?:fn|const|static)\s+([A-Za-z_][A-Za-z0-9_]*)", prefix
    ))
    if declarations:
        header = prefix[:declarations[0].start()]
        items = []
        for index, declaration in enumerate(declarations):
            item_end = declarations[index + 1].start() if index + 1 < len(declarations) else len(prefix)
            items.append((declaration[1], prefix[declaration.start():item_end]))
        required = set()
        pending = selected_source
        if chart and "proportions()" in selected_source:
            pending += "values()"
        while True:
            added = False
            for identifier, item in items:
                if identifier not in required and re.search(rf"\b{re.escape(identifier)}\b", pending):
                    required.add(identifier)
                    pending += item
                    added = True
            if not added:
                break
        prefix = header + "".join(item for identifier, item in items if identifier in required)
    variable = "id" if chart else "name"
    signature = "id: &str, theme: sapid::Theme" if chart else "name: &str, theme: Theme"
    proportions = "    let proportions = || values(&[(\"a\", \"Product A\", 42.), (\"b\", \"Product B\", 31.), (\"c\", \"Product C\", 18.), (\"other\", \"Other\", 9.)]);\n" if chart and "proportions()" in selected_source else ""
    return (prefix + f"pub fn demo_for({signature}) -> Box<dyn Widget> {{\n" + proportions
            + f"    match {variable} {{\n" + selected_source
            + " " * indent + f"_ => unreachable!(\"unsupported named demo: {{{variable}}}\"),\n    }}\n}}\n")


def named_bundle(name, task):
    family = "charts" if task.startswith("P11-") else "mantine_pages"
    body = (WEB / f"src/focused_demos/{family}.rs").read_text(encoding="utf-8")
    body = named_source(body, name, family)
    controls = ''
    if 'super::controls' in body:
        controls = (WEB / "src/focused_demos/controls.rs").read_text(encoding="utf-8")
        controls = '\n\n#[allow(dead_code, unused_imports)]\nmod controls {\n' + controls + '\n}\n'
    return formatted('use glory::prelude::*;\n\npub fn demo(theme: sapid::Theme) -> impl Widget {\n    '
                     + f'focused::demo_for({literal(name)}, theme)\n'
                     + '}' + controls + '\n\n#[allow(dead_code, unused_imports)]\nmod focused {\n'
                     + body + '\n}\n')


def api_sources(task):
    paths = []
    for value in task["evidence"]["implementation"]:
        path = value.split(" (")[0]
        if path.endswith(".rs") and (ROOT / path).is_file() and not path.startswith("examples/"):
            paths.append(path)
    return sorted(set(paths))


API_INFRASTRUCTURE = {
    "blue_light", "blue_dark", "green_light", "green_dark", "light", "dark", "new", "validate",
    "from_default", "from_palette", "palette", "with_tokens", "stylesheet", "typography", "colors",
    "build", "build_and_mount", "as_str", "is_finite", "resolve", "validate_input_placeholder",
}
API_TYPES = {
    "InlineDateTimePicker": ["InlineDateTimePicker", "InlineDateTimeRangePicker", "DateTimeBounds", "DateTimeRangeValue", "DateTimePolicy"],
    "DateInput": ["DateInput", "DateInputFormat", "DateParse", "DateDraftStatus"],
    "DatePicker": ["DatePicker", "DatePickerMode", "DatePickerLevel", "DateSelection", "DatePickerPreset"],
    "Calendar": ["Calendar", "DatesProvider", "DateLocale", "Weekday"],
    "TimeInput": ["TimeInput", "TimeFormat", "TimeBounds", "TimeDraftStatus"],
    "TimePicker": ["TimePicker", "DurationPicker", "TimePickerPresetGroup", "TimePickerPresets", "TimePickerStatus", "TimeFormat", "TimeDuration"],
    "TimeValue": ["TimeValue", "TimeFormat", "TimePrecision", "HourCycle"],

    "Portal": ["Portal", "PortalContext", "PortalTarget"],
    "ScrollArea": ["ScrollArea", "ScrollHandle", "ScrollContext", "Scrollbars", "ScrollbarOffset", "OverscrollBehavior"],
    "Toolbar": ["Toolbar", "ToolbarGroup"],
    "Avatar": ["Avatar", "AvatarGroup", "AvatarShape"], "Badge": ["Badge", "BadgeVariant", "BadgeSection"],
    "DataList": ["DataList", "DataListItem", "DataListOrientation"], "Indicator": ["Indicator", "IndicatorPosition"],
    "Kbd": ["Kbd", "KbdKey", "KbdPlatform"], "OverflowList": ["OverflowList", "OverflowItem", "OverflowHandle"],
    "NumberFormatter": ["NumberFormatter", "NumberLocale", "NumberFormat", "Currency"], "ThemeIcon": ["ThemeIcon", "Icon", "ThemeIconVariant"],
    "Grid": ["Grid", "GridItem"],
    "SimpleGrid": ["SimpleGrid", "GridItem"],
    "AppShell": ["AppShell", "Sidebar", "ShellSection"],
    "Menu": ["Menu", "DropdownMenu", "MenuItem", "MenuLabel", "MenuSearch", "MenuSubmenu", "MenuRadioGroup", "MenuRadioItem"],
    "Menubar": ["Menubar", "MenubarMenu", "MenuItem", "MenuLabel", "MenuSearch", "MenuSubmenu", "MenuRadioGroup", "MenuRadioItem"],
    "TextInput": ["TextField"], "Input": ["Field"],
    "Collapse": ["Collapsible"], "Modal": ["Dialog"], "FocusTrap": ["FocusScope"],
    "List": ["TextList", "TextListItem"], "Divider": ["Separator", "SeparatorVariant", "SeparatorLabelPosition"], "Spotlight": ["Command"],
    "Typography": ["Typography", "TypographyContext"],
    "Table": ["Table", "TableSection", "TableRow", "TableCell", "TableCaptionSide", "TableStripe"],
    "Loader": ["Spinner", "LoaderType"], "Dialog": ["FloatingDialog"], "Modals manager": ["ModalsManager"],
    "Dropzone": ["FileInput"], "Rich text editor": ["RichTextEditor"],
    "MiniCalendar": ["MiniCalendar"],
    "Heatmap": ["Heatmap", "HeatmapData", "HeatmapRange", "HeatmapDomain"],
    "Transition": ["Transition"],
    "MonthPicker": ["PeriodPicker", "PeriodSelection", "DatePickerMode", "YearMonth"], "YearPicker": ["PeriodPicker", "PeriodSelection", "DatePickerMode", "CivilYear"],
    "MonthPickerInput": ["PeriodPickerInput"], "YearPickerInput": ["PeriodPickerInput"],
    "RingProgress": ["Progress"], "SemiCircleProgress": ["Progress"],
    "Chart / ChartTheme / Legend / Tooltip": ["Chart"],
    "ComboboxPopover": ["Select", "SelectTriggerContext"],
    "UnstyledButton": ["Button"],
    "Notifications system": ["Toaster"],
    "DateTimePicker": ["DateTimePicker", "DateTimeRangePicker"],
    "InlineDateTimePicker": ["InlineDateTimePicker", "InlineDateTimeRangePicker"],
    "DayView": ["Schedule"], "WeekView": ["Schedule"], "MonthView": ["Schedule"],
    "YearView": ["Schedule"], "MobileMonthView": ["Schedule"], "AgendaView": ["Schedule"],
    "Schedule": ["Schedule"],
    "ResourcesDayView": ["ResourceSchedule"], "ResourcesWeekView": ["ResourceSchedule"],
    "ResourcesMonthView": ["ResourceSchedule"], "ResourcesSchedule": ["ResourceSchedule"],
    "Recurring events": ["ScheduleRecurrence"], "Events data": ["ScheduleEvent"],
    **{name: ["Chart", "ChartData", "ChartSeries", "ChartFrame", "CartesianOptions", "ChartDomain", "ChartBrush", "ChartReferenceLine", "ChartReferenceArea", "ChartReferenceDot", "ChartSelection"] for name in ["AreaChart", "BarChart", "LineChart", "CompositeChart"]},
    **{name: ["Plot", "PlotData", "PlotOptions", "PlotAngles", "WaffleGrid", "BubbleRadius", "ChartError", "ChartFrame", "ValueDatum", "PointDatum", "CandleDatum", "TargetDatum", "HierarchyDatum", "FlowData", "MatrixData"] for name in [
        "DonutChart", "PieChart", "RadialBarChart", "CandlestickChart", "ScatterChart", "BubbleChart",
        "FunnelChart", "RadarChart", "Sparkline", "WaffleChart", "MatrixChart", "BarsList",
        "BulletChart", "GaugeChart", "Treemap", "SunburstChart", "SankeyChart",
    ]},
    **{'Schedule': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'DayView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'WeekView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'MonthView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'YearView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'MobileMonthView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'AgendaView': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'ResourcesDayView': ['ResourceSchedule', 'ResourceScheduleView', 'ResourceScheduleRequest', 'ResourceScheduleData', 'ScheduleResources', 'ScheduleResource', 'ScheduleResourceWindow', 'ResourceAxis', 'ScheduleSelection', 'ScheduleEdit', 'SchedulePermissions'], 'ResourcesWeekView': ['ResourceSchedule', 'ResourceScheduleView', 'ResourceScheduleRequest', 'ResourceScheduleData', 'ScheduleResources', 'ScheduleResource', 'ScheduleResourceWindow', 'ResourceAxis', 'ScheduleSelection', 'ScheduleEdit', 'SchedulePermissions'], 'ResourcesMonthView': ['ResourceSchedule', 'ResourceScheduleView', 'ResourceScheduleRequest', 'ResourceScheduleData', 'ScheduleResources', 'ScheduleResource', 'ScheduleResourceWindow', 'ResourceAxis', 'ScheduleSelection', 'ScheduleEdit', 'SchedulePermissions'], 'ResourcesSchedule': ['ResourceSchedule', 'ResourceScheduleView', 'ResourceScheduleRequest', 'ResourceScheduleData', 'ScheduleResources', 'ScheduleResource', 'ScheduleResourceWindow', 'ResourceAxis', 'ScheduleSelection', 'ScheduleEdit', 'SchedulePermissions'], 'Recurring events': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError'], 'Events data': ['Schedule', 'ScheduleView', 'ScheduleData', 'ScheduleWindow', 'ScheduleInterval', 'ScheduleEvent', 'ScheduleSelection', 'ScheduleRequest', 'ScheduleRecurrence', 'RecurrenceFrequency', 'RecurrenceEnd', 'MissingScheduleDay', 'ScheduleContext', 'ScheduleZone', 'ScheduleError']},
}
API_PATHS = {
    **{name: ['src/schedule/mod.rs', 'src/schedule/resources/mod.rs', 'src/schedule_model.rs', 'src/schedule_recurrence.rs'] for name in ['Schedule', 'DayView', 'WeekView', 'MonthView', 'YearView', 'MobileMonthView', 'AgendaView', 'ResourcesDayView', 'ResourcesWeekView', 'ResourcesMonthView', 'ResourcesSchedule', 'Recurring events', 'Events data']},
    **{name: ['src/chart_widget.rs', 'src/chart_scene.rs', 'src/chart_model.rs'] for name in ['AreaChart', 'BarChart', 'LineChart', 'CompositeChart']},
    **{name: ['src/plot_widget.rs', 'src/plot_scene.rs', 'src/plot_model.rs', 'src/chart_scene.rs', 'src/chart_model.rs'] for name in ['DonutChart', 'PieChart', 'RadialBarChart', 'CandlestickChart', 'ScatterChart', 'BubbleChart', 'FunnelChart', 'RadarChart', 'Sparkline', 'WaffleChart', 'MatrixChart', 'BarsList', 'BulletChart', 'GaugeChart', 'Treemap', 'SunburstChart', 'SankeyChart']},
    "InlineDateTimePicker": ["src/date_time_picker.rs", "src/date_time_range_picker.rs", "src/date_time_model.rs"],
    "DateInput": ["src/date_input.rs", "src/date_draft.rs"],
    "DatePicker": ["src/date_picker.rs", "src/date_selection.rs", "src/dates.rs"],
    "Calendar": ["src/calendar.rs", "src/dates.rs"],
    "MonthPicker": ["src/period_picker.rs", "src/period_model.rs", "src/dates.rs"],
    "YearPicker": ["src/period_picker.rs", "src/period_model.rs", "src/dates.rs"],
    "TimeInput": ["src/time_input.rs", "src/time_model.rs"],
    "TimePicker": ["src/time_picker.rs", "src/time_parts.rs", "src/time_model.rs", "src/duration_picker.rs"],
    "TimeValue": ["src/time_value.rs", "src/time_model.rs"],

    "ComboboxPopover": ["src/select.rs"],
    "Switch": ["src/switch/mod.rs", "src/switch/visual.rs"],
    "AppShell": ["src/app_shell.rs"],
    "PasswordInput": ["src/password_input.rs", "src/text_input.rs"],
    "Transition": ["src/transition.rs"],
    "Heatmap": ["src/heatmap.rs", "src/heatmap_model.rs"],
    "MiniCalendar": ["src/mini_calendar.rs"],
    "Menu": ["src/menu/mod.rs", "src/menu/content.rs", "src/menu/search.rs"],
    "Menubar": ["src/menu/bar/mod.rs", "src/menu/mod.rs", "src/menu/content.rs", "src/menu/search.rs"],
    "ColorInput": ["src/color_input.rs"],
    "ColorPicker": ["src/color_picker/mod.rs"],
    "Pill": ["src/display.rs"], "Badge": ["src/display.rs"],
    "PillsInput": ["src/pills_input.rs"], "TagsInput": ["src/tags_input.rs"],
    "Button": ["src/button.rs"], "UnstyledButton": ["src/button.rs"],
    "Card": ["src/display.rs"], "Paper": ["src/display.rs"],
    "Code": ["src/document.rs"], "Highlight": ["src/document.rs"], "Mark": ["src/document.rs"],
    "Text": ["src/typography.rs"], "Title": ["src/typography.rs"], "Typography": ["src/typography.rs"],
    "FocusTrap": ["src/focus_scope.rs"], "Portal": ["src/portal.rs"],
    "RollingNumber": ["src/rolling_number.rs"], "Marquee": ["src/marquee.rs"],
    "DateTimePicker": ["src/date_time_picker_input.rs", "src/date_time_range_picker.rs"],
    "InlineDateTimePicker": ["src/date_time_picker.rs", "src/date_time_range_picker.rs"],
    "DayView": ["src/schedule/mod.rs"], "WeekView": ["src/schedule/mod.rs"],
    "MonthView": ["src/schedule/mod.rs"], "YearView": ["src/schedule/mod.rs"],
    "MobileMonthView": ["src/schedule/mod.rs"], "AgendaView": ["src/schedule/mod.rs"],
    "Schedule": ["src/schedule/mod.rs"],
    "ResourcesDayView": ["src/schedule/resources/mod.rs"],
    "ResourcesWeekView": ["src/schedule/resources/mod.rs"],
    "ResourcesMonthView": ["src/schedule/resources/mod.rs"],
    "ResourcesSchedule": ["src/schedule/resources/mod.rs"],
    "Recurring events": ["src/schedule_recurrence.rs"], "Events data": ["src/schedule_model.rs"],
    **{name: ["src/chart_widget.rs"] for name in ["AreaChart", "BarChart", "LineChart", "CompositeChart"]},
    **{name: ["src/plot_widget.rs"] for name in [
        "DonutChart", "PieChart", "RadialBarChart", "CandlestickChart", "ScatterChart", "BubbleChart",
        "FunnelChart", "RadarChart", "Sparkline", "WaffleChart", "MatrixChart", "BarsList",
        "BulletChart", "GaugeChart", "Treemap", "SunburstChart", "SankeyChart",
    ]},
}
LAYOUT_NAMES = {"Box", "Flex", "Group", "Stack", "Center", "Container", "Space"}


API_TYPES.update({'CodeHighlight': ['CodeHighlight', 'CodeBlock', 'CodeLanguage', 'CopyButton', 'CopyState'], 'JsonViewer': ['JsonViewer', 'JsonViewLimits', 'JsonViewError', 'CopyButton', 'CopyState'], 'Notifications system': ['Toaster', 'NotificationsHandle', 'NotificationRequest', 'NotificationToken', 'NotificationClock', 'Notification', 'NotificationCloseReason'], 'Spotlight': ['Command', 'CommandHandle', 'CommandAction', 'CommandGroup', 'CommandResponse', 'CommandResults'], 'Lightbox': ['Lightbox', 'LightboxView', 'LightboxImage', 'LightboxZoom'], 'NavigationProgress': ['NavigationProgress', 'NavigationProgressHandle', 'NavigationProgressToken'], 'Modals manager': ['ModalsManager', 'ModalsHandle', 'ModalRequest', 'ModalToken', 'ModalOutcome']})
API_PATHS.update({'CodeHighlight': ['src/code_block.rs', 'src/code_model.rs', 'src/copy_button.rs'], 'JsonViewer': ['src/json_viewer/mod.rs', 'src/json_tree_model.rs', 'src/copy_button.rs'], 'Notifications system': ['src/notifications.rs'], 'Spotlight': ['src/command/mod.rs'], 'Lightbox': ['src/lightbox.rs'], 'NavigationProgress': ['src/navigation_progress.rs'], 'Modals manager': ['src/modals.rs']})

def rust_signatures(text):
    signatures = set()
    for match in re.finditer(r"pub fn ([a-z_0-9]+)(?:<[^>{}]*>)?\s*\(", text):
        if match[1] in API_INFRASTRUCTURE:
            continue
        segment = text[match.start():]
        boundary = segment.find("{")
        if boundary < 0:
            continue
        signature = re.sub(r"\s+", " ", segment[:boundary].strip()).replace("$crate::", "crate::")
        if "-> Self" in signature or re.search(r"->\s*[A-Z][A-Za-z0-9_]*$", signature):
            signatures.add(signature)
    return signatures


def braced_block(text, start):
    opening = text.find("{", start)
    if opening < 0:
        return ""
    depth = 0
    for index in range(opening, len(text)):
        if text[index] == "{": depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0: return text[opening:index + 1]
    return ""


def macro_definition(text, name):
    match = re.search(r"macro_rules!\s+" + re.escape(name) + r"\s*\{", text)
    return braced_block(text, match.start()) if match else ""


def implementation_surface(text, widget_type):
    pattern = re.compile(r"\bimpl(?:\s*<[^>{}]*>)?\s+" + re.escape(widget_type)
                         + r"(?:\s*<[^>{}]*>)?\s*\{")
    blocks = [braced_block(text, match.start()) for match in pattern.finditer(text)]
    generated = []
    for macro in ["overlay", "inline_text", "methods"]:
        if re.search(r"\b" + macro + r"!\s*\(\s*" + re.escape(widget_type) + r"\s*[,)]", text):
            generated.append(macro_definition(text, macro))
    return "\n".join(block for block in blocks + generated if block)


def expand_macros(body, text):
    support = {"common_methods": ROOT / "src/appearance.rs", "text_input_methods": ROOT / "src/text_input.rs"}
    result = [body]
    # Forwarded builder methods are compiled public API, not missing props.
    for match in re.finditer(r"\bforward!\s*\(\s*([a-z_0-9]+)\s*,\s*([^;]+?)\)\s*;", body):
        result.append(f"pub fn {match[1]}(mut self, value: {match[2].strip()}) -> Self {{}}")
    for match in re.finditer(r"\bforward!\s*\(", body):
        start = match.end()
        depth = 1
        end = start
        while end < len(body) and depth:
            if body[end] == "(": depth += 1
            elif body[end] == ")": depth -= 1
            end += 1
        block = body[start:end - 1]
        for method in re.finditer(r"([a-z_0-9]+)\s*\(\s*([a-z_0-9]+)\s*:\s*([^()]+)\)", block):
            result.append(f"pub fn {method[1]}(mut self, {method[2]}: {method[3].strip()}) -> Self {{}}")
    # Delegating menu methods are real public methods on the configured wrapper.
    for match in re.finditer(r"\bdropdown_methods!\s*\{", body):
        block = braced_block(body, match.start())
        for declaration in block[1:-1].split(";"):
            method = re.fullmatch(r"\s*([a-z_0-9]+)\s*\((.*)\)\s*", declaration, re.S)
            if method:
                result.append(f"pub fn {method[1]}(mut self, {method[2]}) -> Self {{}}")
    for name in set(re.findall(r"\b([a-z_0-9]+)!\s*\(\s*\)", body)):
        definition = macro_definition(text, name)
        if not definition and name in support:
            definition = macro_definition(support[name].read_text(encoding="utf-8"), name)
        if definition: result.append(definition)
    for match in re.finditer(
        r"\bsource_method!\s*\(\s*([a-z_0-9]+)\s*,\s*[a-z_0-9]+\s*,\s*([^\)]+?)\s*\)",
        body,
    ):
        method, value_type = match.groups()
        value_type = re.sub(r"\s+", " ", value_type.strip())
        result.append(
            f"pub fn {method}(mut self, value: impl Into<Vary<{value_type}>>) -> Self {{"
        )
    return "\n".join(result)


def api_surface(task, item):
    name = item["name"]
    paths = API_PATHS.get(name, api_sources(task))
    types = API_TYPES.get(name, [name])
    builders, classes, signatures = set(), set(), set()
    factory = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower().replace(" ", "_")
    for path in paths:
        text = (ROOT / path).read_text(encoding="utf-8")
        if re.search(r"pub fn " + re.escape(factory) + r"\s*\(", text): builders.add(factory)
        if name in LAYOUT_NAMES:
            body = macro_definition(text, "methods")
            if name in {"Flex", "Group", "Stack"}: body += macro_definition(text, "flex_methods")
            body += implementation_surface(text, name)
        else:
            body = "\n".join(implementation_surface(text, widget_type) for widget_type in types)
            body = expand_macros(body, text)
        signatures.update(rust_signatures(body))
        classes.update(re.findall(r"\b(sapid-[a-z0-9-]+)", body))
        for widget_type in types:
            type_factory = re.sub(r"(?<!^)(?=[A-Z])", "_", widget_type).lower()
            if re.search(r"pub fn " + re.escape(type_factory) + r"\s*\(", text):
                builders.add(type_factory)
            base = re.sub(r"(?<!^)(?=[A-Z])", "-", widget_type).lower()
            classes.update(value for value in re.findall(r"\b(sapid-[a-z0-9-]+)", text)
                           if value == f"sapid-{base}" or value.startswith(f"sapid-{base}-"))
    builders.update(match[1] for signature in signatures
                    if (match := re.match(r"pub fn ([a-z_0-9]+)", signature)))
    return sorted(builders), sorted(classes), sorted(signatures)


def api_entry(task, item, display_id):
    builders, classes, signatures = api_surface(task, item)
    builder_values = ", ".join(literal(value) for value in builders)
    signature_values = ", ".join(literal(value) for value in signatures)
    class_values = ", ".join(literal(value) for value in classes)
    return (
        f"    ApiEntry {{ id: {literal(display_id)}, builders: &[{builder_values}], "
        f"signatures: &[{signature_values}], classes: &[{class_values}] }},"
    )


def api_catalog(tasks):
    task_map = {task["id"]: task for task in tasks}
    body = "\n".join(api_entry(task_map[item["task"]], item, display_id)
                     for task in tasks if task["id"] in MANTINE_BY_TASK
                     for (display_id, _, _), item in zip(display_items(task), sorted(
                         MANTINE_BY_TASK[task["id"]], key=lambda value: MANTINE_ORDER.get(value["name"], (99, 99)))))
    return """//! Generated Rust builder API metadata; edit sources and run showcase-catalog.py.
#[derive(Clone, Copy, Debug)]
pub struct ApiEntry {
    pub id: &'static str,
    pub builders: &'static [&'static str],
    pub signatures: &'static [&'static str],
    pub classes: &'static [&'static str],
}
#[rustfmt::skip]
pub const ENTRIES: &[ApiEntry] = &[
""" + body + """
];

pub fn find(id: &str) -> Option<&'static ApiEntry> {
    ENTRIES.iter().find(|entry| entry.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn button_has_real_rust_builder_api() {
        let entry = find("P0-01").unwrap();
        assert!(entry.builders.contains(&"button"));
        assert!(entry.builders.contains(&"left_section"));
        assert!(entry.signatures.iter().any(|signature| signature.contains("compact")));
        assert!(!entry.builders.contains(&"blue_dark"));
        assert!(entry.classes.contains(&"sapid-button"));
        assert!(find("unknown").is_none());
        assert!(!find("P7-08").unwrap().builders.contains(&"plane_size"));
        assert!(find("P7-08-colorpicker").unwrap().builders.contains(&"plane_size"));
        let range = find("P7-03").unwrap();
        for method in ["min", "max", "step", "minimum_gap", "labels", "value_text"] {
            assert!(range.builders.contains(&method), "missing macro builder {method}");
        }
    }
}
"""


pages = {}
entries = []
outputs = {}
modules = {}
for task in tasks:
    identity = task["id"]
    if identity not in MANTINE_BY_TASK:
        continue
    page = EXTRA_PAGES.get(identity) or next(iter(records[identity]["pages"]), None)
    if not page: raise SystemExit(f"missing explicit demo for {identity}")
    shared_code = None
    if identity not in NAMED_DEMO_TASKS or identity in PRESERVE_PRIMARY:
        if identity in {"P0-01", "P0-02", "P0-03", "P0-04", "P0-07", "P0-08", "P0-09", "P2-08", "P3-10", "P4-08", "P4-09"}:
            # A replaced core demo is no longer emitted. Read its maintained
            # source so the next generation does not depend on a deleted output.
            maintained = {"P0-01": "button", "P0-02": "text_input", "P0-03": "checkbox", "P0-04": "collapse", "P0-07": "modal", "P0-08": "drawer", "P0-09": "popover", "P2-08": "action_icon", "P3-10": "pill", "P4-08": "multi_select", "P4-09": "pills_input"}[identity]
            shared_code = (maintained + "_usage", formatted((WEB / f"src/dedicated_demos/{maintained}.rs").read_text(encoding="utf-8")))
        elif identity in CORE:
            key = 'core_' + CORE[identity]
            code = (WEB / f"src/showcase_demos/{key}.rs").read_text(encoding="utf-8")
            code = code.split('\n#[allow(dead_code, unused_imports)]\nmod controls {', 1)[0].rstrip()
            if 'controls::' in code:
                controls = (WEB / "src/focused_demos/controls.rs").read_text(encoding="utf-8")
                code += '\n\n#[allow(dead_code, unused_imports)]\nmod controls {\n' + controls + '\n}\n'
            shared_code = (key, formatted(code))
        elif identity in FOCUSED:
            shared_code = (FOCUSED_NAMES[identity], focused_bundle(identity, FOCUSED[identity]))
        else:
            key = page.replace('-', '_')
            pages[page] = key
            shared_code = (key, bundle(page))
    description = DESCRIPTIONS[identity]
    for display_id, display_name, display_group in display_items(task):
        input_examples = {"Select": "select", "DatePickerInput": "date_picker_input", "DateTimePicker": "date_time_picker", "MonthPickerInput": "month_picker_input", "YearPickerInput": "year_picker_input", "FloatingWindow": "floating_window", "Carousel": "carousel", "Dropzone": "dropzone", "Menu": "menu", "Menubar": "menubar", "MiniCalendar": "mini_calendar", "Heatmap": "heatmap", "Transition": "transition", "PasswordInput": "password_input"}
        input_examples.update({name: key for name, key in [
            ("AlphaSlider", "alpha_slider"), ("HueSlider", "hue_slider"), ("AngleSlider", "angle_slider"),
            ("ColorInput", "color_input"), ("ColorPicker", "color_picker"),
            ("Checkbox", "checkbox"), ("Radio", "radio"), ("Chip", "chip"), ("Fieldset", "fieldset"),
            ("Slider", "slider"), ("RangeSlider", "range_slider"), ("Rating", "rating"), ("SegmentedControl", "segmented_control"), ("Switch", "switch"),
            ("Input", "input"), ("TextInput", "text_input"), ("Textarea", "textarea"),
            ("JsonInput", "json_input"), ("NumberInput", "number_input"),
            ("MaskInput", "mask_input"), ("NativeSelect", "native_select"), ("FileInput", "file_input"),
            ("Autocomplete", "autocomplete"), ("Combobox", "combobox"), ("ComboboxPopover", "combobox_popover"), ("MultiSelect", "multi_select"), ("Pill", "pill"), ("PillsInput", "pills_input"), ("TagsInput", "tags_input"), ("TreeSelect", "tree_select"), ("Cascader", "cascader"),
            ("Avatar", "avatar"), ("Badge", "badge"), ("Card", "card"), ("ColorSwatch", "color_swatch"), ("DataList", "data_list"), ("Image", "image"), ("Indicator", "indicator"), ("Kbd", "kbd"), ("NumberFormatter", "number_formatter"), ("OverflowList", "overflow_list"), ("RollingNumber", "rolling_number"), ("Spoiler", "spoiler"), ("ThemeIcon", "theme_icon"),
            ("AreaChart", "area_chart"), ("BarChart", "bar_chart"), ("LineChart", "line_chart"), ("CompositeChart", "composite_chart"), ("CandlestickChart", "candlestick_chart"), ("DonutChart", "donut_chart"), ("PieChart", "pie_chart"), ("FunnelChart", "funnel_chart"), ("RadarChart", "radar_chart"), ("ScatterChart", "scatter_chart"), ("BubbleChart", "bubble_chart"), ("RadialBarChart", "radial_bar_chart"), ("Sparkline", "sparkline"), ("BarsList", "bars_list"), ("BulletChart", "bullet_chart"), ("GaugeChart", "gauge_chart"), ("WaffleChart", "waffle_chart"), ("Treemap", "treemap"), ("SunburstChart", "sunburst_chart"), ("SankeyChart", "sankey_chart"), ("MatrixChart", "matrix_chart"),
            ("Schedule", "schedule"), ("DayView", "day_view"), ("WeekView", "week_view"), ("MonthView", "month_view"), ("YearView", "year_view"), ("MobileMonthView", "mobile_month_view"), ("AgendaView", "agenda_view"), ("ResourcesDayView", "resources_day_view"), ("ResourcesWeekView", "resources_week_view"), ("ResourcesMonthView", "resources_month_view"), ("ResourcesSchedule", "resources_schedule"), ("Recurring events", "recurring_events"), ("Events data", "events_data"),
            ("CodeHighlight","code_highlight"), ("JsonViewer","json_viewer"), ("Notifications system","notifications_system"), ("Spotlight","spotlight"), ("Lightbox","lightbox"), ("NavigationProgress","navigation_progress"), ("Modals manager","modals_manager"),
            ("DatePicker", "date_picker"), ("InlineDateTimePicker", "inline_date_time_picker"), ("DateInput", "date_input"), ("MonthPicker", "month_picker"), ("YearPicker", "year_picker"), ("TimeInput", "time_input"), ("TimePicker", "time_picker"), ("TimeValue", "time_value"), ("Calendar", "calendar"),
            ("Box", "box"), ("Collapse", "collapse"), ("Divider", "divider"), ("FocusTrap", "focus_trap"), ("Marquee", "marquee"), ("Paper", "paper"), ("Portal", "portal"), ("ScrollArea", "scroll_area"), ("Toolbar", "toolbar"), ("VisuallyHidden", "visually_hidden"),
            ("Blockquote", "blockquote"), ("Code", "code"), ("Highlight", "highlight"), ("List", "list"), ("Mark", "mark"), ("Table", "table"), ("Text", "text"), ("Title", "title"), ("Typography", "typography"),
            ("Stepper", "stepper"), ("TableOfContents", "table_of_contents"), ("Tree", "tree"),
            ("ActionBar", "action_bar"), ("Dialog", "floating_dialog"), ("Modal", "modal"), ("Drawer", "drawer"),
            ("FloatingIndicator", "floating_indicator"), ("HoverCard", "hover_card"), ("LoadingOverlay", "loading_overlay"),
            ("Overlay", "overlay"), ("Popover", "popover"), ("Tooltip", "tooltip"), ("Tour", "tour"),
            ("Alert", "alert"), ("EmptyState", "empty_state"), ("Loader", "loader"), ("Notification", "notification"), ("Skeleton", "skeleton"), ("Progress", "progress"), ("RingProgress", "ring_progress"), ("SemiCircleProgress", "semi_circle_progress"),
            ("ActionIcon", "action_icon"), ("Button", "button"), ("CloseButton", "close_button"),
            ("CopyButton", "copy_button"), ("FileButton", "file_button"), ("Toggle", "toggle"), ("UnstyledButton", "unstyled_button"),
            ("AspectRatio", "aspect_ratio"), ("Center", "center"), ("Container", "container"),
            ("Flex", "flex"), ("Grid", "grid"), ("Group", "group"), ("SimpleGrid", "simple_grid"),
            ("Space", "space"), ("Stack", "stack"), ("Splitter", "splitter"),
        ]})
        if display_name in input_examples:
            source_key = input_examples[display_name]
            key = source_key + "_usage"
            code = formatted((WEB / f"src/dedicated_demos/{source_key}.rs").read_text(encoding="utf-8"))
        elif display_name in {"Anchor", "Burger", "NavLink", "PinInput", "Scroller", "Rich text editor", "Affix", "BackgroundImage", "TimeGrid"}:
            key = {"Anchor": "anchor", "Burger": "burger", "NavLink": "nav_link", "PinInput": "pin_input", "Scroller": "scroller", "Rich text editor": "rich_text_editor", "Affix":"affix", "BackgroundImage":"background_image", "TimeGrid":"time_grid"}[display_name]
            code = formatted((WEB / f"src/dedicated_demos/{key}.rs").read_text(encoding="utf-8"))
        elif display_id in FOCUSED_DISPLAY:
            key, family = FOCUSED_DISPLAY[display_id]
            code = focused_bundle(display_id, family)
        elif shared_code is None or (identity in PRESERVE_PRIMARY and display_id != identity):
            slug = re.sub(r"[^a-z0-9]+", "_", display_name.lower()).strip("_")
            key = "focused_" + slug
            code = named_bundle(display_name, identity)
        else:
            key, code = shared_code
        if len(code.encode()) > 65_536: raise SystemExit(f"source exceeds CodeHighlight budget: {display_id}")
        outputs[WEB / f"src/showcase_demos/{key}.rs"] = code
        modules[key] = code
        slug = public_slug(display_name)
        values = [("id", display_id), ("slug", slug), ("task", identity), ("name", display_name), ("group", display_group),
                  ("description", DISPLAY_DESCRIPTIONS.get(display_name, description)), ("example", page),
                  ("web", records[identity]["web"]["state"]),
                  ("native", records[identity]["native"]["state"]),
                  ("native_note", {"Transition": "Apace supports clock-driven fade/none and custom opacity; animated transforms and Wry presence are explicitly rejected. Engine pixels are separate from OS input/readers.", "MiniCalendar": "Independent 1–31 day strip; native four-theme geometry, keyboard, bounds, RTL and stable day buttons tested.", "Heatmap": "Native dated-grid geometry, pixels, focus and retained identities tested; with_tooltip(true) is explicitly rejected."}.get(display_name, records[identity]["native"]["notes"]))]
        entries.append((display_group, display_name, display_id, "    Entry { " + ", ".join(f"{key}: {literal(value)}" for key, value in values)
                       + f', code: include_str!("showcase_demos/{key}.rs"), build: crate::showcase_demos::{key}_widget' + " },"))

GROUP_ORDER = list(MANTINE_GROUPS) + ["Charts", "Dates", "Extensions"]
entries = sorted(entries, key=lambda row: (
    GROUP_ORDER.index(row[0]),
    MANTINE_ORDER.get(row[1], (99, 999))[1],
    row[2],
))
core_names = [name for _, name, _, _ in entries[:121]]
expected_core = [name for names in MANTINE_GROUPS.values() for name in names]
if core_names != expected_core:
    raise SystemExit("Core navigation names or order drifted from the Mantine catalogue")
entries = [row[3] for row in entries]

catalog = '''//! Generated by `python scripts/showcase-catalog.py`; edit canonical inventory or demo sources.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub id: &'static str,
    pub slug: &'static str,
    pub task: &'static str,
    pub name: &'static str,
    pub group: &'static str,
    pub description: &'static str,
    pub example: &'static str,
    pub code: &'static str,
    pub build: fn(sapid::Theme) -> Box<dyn glory::ui::Widget>,
    pub web: &'static str,
    pub native: &'static str,
    pub native_note: &'static str,
}

#[rustfmt::skip]
pub const ENTRIES: &[Entry] = &[
''' + "\n".join(entries) + '''
];

pub fn find(slug: &str) -> Option<&'static Entry> {
    ENTRIES.iter().find(|entry| entry.slug == slug)
}

pub fn find_by_id(id: &str) -> Option<&'static Entry> {
    ENTRIES.iter().find(|entry| entry.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_covers_every_work_item_once_with_real_rust_examples() {
        assert_eq!(ENTRIES.len(), 181);
        for (index, entry) in ENTRIES.iter().enumerate() {
            assert!(ENTRIES[..index].iter().all(|other| other.id != entry.id));
            assert!(ENTRIES[..index].iter().all(|other| other.slug != entry.slug));
            assert_eq!(find_by_id(entry.id).unwrap().name, entry.name);
            assert_eq!(find(entry.slug).unwrap().id, entry.id);
            assert!(
                !entry.name.is_empty()
                    && !entry.group.is_empty()
                    && !entry.description.is_empty()
            );
            assert!(entry.code.contains("fn ") && !entry.code.contains("iframe"));
        }
        assert!(find("unknown").is_none());
    }
}
'''
registry = "//! Generated compiled widget examples; source panels include these exact files.\nuse glory::ui::Widget;\n"
for key in sorted(modules):
    registry += f"mod {key};\n"
    registry += f"pub fn {key}_widget(theme: sapid::Theme) -> Box<dyn Widget> {{ Box::new({key}::demo(theme)) }}\n"
outputs[WEB / "src/showcase_demos/mod.rs"] = formatted(registry)
outputs[CATALOG] = formatted(catalog)
outputs[WEB / "src/showcase_api.rs"] = formatted(api_catalog(tasks))
stale = []
previous = {path.stem for path in (WEB / "src/showcase_demos").glob("*.rs")} - {"mod"}
for retired in previous - set(modules):
    path = WEB / f"src/showcase_demos/{retired}.rs"
    if args.check:
        stale.append(path.relative_to(ROOT).as_posix())
    elif path.is_file():
        path.unlink()
for path, content in outputs.items():
    if args.check:
        if not path.is_file() or path.read_text(encoding="utf-8") != content:
            stale.append(path.relative_to(ROOT).as_posix())
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8", newline="\n")
if stale:
    raise SystemExit(f"stale generated showcase files ({len(stale)}):\n"
                     + "\n".join(f"  {path}" for path in stale)
                     + "\nRun python scripts/showcase-catalog.py to regenerate them.")
print(f"showcase catalogue: {len(entries)} entries, {len(pages)} fixture families, {len(CORE)} core demos, {len(FOCUSED)} focused demos; all source modules compiled")
