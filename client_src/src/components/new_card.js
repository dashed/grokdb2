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

    CARD_TITLE,
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,

    POST_TO
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');

const RenderSourceTitleComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[CARD_TITLE][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [CARD_TITLE, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

const __NewCardContainer = function(props) {

    const {
        mathjaxifyCardTitle,
        fields: { title, description, question, answer },
        submitting,
        handleSubmit,
        postURL
    } = props;

    const __description = assign({}, description);

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <label className='label' htmlFor='input-card-title'>{'Title'}</label>
                        <MathJaxLine
                            content={title.value}
                            mathjaxify={mathjaxifyCardTitle}
                            notice={'No card title rendered.  Click on "Source" tab and enter a card title.'}
                        >
                            <p className='control'>
                                <input
                                    id='input-card-title'
                                    className='input'
                                    type='text'
                                    placeholder='Card Title'
                                    autoFocus
                                    {...assign({}, title)}
                                />
                            </p>
                        </MathJaxLine>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceTitleComponent
                        extraClasses='is-small'
                        reverse
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    something
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                    className={classnames('button is-success', {
                        'is-disabled': submitting || String(title.value).trim().length <= 0,
                        'is-loading': submitting
                    })}
                    // onClick={handleSubmit(addNewDeck.bind(null, postURL))}
                    >
                        {'Add Card'}
                    </a>
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __NewCardContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        mathjaxifyCardName: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired
    };
}

const NewCardContainer = reduxForm(

    // config
    {
        form: 'new_card',
        fields: ['title', 'description', 'question', 'answer'],
        initialValues: {
            title: '',
            description: '',
            question: '',
            answer: ''
        }
    },

    // mapStateToProps
    (state) => {
        return {
            mathjaxifyCardName: state[CARD_TITLE][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            postURL: state[POST_TO]
        };
    }

)(__NewCardContainer);

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

    [POST_TO]: '',

    [CARD_TITLE]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    // CURRENT_TAB = CARD_QUESTION | CARD_ANSWER | CARD_DESCRIPTION
    'CURRENT_TAB': CARD_QUESTION,

    [CARD_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    [CARD_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    [CARD_ANSWER]: {
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

    const component = (
        <Provider store={store}>
            <NewCardContainer />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
