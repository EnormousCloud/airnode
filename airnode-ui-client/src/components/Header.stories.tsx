import { ComponentStory, ComponentMeta } from '@storybook/react';
import '../index.css';

import { Header } from './Header';

export default {
  title: 'Example/Header',
  component: Header,
} as ComponentMeta<typeof Header>;

const Template: ComponentStory<typeof Header> = (args) => <Header {...args} />;

export const Default = Template.bind({});
Default.args = {
  user: {},
};

// export const LoggedOut = Template.bind({});
// LoggedOut.args = {};
