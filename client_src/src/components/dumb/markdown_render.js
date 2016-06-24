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


const MarkdownRender = React.createClass({
    propTypes: {
        contents: React.PropTypes.string.isRequired
    },

    render() {

        const content = markdownParser.render(this.props.contents || '').trim();

        if(content.length > 0) {
            return (
                <div
                    ref='markdown_render'
                    dangerouslySetInnerHTML={{__html: content}}
                />
            );
        }

        // NOTE: dangerouslySetInnerHTML is not applied here

        return (
            <div>
                <div className='toast toast-primary'>
                    {'No content was rendered. Click on source tab and enter some text.'}
                </div>
                <div
                    ref='markdown_render'
                />
            </div>
        );


    }
});

module.exports = MarkdownRender;
