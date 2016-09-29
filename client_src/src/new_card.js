const client = require('helpers/client_entry');

const maker = require('components/new_card');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
}

const preRenderState = window.__PRE_RENDER_STATE__;
const postRenderState = maker.initialState;

client(maker, preRenderState, postRenderState, document.getElementById('new_card_container'));
