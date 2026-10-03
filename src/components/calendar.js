// This adapter targets bulma-calendar 7.1.1. Keep its DOM separate from Yew's input.

// Test hooks belong to the control a person sees: while the picker is mounted
// that is the picker's input, so the hidden fallback gives them up.
const TEST_ATTRIBUTES = ['data-testid', 'data-cy'];

// The fallback's classes, minus `input`: the picker's input already sits in a
// box styled like one, and keeps its own class, which leaves room for the icon.
function modifierClasses(fallback) {
    return Array.from(fallback.classList).filter(name => name !== 'input');
}

// `hidden` alone is not enough: Bulma's `.input { display: inline-flex }` is an
// author rule and beats the user-agent rule behind the attribute, so the
// fallback stayed on screen beside the picker.
function hideFallback(state) {
    const fallback = state.fallback;
    state.fallbackDisplay = fallback.style.display;
    state.testAttributes = TEST_ATTRIBUTES.filter(name => fallback.hasAttribute(name))
        .map(name => [name, fallback.getAttribute(name)]);
    state.testAttributes.forEach(([name]) => fallback.removeAttribute(name));
    fallback.hidden = true;
    fallback.style.display = 'none';
}

function showFallback(state) {
    const fallback = state.fallback;
    fallback.hidden = false;
    fallback.style.display = state.fallbackDisplay || '';
    (state.testAttributes || []).forEach(([name, value]) => fallback.setAttribute(name, value));
    state.testAttributes = [];
}
export function mountCalendar(host, fallback, callback, kind, mode) {
    const Calendar = globalThis.bulmaCalendar;
    if (typeof Calendar !== 'function') return null;
    const input = document.createElement('input');
    input.className = fallback.className;
    host.appendChild(input);
    const state = { host, fallback, callback, input, picker: null, active: true, syncing: false, disabled: false };
    try {
        state.picker = new Calendar(input, {
            type: kind === 'datetime-local' ? 'datetime' : kind,
            dateFormat: 'yyyy-MM-dd', timeFormat: 'HH:mm',
            displayMode: ['default', 'dialog', 'inline'].includes(mode) ? mode : 'default',
            color: 'info', lang: 'en', showTodayButton: false
        });
        const emit = () => {
            if (!state.active || state.syncing || state.disabled) return;
            callback(state.picker.value());
        };
        state.picker.on('select', emit);
        state.picker.on('clear', () => {
            if (state.active && !state.syncing && !state.disabled) callback('');
        });
        state.picker.on('validate', () => { emit(); state.picker.hide(); });
        const visible = state.picker._ui.dummy.dummy_1;
        visible.classList.add(...modifierClasses(fallback));
        const labels = Array.from(fallback.labels || []);
        if (labels.length) visible.setAttribute('aria-label', labels.map(label => label.textContent.trim()).join(' '));
        for (const name of TEST_ATTRIBUTES) {
            if (fallback.hasAttribute(name)) visible.setAttribute(name, fallback.getAttribute(name));
        }
        state.labels = labels;
        state.onLabelClick = event => {
            event.preventDefault();
            event.stopPropagation();
            if (!state.disabled) { visible.focus(); state.picker.show(); }
        };
        labels.forEach(label => label.addEventListener('click', state.onLabelClick));
        hideFallback(state);
        return state;
    } catch (error) {
        state.picker = state.picker || input.bulmaCalendar;
        unmountCalendar(state);
        console.warn('Bulma Calendar unavailable; using native input.', error);
        return null;
    }
}

export function syncCalendar(state, value, disabled) {
    if (!state || !state.active) return;
    state.syncing = true;
    try {
        const picker = state.picker;
        const formatted = value.replace('T', ' ');
        if (picker.value() !== formatted) {
            if (formatted) picker.value(formatted);
            else { picker.hide(); picker.clear(); }
        }
        if (!formatted) picker._ui.dummy.dummy_1.value = '';
        state.disabled = disabled;
        if (disabled) picker.hide();
        state.host.querySelectorAll('input, button, select').forEach(control => { control.disabled = disabled; });
        state.host.inert = disabled;
        state.host.style.pointerEvents = disabled ? 'none' : '';
        state.host.style.opacity = disabled ? '0.6' : '';
        // Label clicks still target the original, stable input id.
        state.fallback.disabled = disabled;
    } catch (error) {
        unmountCalendar(state);
        console.warn('Bulma Calendar update failed; using native input.', error);
    } finally {
        state.syncing = false;
    }
}

export function unmountCalendar(state) {
    if (!state) return;
    state.active = false;
    state.callback = null;
    (state.labels || []).forEach(label => label.removeEventListener('click', state.onLabelClick));
    const picker = state.picker;
    if (picker) {
        // The upstream destroy() uses document.getElementById and does not unbind
        // body listeners. Never call it on DOM that Yew may already have removed.
        (picker._clickEvents || []).forEach(event => {
            document.body.removeEventListener(event, picker.onDocumentClickDateTimePicker);
        });
        try { picker.hide(); } catch (_) { /* already detached or partially initialized */ }
    }
    state.host.replaceChildren();
    state.host.inert = false;
    state.host.style.pointerEvents = '';
    state.host.style.opacity = '';
    showFallback(state);
    state.picker = null;
}