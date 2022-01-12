import React, { CSSProperties } from 'react';
import { Checked, Warn } from './Check';

export interface TextInputProps {
  type?: string
  id?: string
  name?: string
  value: any
  style?: CSSProperties
  onChange: Function
  onEnter?: Function
  onEsc?: Function
  prefix?: string
  postfix?: string
  children?: any
  min?: number
  max?: number
  placeholder?: string
  readOnly?: boolean
  maxLength?: number
  width?: number|string
  left?: boolean
  warning?: string
  success?: boolean
}

export interface TextInputState {
  isFocused: boolean
}

const stylePrefixes: CSSProperties = {
  position: 'absolute',
  top: '0px',
  left: '10px',
  fontWeight: 'normal',
  fontSize: '0.8rem',
  lineHeight: '2rem',
};
const stylePostfix: CSSProperties = {
  position: 'absolute',
  top: '5px',
  right: '5px',
  fontWeight: 'bold',
  fontSize: '1.1rem'
};
const styleCheck = {
  marginLeft: 10,
};

const safeType = (type: any): string => {
  if (type === 'currency') return 'number';
  if (type === 'phone') return 'phone';
  if (type === 'email') return 'email';
  if (typeof type === undefined) return 'text';
  if (!type) return 'text';
  return type;
}

const typeClass = (type: any): string => {
  if (type === 'currency') return 'currency';
  if (typeof type === undefined) return 'text';
  if (!type) return 'text';
  return type;
}

const convertToCurrency = (value: number|string) : string => {
  if (typeof value === 'undefined') {
    return '0';
  }
  return value.toString()
  // const val = value.toString().replace(/([a-zA-Z]|,)/g, '');
  // if (!isNaN(parseInt(val, 10))) {
  //   return val.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
  // } 
  // return '';
}

const convertCurrencyToNum = (value: number|string): string =>{
  return '' + value;
  // disabled to have only numbers input
  // return value.toString().replace(/([a-zA-Z]|,)/g, '');
}

export const TextInput = (props: TextInputProps) => {
  const refInput = React.createRef<HTMLInputElement>();
  
  const [state, setState] = React.useState<TextInputState>({
    isFocused: false
  })
  const onFocus = () => {
    setState({ isFocused: true });
    if (props.type === 'currency' && refInput.current) { 
      refInput.current.value = convertCurrencyToNum(refInput.current.value); 
    }
  }
  const onBlur = () =>  {
    setState({ isFocused: false });
    if (props.type === 'currency' && refInput.current) { 
      refInput.current.value = convertToCurrency(refInput.current.value);
    }
  }

  const handleKeyPress = (e: any) => {
    if (e.which === 13 && typeof props.onEnter === 'function') {
      props.onEnter();
    }
    if (e.which === 27 && typeof props.onEsc === 'function') {
      props.onEsc();
    }
  }

  const styleInput: CSSProperties = {
    paddingLeft: props.prefix ? '30px' : '',
    paddingRight: props.postfix ? '20px' : ''
  };
  const styleObj = Object.assign({}, props.style, styleInput);

  const onChange = (e: any) => {
    if (props.type === 'currency' && refInput.current) { 
       props.onChange(convertCurrencyToNum(refInput.current.value));
    } else {
      props.onChange(e.target.value);
    }
  };
  const safeValue = typeof props.value !== 'undefined' ? props.value : '';
  React.useEffect(() => {
    if (refInput.current && !state.isFocused) {
      if (props.type === 'currency') {
        refInput.current.value = convertToCurrency(safeValue); 
      } else {
        refInput.current.value = safeValue; 
      }
    }
    return () => {};
  });
  return (
    <div className={props.readOnly ? 'disabled text-input': 'text-input'}>
      <div className={props.left ? 'left it-' + typeClass(props.type) : 'full it-' + typeClass(props.type)} style={{ position: 'relative', width: props.width, float: props.left ? 'left' : 'none' }}>
        {props.prefix ? <div style={stylePrefixes} >{props.prefix}</div> : null}
        <input
          id={props.id}
          name={props.name}
          maxLength={props.maxLength}
          ref={refInput}
          onChange={onChange}
          onKeyPress={handleKeyPress}
          readOnly={props.readOnly}
          placeholder={props.placeholder}
          className='form-control'
          onFocus={onFocus} 
          onBlur={onBlur}
          style={styleObj}
          defaultValue={safeValue}
          type={props.type ? safeType(props.type) : 'text'}
          min={props.min}
          max={props.max}
        />
        {props.postfix ? <div style={stylePostfix} >{props.postfix}</div> : null}
        {props.warning ? <div style={styleCheck} title={props.warning}><Warn /></div> : null}
        {props.success ? <div style={styleCheck} ><Checked /></div> : null}
      </div>
      {props.children ? <div className='block-input__value-format-tip' >{props.children}</div> : ''}
      {props.left ? null : <div style={{ clear: 'both'}}></div>}
      {props.warning ? <div className='input-warn'>{props.warning}</div> : null}
    </div>
  );

}

export default TextInput;
