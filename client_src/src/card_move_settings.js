/*

- Find all elements with 'move_to_deck' class.
- Render move button

*/
require('global/normalize');


const forEach = require('lodash/forEach');

const bootstrap = require('components/card_settings/move');

forEach(document.getElementsByClassName('move_to_deck'), function(elem) {

    setTimeout(function(){
        const deckID = elem.getAttribute('data-deck-id');
        const moveTo = elem.getAttribute('data-move-to');

        bootstrap(elem, deckID, moveTo);
    }, 0);

});
