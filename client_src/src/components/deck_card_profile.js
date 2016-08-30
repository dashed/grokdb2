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
    CARD_IS_ACTIVE,

    POST_TO,
    IS_EDITING
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');

const __ToolBar = function(props) {

    const {isEditing, dispatch, submitting} = props;

    if(isEditing) {

        const {handleSubmit, postURL, resetForm} = props;

        const cancel = function() {
            resetForm();
        };

        return (
            <div className='level'>
                <div className='level-left'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-success', {
                                'is-disabled': submitting ||
                                    !shouldSaveCard(props.title, props.question),
                                'is-loading': submitting
                            })}
                            onClick={handleSubmit(saveCard.bind(null, dispatch, postURL))}
                            >
                            {'Save'}
                        </a>
                    </div>
                </div>
                <div className='level-right'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-danger')}
                            onClick={switchEditMode(dispatch, false, cancel)}>
                            {'Cancel & Discard'}
                        </a>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className='level'>
            <div className='level-left'>
                <div className='level-item'>
                    <a
                        className={classnames('button is-success')}
                        onClick={switchEditMode(dispatch, true)}>
                        {'Edit'}
                    </a>
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __ToolBar.propTypes = {
        isEditing: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        resetForm: React.PropTypes.func.isRequired,
        postURL: React.PropTypes.string.isRequired,
        title: React.PropTypes.string.isRequired,
        question: React.PropTypes.string.isRequired,
    };
}

const ToolBar = connect(
    // mapStateToProps
    (state) => {
        return {
            isEditing: state[IS_EDITING],
            postURL: state[POST_TO]
        };
    }

)(__ToolBar);

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
                        editable={props.isEditing}
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
        isEditing: React.PropTypes.bool.isRequired
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

    return (
        <div>
            <div key='question' style={questionStyle}>
                <TabComponent
                    tab={CARD_QUESTION}
                    placeholder={'Card Question'}
                    reduxFormField={props.question}
                    isEditing={props.isEditing}
                />
            </div>
            <div key='answer' style={answerStyle}>
                <TabComponent
                    tab={CARD_ANSWER}
                    placeholder={'Card Answer'}
                    reduxFormField={props.answer}
                    isEditing={props.isEditing}
                />
            </div>
            <div key='description' style={descriptionStyle}>
                <TabComponent
                    tab={CARD_DESCRIPTION}
                    placeholder={'Card Description'}
                    reduxFormField={props.description}
                    isEditing={props.isEditing}
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
        isEditing: React.PropTypes.bool.isRequired,
    };
}

const __CardProfileContainer = function(props) {

    const {
        mathjaxifyCardTitle,
        fields: { title, description, question, answer, is_active },
        submitting,
        handleSubmit,
        resetForm,
        // TODO: remove
        // postURL,
        dispatch,
        currenTab,
        isEditing
    } = props;

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <ToolBar
                        handleSubmit={handleSubmit}
                        resetForm={resetForm}
                        submitting={submitting}
                        question={question.value || ''}
                        title={title.value || ''}
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
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
                                    disabled={!isEditing}
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
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <label className='checkbox'>
                        <input type='checkbox' {...assign({}, is_active)} disabled={!isEditing} />
                        {' Active for review'}
                    </label>
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
                        isEditing={isEditing}
                    />
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __CardProfileContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        mathjaxifyCardTitle: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION]),
        dispatch: React.PropTypes.func.isRequired,
        resetForm: React.PropTypes.func.isRequired,
        isEditing: React.PropTypes.bool.isRequired,
    };
}

const cardProfileContainerFactory = function(preRenderState) {

    const CardProfileContainer = reduxForm(

        // config
        {
            form: 'card',
            fields: ['title', 'description', 'question', 'answer', 'is_active'],
            initialValues: {
                title: preRenderState[CARD_TITLE][MARKDOWN_CONTENTS],
                description: preRenderState[CARD_DESCRIPTION][MARKDOWN_CONTENTS],
                question: preRenderState[CARD_QUESTION][MARKDOWN_CONTENTS],
                answer: preRenderState[CARD_ANSWER][MARKDOWN_CONTENTS],
                is_active: preRenderState[CARD_IS_ACTIVE].VALUE,
            }
        },

        // mapStateToProps
        (state) => {
            return {
                mathjaxifyCardTitle: state[CARD_TITLE][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
                postURL: state[POST_TO],
                currenTab: state.CURRENT_TAB,
                isEditing: state[IS_EDITING],
                initialValues: {
                    title: state[CARD_TITLE][MARKDOWN_CONTENTS],
                    description: state[CARD_DESCRIPTION][MARKDOWN_CONTENTS],
                    question: state[CARD_QUESTION][MARKDOWN_CONTENTS],
                    answer: state[CARD_ANSWER][MARKDOWN_CONTENTS],
                    is_active: state[CARD_IS_ACTIVE].VALUE,
                }
            };
        }

    )(__CardProfileContainer);

    return CardProfileContainer;
};



/* redux action dispatchers */
// NOTE: FSA compliant

const saveCard = function(dispatch, postURL, formData) {

    return new Promise((resolve, reject) => {

        const title = String(formData.title).trim();
        const description = String(formData.description).trim();
        const question = String(formData.question).trim();
        const answer = String(formData.answer).trim();
        const isActive = !!formData.is_active;

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                title: title,
                description: description,
                question: question,
                answer: answer,
                is_active: isActive
            })
        })
        .then(function(response) {

            let jsonResponse = {};
            try {
                jsonResponse = response.json();
            } catch(_err) {
                jsonResponse = {};
            };

            return Promise.all([response.status, jsonResponse]);
        }, function(/*err*/) {

            // network error
            // console.log('network err:', err);

            reject({
                _error: {
                    message: 'Unable to send request to update card. Please try again.'
                }
            });
        })
        .then(function([statusCode, jsonResponse]) {

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                reject({
                    _error: {
                        message: jsonResponse.userMessage
                    }
                });

                return;
                break;

            case 200: // Ok

                // update card contents
                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [CARD_TITLE, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: jsonResponse.title
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [CARD_DESCRIPTION, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: jsonResponse.description
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [CARD_QUESTION, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: jsonResponse.question
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [CARD_ANSWER, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: jsonResponse.answer
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        isActiveReducer,
                        // path
                        [CARD_IS_ACTIVE, 'VALUE'],
                        // action
                        {
                            type: !!jsonResponse.is_active
                        }
                    )
                );

                // switch out of edit mode
                dispatch(
                    reduceIn(
                        // reducer
                        editingReducer,
                        // path
                        [IS_EDITING],
                        // action
                        {
                            type: false
                        }
                    )
                );

                // reset form to render mode
                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [CARD_TITLE, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [CARD_DESCRIPTION, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [CARD_QUESTION, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [CARD_ANSWER, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                resolve();

                break;

            default: // Unexpected http status code
                reject({
                    _error: {
                        message: 'Unable to update deck description.'
                    }
                });
            }

        }, function(/*err*/) {

            // json parsing fail
            // console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to update deck description.'
                }
            });
        })
        .catch(function(/*err*/) {

            // any other errors
            // console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to update deck description.'
                }
            });
        });

    });

};

const NOTHING = function() {};
const switchEditMode = function(dispatch, isEditing, after = NOTHING) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                editingReducer,
                // path
                [IS_EDITING],
                // action
                {
                    type: isEditing
                }
            )
        );
        after();

        const newMarkdownView = isEditing ? MARKDOWN_VIEW_SOURCE : MARKDOWN_VIEW_RENDER;

        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [CARD_TITLE, MARKDOWN_VIEW],
                // action
                {
                    type: newMarkdownView
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [CARD_DESCRIPTION, MARKDOWN_VIEW],
                // action
                {
                    type: newMarkdownView
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [CARD_QUESTION, MARKDOWN_VIEW],
                // action
                {
                    type: newMarkdownView
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [CARD_ANSWER, MARKDOWN_VIEW],
                // action
                {
                    type: newMarkdownView
                }
            )
        );
    }
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
    };
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');
const tabReducer = require('reducers/card_tab');
const editingReducer = require('reducers/bool');
const isActiveReducer = require('reducers/bool');
const markdownContentsReducer = require('reducers/markdown_contents');

/* default state */

const initialState = {

    [POST_TO]: '',

    [IS_EDITING]: false,

    [CARD_TITLE]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        // NOTE: contents is stored and handled by redux-form
    },

    // CURRENT_TAB = CARD_QUESTION | CARD_ANSWER | CARD_DESCRIPTION
    'CURRENT_TAB': CARD_QUESTION,

    [CARD_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        // NOTE: contents is stored and handled by redux-form
    },

    [CARD_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        // NOTE: contents is stored and handled by redux-form
    },

    [CARD_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        // NOTE: contents is stored and handled by redux-form
    },

    [CARD_IS_ACTIVE]: {
        // [VALUE]
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};


/* exports */

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const __Component = cardProfileContainerFactory(store.getState());

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;

/* helpers */

const shouldSaveCard = function(title, question) {

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
