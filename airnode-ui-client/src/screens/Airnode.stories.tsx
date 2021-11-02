import { ComponentStory, ComponentMeta } from "@storybook/react";
import { airnode, rrp } from "../fixtures/menu";
import { AirnodeRequests } from "./AirnodeRequests";
import { AirnodeOperations } from "./AirnodeOperations";
import { AirnodeEndpoints } from "./AirnodeEndpoints";
import { AirnodeWhitelist } from "./AirnodeWhitelist";
import { AirnodeWithdrawals } from "./AirnodeWithdrawals";

export default {
  title: "Screens/Airnode",
  component: AirnodeRequests,
} as ComponentMeta<typeof AirnodeRequests>;

const TemplateRequests: ComponentStory<typeof AirnodeRequests> = (args) => (
  <AirnodeRequests {...args} />
);

export const Requests = TemplateRequests.bind({});
Requests.args = {
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
    <AirnodeOperations {...args} />
);

export const Operations = TemplateOperations.bind({});
Operations.args = {
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
    <AirnodeWhitelist {...args} />
);

export const Whitelist = TemplateWhitelist.bind({});
Whitelist.args = {
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