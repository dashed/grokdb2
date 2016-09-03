const {
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,
    CARD_SETTINGS
} = require('global/constants');

const tabReducer = function(state = CARD_QUESTION, action) {

    switch(action.type) {
    case CARD_QUESTION:
    case CARD_ANSWER:
    case CARD_DESCRIPTION:
    case CARD_SETTINGS:
        state = action.type;
        break;

    default:
        state = CARD_QUESTION;
    }

    return state;
};

module.exports = tabReducer;
