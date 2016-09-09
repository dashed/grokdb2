require('global/normalize');

const React = require('react');

const {Provider, connect} = require('react-redux');

const {

    DELETE_TO,

} = require('global/constants');

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [DELETE_TO]: '',

};

const DeleteDeck = function(props) {
    return (
        <div>
            {'delete'}
        </div>
    );
};

/* exports */

const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <DeleteDeck />
        </Provider>
    );

    return component;

});

module.exports.initialState = initialState;
