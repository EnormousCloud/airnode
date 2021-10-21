import { ComponentStory, ComponentMeta } from '@storybook/react';
import '../index.css';
import './Footer.css';

import { Footer } from './Footer';

export default {
  title: 'Example/Footer',
  component: Footer,
} as ComponentMeta<typeof Footer>;

const Template: ComponentStory<typeof Footer> = (args) => <Footer {...args} />;

export const Default = Template.bind({});
Default.args = {};
