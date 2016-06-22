/* constants */

/*
enum Tab {
    Question,
    Answer,
    Description
}
 */
const TAB = 'TAB'; // key
const TAB_QUESTION = 'TAB_QUESTION';
const TAB_ANSWER = 'TAB_ANSWER';
const TAB_DESCRIPTION = 'TAB_DESCRIPTION';


/*
enum MarkdownView {
    Render,
    Source
}
 */
const MARKDOWN_VIEW = 'MARKDOWN_VIEW'; // key
const MARKDOWN_VIEW_RENDER = 'MARKDOWN_VIEW_RENDER';
const MARKDOWN_VIEW_SOURCE = 'MARKDOWN_VIEW_SOURCE';

// key
const CARD_CONTENTS = 'CARD_CONTENTS';


/*
// UI state machine
enum CardPerformanceControl {
    Initial, // empty ui
    DefaultChoices,
    CustomScore,
}
 */
const CARD_PERF_CONTROL_VIEW = 'CARD_PERF_CONTROL_VIEW'; // key
const CARD_PERF_CONTROL__INITIAL = 'CARD_PERF_CONTROL__INITIAL'; // empty UI
const CARD_PERF_CONTROL__DEFAULT_CHOICES = 'CARD_PERF_CONTROL__DEFAULT_CHOICES';


/*
enum SkipCardView {
    Initial, // not confirming skip
    Confirm
}
 */
const SKIPCARD_VIEW = 'SKIPCARD_VIEW';
const SKIPCARD_INITIAL = 'SKIPCARD_INITIAL';
const SKIPCARD_CONFIRM = 'SKIPCARD_CONFIRM';

module.exports = {

    TAB,
    TAB_QUESTION,
    TAB_ANSWER,
    TAB_DESCRIPTION,

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    CARD_CONTENTS,

    CARD_PERF_CONTROL_VIEW,
    CARD_PERF_CONTROL__INITIAL,
    CARD_PERF_CONTROL__DEFAULT_CHOICES,

    SKIPCARD_VIEW,
    SKIPCARD_INITIAL,
    SKIPCARD_CONFIRM,

};
