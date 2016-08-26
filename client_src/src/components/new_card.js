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

    // TODO: remove
    // MARKDOWN_CONTENTS,

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


const RenderSourceTabComponent = connect(
    // mapStateToProps
    (state, ownProps) => {
        return {
            [MARKDOWN_VIEW]: state[ownProps.currenTab][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch, ownProps) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [ownProps.currenTab, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

const __TabComponent = function(props) {

    const markdownView = props[MARKDOWN_VIEW];

    let sourceStyle = {};
    let renderStyle = {};

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:
        sourceStyle.display = 'none';
        break;

    case MARKDOWN_VIEW_SOURCE:
    default:
        renderStyle.display = 'none';
    }

    const contents = props.reduxFormField.value;

    return (
        <div>
            <div style={renderStyle}>
                <MarkdownRender contents={contents} />
            </div>
            <div>
                <div style={sourceStyle}>
                    <MarkdownSource
                        contents={contents}
                        placeholder={props.placeholder}
                        assignProps={props.reduxFormField}
                        editable
                    />
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __TabComponent.propTypes = {
        [MARKDOWN_VIEW]: React.PropTypes.oneOf([MARKDOWN_VIEW_RENDER, MARKDOWN_VIEW_SOURCE]),
        reduxFormField: React.PropTypes.object.isRequired,
        placeholder: React.PropTypes.string.isRequired
    };
}

const TabComponent = connect(
    // mapStateToProps
    (state, ownProps) => {

        // validate ownProps.tab
        if(process.env.NODE_ENV !== 'production') {
            switch(ownProps.tab) {
            case CARD_QUESTION:
            case CARD_ANSWER:
            case CARD_DESCRIPTION:
                break;
            default:
                throw Error();
            }
        }

        return {
            [MARKDOWN_VIEW]: state[ownProps.tab][MARKDOWN_VIEW]
        };
    }
)(__TabComponent);

const TabGroupComponent = function(props) {

    let questionStyle = {display: 'none'};
    let answerStyle = {display: 'none'};
    let descriptionStyle = {display: 'none'};

    switch(props.currenTab) {
    case CARD_QUESTION:
        questionStyle = {};
        break;
    case CARD_ANSWER:
        answerStyle = {};
        break;
    case CARD_DESCRIPTION:
        descriptionStyle = {};
        break;
    }

    // TODO: complete

    return (
        <div>
            <div key='question' style={questionStyle}>
                <TabComponent
                    tab={CARD_QUESTION}
                    placeholder={'Card Question'}
                    reduxFormField={props.question}
                />
            </div>
            <div key='answer' style={answerStyle}>
                <TabComponent
                    tab={CARD_ANSWER}
                    placeholder={'Card Answer'}
                    reduxFormField={props.answer}
                />
            </div>
            <div key='description' style={descriptionStyle}>
                <TabComponent
                    tab={CARD_DESCRIPTION}
                    placeholder={'Card Description'}
                    reduxFormField={props.description}
                />
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    TabGroupComponent.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION]),
        question: React.PropTypes.object.isRequired,
        answer: React.PropTypes.object.isRequired,
        description: React.PropTypes.object.isRequired,
    };
}


const __NewCardContainer = function(props) {

    const {
        mathjaxifyCardTitle,
        fields: { title, description, question, answer },
        submitting,
        handleSubmit,
        postURL,
        dispatch,
        currenTab
    } = props;

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
                    <div className='tabs is-boxed'>
                        <ul className='is-left'>
                            <li
                                className={classnames({
                                    'is-active is-bold': currenTab === CARD_QUESTION,
                                })}>
                                <a
                                    href='#question'
                                    onClick={switchTab(dispatch, CARD_QUESTION)}
                                >
                                    <span>{'Question'}</span>
                                </a>
                            </li>
                            <li
                                className={classnames({
                                    'is-active is-bold': currenTab === CARD_ANSWER
                                })}>
                                <a
                                    href='#answer'
                                    onClick={switchTab(dispatch, CARD_ANSWER)}
                                >
                                    <span>{'Answer'}</span>
                                </a>
                            </li>
                        </ul>
                        <ul className='is-right'>
                            <li
                                className={classnames({
                                    'is-active is-bold': currenTab === CARD_DESCRIPTION
                                })}>
                                <a
                                    href='#description'
                                    onClick={switchTab(dispatch, CARD_DESCRIPTION)}
                                >
                                    <span>{'Description'}</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceTabComponent
                        currenTab={currenTab}
                        reverse
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <TabGroupComponent
                        currenTab={currenTab}
                        answer={assign({}, answer)}
                        question={assign({}, question)}
                        description={assign({}, description)}
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
                    <label className='checkbox'>
                        <input type='checkbox' checked />
                        {' Active for review'}
                    </label>
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
        mathjaxifyCardTitle: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION]),
        dispatch: React.PropTypes.func.isRequired
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
            mathjaxifyCardTitle: state[CARD_TITLE][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            postURL: state[POST_TO],
            currenTab: state.CURRENT_TAB
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

const switchTab = function(dispatch, newTab) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                tabReducer,
                // path
                ['CURRENT_TAB'],
                // action
                {
                    type: newTab
                }
            )
        );
    }
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');

const tabReducer = function(state = CARD_QUESTION, action) {

    switch(action.type) {
    case CARD_QUESTION:
    case CARD_ANSWER:
    case CARD_DESCRIPTION:
        state = action.type;
        break;

    default:
        state = CARD_QUESTION;
    }

    return state;
};

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
