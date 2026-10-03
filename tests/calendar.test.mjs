// npm install --prefix target/calendar-tests --no-save --package-lock=false jsdom@26.1.0
// BULMA_CALENDAR_JS=/path/to/bulma-calendar-7.1.1.js node --test tests/calendar.test.mjs
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { JSDOM } from '../target/calendar-tests/node_modules/jsdom/lib/api.js';

const source = readFileSync(new URL('../src/components/calendar.js', import.meta.url), 'utf8');
const { mountCalendar, syncCalendar, unmountCalendar } = await import(`data:text/javascript;base64,${Buffer.from(source).toString('base64')}`);

test('missing library leaves the native control working', () => {
    delete globalThis.bulmaCalendar;
    const dom = new JSDOM('<input type="date"><div></div>');
    const input = dom.window.document.querySelector('input');
    assert.equal(mountCalendar(dom.window.document.querySelector('div'), input, () => {}, 'date', ''), null);
    assert.equal(input.type, 'date');
    assert.equal(input.hidden, false);
    dom.window.close();
});

test('real Bulma picker: sync, clear, disable, isolation and detached teardown', () => {
    assert.ok(process.env.BULMA_CALENDAR_JS, 'Provide the actual bulma-calendar 7.1.1 bundle');
    const dom = new JSDOM('<label for="date">Date</label><input id="date" type="date"><div></div><input id="other" type="date"><div></div>', { runScripts: 'outside-only' });
    dom.window.matchMedia = () => ({ matches: false });
    dom.window.eval(readFileSync(process.env.BULMA_CALENDAR_JS, 'utf8'));
    globalThis.document = dom.window.document;
    globalThis.bulmaCalendar = dom.window.bulmaCalendar;
    const inputs = document.querySelectorAll('input');
    const hosts = document.querySelectorAll('div');
    const values = [];
    const first = mountCalendar(hosts[0], inputs[0], value => values.push(value), 'date', 'dialog');
    const second = mountCalendar(hosts[1], inputs[1], () => {}, 'date', 'inline');
    assert.ok(first);
    assert.ok(second);
    assert.equal(inputs[0].hidden, true);
    syncCalendar(first, '2030-12-31', false);
    syncCalendar(second, '2026-10-01', false);
    assert.equal(first.picker.value(), '2030-12-31');
    assert.deepEqual(values, []);
    first.picker.emit('select', first.picker);
    assert.deepEqual(values, ['2030-12-31']);
    syncCalendar(first, '', false);
    assert.equal(first.picker.value(), '');
    assert.equal(values.length, 1);
    first.picker.clear();
    assert.equal(values.at(-1), '');
    syncCalendar(first, '2030-12-31', true);
    first.picker.emit('select', first.picker);
    assert.equal(values.length, 2);
    assert.equal(first.picker._ui.dummy.dummy_1.disabled, true);
    syncCalendar(first, '2030-12-31', false);
    document.querySelector('label').click();
    assert.equal(first.picker.isOpen(), true);
    hosts[0].remove();
    unmountCalendar(first);
    assert.equal(inputs[0].hidden, false);
    document.body.click();
    assert.equal(second.picker.value(), '2026-10-01');
    unmountCalendar(second);
    for (const [kind, value, expected] of [
        ['time', '01:02', '01:02'],
        ['datetime-local', '2030-12-31T01:02', '2030-12-31 01:02']
    ]) {
        const selected = [];
        const state = mountCalendar(hosts[1], inputs[1], value => selected.push(value), kind, 'unsupported');
        assert.ok(state);
        syncCalendar(state, value, false);
        assert.equal(state.picker.value(), expected);
        state.picker.emit('validate', state.picker);
        assert.deepEqual(selected, [expected]);
        syncCalendar(state, '', false);
        assert.deepEqual(selected, [expected]);
        assert.equal(state.picker._ui.dummy.dummy_1.value, '');
        state.picker.clear();
        assert.equal(selected.at(-1), '');
        unmountCalendar(state);
    }
    dom.window.close();
});
test('the fallback stays hidden under Bulma, the picker keeps its look and takes the test hooks', () => {
    assert.ok(process.env.BULMA_CALENDAR_JS, 'Provide the actual bulma-calendar 7.1.1 bundle');
    // Bulma's `.input { display: inline-flex }` beats the `hidden` attribute's user-agent rule.
    const dom = new JSDOM('<style>.input { display: inline-flex; }</style><label for="day">Day</label>'
        + '<input id="day" type="date" class="input is-small" data-testid="day" data-cy="day-cy"><div></div>',
        { runScripts: 'outside-only' });
    dom.window.matchMedia = () => ({ matches: false });
    dom.window.eval(readFileSync(process.env.BULMA_CALENDAR_JS, 'utf8'));
    globalThis.document = dom.window.document;
    globalThis.bulmaCalendar = dom.window.bulmaCalendar;
    const fallback = document.querySelector('#day');
    const state = mountCalendar(document.querySelector('div'), fallback, () => {}, 'date', 'dialog');
    assert.ok(state);
    assert.equal(dom.window.getComputedStyle(fallback).display, 'none');

    const visible = state.picker._ui.dummy.dummy_1;
    assert.ok(visible.classList.contains('datetimepicker-dummy-input'), visible.className);
    assert.ok(visible.classList.contains('is-small'), visible.className);
    assert.ok(!visible.classList.contains('input'), visible.className);
    assert.deepEqual(Array.from(document.querySelectorAll('[data-testid="day"]')), [visible]);
    assert.deepEqual(Array.from(document.querySelectorAll('[data-cy="day-cy"]')), [visible]);

    unmountCalendar(state);
    assert.equal(dom.window.getComputedStyle(fallback).display, 'inline-flex');
    assert.equal(fallback.getAttribute('data-testid'), 'day');
    assert.equal(fallback.getAttribute('data-cy'), 'day-cy');
    dom.window.close();
});


test('Cancel closes the picker and leaves the value as it was', () => {
    assert.ok(process.env.BULMA_CALENDAR_JS, 'Provide the actual bulma-calendar 7.1.1 bundle');
    const dom = new JSDOM('<input id="day" type="date" class="input"><div></div>', { runScripts: 'outside-only' });
    dom.window.matchMedia = () => ({ matches: false });
    dom.window.eval(readFileSync(process.env.BULMA_CALENDAR_JS, 'utf8'));
    globalThis.document = dom.window.document;
    globalThis.bulmaCalendar = dom.window.bulmaCalendar;
    const values = [];
    const state = mountCalendar(document.querySelector('div'), document.querySelector('#day'),
        value => values.push(value), 'date', 'dialog');
    syncCalendar(state, '2026-10-03', false);
    // bulma's own Cancel handler, which does not close in a real browser, is
    // taken out so this checks the adapter's.
    const ownCancel = state.cancel.cloneNode(true);
    state.cancel.replaceWith(ownCancel);
    ownCancel.addEventListener('click', state.onCancel);
    state.picker.show();
    assert.equal(state.picker.isOpen(), true);
    ownCancel.click();
    assert.equal(state.picker.isOpen(), false, 'Cancel closes the picker');
    assert.equal(state.picker.value(), '2026-10-03');
    assert.deepEqual(values, [], 'and changes nothing');
    unmountCalendar(state);
    dom.window.close();
});
