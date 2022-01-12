import { ComponentStory, ComponentMeta } from '@storybook/react';
import { Button } from './Button';

export default {
  title: 'Components/Buttons',
  component: Button,
} as ComponentMeta<typeof Button>;

const Template: ComponentStory<typeof Button> = (args) => (
    <div className="centered">
        <div style={{ padding: 20 }}><Button {...args} /></div>
        <div style={{ padding: 20 }}><Button {...args} disabled={true} /></div>
    </div>
);

export const Default = Template.bind({});
Default.args = {
    label: 'lorem ipsum',
    type: '',
    width: '240'
}

export const Primary = Template.bind({});
Primary.args = {
    label: 'lorem ipsum',
    type: 'primary',
    width: '240'
}

export const Link = Template.bind({});
Link.args = {
    label: 'lorem ipsum',
    type: 'link'
}
