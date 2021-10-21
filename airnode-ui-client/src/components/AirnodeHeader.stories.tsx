import { ComponentStory, ComponentMeta } from '@storybook/react';
import '../index.css';
import './AirnodeHeader.css';

import { AirnodeHeader } from './AirnodeHeader';

export default {
  title: 'Example/Header',
  component: AirnodeHeader,
} as ComponentMeta<typeof AirnodeHeader>;

const Template: ComponentStory<typeof AirnodeHeader> = (args) => <AirnodeHeader {...args} />;

export const Default = Template.bind({});
Default.args = {
  filter: "",
  onClickSelect: () => {},
};

// export const LoggedOut = Template.bind({});
// LoggedOut.args = {};
