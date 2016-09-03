require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');

const {Provider, connect} = require('react-redux');
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
    CARD_SETTINGS,
    CARD_META,

    POST_TO,
    VALUE,

    IS_CONFIRM_SKIP,
    CURRENT_TAB
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

// TODO: move to constants?
const SHOW_MAIN_CONTROLS = 'SHOW_MAIN_CONTROLS';

const CHOSEN_PERFORMANCE = 'CHOSEN_PERFORMANCE';
const NOT_SELECTED = 'NOT_SELECTED';
const RIGHT = 'RIGHT';
const WRONG = 'WRONG';
const FORGOT = 'FORGOT';
const SHOW_PREVIEW_SOURCE_BUTTONS = 'SHOW_PREVIEW_SOURCE_BUTTONS';

/* react components */

const CardTitle = require('components/dumb/card_title');
const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');

const __PreviewSourceTitleComponent = connect(
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

const __PreviewSourceTitleComponentWrap = function(props) {

    if(!props.revealPreviewSourceChecked) {
        return null;
    }

    return (
        <div className='columns'>
            <div className='column'>
                <__PreviewSourceTitleComponent
                    extraClasses='is-small'
                />
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __PreviewSourceTitleComponentWrap.propTypes = {
        revealPreviewSourceChecked: React.PropTypes.bool.isRequired,
    };
}

const PreviewSourceTitleComponentWrap = connect(
    // mapStateToProps
    (state) => {
        return {
            revealPreviewSourceChecked: state[SHOW_PREVIEW_SOURCE_BUTTONS]
        };
    }

)(__PreviewSourceTitleComponentWrap);

const PreviewSourceTabComponent = connect(
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

const __PerformanceControls = function(props) {

    const isConfirmSkip = props[IS_CONFIRM_SKIP];

    if(isConfirmSkip) {
        return null;
    }

    const showMainControls = props[SHOW_MAIN_CONTROLS];

    if(!showMainControls) {
        return null;
    }

    const chosenPerformance = props[CHOSEN_PERFORMANCE];

    const rightClassValue = chosenPerformance != RIGHT;
    const wrongClassValue = chosenPerformance != WRONG;
    const forgotClassValue = chosenPerformance != FORGOT;
    return (
        <div className='columns'>
            <div className='column is-one-third'>
                <a
                    className={classnames('button is-success is-fullwidth is-bold',
                        {
                            'is-outlined': rightClassValue
                        })}
                    onClick={switchPerformance(props.dispatch, RIGHT)}
                >
                    {'Right'}
                </a>
            </div>
            <div className='column is-one-third'>
                <a
                    className={classnames('button is-danger is-fullwidth is-bold',
                        {
                            'is-outlined': wrongClassValue
                        })}
                    onClick={switchPerformance(props.dispatch, WRONG)}
                >
                    {'Wrong'}
                </a>
            </div>
            <div className='column is-one-third'>
                <a
                    className={classnames('button is-warning is-fullwidth is-bold',
                        {
                            'is-outlined': forgotClassValue
                        })}
                    style={{color: '#978b52'}}
                    onClick={switchPerformance(props.dispatch, FORGOT)}
                >
                    {'Forgot'}
                </a>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __PerformanceControls.propTypes = {
        [IS_CONFIRM_SKIP]: React.PropTypes.bool.isRequired,
        [SHOW_MAIN_CONTROLS]: React.PropTypes.bool.isRequired,
        [CHOSEN_PERFORMANCE]: React.PropTypes.oneOf([
            NOT_SELECTED,
            FORGOT,
            RIGHT,
            WRONG
        ]),
        dispatch: React.PropTypes.func.isRequired
    };
}

const PerformanceControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP],
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS],
            [CHOSEN_PERFORMANCE]: state[CHOSEN_PERFORMANCE]
        };
    }

)(__PerformanceControls);

const __CommitButton = function(props) {

    const showMainControls = props[SHOW_MAIN_CONTROLS];

    if(showMainControls) {

        const chosenPerformance = props[CHOSEN_PERFORMANCE];

        if(chosenPerformance === NOT_SELECTED) {
            return (
                <a className='button is-fullwidth is-bold is-disabled'>
                    {'How well did you answer the card?'}
                </a>
            );
        }

        return (
            <a
                href='#next_card'
                className='button is-success is-fullwidth is-bold'
            >
                {'Next Card'}
            </a>
        );
    }

    const {dispatch} = props;

    return (
        <a
            href='#reveal_answer'
            className='button is-success is-fullwidth is-bold'
            onClick={shouldRevealAnswer(dispatch, true)}
        >
            {'Reveal Answer'}
        </a>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __CommitButton.propTypes = {
        [SHOW_MAIN_CONTROLS]: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        [CHOSEN_PERFORMANCE]: React.PropTypes.oneOf([
            NOT_SELECTED,
            FORGOT,
            RIGHT,
            WRONG
        ]),
    };
}

const CommitButton = connect(
    // mapStateToProps
    (state) => {
        return {
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS],
            [CHOSEN_PERFORMANCE]: state[CHOSEN_PERFORMANCE]
        };
    }

)(__CommitButton);

const __MainControls = function(props) {

    const {dispatch} = props;
    const isConfirmSkip = props[IS_CONFIRM_SKIP];

    if(isConfirmSkip) {
        return (

                <div className='columns'>
                    <div className='column is-half'>
                        <a
                            href='#confirm_skip'
                            className='button is-success is-fullwidth is-bold'
                        >
                            {'Yes, skip'}
                        </a>
                    </div>
                    <div className='column is-half'>
                        <a
                            href='#cancel_confirm_skip'
                            className='button is-danger is-fullwidth is-bold'
                            onClick={shouldConfirmSkip(dispatch, false)}
                        >
                            {'No, do not skip'}
                        </a>
                    </div>
                </div>
        );
    }

    return (
        <div className='columns'>
            <div className='column is-two-thirds'>
                <CommitButton />
            </div>
            <div className='column is-one-third'>
                <a
                    href='#confirm_skip'
                    className='button is-danger is-fullwidth is-bold'
                    onClick={shouldConfirmSkip(dispatch, true)}
                >
                    {'Skip Card'}
                </a>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __MainControls.propTypes = {
        [IS_CONFIRM_SKIP]: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired
    };
}

const MainControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP]
        };
    }

)(__MainControls);

const AdvancedControls = function() {

    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <h4 className='title is-4'>{'Advanced Review Controls'}</h4>
                </div>
            </div>

            <div className='columns'>
                <div className='column'>
                    <h5 className='title is-5'>{'Amount of time to wait to pick this card for review'}</h5>
                </div>
            </div>

            <div className='columns'>
                <div className='column is-two-thirds'>
                    <input className='input' type='number' min='1' value='3' />
                </div>
                <div className='column is-one-third'>
                    <div className='tabs is-toggle is-fullwidth'>
                        <ul>
                            <li>
                                <a>
                                    {'Minutes'}
                                </a>
                            </li>
                            <li>
                                <a>
                                    {'Hours'}
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>

            <div className='columns'>
                <div className='column'>
                    <h5 className='title is-5'>{'Number of other cards to review before picking this card for review'}</h5>
                </div>
            </div>

            <div className='columns'>
                <div className='column'>
                    <input className='input' type='number' min='0' value='1' />
                </div>
            </div>
        </div>
    );
};

const ReviewControls = function() {

    return (
        <div>
            <MainControls />
            <PerformanceControls />
        </div>
    );

};

const __Meta = function(props) {

    let cachedReviewProcedure = props.cachedReviewProcedure;

    try {
        cachedReviewProcedure = JSON.parse(cachedReviewProcedure);
    } catch(_err) {
        return (
            <div>
                {'No meta info.'}
            </div>
        );
    }

    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <strong>{'This card was chosen for require via: '}</strong>
                    {cachedReviewProcedure.sub_selection}
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __Meta.propTypes = {
        cachedReviewProcedure: React.PropTypes.string.isRequired,
    };
}

const Meta = connect(
    // mapStateToProps
    (state) => {
        return {
            cachedReviewProcedure: state[CARD_META] || ''
        };
    }

)(__Meta);

const __Settings = function(props) {

    const {dispatch, revealPreviewSourceChecked} = props;

    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <label className='checkbox'>
                        <input type='checkbox'
                            onChange={switchRevealPreviewSource(dispatch)}
                            checked={revealPreviewSourceChecked}
                        />
                        {' Reveal preview/source buttons'}
                    </label>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <AdvancedControls />
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __Settings.propTypes = {
        dispatch: React.PropTypes.func.isRequired,
        revealPreviewSourceChecked: React.PropTypes.bool.isRequired,
    };
}

const Settings = connect(
    // mapStateToProps
    (state) => {
        return {
            revealPreviewSourceChecked: state[SHOW_PREVIEW_SOURCE_BUTTONS]
        };
    }

)(__Settings);

const __AnswerTab = function(props) {

    const {shouldReveal} = props;

    if(!shouldReveal) {
        return null;
    }

    const {currenTab, dispatch} = props;

    return (
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
    );

};

if(process.env.NODE_ENV !== 'production') {
    __AnswerTab.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION, CARD_SETTINGS, CARD_META]),
        dispatch: React.PropTypes.func.isRequired,
        shouldReveal: React.PropTypes.bool.isRequired,
    };
}

const AnswerTab = connect(
    // mapStateToProps
    (state) => {
        return {
            currenTab: state[CURRENT_TAB],
            shouldReveal: state[SHOW_MAIN_CONTROLS]
        };
    }

)(__AnswerTab);

const __PreviewSource = function(props) {

    const {currenTab} = props;

    if (currenTab === CARD_SETTINGS) {
        return null;
    }

    if (currenTab === CARD_META) {
        return null;
    }

    if (!props.revealPreviewSource) {
        return null;
    }

    return (
        <div className='columns'>
            <div className='column'>
                <PreviewSourceTabComponent currenTab={currenTab} />
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __PreviewSource.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION, CARD_SETTINGS, CARD_META]),
        revealPreviewSource: React.PropTypes.bool.isRequired
    };
}

const PreviewSource = connect(
    // mapStateToProps
    (state) => {
        return {
            revealPreviewSource: state[SHOW_PREVIEW_SOURCE_BUTTONS]
        };
    }

)(__PreviewSource);

const __CardContentTabs = function(props) {

    const {currenTab, dispatch} = props;

    return (
        <div>
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
                            <AnswerTab />
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
                            <li
                                className={classnames({
                                    'is-active is-bold': currenTab === CARD_SETTINGS
                                })}>
                                <a
                                    href='#settings'
                                    onClick={switchTab(dispatch, CARD_SETTINGS)}
                                >
                                    <span>{'Settings'}</span>
                                </a>
                            </li>
                            <li
                                className={classnames({
                                    'is-active is-bold': currenTab === CARD_META
                                })}>
                                <a
                                    href='#meta'
                                    onClick={switchTab(dispatch, CARD_META)}
                                >
                                    <span>{'Meta'}</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <PreviewSource currenTab={currenTab} />
            <div className='columns'>
                <div className='column'>
                    <TabGroupComponent />
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __CardContentTabs.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION, CARD_SETTINGS, CARD_META]),
        dispatch: React.PropTypes.func.isRequired
    };
}

const CardContentTabs = connect(
    // mapStateToProps
    (state) => {
        return {
            currenTab: state[CURRENT_TAB],
        };
    }

)(__CardContentTabs);

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

    return (
        <div>
            <div style={renderStyle}>
                <MarkdownRender contents={props.contents} />
            </div>
            <div>
                <div style={sourceStyle}>
                    <MarkdownSource
                        contents={props.contents}
                        placeholder={props.placeholder}
                        editable={false}
                    />
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __TabComponent.propTypes = {
        [MARKDOWN_VIEW]: React.PropTypes.oneOf([MARKDOWN_VIEW_RENDER, MARKDOWN_VIEW_SOURCE]),
        contents: React.PropTypes.string.isRequired,
        placeholder: React.PropTypes.string.isRequired,
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

const __TabGroupComponent = function(props) {

    let questionStyle = {display: 'none'};
    let answerStyle = {display: 'none'};
    let descriptionStyle = {display: 'none'};
    let settingsStyle = {display: 'none'};
    let metaStyle = {display: 'none'};

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
    case CARD_SETTINGS:
        settingsStyle = {};
        break;
    case CARD_META:
        metaStyle = {};
        break;
    }

    return (
        <div>
            <div key='question' style={questionStyle}>
                <TabComponent
                    tab={CARD_QUESTION}
                    placeholder={'Card Question'}
                    contents={props.question}
                    isEditing={false}
                />
            </div>
            <div key='answer' style={answerStyle}>
                <TabComponent
                    tab={CARD_ANSWER}
                    placeholder={'Card Answer'}
                    contents={props.answer}
                    isEditing={false}
                />
            </div>
            <div key='description' style={descriptionStyle}>
                <TabComponent
                    tab={CARD_DESCRIPTION}
                    placeholder={'Card Description'}
                    contents={props.description}
                    isEditing={false}
                />
            </div>
            <div key='settings' style={settingsStyle}>
                <Settings />
            </div>
            <div key='meta' style={metaStyle}>
                <Meta />
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __TabGroupComponent.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION, CARD_SETTINGS, CARD_META]),
        question: React.PropTypes.string.isRequired,
        answer: React.PropTypes.string.isRequired,
        description: React.PropTypes.string.isRequired,
    };
}

const TabGroupComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            question: state[CARD_QUESTION][MARKDOWN_CONTENTS],
            answer: state[CARD_ANSWER][MARKDOWN_CONTENTS],
            description: state[CARD_DESCRIPTION][MARKDOWN_CONTENTS],
            currenTab: state[CURRENT_TAB],
        };
    }

)(__TabGroupComponent);

const __Card = function(props) {

    const {title, mathjaxifyCardTitle} = props;

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <CardTitle
                        content={title}
                        mathjaxify={mathjaxifyCardTitle}
                        notice={'No card title rendered.'}
                        isEditing={false}
                    />
                </div>
            </div>
            <PreviewSourceTitleComponentWrap />
            <CardContentTabs />
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __Card.propTypes = {
        title: React.PropTypes.string.isRequired,
        mathjaxifyCardTitle: React.PropTypes.bool.isRequired,
    };
}

const Card = connect(
    // mapStateToProps
    (state) => {
        return {
            mathjaxifyCardTitle: state[CARD_TITLE][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            title: state[CARD_TITLE][MARKDOWN_CONTENTS],
        };
    }

)(__Card);

const DeckReview = function() {

    return (
        <div>
            <Card />
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <ReviewControls />
        </div>
    );
};

/* redux action dispatchers */
// NOTE: FSA compliant

const shouldConfirmSkip = function(dispatch, isConfirming) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [IS_CONFIRM_SKIP],
                // action
                {
                    type: isConfirming
                }
            )
        );
    };
};

const shouldRevealAnswer = function(dispatch, shouldReveal) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SHOW_MAIN_CONTROLS],
                // action
                {
                    type: shouldReveal
                }
            )
        );
        dispatch(
            reduceIn(
                // reducer
                tabReducer,
                // path
                [CURRENT_TAB],
                // action
                {
                    type: CARD_ANSWER
                }
            )
        );
    };
};

const switchPerformance = function(dispatch, performanceValue) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                performanceReducer,
                // path
                [CHOSEN_PERFORMANCE],
                // action
                {
                    type: performanceValue
                }
            )
        );
    };
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

const switchRevealPreviewSource = function(dispatch) {
    return function(event) {
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SHOW_PREVIEW_SOURCE_BUTTONS],
                // action
                {
                    type: event.target.checked
                }
            )
        );
    };
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');
const boolReducer = require('reducers/bool');
const tabReducer = require('reducers/card_tab');

const performanceReducer = function(state = NOT_SELECTED, action) {

    switch(action.type) {
    case NOT_SELECTED:
    case RIGHT:
    case WRONG:
    case FORGOT:
        if(state === action.type && state != NOT_SELECTED) {
            state = NOT_SELECTED;
        } else {
            state = action.type;
        }

        break;
    default:
        state = NOT_SELECTED;
    }

    return state;
}

/* default state */

const initialState = {

    [POST_TO]: '',

    [CARD_TITLE]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    // CURRENT_TAB = CARD_QUESTION | CARD_ANSWER | CARD_DESCRIPTION
    [CURRENT_TAB]: CARD_QUESTION,

    [CARD_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_IS_ACTIVE]: {
        [VALUE]: true
    },

    [IS_CONFIRM_SKIP]: false,
    [SHOW_MAIN_CONTROLS]: false,
    [SHOW_PREVIEW_SOURCE_BUTTONS]: false,

    [CHOSEN_PERFORMANCE]: NOT_SELECTED,

    [CARD_META]: ''

};

/* exports */

const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <DeckReview />
        </Provider>
    );

    return component;

});

module.exports.initialState = initialState;
