import { ComponentStory, ComponentMeta } from "@storybook/react";
import { mockMenu } from "../fixtures/menu";
import { AddContract } from "./AddContract";
import { ChangeContract } from "./ChangeContract";
import { ChangeFilter } from "./ChangeFilter";

export default {
  title: "Screens/Contract",
  component: ChangeContract,
} as ComponentMeta<typeof ChangeContract>;

const TemplateChange: ComponentStory<typeof ChangeContract> = (args) => (
  <ChangeContract {...args} />
);
export const Change = TemplateChange.bind({});
Change.args = {
  menu: { ...mockMenu, airnode: null },
};

const TemplateAdd: ComponentStory<typeof AddContract> = (args) => (
  <AddContract {...args} />
);
export const Add = TemplateAdd.bind({});
Add.args = {};

const TemplateFilter: ComponentStory<typeof ChangeFilter> = (args) => (
  <ChangeFilter {...args} />
);
export const Filter = ChangeFilter.bind({});
Filter.args = {
  menu: { ...mockMenu, airnode: null },
};