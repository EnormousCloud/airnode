import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { AirnodeRequests } from "./AirnodeRequests";

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