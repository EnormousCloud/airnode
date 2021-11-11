import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Select } from "./Select";
import fullState from './../fixtures/state';

export default {
  title: "Screens/Select Airnode",
  component: Select,
} as ComponentMeta<typeof Select>;

const Template: ComponentStory<typeof Select> = (args) => <Select {...args} />;

export const Default = Template.bind({});
Default.args = { fullState };
