import { ComponentStory, ComponentMeta } from '@storybook/react';

import { TextInput } from './TextInput';

export default {
  title: 'Components/TextInput',
  component: TextInput,
} as ComponentMeta<typeof TextInput>;

const Template: ComponentStory<typeof TextInput> = (args) => (
    <main><div className="inner"><div><h3>Text Input:</h3><TextInput {...args} /></div></div></main>
);

export const WithDefaults = Template.bind({});
WithDefaults.args = {
    value: 'lorem ipsum',
    onChange: (value) => { console.log(value); }
}

export const WithSuccess = Template.bind({});
WithSuccess.args = {
    value: 'lorem ipsum',
    success: true,
}

export const WithWarning = Template.bind({});
WithWarning.args = {
    value: 'lorem ipsum',
    warning: true,
}
