import { ComponentStory, ComponentMeta } from "@storybook/react";
import { ChangeContract } from "./ChangeContract";
import { mockMenu } from "./../fixtures/menu";

export default {
  title: "Screens/Change Contract",
  component: ChangeContract,
} as ComponentMeta<typeof ChangeContract>;

const Template: ComponentStory<typeof ChangeContract> = (args) => <ChangeContract {...args} />;

export const Default = Template.bind({});
Default.args = {
    menu: { ...mockMenu, airnode: null }
};
