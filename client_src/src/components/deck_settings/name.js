require('global/normalize');

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

    DECK_NAME,

    POST_TO,

} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MathJaxLine = require('components/dumb/mathjax_line');

const RenderSourceNameComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[DECK_NAME][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [DECK_NAME, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

const __DeckSettingsNameContainer = function(props) {

    const {
        mathjaxifyDeckName,
        fields: { name },
        submitting,
        handleSubmit,
    } = props;

    // TODO: needed?
    // props.showNoContentMessage

    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceNameComponent
                        extraClasses='is-small'
                        reverse
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <MathJaxLine
                        content={name.value}
                        mathjaxify={mathjaxifyDeckName}
                        notice={'No deck name rendered.  Click on "Source" tab and enter a name.'}
                    >
                        <p className='control'>
                            <input
                                id='input-deck-name'
                                className='input'
                                type='text'
                                placeholder='Enter a deck name'
                                autoFocus
                                {...assign({}, name)}
                            />
                        </p>
                    </MathJaxLine>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                        className={classnames('button is-success')}
                    >
                        {'Rename'}
                    </a>
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __DeckSettingsNameContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        resetForm: React.PropTypes.func.isRequired,
        showNoContentMessage: React.PropTypes.bool.isRequired,
        mathjaxifyDeckName: React.PropTypes.bool.isRequired,
    };
}

const deckSettingsNameContainerFactory = function(preRenderState) {

    return reduxForm(

        // config
        {
            form: 'deck_settings_name',
            fields: ['name'],
            // initialValues: {
            //     name: preRenderState[DECK_NAME][MARKDOWN_CONTENTS]
            // }
        },

        // mapStateToProps
        (state) => {
            return {
                postURL: state[POST_TO],
                initialValues: {
                    name: state[DECK_NAME][MARKDOWN_CONTENTS],
                },
                showNoContentMessage: state[DECK_NAME].showNoContentMessage,
                mathjaxifyDeckName: state[DECK_NAME][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            };
        }

    )(__DeckSettingsNameContainer);

};

/* redux action dispatchers */
// NOTE: FSA compliant

const switchMarkdownView = function(dispatch, path, markdownView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                path,
                // action
                {
                    type: markdownView
                }
            )
        );
    }
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [POST_TO]: '',

    [DECK_NAME]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        [MARKDOWN_CONTENTS]: 'foo',
        showNoContentMessage: false
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

/* exports */

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const __Component = deckSettingsNameContainerFactory(store.getState());

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
