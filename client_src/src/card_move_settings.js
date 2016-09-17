/*

- Find all elements with 'move_to_deck' class.
- Render move button

*/
require('global/normalize');


const forEach = require('lodash/forEach');

const bootstrap = require('components/card_settings/move');

const moveTo = window.__PRE_RENDER_STATE__.MOVE_TO;

forEach(document.getElementsByClassName('move_to_deck'), function(elem) {

    setTimeout(function(){
        const deckID = elem.getAttribute('data-deck-id');

        bootstrap(elem, deckID, moveTo);
    }, 0);

});
