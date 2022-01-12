import { ComponentStory, ComponentMeta } from "@storybook/react";

import { Menu } from "./Menu";
import { NodeMenuProps } from "./MenuPanel";
import { mockMenu } from "./../fixtures/menu";
const items = (mockMenu.airnode as NodeMenuProps).items;

export default {
  title: "Components/Menu",
  component: Menu,
} as ComponentMeta<typeof Menu>;

const Template: ComponentStory<typeof Menu> = (args) => (
  <div className="menu-panel"><Menu {...args} /></div>
);

export const Requests = Template.bind({});
Requests.args = {
  items: items.map(item => ((item.name === 'Requests') ? { ...item,  active: true } : item)),
};

export const Operations = Template.bind({});
Operations.args = {
  items: items.map(item => ((item.name === 'Operations') ? { ...item,  active: true } : item)),
};

export const Endpoints = Template.bind({});
Endpoints.args = {
  items: items.map(item => ((item.name === 'Endpoints') ? { ...item,  active: true } : item)),
};

export const Whitelist = Template.bind({});
Whitelist.args = {
  items: items.map(item => ((item.name === 'Whitelist') ? { ...item,  active: true } : item)),
};

export const Withdrawals = Template.bind({});
Withdrawals.args = {
  items: items.map(item => ((item.name === 'Withdrawals') ? { ...item,  active: true } : item)),
};
