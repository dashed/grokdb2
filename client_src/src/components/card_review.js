require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');
const get = require('lodash/get');
const isPlainObject = require('lodash/isPlainObject');

const {Provider, connect} = require('react-redux');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const jsonDecode = require('helpers/json_decode');

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    MARKDOWN_CONTENTS,

    CARD_ID,
    CARD_TITLE,
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,
    CARD_IS_ACTIVE,
    CARD_SETTINGS,
    CARD_META,

    POST_TO,
    VALUE,

    IS_CONFIRM_SKIP,
    CURRENT_TAB,

    ERROR,
    ERROR_MESSAGE,

} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* exports */

const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <CardReview />
        </Provider>
    );

    return component;

});

module.exports.initialState = initialState;
