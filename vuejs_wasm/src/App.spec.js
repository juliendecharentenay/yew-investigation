import { mount } from '@vue/test-utils';
import App from './App.vue';

import { fibonacci } from "@/pkg/index.js";
jest.mock('@/pkg/index.js', () => ({
  fibonacci: jest.fn(() => 42),
}));

describe('App.vue', () => {
  it('renders', async () => {
    const wrapper = mount(App);

    expect(wrapper.find('#p-result').exists()).toBe(false);
    await wrapper.find('#btn-calculate').trigger('click');

    expect(fibonacci).toHaveBeenCalledTimes(1);
    expect(wrapper.find('#p-result').exists()).toBe(true);

  });
});
