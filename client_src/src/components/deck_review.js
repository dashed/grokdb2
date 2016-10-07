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

// TODO: move to constants?
const SHOW_MAIN_CONTROLS = 'SHOW_MAIN_CONTROLS';

const CHOSEN_PERFORMANCE = 'CHOSEN_PERFORMANCE';
const NOT_SELECTED = 'NOT_SELECTED';
const EASY = 'EASY';
const RIGHT = 'RIGHT';
const HARD = 'HARD';
const WRONG = 'WRONG';
const FORGOT = 'FORGOT';
const SHOW_PREVIEW_SOURCE_BUTTONS = 'SHOW_PREVIEW_SOURCE_BUTTONS';
const SUBMITTING = 'SUBMITTING';
const SET_CARD = 'SET_CARD';
const HAS_CARD_FOR_REVIEW = 'HAS_CARD_FOR_REVIEW';
const PROFILE_URL = 'PROFILE_URL';
const MINUTES = 'MINUTES';
const HOURS = 'HOURS';
const TIME_CONTROL = 'TIME_CONTROL';
const TIME_TILL_AVAILABLE_FOR_REVIEW = 'TIME_TILL_AVAILABLE_FOR_REVIEW';
const CARDS_TILL_AVAILABLE_FOR_REVIEW = 'CARDS_TILL_AVAILABLE_FOR_REVIEW';


/* react components */

const CardTitle = require('components/dumb/card_title');
const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const ErrorComponent = require('components/dumb/error');

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

    const easyClassValue = chosenPerformance != EASY;
    const rightClassValue = chosenPerformance != RIGHT;
    const hardClassValue = chosenPerformance != HARD;
    const wrongClassValue = chosenPerformance != WRONG;
    const forgotClassValue = chosenPerformance != FORGOT;

    const submitting = props.submitting;

    return (
        <div>
            <div className='columns'>
                <div className='column is-one-third'>
                    <a
                        href='#easy'
                        className={classnames('button is-success is-fullwidth is-bold is-medium',
                            {
                                'is-outlined': easyClassValue,
                                'is-disabled': submitting || !easyClassValue
                            })}
                        onClick={submitting ? noOp() : switchPerformance(props.dispatch, EASY)}
                    >
                        {'Easy'}
                    </a>
                </div>
                <div className='column is-one-third'>
                    <a
                        href='#right'
                        className={classnames('button is-success is-fullwidth is-bold is-medium',
                            {
                                'is-outlined': rightClassValue,
                                'is-disabled': submitting || !rightClassValue
                            })}
                        onClick={submitting ? noOp() : switchPerformance(props.dispatch, RIGHT)}
                    >
                        {'Right'}
                    </a>
                </div>
                <div className='column is-one-third'>
                    <a
                        href='#hard'
                        className={classnames('button is-success is-fullwidth is-bold is-medium',
                            {
                                'is-outlined': hardClassValue,
                                'is-disabled': submitting || !hardClassValue
                            })}
                        onClick={submitting ? noOp() : switchPerformance(props.dispatch, HARD)}
                    >
                        {'Hard'}
                    </a>
                </div>
            </div>
            <div className='columns'>
                <div className='column is-one-half'>
                    <a
                        href='#wrong'
                        className={classnames('button is-danger is-fullwidth is-bold is-medium',
                            {
                                'is-outlined': wrongClassValue,
                                'is-disabled': submitting || !wrongClassValue
                            })}
                        onClick={submitting ? noOp() : switchPerformance(props.dispatch, WRONG)}
                    >
                        {'Wrong'}
                    </a>
                </div>
                <div className='column is-one-half'>
                    <a
                        href='#forgot'
                        className={classnames('button is-warning is-fullwidth is-bold is-medium',
                            {
                                'is-outlined': forgotClassValue,
                                'is-disabled': submitting || !forgotClassValue
                            })}
                        // style={{
                        //     color: '#454552'
                        // }}
                        onClick={submitting ? noOp() : switchPerformance(props.dispatch, FORGOT)}
                    >
                        {'Forgot'}
                    </a>
                </div>
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
            EASY,
            RIGHT,
            HARD,
            WRONG
        ]),
        submitting: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired
    };
}

const PerformanceControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP],
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS],
            [CHOSEN_PERFORMANCE]: state[CHOSEN_PERFORMANCE],
            submitting: state[SUBMITTING],
        };
    }

)(__PerformanceControls);

const __CommitButton = function(props) {

    const showMainControls = props[SHOW_MAIN_CONTROLS];
    const {dispatch} = props;

    if(showMainControls) {

        const chosenPerformance = props[CHOSEN_PERFORMANCE];

        if(chosenPerformance === NOT_SELECTED) {
            return (
                <a
                    className='button is-fullwidth is-bold is-disabled is-medium'
                    style={{borderColor: 'transparent'}}
                >
                    {'How well did you answer the card?'}
                </a>
            );
        }

        return (
            <a
                href='#next_card'
                className={classnames('button is-success is-fullwidth is-bold is-medium', {
                    'is-disabled is-loading': props.submitting
                })}
                onClick={reviewCard(dispatch, props.postURL, props.reviewRequest, props.submitting)}
                // style={{color: '#454552'}}
            >
                {'Answer, and get next card'}
            </a>
        );
    }

    return (
        <a
            href='#reveal_answer'
            className='button is-success is-fullwidth is-bold is-medium'
            onClick={shouldRevealAnswer(dispatch, true)}
        >
            {'Reveal answer'}
        </a>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __CommitButton.propTypes = {
        [SHOW_MAIN_CONTROLS]: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        [CHOSEN_PERFORMANCE]: React.PropTypes.oneOf([
            NOT_SELECTED,
            FORGOT,
            EASY,
            RIGHT,
            HARD,
            WRONG
        ]),
        reviewRequest: React.PropTypes.object.isRequired,
    };
}

const CommitButton = connect(
    // mapStateToProps
    (state) => {
        return {
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS],
            [CHOSEN_PERFORMANCE]: state[CHOSEN_PERFORMANCE],
            submitting: state[SUBMITTING],
            postURL: state[POST_TO],
            reviewRequest: generateReviewRequest(state)
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
                            className={classnames('button is-success is-fullwidth is-bold is-medium', {
                                'is-disabled': props.submitting
                            })}
                            onClick={reviewCard(dispatch, props.postURL, props.reviewRequest, props.submitting)}
                        >
                            {'Yes, skip'}
                        </a>
                    </div>
                    <div className='column is-half'>
                        <a
                            href='#cancel_confirm_skip'
                            className={classnames('button is-danger is-fullwidth is-bold is-medium', {
                                'is-disabled': props.submitting
                            })}
                            onClick={shouldConfirmSkip(dispatch, false)}
                        >
                            {'No, do not skip'}
                        </a>
                    </div>
                </div>
        );
    }

    const skipCardOnClick = props.submitting ? noOp() : shouldConfirmSkip(dispatch, true);

    return (
        <div className='columns'>
            <div className='column is-two-thirds'>
                <CommitButton />
            </div>
            <div className='column is-one-third'>
                <a
                    href='#confirm_skip'
                    className={classnames('button is-danger is-fullwidth is-bold is-medium', {
                        'is-disabled': props.submitting
                    })}

                    onClick={skipCardOnClick}
                >
                    {'Skip card'}
                </a>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __MainControls.propTypes = {
        [IS_CONFIRM_SKIP]: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        reviewRequest: React.PropTypes.object.isRequired,
        postURL: React.PropTypes.string.isRequired,
    };
}

const MainControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP],
            submitting: state[SUBMITTING],
            reviewRequest: generateReviewRequest(state),
            postURL: state[POST_TO],
        };
    }

)(__MainControls);

const __AdvancedControls = function(props) {

    const isConfirmSkip = props[IS_CONFIRM_SKIP];
    const showMainControls = props[SHOW_MAIN_CONTROLS];

    if(!(isConfirmSkip || showMainControls)) {
        return null;
    }

    const timeControl = props[TIME_CONTROL];
    const {dispatch} = props;

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
                    <input
                        className='input'
                        type='number'
                        min={0}
                        value={props[TIME_TILL_AVAILABLE_FOR_REVIEW]}
                        onChange={changeTimeTillReview(dispatch)}
                    />
                </div>
                <div className='column is-one-third'>
                    <div className='tabs is-toggle is-fullwidth'>
                        <ul>
                            <li
                                className={classnames({
                                    'is-active': timeControl === MINUTES
                                })}
                            >
                                <a
                                    className={'button is-bold'}
                                    onClick={switchTime(dispatch, MINUTES, props[TIME_TILL_AVAILABLE_FOR_REVIEW])}
                                >
                                    {'Minutes'}
                                </a>
                            </li>
                            <li
                                className={classnames({
                                    'is-active': timeControl === HOURS
                                })}
                            >
                                <a
                                    className={'button is-bold'}
                                    onClick={switchTime(dispatch, HOURS, props[TIME_TILL_AVAILABLE_FOR_REVIEW])}
                                >
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
                    <input
                        className='input'
                        type='number'
                        min={0}
                        value={props[CARDS_TILL_AVAILABLE_FOR_REVIEW]}
                        onChange={changeCardsTillReview(dispatch)}
                    />
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __AdvancedControls.propTypes = {
        dispatch: React.PropTypes.func.isRequired
    };
}

const AdvancedControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP],
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS],
            [TIME_CONTROL]: state[TIME_CONTROL],
            [TIME_TILL_AVAILABLE_FOR_REVIEW]: state[TIME_TILL_AVAILABLE_FOR_REVIEW],
            [CARDS_TILL_AVAILABLE_FOR_REVIEW]: state[CARDS_TILL_AVAILABLE_FOR_REVIEW],
        };
    }

)(__AdvancedControls);

const __ReviewControls = function(props) {

    const {currenTab} = props;

    // Don't show main controls unless user is viewing question or answer
    if(!(currenTab === CARD_QUESTION || currenTab === CARD_ANSWER)) {
        return null;
    }

    return (
        <div>
            <MainControls />
            <PerformanceControls />
            <AdvancedControls />
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __ReviewControls.propTypes = {
        currenTab: React.PropTypes.oneOf([CARD_QUESTION, CARD_ANSWER, CARD_DESCRIPTION, CARD_SETTINGS, CARD_META]),
    };
}

const ReviewControls = connect(
    // mapStateToProps
    (state) => {
        return {
            currenTab: state[CURRENT_TAB],
        };
    }

)(__ReviewControls);

const __Meta = function(props) {

    let cachedReviewProcedure = props.cachedReviewProcedure;

    if(!cachedReviewProcedure || !cachedReviewProcedure.sub_selection) {
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
        cachedReviewProcedure: React.PropTypes.object.isRequired,
    };
}

const Meta = connect(
    // mapStateToProps
    (state) => {
        return {
            cachedReviewProcedure: state[CARD_META] || {}
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

        renderStyle.marginTop = '20px';
        renderStyle.marginBottom = '20px';
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

    const {title, mathjaxifyCardTitle, cardID, profileURL} = props;

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <div className='level'>
                        <div className='level-left'>
                            <div className='level-item'>
                                <CardTitle
                                    content={title}
                                    mathjaxify={mathjaxifyCardTitle}
                                    notice={'No card title rendered.'}
                                    isEditing={false}
                                />
                            </div>
                        </div>
                        <div className='level-right'>
                            <div className='level-item'>
                                <a href={profileURL} className='button is-outlined'>
                                    {'Card #' + cardID}
                                </a>
                            </div>
                        </div>
                    </div>

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
        cardID: React.PropTypes.number.isRequired,
        profileURL: React.PropTypes.string.isRequired,
    };
}

const Card = connect(
    // mapStateToProps
    (state) => {
        return {
            mathjaxifyCardTitle: state[CARD_TITLE][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            title: state[CARD_TITLE][MARKDOWN_CONTENTS],
            cardID: state[CARD_ID],
            profileURL: state[PROFILE_URL]
        };
    }

)(__Card);

const __DeckReview = function(props) {

    const {error, dispatch, hasCardForReview} = props;

    if(!hasCardForReview) {
        return (
            <div>
                <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
                <div className='columns'>
                    <div className='column'>
                        <div className='card is-fullwidth'>
                            <div className='card-content has-text-centered'>
                                <h1 className='title'>{'No card available for review'}</h1>

                                <a
                                    href='#fetch_card'
                                    className={classnames('button is-primary is-bold is-medium', {
                                        'is-disabled is-loading': props.fetchSubmitting
                                    })}
                                    onClick={fetchCard(dispatch, props.postURL, props.fetchSubmitting)}
                                >
                                    {'Fetch a card'}
                                </a>
                            </div>
                        </div>

                    </div>
                </div>

            </div>
        );
    }

    return (
        <div>
            <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
            <Card />
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
            <ReviewControls />
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __DeckReview.propTypes = {
        error: React.PropTypes.object,
        dispatch: React.PropTypes.func.isRequired,
        hasCardForReview: React.PropTypes.bool.isRequired,
        fetchSubmitting: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
    };
}


const DeckReview = connect(
    // mapStateToProps
    (state) => {
        return {
            error: state[ERROR],
            hasCardForReview: state[HAS_CARD_FOR_REVIEW],
            fetchSubmitting: state[SUBMITTING],
            postURL: state[POST_TO]
        };
    }
)(__DeckReview);

/* redux action dispatchers */
// NOTE: FSA compliant

const noOp = function() {
    return function(event) {
        event.preventDefault();
    };
};

const defaultFetchRESTError = 'Unable to fetch a card for review. Please try again.';
const fetchCard = function(dispatch, fetchURL, fetchSubmitting = true) {
    return function(event) {
        event.preventDefault();

        if(fetchSubmitting) {
            return;
        }

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: true
                }
            )
        );

        fetch(fetchURL, {
            method: 'GET'
        })
        .then(function(response) {
            return Promise.all([response.status, jsonDecode(response)]);
        })
        .then(function([statusCode, jsonResponse]) {

            dispatch(
                reduceIn(
                    // reducer
                    boolReducer,
                    // path
                    [SUBMITTING],
                    // action
                    {
                        type: false
                    }
                )
            );

            switch(statusCode) {
            case 500: // Internal Server Error

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], defaultFetchRESTError)
                        }
                    )
                );

                return;
                break;

            case 400: // Bad Request

                if(jsonResponse.payload) {

                    dispatch(
                        reduceIn(
                            // reducer
                            errorReducer,
                            // path
                            [ERROR],
                            // action
                            {
                                type: ERROR_MESSAGE,
                                payload: get(jsonResponse, ['error'], '')
                            }
                        )
                    );

                    dispatch(
                        reduceIn(
                            // reducer
                            cardReducer,
                            // path
                            [],
                            // action
                            {
                                type: SET_CARD,
                                payload: jsonResponse.payload
                            }
                        )
                    );

                } else {

                    dispatch(
                        reduceIn(
                            // reducer
                            errorReducer,
                            // path
                            [ERROR],
                            // action
                            {
                                type: ERROR_MESSAGE,
                                payload: get(jsonResponse, ['error'], 'Unable to review this card.')
                            }
                        )
                    );

                }

                return;
                break;

            case 200: // Ok

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], '')
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        cardReducer,
                        // path
                        [],
                        // action
                        {
                            type: SET_CARD,
                            payload: jsonResponse.payload
                        }
                    )
                );

                return;
                break;

            default: // Unexpected http status code
                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], defaultFetchRESTError)
                        }
                    )
                );
            }
        })
        .catch(function(/*err*/) {

            // any other errors
            // console.log('err:', err);

            dispatch(
                reduceIn(
                    // reducer
                    boolReducer,
                    // path
                    [SUBMITTING],
                    // action
                    {
                        type: false
                    }
                )
            );

            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: defaultFetchRESTError
                    }
                )
            );

        });
    };
};

const defaultRESTError = 'Unable to review this card. Please try again.';
const reviewCard = function(dispatch, postURL, reviewRequest, submitting = true) {
    return function(event) {
        event.preventDefault();

        if(submitting) {
            return;
        }

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: true
                }
            )
        );

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(reviewRequest)
        })
        .then(function(response) {
            return Promise.all([response.status, jsonDecode(response)]);
        })
        .then(function([statusCode, jsonResponse]) {

            dispatch(
                reduceIn(
                    // reducer
                    boolReducer,
                    // path
                    [SUBMITTING],
                    // action
                    {
                        type: false
                    }
                )
            );

            switch(statusCode) {
            case 500: // Internal Server Error

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], defaultRESTError)
                        }
                    )
                );

                return;
                break;

            case 400: // Bad Request

                if(jsonResponse.payload) {

                    dispatch(
                        reduceIn(
                            // reducer
                            errorReducer,
                            // path
                            [ERROR],
                            // action
                            {
                                type: ERROR_MESSAGE,
                                payload: get(jsonResponse, ['error'], '')
                            }
                        )
                    );

                    dispatch(
                        reduceIn(
                            // reducer
                            cardReducer,
                            // path
                            [],
                            // action
                            {
                                type: SET_CARD,
                                payload: jsonResponse.payload
                            }
                        )
                    );

                } else {

                    dispatch(
                        reduceIn(
                            // reducer
                            errorReducer,
                            // path
                            [ERROR],
                            // action
                            {
                                type: ERROR_MESSAGE,
                                payload: get(jsonResponse, ['error'], 'Unable to review this card.')
                            }
                        )
                    );

                }

                return;
                break;

            case 200: // Ok

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], '')
                        }
                    )
                );

                dispatch(
                    reduceIn(
                        // reducer
                        cardReducer,
                        // path
                        [],
                        // action
                        {
                            type: SET_CARD,
                            payload: jsonResponse.payload
                        }
                    )
                );

                return;
                break;

            default: // Unexpected http status code
                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], defaultRESTError)
                        }
                    )
                );
            }
        })
        .catch(function(/*err*/) {

            // any other errors
            // console.log('err:', err);

            dispatch(
                reduceIn(
                    // reducer
                    boolReducer,
                    // path
                    [SUBMITTING],
                    // action
                    {
                        type: false
                    }
                )
            );

            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: defaultRESTError
                    }
                )
            );

        });

    };
};

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

const confirmError = function(dispatch) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                errorReducer,
                // path
                [ERROR],
                // action
                {
                    type: ERROR_MESSAGE,
                    payload: ''
                }
            )
        );
    };
};

const switchTime = function(dispatch, timeType = HOURS, time = 0) {
    return function(event) {
        event.preventDefault();

        time = Number(time);

        if(timeType === MINUTES) {
            time = time * 60;
        } else {
            time = time / 60;
        }

        dispatch(
            reduceIn(
                // reducer
                typeReducer,
                // path
                [TIME_TILL_AVAILABLE_FOR_REVIEW],
                // action
                {
                    type: time
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                timeReducer,
                // path
                [TIME_CONTROL],
                // action
                {
                    type: timeType
                }
            )
        );
    };
};

const changeTimeTillReview = function(dispatch) {
    return function(event) {
        dispatch(
            reduceIn(
                // reducer
                typeReducer,
                // path
                [TIME_TILL_AVAILABLE_FOR_REVIEW],
                // action
                {
                    type: Number(event.target.value)
                }
            )
        );
    };
};

const changeCardsTillReview = function(dispatch) {
    return function(event) {
        dispatch(
            reduceIn(
                // reducer
                typeReducer,
                // path
                [CARDS_TILL_AVAILABLE_FOR_REVIEW],
                // action
                {
                    type: Number(event.target.value)
                }
            )
        );
    };
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');
const boolReducer = require('reducers/bool');
const tabReducer = require('reducers/card_tab');
const errorReducer = require('reducers/error_message');
const typeReducer = require('reducers/type');

const performanceReducer = function(state = NOT_SELECTED, action) {

    switch(action.type) {
    case NOT_SELECTED:
    case RIGHT:
    case WRONG:
    case FORGOT:
    case EASY:
    case HARD:
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

const generateReviewAction = function(performance) {

    switch(performance) {
    case EASY:
        return 'Easy';
    case HARD:
        return 'Hard';
    case RIGHT:
        return 'Right';
    case WRONG:
        return 'Wrong';
    case FORGOT:
        return 'Forgot';
    }

    return 'Skip';
};

const cardReducer = function(state, action) {

    const {payload} = action;

    let card = {
        id: 0,
        title: '',
        description: '',
        question: '',
        answer: '',
        is_active: false
    };
    let cardMeta = {};
    let hasCardForReview = false;
    let profileURL = '';
    let timeTillReview = 0;
    let cardsTillReview = 0;
    let timeControl = HOURS;

    switch(action.type) {
    case SET_CARD:

        if(payload.has_card_for_review) {

            hasCardForReview = true;

            profileURL = payload.card_for_review.profile_url;

            const __card = payload.card_for_review.card;

            cardMeta = isPlainObject(payload.card_for_review.card_meta) ?
                payload.card_for_review.card_meta : {};

            card = {
                id: __card.id,
                title: __card.title,
                description: __card.description,
                question: __card.question,
                answer: __card.answer,
                is_active: __card.is_active
            };

            timeTillReview = payload.card_for_review.card_score.review_after_normalized;
            cardsTillReview = payload.card_for_review.card_score.cards_till_ready_for_review;
            timeControl = payload.card_for_review.card_score.review_after_time_control;

        }

        break;
    }

    const newState = {

        [PROFILE_URL]: profileURL,

        [CARD_ID]: card.id,

        [CARD_TITLE]: {
            [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
            [MARKDOWN_CONTENTS]: card.title
        },

        [CURRENT_TAB]: CARD_QUESTION,

        [CARD_DESCRIPTION]: {
            [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
            [MARKDOWN_CONTENTS]: card.description
        },

        [CARD_QUESTION]: {
            [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
            [MARKDOWN_CONTENTS]: card.question
        },

        [CARD_ANSWER]: {
            [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
            [MARKDOWN_CONTENTS]: card.answer
        },

        [CARD_IS_ACTIVE]: {
            [VALUE]: card.is_active
        },

        [IS_CONFIRM_SKIP]: false,
        [SHOW_MAIN_CONTROLS]: false,
        [SHOW_PREVIEW_SOURCE_BUTTONS]: false,
        [HAS_CARD_FOR_REVIEW]: hasCardForReview,

        [CHOSEN_PERFORMANCE]: NOT_SELECTED,

        [CARD_META]: cardMeta,

        [TIME_TILL_AVAILABLE_FOR_REVIEW]: timeTillReview,
        [CARDS_TILL_AVAILABLE_FOR_REVIEW]: cardsTillReview,
        [TIME_CONTROL]: timeControl,

    };

    return assign({}, state, newState);
};

const timeReducer = function(state, action) {

    switch(action.type) {
    case MINUTES:
    case HOURS:
        state = action.type;

        break;

    default:
        state = HOURS;
    }

    return state;
}

// TODO: this is not a reducer; move this somewhere
const generateReviewRequest = function(state) {

    let timeTillReview = Number(state[TIME_TILL_AVAILABLE_FOR_REVIEW]);

    if(state[TIME_CONTROL] === HOURS) {
        timeTillReview = timeTillReview * 60;
    }

    timeTillReview = timeTillReview < 0 ? (3 * 60) : // 3 hours = 180 mins
        timeTillReview;

    let cardsTillReview = Number(state[CARDS_TILL_AVAILABLE_FOR_REVIEW]);

    cardsTillReview = cardsTillReview < 0 ? 1 :
        cardsTillReview;

    return {
        card_id: Number(state[CARD_ID]),
        review_action: generateReviewAction(!!state[IS_CONFIRM_SKIP] ? void 0 : state[CHOSEN_PERFORMANCE]),

        time_till_available_for_review:  timeTillReview,
        cards_till_available_for_review: cardsTillReview
    };
};

/* app state */

const initialState = {

    [POST_TO]: '',
    [PROFILE_URL]: '',

    [CARD_ID]: 0,

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
    [HAS_CARD_FOR_REVIEW]: true,

    [CHOSEN_PERFORMANCE]: NOT_SELECTED,

    [CARD_META]: {},

    [SUBMITTING]: false,

    [TIME_TILL_AVAILABLE_FOR_REVIEW]: 3,
    [CARDS_TILL_AVAILABLE_FOR_REVIEW]: 1,
    [TIME_CONTROL]: HOURS,

    [ERROR]: errorReducer(),

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
