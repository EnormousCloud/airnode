import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { RrpAirnodes } from "./RrpAirnodes";
import { RrpOperations } from "./RrpOperations";

export default {
  title: "Screens/RRP",
  component: RrpAirnodes,
} as ComponentMeta<typeof RrpAirnodes>;

const TemplateAirnodes: ComponentStory<typeof RrpAirnodes> = (args) => (
  <RrpAirnodes {...args} />
);

const rrp = { ...mockMenu.rrp };
rrp.items = rrp.items.map(item => ({ ...item, active: false }));

export const Airnodes = TemplateAirnodes.bind({});
Airnodes.args = {
  menu: { rrp, airnode: null },
};

const TemplateOperations: ComponentStory<typeof RrpOperations> = (args) => (
    <RrpOperations {...args} />
);

const menuOps = { ...mockMenu };
menuOps.rrp.items = menuOps.rrp.items.map(item => ((item.name === 'Operations') ? { ...item,  active: true } : item));

export const Operations = TemplateOperations.bind({});
Operations.args = {
  menu: { ...menuOps, airnode: null },
};
