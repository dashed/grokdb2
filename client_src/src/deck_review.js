const client = require('helpers/client_entry');

const maker = require('components/deck_review');

// TODO: debug
const lodashMerge = require('lodash/merge');
const initialState = lodashMerge({}, maker.initialState);
initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
window.__INITIAL_STATE__ = initialState;

client(maker, window.__INITIAL_STATE__, document.getElementById('deck-review-container'));
