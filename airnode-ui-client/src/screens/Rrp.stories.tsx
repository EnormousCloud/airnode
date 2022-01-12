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
const opKey = chainId+ '-' + contractAddress;

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
  nodeState: fullState.find((x: any) => (x.chain_id == chainId && x.contract_address == contractAddress)),
};

const TemplateOperations: ComponentStory<typeof RrpOperations> = (args) => (
  <RrpOperations {...args} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
  chainId, 
  contractAddress,
  dataStatus: DataIsReady,
  operations: { [opKey]: operations },
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

const menuAdmins = {
  airnode: null,
  rrp: {
    ...rrp,
    items: rrp.items.map((item) => ({
      ...item,
      active: item.name === "Admins",
    })),
  },
};

export const NoAdmins = TemplateAdmins.bind({});
NoAdmins.args = {
  chainId, 
  contractAddress,
  nodeState: fullState.find((x: any) => (x.chain_id == chainId && x.contract_address == contractAddress)),
  menu: menuAdmins,
  dataStatus: DataIsReady,
};

export const Admins = TemplateAdmins.bind({});
Admins.args = {
  chainId, 
  contractAddress,
  nodeState: {
    ...fullState.find((x: any) => (x.chain_id == chainId && x.contract_address == contractAddress)),
    admins: {
      "0x37c28b4d364cdd2de525c054cf275e437ab3ded9": { address: "0x37c28b4d364cdd2de525c054cf275e437ab3ded9", rank: 10 },
      "0x00c28b4d364cdd2de525c054cf275e437ab3ded9": { address: "0x0028b4d364cdd2de525c054cf275e437ab3ded9", rank: 8 },
    }
  },
  menu: menuAdmins,
  dataStatus: DataIsReady,
  operations: {
    [opKey]: operations.slice(0, 3).map((x:any) => ({ ...x, e: { ... x.e, type: 'SetRank' } })),
  }
};