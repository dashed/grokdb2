/* constants */

/*
enum MarkdownView {
    Render,
    Source
}
 */
const MARKDOWN_VIEW = 'MARKDOWN_VIEW'; // key
const MARKDOWN_VIEW_RENDER = 'MARKDOWN_VIEW_RENDER';
const MARKDOWN_VIEW_SOURCE = 'MARKDOWN_VIEW_SOURCE';


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
const CARD_SETTINGS = 'CARD_SETTINGS';
const CARD_META = 'CARD_META';
const CARD_ID = 'CARD_ID';

const MARKDOWN_CONTENTS = 'MARKDOWN_CONTENTS';
const POST_TO = 'POST_TO'; // URL to send POST request
const IS_EDITING = 'IS_EDITING';

const VALUE = 'VALUE';

const IS_CONFIRM_SKIP = 'IS_CONFIRM_SKIP';
const CURRENT_TAB = 'CURRENT_TAB';

module.exports = {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    // keys
    DECK_NAME,
    DECK_DESCRIPTION,

    CARD_TITLE,
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,
    CARD_SETTINGS,
    CARD_IS_ACTIVE,
    CARD_META,
    CARD_ID,

    MARKDOWN_CONTENTS,
    POST_TO,
    IS_EDITING,

    VALUE,

    IS_CONFIRM_SKIP,
    CURRENT_TAB
};
