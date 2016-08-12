const client = require('helpers/client_entry');

const maker = require('components/new_deck');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(!window.__POST_RENDER_STATE__, 'we do not expect to consume window.__POST_RENDER_STATE__');
    invariant(!window.__PRE_RENDER_STATE__, 'we do not expect to consume window.__PRE_RENDER_STATE__');
}

const preRenderState = void 0;
const postRenderState = maker.initialState;

client(maker, preRenderState, postRenderState, document.getElementById('new_deck_container'));
