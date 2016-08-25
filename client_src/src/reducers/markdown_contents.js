const {
    MARKDOWN_CONTENTS
} = require('global/constants');

const markdownContentsReducer = function(state = '', action) {

    switch(action.type) {
    case MARKDOWN_CONTENTS:
        state = String(action.payload);
        break;
    default:
        state = '';
    }

    return state;
};

module.exports = markdownContentsReducer;
