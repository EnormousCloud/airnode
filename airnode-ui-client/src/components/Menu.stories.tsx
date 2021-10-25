import { ComponentStory, ComponentMeta } from "@storybook/react";

import { Menu, MenuItem } from "./Menu";
import { MenuPanel } from "./MenuPanel";

export default {
  title: "Components/Menu",
  component: Menu,
} as ComponentMeta<typeof Menu>;

const items: Array<MenuItem> = [
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

const Template: ComponentStory<typeof Menu> = (args) => (
  <MenuPanel><Menu {...args} /></MenuPanel>
);

export const Operations = Template.bind({});
Operations.args = {
  items: items.map(item => ((item.name === 'Operations') ? { ...item,  active: true } : item)),
};

export const Endpoints = Template.bind({});
Endpoints.args = {
  items: items.map(item => ((item.name === 'Endpoints') ? { ...item,  active: true } : item)),
};

export const Whitelists = Template.bind({});
Whitelists.args = {
  items: items.map(item => ((item.name === 'Whitelists') ? { ...item,  active: true } : item)),
};

export const Withdrawals = Template.bind({});
Withdrawals.args = {
  items: items.map(item => ((item.name === 'Withdrawals') ? { ...item,  active: true } : item)),
};
