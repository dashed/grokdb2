const client = require('helpers/client_entry');

const maker = require('components/new_deck');

// NOTE: we don't expect to consume window.__INITIAL_STATE__

client(maker, maker.initialState, document.getElementById('new-deck-container'));
