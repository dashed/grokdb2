const React = require('react');

const markdownParser = require('markdown-it')({
    html: true,
    linkify: true
})
// custom plugin to mark mathjax markup to not be escaped by markdown-it
// and related plugins
// .use(require('helpers/mathjaxinline'))

// load with plugins (officially supported by markdown-it org)
.use(require('markdown-it-link-target'));

const generateMarkdown = (contents) => {
    return {
        __html: markdownParser.render(contents)
    };
}

const MarkdownRender = React.createClass({
    propTypes: {
        contents: React.PropTypes.string.isRequired
    },

    render() {
        return (
            <div
                ref="markdown_render"
                dangerouslySetInnerHTML={generateMarkdown(this.props.contents || '')}
            />
        );
    }
});

module.exports = MarkdownRender;
