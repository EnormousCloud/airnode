import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu, airnode, rrp } from "../fixtures/menu";

import { RrpAirnodes } from "./RrpAirnodes";
import { RrpOperations } from "./RrpOperations";
import { RrpAdmins } from "./RrpAdmins";

const chainId = 4;
const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";

export default {
  title: "Screens/RRP",
  component: RrpAirnodes,
} as ComponentMeta<typeof RrpAirnodes>;

const TemplateAirnodes: ComponentStory<typeof RrpAirnodes> = (args) => (
  <RrpAirnodes {...args} />
);

export const Airnodes = TemplateAirnodes.bind({});
Airnodes.args = {
  chainId, 
  contractAddress,
  menu: { rrp, airnode: null },
};

const TemplateOperations: ComponentStory<typeof RrpOperations> = (args) => (
  <RrpOperations {...args} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
  chainId, 
  contractAddress,
  menu: {
    airnode: null,
    rrp: {
      ...rrp,
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
  chainId, 
  contractAddress,
  menu: {
    airnode: null,
    rrp: {
      ...rrp,
      items: rrp.items.map((item) => ({
        ...item,
        active: item.name === "Admins",
      })),
    },
  },
};
