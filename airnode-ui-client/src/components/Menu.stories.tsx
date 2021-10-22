import { ComponentStory, ComponentMeta } from '@storybook/react';

import { Menu } from './Menu';

export default {
  title: 'Example/Menu',
  component: Menu,
} as ComponentMeta<typeof Menu>;

const Template: ComponentStory<typeof Menu> = (args) => <Menu {...args} />;

export const Default = Template.bind({});
Default.args = {};
