async (hostPage) => {
  const browser = hostPage.context().browser();
  const context = await browser.newContext({viewport:{width:1440,height:1000}});
  const base = process.env.SAPID_SHOWCASE_BASE || 'http://127.0.0.1:8795';
  const reports = [], errors = [];
  let activePage;
  const assert = (value, message) => { if (!value) throw Error(message); };
  const setup = page => {
    page.on('pageerror', e => errors.push(e.message));
    page.on('console', e => { if (e.type() === 'error') errors.push(e.text()); });
  };
  const go = async (page, name) => {
    await page.goto(`${base}/${name}`);
    await page.waitForFunction(() => window.ready === true);
    await page.locator('.sapid-site').waitFor();
    assert(await page.locator('iframe').count() === 0, `${name}: no iframe UI`);
    assert(await page.locator('.site-subnav').count() === 0, `${name}: secondary row removed`);
    assert(await page.locator('.site-header-tools > .site-palettes + .site-theme-toggle').count() === 1, `${name}: palettes directly precede scheme toggle`);

  };
  const shot = (page, name) => page.screenshot({path:`D:/BuildCache/sapid/showcase-${name}.png`,fullPage:true});
  try {
    await context.grantPermissions(['clipboard-read','clipboard-write'],{origin:base});
    const p = await context.newPage(); activePage = p; setup(p);
    await go(p, 'showcase.html');
    assert(await p.getByRole('banner').count() === 1 && await p.getByRole('main').count() === 1, 'home has real landmarks');
    await shot(p, 'home-light');
    reports.push({suite:'showcase-rust-home',status:'passed'});

    await go(p, 'showcase.html?widget=button');
    const routes = await p.locator('.site-nav-link').evaluateAll(elements => elements.map(e => ({href:e.getAttribute('href'),label:e.textContent.trim()})));
    assert(routes.length === 181 && new Set(routes.map(r => new URL(r.href, base).searchParams.get('widget'))).size === 181, '181 unique canonical navigation links');
    for (const route of routes) {
      const id = new URL(route.href, base).searchParams.get('widget');
      const anchor = p.locator(`.site-nav-link[href="${route.href}"]`);
      await anchor.click();
      await p.waitForFunction(id=>document.documentElement.getAttribute('data-sapid-widget')===id,id);
      await p.locator('h1.site-title').waitFor();
      assert(new URL(p.url()).searchParams.get('widget') === id, `${id}: actual route navigation`);
      assert(await p.locator('.site-demo .site-example').count() === 1, `${id}: real embedded Glory widget`);
      assert((await p.locator('.site-code').first().textContent()).trim().length > 30, `${id}: readable source`);
      assert(await p.locator('iframe').count() === 0, `${id}: no fixture iframe`);
      reports.push({suite:`showcase-route-${id}`,status:'passed'});
      console.log(`showcase route ${reports.length - 1}/181 ${id} passed`);
    }

    await go(p, 'showcase.html?widget=button');
    const search = p.getByRole('textbox',{name:'Search widgets',exact:true});
    await search.fill('TextInput');
    await p.waitForFunction(() => document.querySelectorAll('.site-nav-link').length === 1);
    assert((await p.locator('.site-nav-link').getAttribute('href')).includes('text-input'), 'search finds the canonical component route');
    await search.fill('no-such-widget-xyz');
    await p.locator('.site-nav-empty').first().waitFor();
    assert(await p.locator('.site-nav-link').count() === 0, 'empty search state');
    await search.fill('');
    await p.waitForFunction(() => document.querySelectorAll('.site-nav-link').length === 181);
    reports.push({suite:'showcase-live-search',status:'passed'});

    const demo = p.locator('.site-demo');
    await demo.getByRole('button',{name:'Filled button',exact:true}).click();
    await demo.getByText('Clicked 1 times',{exact:true}).waitFor();
    const preview = demo.locator('.site-playground-button');
    await preview.evaluate(e => window.showcasePreview = e);
    const mediumHeight = (await preview.boundingBox()).height;
    await demo.getByRole('combobox',{name:'Variant',exact:true}).selectOption('outline');
    await demo.getByRole('button',{name:'Outline button',exact:true}).waitFor();
    await demo.getByRole('button',{name:'Outline button',exact:true}).click();
    await demo.getByText('Clicked 2 times',{exact:true}).waitFor();
    await demo.getByRole('combobox',{name:'Size',exact:true}).selectOption('large');
    await p.waitForFunction(height => document.querySelector('.site-playground-button').getBoundingClientRect().height > height, mediumHeight);
    await demo.getByRole('checkbox',{name:'Disabled',exact:true}).check();
    await p.waitForFunction(() => document.querySelector('.site-playground-button').disabled);
    await demo.getByRole('checkbox',{name:'Disabled',exact:true}).uncheck();
    await demo.getByRole('checkbox',{name:'Loading',exact:true}).check();
    await p.waitForFunction(() => {const b=document.querySelector('.site-playground-button'); return b.disabled && b.getAttribute('aria-busy') === 'true';});
    await demo.getByRole('button',{name:'Loading…',exact:true}).waitFor();
    await demo.getByRole('checkbox',{name:'Loading',exact:true}).uncheck();
    await p.waitForFunction(() => !document.querySelector('.site-playground-button').disabled);
    assert(await preview.evaluate(e => e === window.showcasePreview), 'real property controls retain the same preview button');
    assert(await demo.getByText('Clicked 2 times',{exact:true}).count() === 1, 'disabled/loading changes never activate demo');
    const snippet = p.locator('.site-code').first();
    const codeBody = snippet.locator('.site-code-body');
    const collapsedHeight = (await codeBody.boundingBox()).height;
    const completeSource = await codeBody.locator('.sapid-code-source').textContent();
    assert(completeSource.includes('pub fn demo(') && completeSource.includes('on_checked_change'), 'collapsed example keeps complete Rust source');
    assert(completeSource.includes('use glory::style::FlexWrap;'), 'copyable Button snippet includes the explicit layout import');
    await snippet.getByRole('button',{name:'Expand code',exact:true}).click();
    await snippet.getByRole('button',{name:'Collapse code',exact:true}).waitFor();
    await p.waitForFunction(height => document.querySelector('.site-code-body').getBoundingClientRect().height > height, collapsedHeight);
    await snippet.getByRole('button',{name:'Copy code',exact:true}).click();
    await p.waitForFunction(async expected => (await navigator.clipboard.readText()) === expected, completeSource);
    await snippet.getByRole('button',{name:'Collapse code',exact:true}).click();
    await snippet.getByRole('button',{name:'Expand code',exact:true}).waitFor();
    assert(Math.abs((await codeBody.boundingBox()).height - collapsedHeight) <= 1, 'code collapse restores bounded preview height');
    reports.push({suite:'showcase-expand-and-complete-copy',status:'passed'});
    await p.getByRole('button',{name:'Source',exact:true}).click();
    await p.getByRole('heading',{name:'Rust source',exact:true}).waitFor();
    await p.locator('.site-code').waitFor();
    assert(await p.locator('.site-demo').count() === 0, 'Rust Source tab replaces demo content');
    await p.getByRole('button',{name:'Documentation',exact:true}).click();
    await p.locator('.site-demo').waitFor();
    await p.getByRole('button',{name:'Capabilities',exact:true}).click();
    await p.getByRole('heading',{name:'Host support',exact:true}).waitFor();
    assert(await p.locator('.site-support-row').first().textContent() === 'WebTested', 'Documentation switches directly to readable host capabilities');
    await p.getByRole('button',{name:'Documentation',exact:true}).click();
    await p.locator('.site-demo').waitFor();
    await p.getByRole('button',{name:'Properties',exact:true}).click();
    await p.getByRole('heading',{name:'Button properties',exact:true}).waitFor();
    assert(await p.locator('.site-api-tokens',{hasText:'left_section'}).count() > 0, 'Sapid Rust builder API is available');
    assert(await p.locator('.site-api-name').count() === 0, 'No upstream React props table is presented as Sapid API');
    assert(!(await p.locator('.site-doc-content').textContent()).includes('Mantine'), 'Sapid properties do not use upstream product branding');
    await p.getByRole('button',{name:'Styles API',exact:true}).click();
    await p.getByRole('heading',{name:'Widget classes',exact:true}).waitFor();
    assert(await p.locator('.site-api-tokens',{hasText:'sapid-button'}).count() > 0, 'Real widget classes are available');
    await p.getByRole('button',{name:'Documentation',exact:true}).click();
    await p.locator('.site-demo').waitFor();
    await shot(p, 'button-light');
    reports.push({suite:'showcase-button-and-source',status:'passed'});

    await p.getByRole('link',{name:'Switch to dark theme',exact:true}).click();
    await p.locator('.sapid-site.site-dark').waitFor();
    assert(new URL(p.url()).searchParams.get('widget') === 'button', 'theme reload retains route');
    await go(p, 'showcase.html?widget=text-input&theme=green&scheme=dark');
    await p.getByRole('textbox',{name:'Your name',exact:true}).fill('Sapid reader');
    await p.getByText('Hello, Sapid reader!',{exact:true}).waitFor();
    await shot(p, 'textfield-dark');
    reports.push({suite:'showcase-theme-and-controlled-text',status:'passed'});

    for (const name of ['blue','green']) for (const scheme of ['light','dark']) {
      await go(p, `showcase.html?widget=button&scheme=${scheme}&theme=${name === 'blue' ? 'green' : 'blue'}`);
      await p.getByRole('link',{name:`Use ${name} palette`,exact:true}).click();
      await p.locator(`.sapid-site.sapid-theme-${name}-${scheme}`).waitFor();
      const query = new URL(p.url()).searchParams;
      assert(query.get('widget') === 'button' && query.get('theme') === name && query.get('scheme') === scheme, 'palette choice preserves guide and light/dark scheme');
      assert(await p.getByRole('link',{name:`Use ${name} palette`,exact:true}).getAttribute('aria-current') === 'location', 'selected palette is exposed as current');
      const swatch = await p.getByRole('link',{name:`Use ${name} palette`,exact:true}).evaluate(e => ({text:e.textContent.trim(),color:getComputedStyle(e).backgroundColor,outline:getComputedStyle(e).outlineColor}));
      assert(swatch.text === '' && swatch.color === (name === 'blue' ? 'rgb(25, 113, 194)' : 'rgb(35, 112, 50)'), 'palette is represented by its actual color, without a text label');

      const frame = await p.locator('.sapid-site').evaluate(e=>({background:getComputedStyle(e).backgroundColor,text:getComputedStyle(e).color}));
      assert(frame.background === (scheme === 'dark' ? 'rgb(20, 21, 23)' : 'rgb(248, 249, 250)'), 'page canvas comes from resolved palette data');
      assert(frame.text === (scheme === 'dark' ? 'rgb(222, 226, 230)' : 'rgb(33, 37, 41)'), 'page text comes from resolved palette data');
      const actual = await p.locator('.site-nav-link.is-active').evaluate(e=>getComputedStyle(e).color);
      const accent = name === 'blue' ? (scheme === 'dark' ? 'rgb(116, 192, 252)' : 'rgb(24, 100, 171)') : (scheme === 'dark' ? 'rgb(140, 233, 154)' : 'rgb(35, 112, 50)');
      assert(actual === accent, `${name}-${scheme}: website follows palette tokens (${actual})`);
      const filled = p.locator('.site-demo').getByRole('button',{name:'Filled button',exact:true});
      assert(await filled.evaluate(e=>getComputedStyle(e).backgroundColor) === (name === 'blue' ? 'rgb(25, 113, 194)' : 'rgb(35, 112, 50)'), 'preview follows the same palette');
      await p.getByRole('link',{name:`Switch to ${scheme === 'dark' ? 'light' : 'dark'} theme`,exact:true}).click();
      await p.locator(`.sapid-site.sapid-theme-${name}-${scheme === 'dark' ? 'light' : 'dark'}`).waitFor();
      assert(new URL(p.url()).searchParams.get('theme') === name, 'scheme choice preserves palette');
      reports.push({suite:`showcase-palette-${name}-${scheme}`,status:'passed'});
    }


    await go(p, 'showcase.html?widget=checkbox');
    const check = p.getByRole('checkbox',{name:'I agree to the terms and conditions',exact:true});
    await check.check(); assert(await check.isChecked(), 'real checkbox toggles');
    await p.getByText('Thank you for confirming.',{exact:true}).waitFor();
    reports.push({suite:'showcase-controlled-checkbox',status:'passed'});

    await go(p, 'showcase.html?widget=textarea');
    await p.getByRole('textbox',{name:'Biography',exact:true}).waitFor();
    assert(await p.getByRole('button',{name:'Toggle error',exact:true}).count() === 0, 'Textarea starts with its own focused example');
    await go(p, 'showcase.html?widget=fieldset');
    await p.getByRole('button',{name:'Toggle fieldset disabled',exact:true}).click();
    await p.locator('.sapid-fieldset[disabled]').waitFor();
    assert(await p.getByRole('textbox',{name:'Your name',exact:true}).isDisabled(), 'Fieldset disables its grouped controls');
    assert(await p.getByRole('textbox',{name:'Biography',exact:true}).count() === 0, 'Fieldset does not reuse the Textarea example');
    reports.push({suite:'showcase-focused-fields',status:'passed'});

    await go(p, 'showcase.html?widget=code-highlight');
    await p.getByRole('button',{name:'JSON example',exact:true}).click();
    await p.locator('.site-code-highlight-demo .sapid-code-source').filter({hasText:'"ready"'}).waitFor();
    await p.locator('.site-demo').getByRole('button',{name:'Copy code',exact:true}).click();
    await p.getByText('Clipboard status: Written',{exact:true}).waitFor();
    const copiedCode = await p.evaluate(() => navigator.clipboard.readText());
    assert(copiedCode.replace(/\r\n/g, '\n') === '{\n  "name": "Sapid",\n  "ready": true,\n  "values": [1, 2, 3]\n}\n', 'CodeHighlight copies the complete accepted JSON source');
    const codeDemo = p.locator('.site-demo');
    await codeDemo.getByRole('button',{name:'Collapse code',exact:true}).click();
    await codeDemo.getByRole('button',{name:'Expand code',exact:true}).waitFor();
    await codeDemo.getByRole('button',{name:'Expand code',exact:true}).click();
    await codeDemo.getByRole('button',{name:'Collapse code',exact:true}).waitFor();
    assert(await p.locator('.site-code-highlight-demo .sapid-code-gutter').count() > 0, 'CodeHighlight renders line numbers');
    reports.push({suite:'showcase-code-highlight-copy-and-expansion',status:'passed'});

    await go(p, 'showcase.html?widget=table');
    assert(await p.getByRole('table').count() === 6, 'Table includes six complete scenarios');
    const stickyHeader = p.locator('.site-sticky-table .sapid-table-cell').first();
    assert(await stickyHeader.evaluate(e => getComputedStyle(e).position) === 'sticky', 'Table header is actually sticky');
    await p.getByRole('checkbox',{name:'Select Carbon',exact:true}).check();
    await p.getByText('Selected: Carbon',{exact:true}).waitFor();
    await p.getByRole('checkbox',{name:'Select Nitrogen',exact:true}).check();
    await p.getByText('Selected: Carbon, Nitrogen',{exact:true}).waitFor();
    await p.getByRole('checkbox',{name:'Select Carbon',exact:true}).uncheck();
    await p.getByText('Selected: Nitrogen',{exact:true}).waitFor();
    reports.push({suite:'showcase-table-rich-scenarios-and-selection',status:'passed'});

    await go(p, 'showcase.html?widget=tree-select');
    assert(await p.getByText('Refuse path selection',{exact:true}).count() === 0, 'TreeSelect is a usage guide, not an engineering workbench');
    const popupFor = async trigger => p.locator(`#${await trigger.getAttribute('aria-controls')}`);
    const treeItem = (root,name) => root.getByRole('treeitem',{name,exact:true});
    const continuousTreeGuides = async root => {
      const geometry = await root.locator('.sapid-tree-item').evaluateAll(rows => rows.map(row => ({
        box: row.getBoundingClientRect().toJSON(),
        guides: [...row.querySelectorAll('.sapid-tree-guide')].map(guide => ({
          box: guide.getBoundingClientRect().toJSON(),
          horizontal: guide.classList.contains('sapid-tree-branch-horizontal'),
          closing: guide.style.bottom === '50%',
        })),
      })));
      let joins = 0, endings = 0;
      for (const [index, row] of geometry.entries()) {
        for (const guide of row.guides.filter(guide => !guide.horizontal)) {
          assert(Math.abs(guide.box.top - row.box.top) < 0.1, 'Vertical guides include the top row border');
          if (guide.closing) {
            const horizontal = row.guides.find(candidate => candidate.horizontal);
            assert(horizontal && Math.abs(guide.box.bottom - horizontal.box.top) < 0.1, 'Last sibling ends exactly at its horizontal branch');
            endings++;
          } else {
            assert(Math.abs(guide.box.bottom - row.box.bottom) < 0.1, 'Continuing guides include the bottom row border');
            const next = geometry[index + 1]?.guides.find(candidate => !candidate.horizontal && Math.abs(candidate.box.x - guide.box.x) < 0.1);
            assert(next && Math.abs(next.box.top - guide.box.bottom) < 0.1, 'Adjacent branch and ancestor guide segments meet without a gap');
            joins++;
          }
        }
      }
      assert(joins > 0 && endings > 0, 'Geometry covers real connecting segments and last siblings');
    };
    const singleTree = p.getByRole('combobox',{name:'Your favorite item',exact:true});
    const singleControl = p.locator('.sapid-tree-select-control').first();
    const singleClear = singleControl.getByRole('button',{name:'Clear tree selection',exact:true});
    const singlePopup = await popupFor(singleTree);
    assert(!await singlePopup.evaluate(el => el.matches(':popover-open')), 'TreeSelect starts with an anchored, closed popup');
    assert((await singleTree.boundingBox()).width > 300, 'Compact clear action does not shrink the trigger to its label');
    assert(await singleTree.evaluate(el => getComputedStyle(el).display === 'flex' && getComputedStyle(el).justifyContent === 'flex-start'), 'TreeSelect label is left aligned');
    assert(await singleClear.count() === 0, 'Empty TreeSelect does not render a useless clear button');
    const chevronBox = await singleControl.locator('.sapid-tree-select-chevron').boundingBox();
    const triggerBox = await singleTree.boundingBox();
    assert(chevronBox.x >= triggerBox.x && chevronBox.x + chevronBox.width <= triggerBox.x + triggerBox.width, 'Dropdown chevron stays inside the trigger');
    assert(await singleControl.locator('.sapid-tree-select-chevron .sapid-theme-icon-graphic').count() === 1, 'Trigger chevron is a vector path');
    await singleTree.click();
    const disclosure = singlePopup.getByRole('button',{name:'Toggle Electronics',exact:true});
    await disclosure.waitFor();
    assert(await disclosure.evaluate(el => ['borderTopWidth','borderRightWidth','borderBottomWidth','borderLeftWidth'].every(side => getComputedStyle(el)[side] === '0px')), 'Tree disclosure has no native button box');
    assert(!['+','−'].includes((await disclosure.textContent()).trim()), 'Tree disclosure uses a directional chevron');
    assert(await disclosure.locator('.sapid-theme-icon-graphic').count() === 1, 'Tree disclosure uses the shared vector icon');
    assert(await singlePopup.locator('.sapid-tree-branch-vertical').count() === 14, 'Both hierarchy levels have absolute vertical borders');
    assert(await singlePopup.locator('.sapid-tree-branch-horizontal').count() === 14, 'Both hierarchy levels have absolute horizontal borders');
    assert(await singlePopup.locator('.sapid-tree-guide-line').count() === 7, 'Second-level children inherit continuous ancestor guides');
    assert(await singlePopup.evaluate(el => getComputedStyle(el).rowGap === '0px'), 'TreeSelect options have no gap that could interrupt guides');
    const guideHeights = await singlePopup.locator('.sapid-tree-branch-vertical').evaluateAll(nodes => nodes.map(node => {const parent=node.parentElement;const available=parent.getBoundingClientRect().height;return {height:node.getBoundingClientRect().height,available,border:getComputedStyle(node).borderLeftWidth,pointer:getComputedStyle(node).pointerEvents,position:getComputedStyle(node).position}}));
    assert(guideHeights.filter(guide => Math.abs(guide.height - guide.available / 2) < 0.1).length === 6 && guideHeights.filter(guide => Math.abs(guide.height - guide.available) < 0.1).length === 8, 'Only last siblings use Mantine 50% vertical guides at both levels');
    assert(guideHeights.every(guide => guide.border === '1px' && guide.pointer === 'none' && guide.position === 'absolute'), 'Tree guides are noninteractive absolute border elements');
    await continuousTreeGuides(singlePopup);
    await disclosure.click();
    await treeItem(singlePopup,'Phones').waitFor({state:'hidden'});
    await continuousTreeGuides(singlePopup);
    await disclosure.click();
    await treeItem(singlePopup,'Phones').waitFor();
    await continuousTreeGuides(singlePopup);
    await treeItem(singlePopup,'iPhone').waitFor();
    await treeItem(singlePopup,'iPhone').click();
    await singleTree.getByText('iPhone',{exact:true}).waitFor();
    await singleClear.waitFor();
    const clearBox = await singleClear.boundingBox();
    assert(clearBox.x >= triggerBox.x && clearBox.x + clearBox.width <= triggerBox.x + triggerBox.width, 'Clear action stays inside the trigger');
    await singleClear.click();
    await singleClear.waitFor({state:'hidden'});
    await singleTree.getByText('Pick value',{exact:true}).waitFor();
    assert(await singleClear.count() === 0, 'Clearing hides the action again');

    const multipleTree = p.getByRole('combobox',{name:'Your favorite items',exact:true});
    const multiplePopup = await popupFor(multipleTree);
    await multipleTree.click();
    await continuousTreeGuides(multiplePopup);
    await treeItem(multiplePopup,'Samsung Galaxy').click();
    await p.getByRole('button',{name:'Remove samsung',exact:true}).click();
    await p.getByRole('button',{name:'Remove samsung',exact:true}).waitFor({state:'detached'});
    assert(await p.getByRole('button',{name:'Remove samsung',exact:true}).count() === 0, 'TreeSelect pills remove one controlled value');
    await p.getByRole('button',{name:'Remove iphone',exact:true}).click();
    await p.getByRole('button',{name:'Remove iphone',exact:true}).waitFor({state:'detached'});
    assert(await p.getByRole('button',{name:'Remove iphone',exact:true}).count() === 0, 'TreeSelect removes the initial pill');

    const cascadeTree = p.getByRole('combobox',{name:'Include categories',exact:true});
    const cascadePopup = await popupFor(cascadeTree);
    await cascadeTree.click();
    const parentIndicator = treeItem(cascadePopup,'Electronics').locator('.sapid-tree-check-indicator');
    const uncheckedBackground = await parentIndicator.evaluate(el => getComputedStyle(el).backgroundColor);
    assert(await parentIndicator.evaluate(el => {const style=getComputedStyle(el);return style.width==='18px'&&style.height==='18px'&&style.display==='flex'&&style.alignItems==='center'&&style.justifyContent==='center'&&style.borderTopWidth==='1px'}), 'Tree checkbox indicator has Mantine dimensions and alignment');
    assert(await parentIndicator.locator('.sapid-tree-check-icon').count() === 1 && !(await parentIndicator.textContent()).trim(), 'Tree checkbox uses a passive vector icon without text glyphs');
    await treeItem(cascadePopup,'Electronics').click();
    await p.waitForFunction(el => el.getAttribute('aria-checked') === 'true', await treeItem(cascadePopup,'Electronics').elementHandle());
    assert(await parentIndicator.evaluate(el => getComputedStyle(el).backgroundColor) !== uncheckedBackground, 'Checked indicator uses its accent background');
    assert(await treeItem(cascadePopup,'Electronics').getAttribute('aria-checked') === 'true', 'TreeSelect parent is checked after cascading');
    assert(await treeItem(cascadePopup,'Phones').getAttribute('aria-checked') === 'true' && await treeItem(cascadePopup,'Laptops').getAttribute('aria-checked') === 'true', 'TreeSelect branch selects its enabled leaves');
    await treeItem(cascadePopup,'Laptops').click();
    await p.waitForFunction(el => el.getAttribute('aria-checked') === 'mixed', await treeItem(cascadePopup,'Electronics').elementHandle());
    assert(await treeItem(cascadePopup,'Electronics').getAttribute('aria-checked') === 'mixed', 'TreeSelect parent derives half-checked state');
    await continuousTreeGuides(cascadePopup);
    await p.keyboard.press('Escape');

    const filteredTree = p.getByRole('combobox',{name:'Filtered items',exact:true});
    const filteredPopup = await popupFor(filteredTree);
    await filteredTree.fill('no-such-item');
    await p.getByText('Nothing found...',{exact:true}).waitFor();
    assert(await filteredPopup.getByRole('treeitem').count() === 0, 'TreeSelect search has a real empty tree state');
    await filteredTree.fill('phone');
    await p.getByText('Nothing found...',{exact:true}).waitFor({state:'detached'});
    assert(await p.getByText('Nothing found...',{exact:true}).count() === 0, 'TreeSelect search recovers from empty results');
    await treeItem(filteredPopup,'iPhone').waitFor();
    assert(await treeItem(filteredPopup,'Electronics').count() === 1 && await treeItem(filteredPopup,'Phones').count() === 1 && await treeItem(filteredPopup,'Laptops').count() === 0, 'TreeSelect search preserves both ancestors and filters sibling branches');
    reports.push({suite:'showcase-tree-select-usage-and-interactions',status:'passed'});

    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) {
      await go(p, `showcase.html?widget=tree-select&theme=${theme}&scheme=${scheme}`);
      for (const name of ['Your favorite item', 'Your favorite items', 'Include categories']) {
        const trigger = p.getByRole('combobox', {name, exact:true});
        await trigger.click();
        const popup = await popupFor(trigger);
        await treeItem(popup, 'Phones').waitFor();
        await continuousTreeGuides(popup);
        await p.keyboard.press('Escape');
      }
      await go(p, `showcase.html?widget=tree&theme=${theme}&scheme=${scheme}`);
      await continuousTreeGuides(p.getByRole('tree', {name:'Project files', exact:true}));
    }
    reports.push({suite:'showcase-tree-guides-four-themes',status:'passed'});

    for (const [id,label] of [['select','Your favorite framework'],['combobox','Your favorite framework'],['autocomplete','Your favorite framework'],['cascader','Your favorite item']]) {
      await go(p, `showcase.html?widget=${id}`);
      assert(await p.getByRole('combobox',{name:label,exact:true}).count() > 0, `${id}: its own usage example is present`);
      assert(await p.getByText(/Before choices|Before combobox|Proposals:|Toggle path Owner/).count() === 0, `${id}: no engineering acceptance workbench remains`);
    }
    reports.push({suite:'showcase-combobox-family-focused-guides',status:'passed'});

    await go(p, 'showcase.html?widget=stepper');
    assert(await p.locator('.site-stepper-demo .sapid-stepper-text').evaluateAll(nodes => nodes.every(node => getComputedStyle(node).display === 'flex' && getComputedStyle(node).flexDirection === 'column')), 'Stepper labels and descriptions use independent columns');
    assert(await p.getByText('Pending',{exact:true}).count() === 0, 'Stepper has no raw pending debug text');
    await p.getByRole('button',{name:'Back',exact:true}).click();
    await p.getByRole('button',{name:'Next',exact:true}).click();
    await p.getByRole('button',{name:'3. Review',exact:true}).click();
    await p.getByText('Step 3 content: Confirm your details',{exact:true}).waitFor();
    await p.getByRole('button',{name:'Next',exact:true}).click();
    await p.getByText('Workflow completed',{exact:true}).waitFor();
    reports.push({suite:'showcase-stepper-layout-and-workflow',status:'passed'});

    for (const [id,title,cards] of [
      ['area-chart','Usage',4],
      ['bar-chart','Usage',4], ['line-chart','Usage',4],
      ['composite-chart','Usage',4], ['donut-chart','Donut chart',1],
      ['pie-chart','Pie chart',1], ['radial-bar-chart','Radial bar chart',1],
      ['candlestick-chart','Candlestick chart',1], ['scatter-chart','Scatter chart',1],
      ['bubble-chart','Bubble chart',1], ['funnel-chart','Funnel chart',1],
      ['radar-chart','Radar chart',1], ['sparkline','Sparkline',1],
      ['heatmap','Heatmap',1], ['matrix-chart','Matrix chart',1],
      ['waffle-chart','Waffle chart',1], ['bars-list','Bars list',1],
      ['bullet-chart','Bullet chart',1], ['gauge-chart','Gauge chart',1],
      ['treemap','Treemap',1], ['sunburst-chart','Sunburst chart',1],
      ['sankey-chart','Sankey chart',1],
    ]) {
      await go(p, `showcase.html?widget=${id}`);
      await p.locator('.site-chart-card').getByRole('heading',{name:title,exact:true}).waitFor();
      assert(await p.locator('.site-chart-card').count() === cards, `${id} renders its own chart family`);
      assert(await p.getByText('Accept chart proposals',{exact:true}).count() === 0, `${id} has no cartesian acceptance workbench`);
      assert(await p.getByText('Accept plot proposals',{exact:true}).count() === 0, `${id} has no plot acceptance workbench`);
      assert(await p.locator('.chart-mode-Line,.plot-secondary').count() === 0, `${id} has no secondary acceptance charts`);
    }
    reports.push({suite:'showcase-dedicated-chart-guides',status:'passed'});

    for (const hue of ['blue','green']) for (const scheme of ['light','dark']) for (const name of ['home','button','textfield','dates','charts']) {
      await go(p, `ssr-${name}-${hue}-${scheme}.html`);
      assert(await p.locator(`.sapid-site.site-${scheme}`).count() === 1, `SSR ${name}: adopted theme`);
      if (name !== 'home') assert(await p.locator('.site-nav-link').count() === 181, `SSR ${name}: full navigation`);
      reports.push({suite:`showcase-ssr-${name}-${hue}-${scheme}`,status:'passed'});
    }
    await p.goto(`${base}/ssr-textfield-blue-light.html?manual`);
    await p.waitForFunction(() => window.moduleReady === true);
    const dirty = p.getByRole('textbox',{name:'Your name',exact:true});
    await dirty.fill('Before hydration');
    await dirty.evaluate(e => {window.beforeShowcaseInput=e;e.focus();e.setSelectionRange(2,8,'backward');});
    await p.evaluate(() => window.showcase.mount(true,'text-input','blue','light'));
    assert(await dirty.evaluate(e => e === window.beforeShowcaseInput && e.value === 'Before hydration' && e.selectionStart === 2 && e.selectionEnd === 8 && e.selectionDirection === 'backward'), 'SSR dirty editor identity/value/backward selection retained');
    await dirty.fill('After hydration');
    await p.getByText('Hello, After hydration!',{exact:true}).waitFor();
    reports.push({suite:'showcase-dirty-ssr-adoption',status:'passed'});

    const mobile = await browser.newContext({viewport:{width:360,height:800},hasTouch:true,isMobile:true});
    try {
      const m = await mobile.newPage(); setup(m);
      await go(m, 'showcase.html?widget=button');
      for (const width of [320,360,390]) {
        await m.setViewportSize({width,height:800});
        const header = await m.locator('.site-header').evaluate(e => {
          const targets=[...e.querySelectorAll('.site-palette,.site-theme-toggle,.site-menu-toggle')].map(x=>{const r=x.getBoundingClientRect();return {left:r.left,right:r.right,top:r.top,bottom:r.bottom,width:r.width,height:r.height}});
          return {targets,overflow:document.body.scrollWidth>innerWidth+1};
        });
        assert(header.targets.length === 4 && !header.overflow && header.targets.every(r=>r.width>=22 && r.height>=22 && r.left>=0 && r.right<=width && r.top>=0 && r.bottom<=60), `${width}px: all four header controls visible without overflow`);
        assert(header.targets.slice(1).every((r,i)=>r.left>=header.targets[i].right), `${width}px: controls do not overlap`);
      }
      await m.setViewportSize({width:360,height:800});
      await m.getByRole('link',{name:'Use green palette',exact:true}).tap();
      await m.locator('.sapid-site.sapid-theme-green-light').waitFor();
      assert(new URL(m.url()).searchParams.get('widget') === 'button', 'mobile swatch preserves guide');

      await m.getByRole('button',{name:'Open navigation',exact:true}).tap();
      await m.getByRole('button',{name:'Close navigation',exact:true}).waitFor();
      assert(await m.locator('.sapid-site.menu-open .site-sidebar').isVisible(), 'mobile Rust menu opens');
      await shot(m, 'mobile-navigation');
      await m.locator('.site-nav-link[href*="widget=text-input&"]').tap();
      await m.locator('h1.site-title').waitFor();
      await m.getByRole('button',{name:'Open navigation',exact:true}).waitFor();
      assert(new URL(m.url()).searchParams.get('widget') === 'text-input', 'mobile chooses real docs route');
      assert(await m.evaluate(() => innerWidth === 360 && document.documentElement.clientWidth === 360 && document.body.scrollWidth <= 361), 'mobile document contains its content');
      assert(await m.locator('.site-doc-note').evaluateAll(notes => notes.length > 0 && notes.every(note => getComputedStyle(note).display === 'block' && note.getClientRects().length === 1)), 'mobile descriptions remain contiguous readable blocks');
      await shot(m, 'mobile-docs');
      await go(m, 'showcase.html?theme=green&scheme=dark');
      await m.getByRole('button',{name:'Open navigation',exact:true}).tap();
      const menuTop = await m.locator('.site-mobile-nav').evaluate(e=>e.getBoundingClientRect().top);
      const headerBottom = await m.locator('.site-header').evaluate(e=>e.getBoundingClientRect().bottom);
      assert(Math.abs(menuTop-headerBottom)<=1, 'home mobile menu starts directly below the single header row');
      await shot(m, 'mobile-home-navigation');

    } finally { await mobile.close(); }
    reports.push({suite:'showcase-mobile-navigation',status:'passed'});

    await go(p, 'showcase.html?widget=does-not-exist');
    await p.getByRole('heading',{name:'Widget not found',exact:true}).waitFor();
    await p.getByRole('link',{name:'Sapid',exact:true}).click();
    await p.waitForFunction(()=>document.documentElement.getAttribute('data-sapid-widget')==='');
    await p.getByRole('main').waitFor();
    assert(!new URL(p.url()).searchParams.get('widget'), 'friendly 404 can return home');
    reports.push({suite:'showcase-friendly-404',status:'passed'});
    return {reports,errors};
  } catch (error) {
    errors.push(error.stack || String(error));
    if (activePage) {
      errors.push(JSON.stringify(await activePage.evaluate(() => ({url:location.href,ready:window.ready,title:document.querySelector('h1')?.textContent,body:document.body.innerText.slice(-1600)})).catch(e => ({diagnostic:String(e)}))));
      await shot(activePage,'acceptance-failure').catch(() => {});
    }
    return {reports,errors};
  }
  finally { await context.close(); }
}
