import { ComponentStory, ComponentMeta } from "@storybook/react";

import { Menu, MenuItem } from "./Menu";

export default {
  title: "Example/Menu",
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

const Template: ComponentStory<typeof Menu> = (args) => <Menu {...args} />;

export const Operations = Template.bind({});
Operations.args = {
  items: items.map(item => ((item.name === 'Operations') ? { ...item,  active: true } : item)),
};
