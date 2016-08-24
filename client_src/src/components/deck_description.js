require('babel-polyfill');
global.Promise = require('bluebird');

const React = require('react');

const assign = require('lodash/assign');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});


const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,
    MARKDOWN_CONTENTS,

    DECK_DESCRIPTION,

    POST_TO
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const ToolBar = function() {
    return (
        <a className={classnames('button is-success')}>
            {'Edit'}
        </a>
    );
};

const __DeckDescriptionContainer = function(props) {

    const {
        mathjaxifyDeckName,
        fields: { description},
        submitting,
        handleSubmit,
        postURL
    } = props;

    // const __name = assign({}, name);
    const __description = assign({}, description);

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <ToolBar />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    adasd
                </div>
            </div>
        </div>
    );

};

const deckDescriptionContainerFactory = function(preRenderState) {

    return reduxForm(

        // config
        {
            form: 'deck_description',
            fields: ['description'],
            initialValues: {
                description: preRenderState[DECK_DESCRIPTION][MARKDOWN_CONTENTS]
            }
        },

        // mapStateToProps
        (state) => {
            return {
                postURL: state[POST_TO]
            };
        }

    )(__DeckDescriptionContainer);

};

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [POST_TO]: '',

    [DECK_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

/* exports */

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const __Component = deckDescriptionContainerFactory(preRenderState);

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
