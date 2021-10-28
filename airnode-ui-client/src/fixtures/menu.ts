import { MenuItem } from "./../components/Menu";
import { MenuPanelProps } from "./../components/MenuPanel";

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
      name: "Whitelists",
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