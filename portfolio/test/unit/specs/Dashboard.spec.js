import { mount } from 'vue-test-utils'

import Vue from 'vue'
import Dashboard from '@/components/Dashboard/Dashboard.vue'

describe('Dashboard.vue', () => {

  const wrapper = mount(Dashboard)

  it('should render the sidebar', () => {
    expect(wrapper.contains('.dashboard-sidebar')).toBe(true)
  })

  it('should render the header', () => {
    expect(wrapper.contains('.dashboard-header')).toBe(true)
  })
})
