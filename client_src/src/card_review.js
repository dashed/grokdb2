const client = require('helpers/client_entry');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
}

const maker = require('components/card_review');

const preRenderState = window.__PRE_RENDER_STATE__;
const postRenderState = maker.initialState;

client(maker, preRenderState, postRenderState, document.getElementById('card_review_container'));

