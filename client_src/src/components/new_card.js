require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');
const get = require('lodash/get');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
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
const CardTitle = require('components/dumb/card_title');

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
                        shouldFocus={props.isQuestion}
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
        placeholder: React.PropTypes.string.isRequired,
        isQuestion: React.PropTypes.bool.isRequired,
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
            [MARKDOWN_VIEW]: state[ownProps.tab][MARKDOWN_VIEW],
            isQuestion: state.CURRENT_TAB === CARD_QUESTION,
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
        fields: { title, description, question, answer, is_active },
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
                    <label className='label' htmlFor='input-card-title'>{'Card Title'}</label>
                        <CardTitle
                            content={title.value}
                            mathjaxify={mathjaxifyCardTitle}
                            notice={'No card title rendered.  Click on "Source" tab and enter a card title.'}
                            isEditing
                            assignField={title}
                            shouldFocus={false}
                        />
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
                        <input type='checkbox' {...assign({}, is_active)} />
                        {' Active for review'}
                    </label>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                    className={classnames('button is-success', {
                        'is-disabled': submitting || !shouldAddCard(title.value, question.value),
                        'is-loading': submitting
                    })}
                    onClick={handleSubmit(addNewCard.bind(null, postURL))}
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

const newCardContainerFactory = function(preRenderState) {

    const NewCardContainer = reduxForm(

        // config
        {
            form: 'new_card',
            fields: ['title', 'description', 'question', 'answer', 'is_active'],
            initialValues: {
                title: get(preRenderState, [CARD_TITLE, MARKDOWN_CONTENTS], ''),
                description: get(preRenderState, [CARD_DESCRIPTION, MARKDOWN_CONTENTS], ''),
                question: get(preRenderState, [CARD_QUESTION, MARKDOWN_CONTENTS], ''),
                answer: get(preRenderState, [CARD_ANSWER, MARKDOWN_CONTENTS], ''),
                is_active: true
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

    return NewCardContainer;

};

/* redux action dispatchers */
// NOTE: FSA compliant

const defaultRESTError = 'Unable to create new card. Please try again.';
const addNewCard = function(postURL, formData) {

    return new Promise((resolve, reject) => {

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                title: String(formData.title).trim(),
                description: String(formData.description).trim(),
                question: String(formData.question).trim(),
                answer: String(formData.answer).trim(),
                is_active: !!(formData.is_active),
            })
        })
        .then(function(response) {
            return Promise.all([response.status, jsonDecode(response)]);
        })
        .then(function([statusCode, jsonResponse]) {

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                reject({
                    _error: {
                        message: get(jsonResponse, ['error'], defaultRESTError)
                    }
                });

                return;
                break;

            case 200: // Ok

                window.location.href = jsonResponse.payload.profile_url;
                break;

            default: // Unexpected http status code
                reject({
                    _error: {
                        message: defaultRESTError
                    }
                });
            }

        })
        .catch(function(/*err*/) {

            // any other errors
            // console.log('err:', err);

            reject({
                _error: {
                    message: defaultRESTError
                }
            });
        });

    });
};

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
const tabReducer = require('reducers/card_tab');

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

    const __Component = newCardContainerFactory(store.getState());

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;

/* helpers */

const shouldAddCard = function(title, question) {

    const __title = String(title).trim();

    if(__title.length > 0) {
        return true;
    }

    const __question = String(question).trim();

    if(__question.length > 0) {
        return true;
    }

    return false;
}
