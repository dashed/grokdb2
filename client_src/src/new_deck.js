const client = require('helpers/client_entry');

const maker = require('components/new_deck');

// TODO: debug
// const initialState = require('lodash').merge({}, maker.initialState);
// initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
// window.__INITIAL_STATE__ = initialState;

client(maker, window.__INITIAL_STATE__, document.getElementById('new-deck-container'));
