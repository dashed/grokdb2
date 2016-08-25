const boolReducer = function(state = false, action) {
    switch(action.type) {
    case true:
    case false:
        state = action.type;
        break;
    default:
        state = false;
    }

    return state;
};

module.exports = boolReducer;
