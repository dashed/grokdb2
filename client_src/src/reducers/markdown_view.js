const {
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE
} = require('global/constants');

const markdownViewReducer = function(state = MARKDOWN_VIEW_RENDER, action) {

    switch(action.type) {
    case MARKDOWN_VIEW_RENDER:
    case MARKDOWN_VIEW_SOURCE:
        state = action.type;
        break;

    default:
        state = MARKDOWN_VIEW_RENDER;
    }

    return state;
};

module.exports = markdownViewReducer;
