import { Link } from 'react-router-dom';
import api3 from './../assets/api3.svg';

import './AirnodeHeader.css';

interface HeaderProps {
    filter?: string
    filterName?: string
    chain: string
    contract: string
    airnode?: string
    hrefSelect: string
    xPubKey: string
}
const shortened = (x: string) => {
    if (!x || !x.startsWith('0x') || x.length < 10) return x;
    return x.substring(0, 4 + 2) + '...' + x.substring(x.length-4);
}

const shortXpub = (x: string, ln: number) => {
    if (!x || !x.startsWith('xpub') || x.length < 2 * ln + 'xpub'.length) return x;
    return x.substring(0, ln + 'xpub'.length) + '...' + x.substring(x.length-ln);
}

export const AirnodeHeader = (props: HeaderProps) => (
  <header>
    <div className="inner">
        <div className="hd-column">
            <Link className="select-airnode" to={props.hrefSelect}>
                <img src={api3} className="mobile-only" alt='API3' style={{ width: 14, height: 14, marginLeft: 0, marginRight: 5 }} />
                <span className="mobile-only">Choose Airnode</span>
                <img src={api3} className="desktop-only" alt='API3' style={{ width: '20%', height: '20%', marginLeft: 20, marginRight: 20 }} />
                <span className="desktop-only" style={{ whiteSpace: "nowrap" }}>Choose<br />Airnode</span>
            </Link>
        </div>
        <div className="hd-column-wide">
            <div className="chain-row">
                <strong title="EVM Chain" className="chain-name">{props.chain}</strong>
                <span className="sep">&nbsp; / &nbsp;</span>
                <strong title="RRP Contract" className="chain-rrp-contract">{shortened(props.contract)}</strong>
                {props.airnode ? [
                    <span className="sep">&nbsp; / &nbsp; </span>,
                    <strong title="Airnode Address" className="chain-airnode">{shortened(props.airnode)}</strong>
                ]: null}
            </div>
            {(props.xPubKey) ? (
                <div className="chain-row">
                    <span className="desktop-only xpub">{shortXpub(props.xPubKey, 20)}</span>
                    <span className="mobile-only xpub">{shortXpub(props.xPubKey, 12)}</span>
                </div>
            ): null}
            {(typeof props.filter !== 'undefined') ? (
                <div className="chain-row">
                    {(props.filterName) ? ([
                        <span className="desktop-only">Filter: &nbsp; </span>,
                        <Link to={props.filter + ''}>{props.filterName}</Link>
                    ]) : (
                        <Link to={props.filter + ''}>All Endpoints, All Templates, All Functions</Link>
                    )}
                </div>
            ): null}
        </div>
    </div>
  </header>
);
