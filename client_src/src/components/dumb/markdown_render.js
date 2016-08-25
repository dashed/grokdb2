const React = require('react');

const MarkdownRenderInline = require('./markdown_render_inline');

const MarkdownRender = function(props) {

    const content = String(props.contents).trim();

    if(content.length > 0) {
        return (
            <MarkdownRenderInline contents={content} />
        );
    }

    if(!props.showNoContentMessage) {
        return null;
    }

    // NOTE: dangerouslySetInnerHTML is not applied here

    return (
        <div className='message is-info'>
            <div className='message-body'>
                {props.noContentMessage}
            </div>
        </div>
    );

};

MarkdownRender.defaultProps = {
    noContentMessage: 'No content was rendered. Click on "Source" tab and enter some text.',
    showNoContentMessage: true
};

MarkdownRender.propTypes = {
    contents: React.PropTypes.string.isRequired,
    noContentMessage: React.PropTypes.string.isRequired,
    showNoContentMessage: React.PropTypes.bool.isRequired
};

module.exports = MarkdownRender;
