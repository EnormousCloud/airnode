import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { RrpAirnodes } from "./RrpAirnodes";

export default {
  title: "Screens/RRP",
  component: RrpAirnodes,
} as ComponentMeta<typeof RrpAirnodes>;

const TemplateAirnodes: ComponentStory<typeof RrpAirnodes> = (args) => (
  <RrpAirnodes {...args} />
);
export const Airnodes = TemplateAirnodes.bind({});
Airnodes.args = {
  menu: { ...mockMenu, airnode: null },
};
