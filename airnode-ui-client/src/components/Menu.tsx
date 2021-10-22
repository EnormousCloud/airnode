import { Link } from 'react-router-dom';

import './Menu.css';

// Individual Menu item
export interface MenuItem {
    // name of the menu
    name: string
    // location reference
    href: string
    // count of items
    counter?: number
    // whether the item is active
    active?: boolean
}

interface MenuProps {
    // list of items
    items: Array<MenuItem>
}

export const Menu = (props: MenuProps) => (
    <div className="desktop-menu">
        <ul>
            {props.items.map((item: MenuItem) => (
                <li key={item.name}>
                    <Link to={item.href} className={(item.active) ? 'active': ''}>
                        {item.name}
                        {(item.counter && item.counter > 0) ? (
                            <span className="badge badge-counter">{item.counter}</span>
                        ) : null}
                    </Link>
                </li>
            ))}
        </ul>
    </div>
);
