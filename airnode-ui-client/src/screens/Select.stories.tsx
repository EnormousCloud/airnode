import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Select } from "./Select";
import { mockMenu } from "./../fixtures/menu";

export default {
  title: "Screens/Select",
  component: Select,
} as ComponentMeta<typeof Select>;

const Template: ComponentStory<typeof Select> = (args) => <Select {...args} />;

export const Default = Template.bind({});
Default.args = { menu: mockMenu };
