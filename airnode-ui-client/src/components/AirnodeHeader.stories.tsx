import { ComponentStory, ComponentMeta } from '@storybook/react';

import { AirnodeHeader } from './AirnodeHeader';

export default {
  title: 'Components/Header',
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
