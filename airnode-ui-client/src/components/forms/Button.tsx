import './Button.css';

export interface ButtonProps {
    disabled?: boolean
    type?: string
    label: string
    onClick?: Function 
    width?: string
}

export const Button = (props: ButtonProps) => {
    const className = '' + 
        ((props.type) ? ' ' + props.type: ' default') + 
        ((props.disabled) ? ' disabled' : '');

    const btnElem = (
        <button 
            className={'button ' + className}
            disabled={props.disabled}
            onClick={() => {
            if (typeof props.onClick === 'function') props.onClick();
        }}>
            {props.label}
        </button>
    )
    if (props.type == 'link') return btnElem;
    const style: any = {};
    if (props.width) style.width = props.width;
    return (
        <div className={'button-wrapper ' + className} style={style}>
            {btnElem}
            <div className={'underline ' + className}></div>
        </div>
    );
}

