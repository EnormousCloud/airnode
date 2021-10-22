interface MenuPanelProps {
    children: any
}

export const MenuPanel = (props: MenuPanelProps) => (
    <div className="menu-panel">{props.children}</div>
);
