<template>
  <div class="dashboard">
    <el-container>
      <el-aside :width="sidebarWidth">
        <dashboard-sidebar />
      </el-aside>
      <el-container>
        <el-header>
          <dashboard-header />
        </el-header>
        <el-main>
          <transition name="slide-fade" mode="out-in">
            <router-view />
          </transition>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script>
import dashboardSidebar from './DashboardSidebar'
import dashboardHeader from './DashboardHeader'

export default {
  data () {
    return {
      sidebarWidth: 'auto'
    }
  },
  components: {
    dashboardSidebar,
    dashboardHeader
  }
}
</script>
