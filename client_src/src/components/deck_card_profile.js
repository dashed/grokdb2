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
    CARD_IS_ACTIVE,

    POST_TO,
    IS_EDITING,
    CURRENT_TAB
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

const MORE_TIMESTAMPS = 'MORE_TIMESTAMPS';

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const CardTitle = require('components/dumb/card_title');
const ErrorComponent = require('components/dumb/error');

const __ToolBar = function(props) {

    const {isEditing, dispatch, submitting} = props;

    if(isEditing) {

        const {handleSubmit, postURL, resetForm} = props;

        const cancel = function() {
            resetForm();
        };

        return (
            <div className='control is-grouped'>
                <p className='control'>
                    <a
                        className={classnames('button is-success is-bold', {
                            'is-disabled': submitting ||
                                !shouldSaveCard(props.title, props.question),
                            'is-loading': submitting
                        })}
                        onClick={handleSubmit(saveCard.bind(null, dispatch, postURL))}
                        >
                        {'Save'}
                    </a>
                </p>
                <p className='control'>
                    <a
                        className={classnames('button is-danger is-bold')}
                        onClick={switchEditMode(dispatch, false, cancel)}>
                        {'Cancel & Discard'}
                    </a>
                </p>
            </div>
        );

    }

    return (
        <div className='control is-grouped'>
            <p className='control'>
                <a
                    className={classnames('button is-success is-bold')}
                    onClick={switchEditMode(dispatch, true)}>
                    {'Edit'}
                </a>
            </p>
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

        renderStyle.marginTop = '20px';
        renderStyle.marginBottom = '20px';

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

const __Timestamps = function(props) {

    const {dispatch, moreTimestamps} = props;

    if(moreTimestamps) {
        return (
            <div>
                <small>{'Created at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
                <small>{'Last edited at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
                <small>{'Last picked for review at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
                <small>{'Last answered at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
                <small><a onClick={switchTimestamps(dispatch, false)}>{'Less'}</a></small>
            </div>
        );
    }

    return (
        <div>
            <small>{'Created at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
            <small>{'Last edited at June 16, 2016 10:30 PM (5 hours ago)'}</small><br/>
            <small><a onClick={switchTimestamps(dispatch, true)}>{'More'}</a></small>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __Timestamps.propTypes = {
        moreTimestamps: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
    };
}

const Timestamps = connect(
    // mapStateToProps
    (state) => {
        return {
            moreTimestamps: state[MORE_TIMESTAMPS]
        };
    }
)(__Timestamps);

const __CardProfileContainer = function(props) {

    const {
        mathjaxifyCardTitle,
        fields: { title, description, question, answer, is_active },
        submitting,
        handleSubmit,
        resetForm,
        dispatch,
        currenTab,
        isEditing,
        error
    } = props;

    return (
        <div>
            <ErrorComponent error={error && error.message || ''} />
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <div className='level'>
                        <div className='level-item'>
                            <CardTitle
                                content={title.value}
                                mathjaxify={mathjaxifyCardTitle}
                                notice={'No card title rendered.  Click on "Source" tab and enter a card title.'}
                                isEditing={isEditing}
                                assignField={title}
                            />
                        </div>
                        <div className='level-right' style={{marginLeft: '10px'}}>
                            <div className='level-item'>
                                <ToolBar
                                    handleSubmit={handleSubmit}
                                    resetForm={resetForm}
                                    submitting={submitting}
                                    question={question.value || ''}
                                    title={title.value || ''}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceTitleComponent
                        extraClasses='is-small'
                    />
                </div>
            </div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <div className='level'>
                        <div className='level-left'>
                            <div className='level-item'>
                                <Timestamps />
                                {' '}
                            </div>
                        </div>
                        <div className='level-right'>
                            <div className='level-left'>
                                <label className='checkbox'>
                                    <input type='checkbox' {...assign({}, is_active)} disabled={!isEditing} />
                                    {' Active for review'}
                                </label>
                            </div>
                        </div>
                    </div>
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
        error: React.PropTypes.object.isRequired,
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
                currenTab: state[CURRENT_TAB],
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

const defaultRESTError = 'Unable to update card. Please try again.';
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
            return Promise.all([response.status, jsonDecode(response)]);
        })
        .then(function([statusCode, __jsonResponse]) {

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                reject({
                    _error: {
                        message: get(__jsonResponse, ['error'], defaultRESTError)
                    }
                });

                return;
                break;

            case 200: // Ok

                const newCard = __jsonResponse.payload;

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
                            payload: newCard.title
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
                            payload: newCard.description
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
                            payload: newCard.question
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
                            payload: newCard.answer
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
                            type: !!newCard.is_active
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

const switchTimestamps = function(dispatch, reveal = false) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [MORE_TIMESTAMPS],
                // action
                {
                    type: reveal
                }
            )
        );
    };
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
                [CURRENT_TAB],
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
const boolReducer = require('reducers/bool');
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
    [CURRENT_TAB]: CARD_QUESTION,

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

    [MORE_TIMESTAMPS]: false,

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
