import React from "react";
import { Menu, MenuItem } from '../components/Menu';
import { Balance } from '../components/Balance';
import { Link } from 'react-router-dom';
import { AddressIcon } from "./forms/EtherscanLink";

const svgOpen = (
    <svg
        version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
	    viewBox="0 0 35.058 35.057"
        aria-hidden={true}
        style={{ width: 10, height: 20 }}
    >
        <g>
            <path fill="#aaa" d="M23.408,19.798L6.607,34.327c-0.556,0.48-1.255,0.73-1.963,0.73c-0.423,0-0.848-0.09-1.247-0.271
                c-1.068-0.487-1.752-1.555-1.752-2.729V3.001c0-1.174,0.685-2.24,1.752-2.729c1.068-0.488,2.322-0.309,3.21,0.459l16.801,14.527
                c0.659,0.569,1.038,1.397,1.038,2.27S24.067,19.227,23.408,19.798z M32.72,16.014L15.92,1.488
                c-0.836-0.724-2.099-0.631-2.821,0.204c-0.723,0.836-0.631,2.1,0.205,2.82l15.052,13.016L13.305,30.546
                c-0.835,0.723-0.927,1.983-0.205,2.819c0.396,0.457,0.953,0.691,1.514,0.691c0.463,0,0.929-0.16,1.307-0.487L32.72,19.042
                c0.439-0.382,0.692-0.934,0.692-1.515S33.161,16.395,32.72,16.014z"/>
        </g>
    </svg>
);

const svgClose = (
    <svg
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        style={{ width: 12, height: 12 }}
        viewBox="0 0 121.31 122.876"
    >
        <g>
            <path fill="#aaa" fill-rule="evenodd" clip-rule="evenodd" d="M90.914,5.296c6.927-7.034,18.188-7.065,25.154-0.068 c6.961,6.995,6.991,18.369,0.068,25.397L85.743,61.452l30.425,30.855c6.866,6.978,6.773,18.28-0.208,25.247 c-6.983,6.964-18.21,6.946-25.074-0.031L60.669,86.881L30.395,117.58c-6.927,7.034-18.188,7.065-25.154,0.068 c-6.961-6.995-6.992-18.369-0.068-25.397l30.393-30.827L5.142,30.568c-6.867-6.978-6.773-18.28,0.208-25.247 c6.983-6.963,18.21-6.946,25.074,0.031l30.217,30.643L90.914,5.296L90.914,5.296z"/>
        </g>
    </svg>
);

export interface NodeMenuProps {
    title: string
    link: string
    chainId: number
    address: string
    items: Array<MenuItem>
    balance: string
    symbol: string
}

export interface MenuPanelProps {
    airnode: NodeMenuProps|null
    rrp: NodeMenuProps|null
}

const shortened = (x: string) => {
    let ln = (x.length > 42) ? 8 : 4;
    if (!x.startsWith('0x') || x.length <= 2*ln + 2) return x;
    return x.substring(0, 2 + ln) + '...' + x.substring(x.length - ln);
}

const NodeMenu = (props: NodeMenuProps) => {
    return (
        <div className="menu-node">
            <h3>{props.title}</h3>
            {props.address ? (
                <div className="menu-address">
                    <Link to={props.link}>{shortened(props.address)}</Link>
                    {(props.address.length == 42) ? (
                        <AddressIcon chainId={''+props.chainId} address={props.address} />
                    ): null}
                </div>
            ) : null}
            <Menu items={props.items} />
            <Balance value={props.balance} symbol={props.symbol} />
        </div>
    );
};


interface MobileMenuState {
    open: boolean
}

export const MenuPanel = (props: MenuPanelProps) => {
    const [state, setState] = React.useState<MobileMenuState>({ open: false });
    return (
        <div>
            {!state.open ? (
                <button
                    className="btn-menu-open mobile-only"
                    onClick={() => setState({ open: true })}
                >
                    {svgOpen}
                </button>
            ): null}
            {state.open ? (
                <div className="mobile-only menu-mobile">
                    <button
                        className="btn-menu-close"
                        onClick={() => setState({ open: false })}
                    >
                        {svgClose}
                    </button>
                    <div className="menu-panel">
                        {props.airnode ? <NodeMenu {...props.airnode} /> : null }
                        {props.rrp ? <NodeMenu {...props.rrp} /> : null }
                    </div>
                </div>
            ): null}
            <div className="menu-panel desktop-only">
                {props.airnode ? <NodeMenu {...props.airnode} /> : null }
                {props.rrp ? <NodeMenu {...props.rrp} /> : null }
            </div>
        </div>
    );
    }