const React = require('react');
const classnames = require('classnames');

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

} = require('global/constants');

const RenderSourceComponent = function(props) {

    const {switchTab, extraClasses, reverse} = props;
    const currentTab = props[MARKDOWN_VIEW];

    if (reverse) {

        return (
            <div>
                <a
                    href='#source'
                    className={classnames(
                        extraClasses,
                        'button',
                        {'is-primary': currentTab === MARKDOWN_VIEW_SOURCE}
                    )}
                    onClick={switchTab(MARKDOWN_VIEW_SOURCE)}
                >{'Source'}</a>
                {' '}
                <a
                    href='#render'
                    className={classnames(
                        extraClasses,
                        'button',
                        {'is-primary': currentTab === MARKDOWN_VIEW_RENDER}
                    )}
                    onClick={switchTab(MARKDOWN_VIEW_RENDER)}
                >{'Render'}</a>
            </div>
        );

    }

    return (
        <div>
            <a
                href='#render'
                className={classnames(
                    extraClasses,
                    'button',
                    {'is-primary': currentTab === MARKDOWN_VIEW_RENDER}
                )}
                onClick={switchTab(MARKDOWN_VIEW_RENDER)}
            >{'Render'}</a>
            {' '}
            <a
                href='#source'
                className={classnames(
                    extraClasses,
                    'button',
                    {'is-primary': currentTab === MARKDOWN_VIEW_SOURCE}
                )}
                onClick={switchTab(MARKDOWN_VIEW_SOURCE)}
            >{'Source'}</a>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {

    RenderSourceComponent.propTypes = {
        [MARKDOWN_VIEW]: React.PropTypes.oneOf([MARKDOWN_VIEW_RENDER, MARKDOWN_VIEW_SOURCE]),
        switchTab: React.PropTypes.func,
        extraClasses: React.PropTypes.string,
        reverse: React.PropTypes.bool
    };

    RenderSourceComponent.defaultProps = {
        extraClasses: '',
        reverse: false
    };
}

module.exports = RenderSourceComponent;
