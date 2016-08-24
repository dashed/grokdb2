const client = require('helpers/client_entry');

const maker = require('components/deck_description');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    // TODO: uncomment
    // invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
    // invariant(window.__POST_RENDER_STATE__, 'we expect to consume window.__POST_RENDER_STATE__');
}

const preRenderState = window.__PRE_RENDER_STATE__;
const postRenderState = window.__POST_RENDER_STATE__;

client(maker, preRenderState, postRenderState, document.getElementById('deck_description_container'));
