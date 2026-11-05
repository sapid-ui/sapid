# Typography public examples

The nine typography pages have dedicated primary examples and 33 independent,
compiled source sections in `examples/web/src/showcase_typography_docs`.
Document page titles and section headings use regular weight throughout the
site. The Title examples also demonstrate explicit regular-weight presentation;
the component still accepts the application's chosen heading metrics.

## Quotes, code and marked content

Blockquote has separate quotation and attribution slots. The examples show a
named author, a real source link, a passive vector icon and long wrapping prose.
The icon row explicitly disables flex wrapping so the first text line stays
beside the icon. Border, background and foreground are independent Style fields.
The component does not guess a citation URL or fetch source content.

Code is semantic code text, escaped rather than interpreted as HTML. Inline
examples sit among ordinary phrasing text. Block presentation uses explicit
Display::Block, Pre whitespace and a local horizontal overflow box; it remains
plain code text. CodeHighlight is the separate syntax-token component. A string
containing `<button>` does not create an interactive button.

Highlight performs exact, case-sensitive, mount-time literal matching. Design,
design and DESIGN differ; punctuation such as `.*` has no regex meaning. UTF-8
queries preserve the original content boundaries. Multiple queries choose the
earliest match and then the longest query at the same start. Repeated matches
produce real Mark ranges; no match produces ordinary inline text. Budgets are
64 KiB content, 32 nonempty queries of at most 256 UTF-8 bytes each and 1024
output ranges. There is no Unicode case folding, regex engine or reactive search
controller. A new matching configuration belongs to a new content owner.

Mark is an explicit passive phrasing fragment. It uses readable theme colors;
custom backgrounds set a corresponding foreground explicitly. Long marked prose
wraps naturally, and selection can span plain and marked fragments. Mark and
Highlight retain the original source text rather than shortened business values.

## Lists and tables

TextList uses actual ordered/unordered lists and list items. Nested lists belong
inside a TextListItem. Optional passive icons are item content and retain the
native list marker; they do not advertise a separate marker replacement API.
Item margins configure spacing. Display lists do not imply selection, a grid
cursor or a virtual source model.

Table examples use a fictional studio project tracker, including caption,
column/row headers, a footer, independent borders/striping/hover, spacing,
vertical facts and application-owned Checkbox selection. Header identities and
cell associations belong to the same finite table declaration. A selection
checkbox remains a separate actual control with explicit application state.
`caption_side(Bottom)` uses an actual typed caption-side layout declaration,
including unstyled output. Caption naming and header associations remain intact;
moving the caption after sections in DOM order alone is insufficient.

Wide tables sit inside `scroll_area(...).axes(ScrollAxes::Both)` with an explicit
minimum table width and bounded viewport height. The default ScrollArea axis is
vertical, so merely placing a wide table inside it does not enable horizontal
keyboard scrolling. The public primary and all wide detail examples explicitly
enable both axes. Scroll and resize retain the table cells and their headers;
overflow remains local to the viewport. The sticky example uses explicit
`sticky_header_offset(Length::Px(0))` and actual browser sticky positioning.

## Text, headings and content scope

Text exposes theme size plus independent font size, line height, weight and
color. Body/Lead/Muted/Small/Caption choose typography roles. `inline(true)`
selects phrasing text; containers own flex, wrapping and alignment. Reactive
content updates the same paragraph and never replaces it with a form control.
Unstyled omits theme declarations while retaining explicit appearance fields.

`truncate(true)` uses single-line ellipsis within an explicit width; line_clamp
limits visible lines. Both keep the complete original semantic string. Title's
constructor selects heading level 1..=6 independently of visual metrics. A
large level-three heading remains h3, and a truncated heading retains its full
accessible name and level.

Typography composes a real article with headings, paragraphs, a list, a quote,
descriptive links and a locally scrolling table. `content` supplies an explicit
TypographyContext for text/title/anchor builders. Other widgets receive
`cx.theme` and `cx.size` explicitly. `fill` preserves children as authored; it
does not rewrite nested widget themes, parse arbitrary HTML or install global
HTML styles. The examples contain article content rather than navigation or
request acceptance workbenches.

## Host boundaries and focused verification

The current native Apace profile explicitly rejects ellipsis, multi-line clamp,
glyph gradients and Sticky positioning. Ordinary wrapping text, headings,
lists/quotes, table layout, associations and local scrolling retain their
declared support. Browser clipping and sticky positioning are separately tested.
The page capability notes distinguish these native limits from actual browser
support; the older note that also claimed browser ellipsis was rejected was
incorrect and has been corrected.

`examples/web/tests/showcase_site.rs` builds the complete source sections in four
themes, checks their navigation and releases their owners. The component suites
cover original text/heading semantics, matching budgets, tables and controlled
selection. `examples/native/tests/typography_docs.rs` imports the actual public
article, quote and table sources: it verifies narrow-column layout, same-row
icon/text, real horizontal scroll with cell identity, and explicit clipping/
sticky rejection. These protocol/CPU checks do not establish OS keyboard,
pointer selection or spoken screen-reader acceptance. The per-page queue records
the actual release/browser result after it completes.
