import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { RrpAirnodes } from "./RrpAirnodes";
import { RrpOperations } from "./RrpOperations";
import { RrpAdmins } from "./RrpAdmins";

export default {
  title: "Screens/RRP",
  component: RrpAirnodes,
} as ComponentMeta<typeof RrpAirnodes>;

const TemplateAirnodes: ComponentStory<typeof RrpAirnodes> = (args) => (
  <RrpAirnodes {...args} />
);

const rrp = { ...mockMenu.rrp };
rrp.items = rrp.items.map((item) => ({ ...item, active: false }));

export const Airnodes = TemplateAirnodes.bind({});
Airnodes.args = {
  menu: { rrp, airnode: null },
};

const TemplateOperations: ComponentStory<typeof RrpOperations> = (args) => (
  <RrpOperations {...args} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
  menu: {
    airnode: null,
    rrp: {
      ...mockMenu.rrp,
      items: rrp.items.map((item) => ({
        ...item,
        active: item.name === "Operations",
      })),
    },
  },
};

const TemplateAdmins: ComponentStory<typeof RrpAdmins> = (args) => (
  <RrpAdmins {...args} />
);

export const Admins = TemplateAdmins.bind({});
Admins.args = {
  menu: {
    airnode: null,
    rrp: {
      ...mockMenu.rrp,
      items: rrp.items.map((item) => ({
        ...item,
        active: item.name === "Admins",
      })),
    },
  },
};
