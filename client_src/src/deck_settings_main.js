const client = require('helpers/client_entry');

if(process.env.NODE_ENV !== 'production') {
    const invariant = require('invariant');
    invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
    invariant(!window.__POST_RENDER_STATE__, 'we do not expect to consume window.__POST_RENDER_STATE__');
}

/* rename component */

const deckSettingsNameMaker = require('components/deck_settings/name');

const preRenderStateName = window.__PRE_RENDER_STATE__.NAME;
const postRenderStateName = deckSettingsNameMaker.initialState;

client(
    deckSettingsNameMaker,
    preRenderStateName,
    postRenderStateName, document.getElementById('deck_settings_main_name_container'));


/* delete component */

const deckSettingsDeleteMaker = require('components/deck_settings/delete');

const preRenderStateDelete = window.__PRE_RENDER_STATE__.DELETE;
const postRenderStateDelete = deckSettingsDeleteMaker.initialState;

client(
    deckSettingsDeleteMaker,
    preRenderStateDelete,
    postRenderStateDelete, document.getElementById('deck_settings_main_delete_container'));
