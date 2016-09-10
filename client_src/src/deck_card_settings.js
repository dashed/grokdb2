const client = require('helpers/client_entry');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
    invariant(!window.__POST_RENDER_STATE__, 'we do not expect to consume window.__POST_RENDER_STATE__');
}

/* delete component */

const cardSettingsDeleteMaker = require('components/card_settings/delete');

const preRenderStateDelete = window.__PRE_RENDER_STATE__;
const postRenderStateDelete = cardSettingsDeleteMaker.initialState;

client(
    cardSettingsDeleteMaker,
    preRenderStateDelete,
    postRenderStateDelete, document.getElementById('card_settings_main_delete_container'));
