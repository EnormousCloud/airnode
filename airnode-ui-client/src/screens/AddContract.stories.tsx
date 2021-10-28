import { ComponentStory, ComponentMeta } from "@storybook/react";
import { AddContract } from "./AddContract";

export default {
  title: "Screens/Add Contract",
  component: AddContract,
} as ComponentMeta<typeof AddContract>;

const Template: ComponentStory<typeof AddContract> = (args) => <AddContract {...args} />;

export const Default = Template.bind({});
Default.args = {};
