const {

    ERROR_MESSAGE

} = require('global/constants');

const DEFAULT = {
    type: {}
};

const errorReducer = function(state, action = DEFAULT) {

    switch(action.type) {
    case ERROR_MESSAGE:

        state = {
            message: action.payload || ''
        };

        break;
    default:

        state = {
            message: ''
        };
    }

    return state;
};

module.exports = errorReducer;
