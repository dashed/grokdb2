const client = require('helpers/client_entry');

if(process.env.NODE_ENV !== 'production') {
    // TODO: uncomment
    // const invariant = require('invariant');
    // invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
    // invariant(!window.__POST_RENDER_STATE__, 'we do not expect to consume window.__POST_RENDER_STATE__');

    window.__PRE_RENDER_STATE__ = {
        DECK_NAME: {
            MARKDOWN_CONTENTS: 'rofl'
        }
    };
}

const deckSettingsNameMaker = require('components/deck_settings/name');

const preRenderState = window.__PRE_RENDER_STATE__;
const postRenderState = deckSettingsNameMaker.initialState;

client(
    deckSettingsNameMaker,
    preRenderState,
    postRenderState, document.getElementById('deck_settings_main_name_container'));
