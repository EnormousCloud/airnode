import { Menu, MenuItem } from '../components/Menu';
import { Balance } from '../components/Balance';
import { Link } from 'react-router-dom';

export interface NodeMenuProps {
    title: string
    link: string
    address: string
    items: Array<MenuItem>
    balance: string
    symbol: string
}

export interface MenuPanelProps {
    airnode: NodeMenuProps,
    rrp: NodeMenuProps,
}

const NodeMenu = (props: NodeMenuProps) => {
    return (
        <div>
            <h3>{props.title}</h3>
            <div className="menu-address">
                <Link to={props.link}>{props.address}</Link>
            </div>
            <Menu items={props.items} />
            <Balance value={props.balance} symbol={props.symbol} />
        </div>
    );
};

export const MenuPanel = (props: MenuPanelProps) => (
    <div className="menu-panel">
        <NodeMenu {...props.airnode} />
        <NodeMenu {...props.rrp} />
    </div>
);
