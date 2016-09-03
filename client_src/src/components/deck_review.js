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

/* react components */

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
                    <hr className='is-marginless'/>
                </div>
            </div>
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
            <AdvancedControls />
        </div>
    );

};

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
            <div className='columns'>
                <div className='column'>
                    <RenderSourceTitleComponent
                        extraClasses='is-small'
                    />
                </div>
            </div>
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

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');
const boolReducer = require('reducers/bool');

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

    [CHOSEN_PERFORMANCE]: NOT_SELECTED

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
