import { MenuItem } from "./../components/Menu";
import { NodeMenuProps, MenuPanelProps } from "./../components/MenuPanel";

const itemsAirnode: Array<MenuItem> = [
  {
    name: "Requests",
    counter: 30,
    href: "/",
  },
  {
    name: "Operations",
    counter: 100,
    href: "/",
  },
  {
    name: "Endpoints",
    counter: 3,
    href: "/",
  },
  {
    name: "Whitelist",
    counter: 0,
    href: "/",
  },
  {
    name: "Withdrawals",
    href: "/",
  },
];

const itemsRRP: Array<MenuItem> = [
  {
    name: "Operations",
    counter: 450,
    href: "/",
  },
  {
    name: "Admins",
    counter: 2,
    href: "/",
  },
];

export const mockMenu: MenuPanelProps = {
  airnode: {
    title: "Airnode",
    link: "/",
    address: "0x4444...0000",
    items: itemsAirnode,
    balance: "0.002231",
    symbol: "ETH",
  },
  rrp: {
    title: "RRP Contract",
    link: "/",
    address: "0xeeee...0000",
    items: itemsRRP,
    balance: "0.000000",
    symbol: "ETH",
  },
};

export const airnode = mockMenu.airnode as NodeMenuProps;
airnode.items = airnode.items?.map((item) => ({ ...item, active: false }));

export const rrp = mockMenu.rrp as NodeMenuProps;
rrp.items = rrp.items?.map((item) => ({ ...item, active: false }));