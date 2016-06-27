const client = require('helpers/client_entry');

const maker = require('components/deck_review');

// TODO: debug
const lodashMerge = require('lodash/merge');
const initialState = lodashMerge({}, maker.initialState);
initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
window.__POST_RENDER_STATE__ = initialState;

// NOTE: we don't expect to consume window.__PRE_RENDER_STATE__

const preRenderState = void 0;
const postRenderState = window.__POST_RENDER_STATE__;

client(maker, preRenderState, postRenderState, document.getElementById('deck-review-container'));

