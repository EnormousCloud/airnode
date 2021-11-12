import { ComponentStory, ComponentMeta } from "@storybook/react";
import { rrp } from "../fixtures/menu";
import fullState from "../fixtures/state";
import operations from '../fixtures/ops-rinkeby';
import { RrpAirnodes } from "./RrpAirnodes";
import { RrpOperations } from "./RrpOperations";
import { RrpAdmins } from "./RrpAdmins";
import { DataIsReady } from "../service/types";

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
  nodeState: fullState.filter((x: any) => (x.chain_id == chainId && x.contract_address == contractAddress)),
};

const TemplateOperations: ComponentStory<typeof RrpOperations> = (args) => (
  <RrpOperations {...args} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
  chainId, 
  contractAddress,
  dataStatus: DataIsReady,
  operations,
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
