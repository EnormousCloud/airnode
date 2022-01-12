import { ComponentStory, ComponentMeta } from "@storybook/react";
import { airnode, rrp } from "../fixtures/menu";
import mockStates from "../fixtures/state";
import { AirnodeRequests } from "./AirnodeRequests";
import { AirnodeOperations } from "./AirnodeOperations";
import { AirnodeEndpoints } from "./AirnodeEndpoints";
import { AirnodeWhitelist } from "./AirnodeWhitelist";
import { AirnodeWithdrawals } from "./AirnodeWithdrawals";
import { DataIsReady } from "../service/types";

export default {
  title: "Screens/Airnode",
  component: AirnodeRequests,
} as ComponentMeta<typeof AirnodeRequests>;

const TemplateRequests: ComponentStory<typeof AirnodeRequests> = (args) => (
  <AirnodeRequests {...args} />
);

const chainId = 4;
const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";
const provider = "0xac498088cd9396b3e8366d7fb267697e49235b4436971d2e3bea9955744cf3b8";

const rrpState = mockStates.find(x => (
    x.chain_id == chainId && x.contract_address == contractAddress
));
const airnodeState = rrpState?.providers[provider];

export const Requests = TemplateRequests.bind({});
Requests.args = {
  chainId, contractAddress, provider, airnodeState,
  menu: {
    airnode: {
        ...airnode,
        items: airnode.items.map((item) => ({
          ...item,
          active: item.name === "Requests",
        })),
    },
    rrp: {
      ...rrp,
      items: rrp.items.map((item) => ({
        ...item,
        active: false,
      })),
    },
  },
};


const TemplateOperations: ComponentStory<typeof AirnodeOperations> = (args) => (
    <AirnodeOperations {...args} dataStatus={DataIsReady} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
    chainId, contractAddress, provider, airnodeState,
    menu: {
        airnode: {
            ...airnode,
            items: airnode.items.map((item) => ({
                ...item,
                active: item.name === "Operations",
            })),
        },
        rrp: {
            ...rrp,
            items: rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};


const TemplateEndpoints: ComponentStory<typeof AirnodeEndpoints> = (args) => (
    <AirnodeEndpoints {...args} />
);

export const Endpoints = TemplateEndpoints.bind({});
Endpoints.args = {
    chainId, contractAddress, provider, airnodeState,
    menu: {
        airnode: {
            ...airnode,
            items: airnode.items.map((item) => ({
                ...item,
                active: item.name === "Endpoints",
            })),
        },
        rrp: {
            ...rrp,
            items: rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};


const TemplateWhitelist: ComponentStory<typeof AirnodeWhitelist> = (args) => (
    <AirnodeWhitelist {...args} dataStatus={DataIsReady} />
);

export const Whitelist = TemplateWhitelist.bind({});
Whitelist.args = {
    chainId, contractAddress, provider, 
    airnodeState: { ...airnodeState, whitelisted: [
        "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec",
        "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec",
    ] },
    menu: {
        airnode: {
            ...airnode,
            items: airnode.items.map((item) => ({
                ...item,
                active: item.name === "Whitelist",
            })),
        },
        rrp: {
            ...rrp,
            items: rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};


const TemplateWithdrawals: ComponentStory<typeof AirnodeWithdrawals> = (args) => (
    <AirnodeWithdrawals {...args} />
);

export const Withdrawals = TemplateWithdrawals.bind({});
Withdrawals.args = {
    chainId, contractAddress, provider, 
    airnodeState: { ...airnodeState, requests: {
        "0x2408fbe31e0dea482c5e16be6d082848d42a683943d8fa063401bc05d68ce4c6": {
            "id": "0x2408fbe31e0dea482c5e16be6d082848d42a683943d8fa063401bc05d68ce4c6",
            "fulfill": 1,
            "fail": 0,
            "withdraw": 1
        }
    }},
    menu: {
        airnode: {
            ...airnode,
            items: airnode.items.map((item) => ({
                ...item,
                active: item.name === "Withdrawals",
            })),
        },
        rrp: {
            ...rrp,
            items: rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};

export const NoWithdrawals = TemplateWithdrawals.bind({});
NoWithdrawals.args = {
    chainId, contractAddress, provider, airnodeState,
    menu: {
        airnode: {
            ...airnode,
            items: airnode.items.map((item) => ({
                ...item,
                active: item.name === "Withdrawals",
            })),
        },
        rrp: {
            ...rrp,
            items: rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};