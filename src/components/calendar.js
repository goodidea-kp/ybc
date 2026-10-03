// This adapter targets bulma-calendar 7.1.1. Keep its DOM separate from Yew's input.
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
        visible.className = fallback.className;
        const labels = Array.from(fallback.labels || []);
        if (labels.length) visible.setAttribute('aria-label', labels.map(label => label.textContent.trim()).join(' '));
        for (const name of ['data-testid', 'data-cy']) {
            if (fallback.hasAttribute(name)) visible.setAttribute(name, fallback.getAttribute(name));
        }
        state.labels = labels;
        state.onLabelClick = event => {
            event.preventDefault();
            event.stopPropagation();
            if (!state.disabled) { visible.focus(); state.picker.show(); }
        };
        labels.forEach(label => label.addEventListener('click', state.onLabelClick));
        fallback.hidden = true;
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
    state.fallback.hidden = false;
    state.picker = null;
}