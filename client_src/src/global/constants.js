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


/* keys */
// deck props
const DECK_NAME = 'DECK_NAME';
const DECK_DESCRIPTION = 'DECK_DESCRIPTION';

// card props
const CARD_TITLE = 'CARD_TITLE';
const CARD_DESCRIPTION = 'CARD_DESCRIPTION';
const CARD_QUESTION = 'CARD_QUESTION';
const CARD_ANSWER = 'CARD_ANSWER';
const CARD_IS_ACTIVE = 'CARD_IS_ACTIVE';

const MARKDOWN_CONTENTS = 'MARKDOWN_CONTENTS';
const POST_TO = 'POST_TO'; // URL to send POST request
const IS_EDITING = 'IS_EDITING';

const VALUE = 'VALUE';

module.exports = {

    TAB,
    TAB_QUESTION,
    TAB_ANSWER,
    TAB_DESCRIPTION,

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    CARD_PERF_CONTROL_VIEW,
    CARD_PERF_CONTROL__INITIAL,
    CARD_PERF_CONTROL__DEFAULT_CHOICES,

    SKIPCARD_VIEW,
    SKIPCARD_INITIAL,
    SKIPCARD_CONFIRM,

    // keys
    DECK_NAME,
    DECK_DESCRIPTION,

    CARD_TITLE,
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,
    CARD_IS_ACTIVE,

    MARKDOWN_CONTENTS,
    POST_TO,
    IS_EDITING,

    VALUE
};
