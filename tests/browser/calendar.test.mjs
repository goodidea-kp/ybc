// ybc::Calendar in a real browser (Bastion BASTION-539): typing, opening the
// picker, moving with the keys, picking, Esc, focus return, and the picker
// inside a ModalCard. Run it with tests/browser/run.sh, which builds the page.
//
// Environment:
//   CALENDAR_PAGE_DIST  the trunk build of tests/browser/calendar_page (required)
//   PLAYWRIGHT_MODULE   where `playwright` is installed (default: `playwright`)
//   BULMA_CSS, FONTAWESOME_CSS  optional stylesheets, for the screenshots
//   SCREENSHOT_DIR      optional: light and dark screenshots of the open picker
import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { readFile, mkdir } from 'node:fs/promises';
import { extname, join, normalize } from 'node:path';
import { pathToFileURL } from 'node:url';

const dist = process.env.CALENDAR_PAGE_DIST;
assert.ok(dist, 'Set CALENDAR_PAGE_DIST to the trunk build of tests/browser/calendar_page');
const playwrightModule = process.env.PLAYWRIGHT_MODULE ?? 'playwright';
const { chromium } = await import(playwrightModule.startsWith('/') ? pathToFileURL(join(playwrightModule, 'index.mjs')).href : playwrightModule);

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css' };
let server;
let browser;
let baseUrl;

before(async () => {
    server = createServer(async (request, response) => {
        const path = normalize(new URL(request.url, 'http://page').pathname).replace(/^\/+/, '') || 'index.html';
        try {
            const body = await readFile(join(dist, path));
            response.writeHead(200, { 'content-type': TYPES[extname(path)] ?? 'application/octet-stream' });
            response.end(body);
        } catch {
            response.writeHead(404).end();
        }
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    baseUrl = `http://127.0.0.1:${server.address().port}/`;
    browser = await chromium.launch();
});

after(async () => {
    await browser?.close();
    await new Promise(resolve => server?.close(resolve));
});

async function openPage(colorScheme = 'light') {
    const context = await browser.newContext({ locale: 'en-US', colorScheme });
    const page = await context.newPage();
    const errors = [];
    page.on('pageerror', error => errors.push(error));
    await page.goto(baseUrl);
    for (const path of [process.env.BULMA_CSS, process.env.FONTAWESOME_CSS].filter(Boolean)) {
        await page.addStyleTag({ path });
    }
    await page.locator('#plain').waitFor();
    return { page, errors, close: () => context.close() };
}

function today() {
    const now = new Date();
    const pad = value => String(value).padStart(2, '0');
    return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}

const focusedDay = page => page.evaluate(() => document.activeElement?.getAttribute('data-ybc-day') ?? document.activeElement?.id);
// The dialog's close event, where focus goes back, is queued after the key
// or the click: wait for focus to arrive rather than read it at once.
async function focusReturnsTo(page, id, how = '') {
    await page.waitForFunction(id => document.activeElement?.id === id, id, { timeout: 2000 })
        .catch(() => assert.fail(`focus did not return to #${id} ${how}`));
}
const pickerOpen = (page, testid) => page.locator(`[data-testid="${testid}-picker-dialog"]`).evaluate(dialog => dialog.open);

test('the field stays typable from the keyboard', async () => {
    const { page, errors, close } = await openPage();
    await page.locator('#plain').focus();
    await page.keyboard.type('11032026');
    await page.keyboard.press('Tab');
    assert.equal(await page.getByTestId('plain-value').textContent(), '2026-11-03');
    assert.equal(await page.locator('#plain').getAttribute('type'), 'date');
    assert.deepEqual(errors, []);
    await close();
});

test('the named button opens the picker on today, with focus on that day', async () => {
    const { page, close } = await openPage();
    await page.getByRole('button', { name: 'Choose a date from a calendar' }).click();
    assert.equal(await pickerOpen(page, 'plain'), true);
    assert.equal(await focusedDay(page), today());
    assert.equal(await page.locator(`[data-ybc-day="${today()}"]`).getAttribute('aria-current'), 'date');
    assert.equal(await page.getByTestId('plain-picker').getAttribute('aria-expanded'), 'true');
    await close();
});

test('Home/End, arrow keys and PageUp move the day, Enter picks it and focus returns to the field', async () => {
    const { page, close } = await openPage();
    await page.locator('#plain').fill('2026-10-24');
    await page.getByTestId('plain-picker').click();
    assert.equal(await focusedDay(page), '2026-10-24');
    assert.equal(await page.locator('#plain-picker-dialog th').first().getAttribute('abbr'), 'Sunday', 'weeks start on Sunday');
    await page.keyboard.press('Home');
    assert.equal(await focusedDay(page), '2026-10-18', 'Home goes back to Sunday');
    await page.keyboard.press('End');
    assert.equal(await focusedDay(page), '2026-10-24', 'End goes to Saturday');
    await page.keyboard.press('ArrowRight');
    assert.equal(await focusedDay(page), '2026-10-25');
    await page.keyboard.press('ArrowDown');
    assert.equal(await focusedDay(page), '2026-11-01');
    assert.match(await page.locator('#plain-picker-title').textContent(), /November 2026/);
    await page.keyboard.press('PageUp');
    assert.equal(await focusedDay(page), '2026-10-01');
    await page.keyboard.press('Enter');
    assert.equal(await page.getByTestId('plain-value').textContent(), '2026-10-01');
    assert.equal(await pickerOpen(page, 'plain'), false);
    await focusReturnsTo(page, 'plain');
    await close();
});

test('a click on a day picks it; the month buttons leave focus where it is', async () => {
    const { page, close } = await openPage();
    await page.locator('#plain').fill('2026-10-24');
    await page.getByTestId('plain-picker').click();
    await page.getByRole('button', { name: 'Next month, November 2026' }).click();
    assert.match(await page.locator('#plain-picker-title').textContent(), /November 2026/);
    assert.equal(await page.evaluate(() => document.activeElement?.getAttribute('data-testid')), 'plain-picker-next');
    await page.getByRole('button', { name: 'Tuesday 17 November 2026' }).click();
    assert.equal(await page.getByTestId('plain-value').textContent(), '2026-11-17');
    assert.equal(await page.locator('#plain').inputValue(), '2026-11-17');
    await focusReturnsTo(page, 'plain');
    await close();
});

test('Esc, Close and a click outside close the picker without changing the value', async () => {
    const { page, close } = await openPage();
    await page.locator('#plain').fill('2026-10-24');
    for (const [how, dismiss] of [
        ['Esc', () => page.keyboard.press('Escape')],
        ['Close', () => page.getByTestId('plain-picker-close').click()],
        ['a click outside', () => page.mouse.click(5, 5)],
    ]) {
        await page.getByTestId('plain-picker').click();
        assert.equal(await pickerOpen(page, 'plain'), true);
        await dismiss();
        assert.equal(await pickerOpen(page, 'plain'), false, how);
        await focusReturnsTo(page, 'plain', how);
        assert.equal(await page.getByTestId('plain-value').textContent(), '2026-10-24', how);
    }
    await close();
});

test('inside a modal: Esc closes only the picker, and a pick keeps the modal open', async () => {
    const { page, close } = await openPage();
    await page.getByTestId('open-modal').click();
    await page.getByRole('button', { name: 'Choose Not after from a calendar' }).click();
    assert.equal(await focusedDay(page), '2026-10-24', 'opens on the US-format value');
    assert.equal(await page.locator('[data-ybc-day="2026-10-24"]').evaluate(day => day.closest('td').getAttribute('aria-selected')), 'true');
    await page.keyboard.press('Escape');
    assert.equal(await pickerOpen(page, 'in-modal'), false);
    assert.equal(await page.getByTestId('modal-state').textContent(), 'open');
    assert.equal(await page.locator('dialog#form').evaluate(dialog => dialog.open), true);
    await focusReturnsTo(page, 'in-modal');
    await page.getByTestId('in-modal-picker').click();
    await page.getByRole('button', { name: 'Thursday 15 October 2026' }).click();
    assert.equal(await page.getByTestId('in-modal-value').textContent(), '10/15/2026');
    assert.equal(await page.getByTestId('modal-state').textContent(), 'open');
    await focusReturnsTo(page, 'in-modal');
    await page.keyboard.press('Escape');
    assert.equal(await page.getByTestId('modal-state').textContent(), 'closed', 'a second Esc closes the modal');
    await close();
});

test('screenshots of the open picker in the light and dark schemes', { skip: !process.env.SCREENSHOT_DIR }, async () => {
    await mkdir(process.env.SCREENSHOT_DIR, { recursive: true });
    for (const scheme of ['light', 'dark']) {
        const { page, close } = await openPage(scheme);
        await page.locator('#plain').fill('2026-10-24');
        await page.getByTestId('plain-picker').click();
        await page.keyboard.press('ArrowRight');
        await page.screenshot({ path: join(process.env.SCREENSHOT_DIR, `calendar-picker-${scheme}.png`) });
        await close();
    }
});
