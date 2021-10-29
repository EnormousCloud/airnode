import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { AirnodeRequests } from "./AirnodeRequests";
import { AirnodeOperations } from "./AirnodeOperations";
import { AirnodeEndpoints } from "./AirnodeEndpoints";
import { AirnodeWhitelist } from "./AirnodeWhitelist";

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
        ...mockMenu.airnode,
        items: mockMenu.airnode.items.map((item) => ({
          ...item,
          active: item.name === "Requests",
        })),
    },
    rrp: {
      ...mockMenu.rrp,
      items: mockMenu.rrp.items.map((item) => ({
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
            ...mockMenu.airnode,
            items: mockMenu.airnode.items.map((item) => ({
                ...item,
                active: item.name === "Operations",
            })),
        },
        rrp: {
            ...mockMenu.rrp,
            items: mockMenu.rrp.items.map((item) => ({
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
            ...mockMenu.airnode,
            items: mockMenu.airnode.items.map((item) => ({
                ...item,
                active: item.name === "Endpoints",
            })),
        },
        rrp: {
            ...mockMenu.rrp,
            items: mockMenu.rrp.items.map((item) => ({
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
            ...mockMenu.airnode,
            items: mockMenu.airnode.items.map((item) => ({
                ...item,
                active: item.name === "Whitelist",
            })),
        },
        rrp: {
            ...mockMenu.rrp,
            items: mockMenu.rrp.items.map((item) => ({
                ...item,
                active: false,
            })),
        },
    },
};