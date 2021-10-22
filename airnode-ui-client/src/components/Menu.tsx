import { Link } from 'react-router-dom';

import './Menu.css';

interface MenuProps {
}

export const Menu = (props: MenuProps) => (
    <div className="desktop-menu">
        <ul>
            <li><Link to="/">Operations <span className="badge badge-counter">100</span></Link></li>
            <li><Link to="/" className="active">Endpoints <span className="badge badge-counter">3</span></Link></li>
            <li><Link to="/">Templates</Link></li>
            <li><Link to="/">Whitelists</Link></li>
            <li><Link to="/">Withdrawals</Link></li>
        </ul>
    </div>
);
