import Vue from 'vue'
import Router from 'vue-router'
import Dashboard from '@/components/Dashboard/Dashboard'
import Home from '@/components/Home'

// Dashboard
import DashboardHome from '@/components/Dashboard/DashboardHome'
import DashboardBlog from '@/components/Dashboard/DashboardBlog'
import DashboardSettings from '@/components/Dashboard/DashboardSettings'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/dashboard',
      component: Dashboard,
      children: [
        {
          path: '',
          component: DashboardHome
        },
        {
          path: 'blog',
          component: DashboardBlog
        },
        {
          path: 'settings',
          component: DashboardSettings
        }
      ]
    },
    {
      path: '/',
      component: Home
    }
  ],
  mode: 'history'
})
