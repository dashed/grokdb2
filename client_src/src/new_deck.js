const client = require('helpers/client_entry');

const maker = require('components/new_deck');

// NOTE: we don't expect to consume window.__POST_RENDER_STATE__

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(!window.__POST_RENDER_STATE__, "we don't expect to consume window.__POST_RENDER_STATE__");
    invariant(window.__PRE_RENDER_STATE__, "window.__PRE_RENDER_STATE__ not found");
}

const preRenderState = window.__PRE_RENDER_STATE__;
const postRenderState = maker.initialState;

client(maker, preRenderState, postRenderState, document.getElementById('new-deck-container'));
