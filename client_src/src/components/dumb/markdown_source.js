const React = require('react');
const TextareaAutosize = require('react-textarea-autosize').default;
const has = require('lodash/has');
const assign = require('lodash/assign');

const MarkdownSource = function(props) {

    const contents = has(props.assignProps, 'value') ?
        (props.assignProps.value || '') : (props.contents || '')

    let etc = {};

    if (props.id) {
        etc.id = props.id;
    }

    const assignProps = assign({}, props.assignProps);

    return (<TextareaAutosize

        style={props.style}
        useCacheForDOMMeasurements
        minRows={6}
        rows={6}
        maxRows={10}
        className='textarea'
        placeholder={props.placeholder}
        editable={props.editable}
        // NOTE: this disables scrolling; better approach is to use readOnly
        // disabled={!props.editable}
        readOnly={!props.editable}
        {...assignProps}
        {...etc}
        value={contents}

    />);
};

MarkdownSource.defaultProps = {
    placeholder: '',
    assignProps: {},
    contents: '',
    editable: false,
    id: void 0,
    style: {}
};

if(process.env.NODE_ENV !== 'production') {
    MarkdownSource.propTypes = {
        style: React.PropTypes.object.isRequired,
        contents: React.PropTypes.string.isRequired,
        editable: React.PropTypes.bool.isRequired,
        placeholder: React.PropTypes.string.isRequired,
        assignProps: React.PropTypes.object,
        id: React.PropTypes.oneOfType([
            React.PropTypes.string,
            React.PropTypes.oneOf([void 0])
        ])
    };
}

module.exports = MarkdownSource;
