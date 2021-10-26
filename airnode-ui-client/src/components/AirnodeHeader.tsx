import { Link } from 'react-router-dom';
import api3 from './../assets/api3.svg';

import './AirnodeHeader.css';

interface HeaderProps {
    onClickSelect?: Function
    filter?: string
}

export const AirnodeHeader = (props: HeaderProps) => (
  <header>
    <div className="inner">
        <div className="hd-column">
            <button className="select" onClick={() => { 
                props.onClickSelect && props.onClickSelect() 
            }} style={{
                background: '#000',
                color: 'white',
                display: 'flex',
                cursor: 'pointer',
                paddingTop: 10,
                paddingBottom: 10,
                alignItems: 'center',
                width: 175,
            }}>
                <img src={api3} alt='API3' style={{ width: '30%', height: '30%'}} />
                <div style={{ whiteSpace: "nowrap" }}>Choose Airnode</div>
            </button>
        </div>
        <div className="hd-column-wide">
            <div className="chain-row">
                <strong title="EVM Chain" className="chain-name">Rinkeby</strong>
                &nbsp; / &nbsp; 
                <strong title="RRP Contract" className="chain-rrp-contract">0x4444...0000</strong>
                &nbsp; / &nbsp; 
                <strong title="Airnode Address" className="chain-airnode">0xeeee...0000</strong>
            </div>
            <div className="chain-row desktop-only">
                xPub: &nbsp; 
                <strong>0x2378832929a2392e3929fc9329292323232acccaa2332320123aa</strong>
            </div>
            {(typeof props.filter !== 'undefined') ? (
                <div className="chain-row">
                    <span className="desktop-only">Filter: &nbsp; </span>
                    <Link to={props.filter + ''}>All Endpoints, All Templates, All Functions</Link>
                </div>
            ): null}
        </div>
    </div>
  </header>
);
