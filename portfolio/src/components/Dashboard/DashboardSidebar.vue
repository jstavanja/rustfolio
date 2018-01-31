<template>
  <el-menu class="dashboard-sidebar" :collapse="isCollapse">
    <el-menu-item index="1" @click="routerNavigate('/dashboard')">
      <i class="el-icon-menu"></i>
      <span slot="title">Dashboard home</span>
    </el-menu-item>
    <el-submenu index="2">
      <template slot="title">
        <i class="el-icon-tickets"></i>
        <span slot="title">Pages</span>
      </template>
      <el-menu-item-group>
        <span slot="title">Typical Pages</span>
        <el-menu-item index="1-1">About</el-menu-item>
        <el-menu-item index="1-2">Contact</el-menu-item>
      </el-menu-item-group>
      <el-menu-item-group title="Custom Pages">
        <el-menu-item index="1-3">New Custom Page</el-menu-item>
      </el-menu-item-group>
      <el-submenu index="1-4">
        <span slot="title">Your pages</span>
        <el-menu-item index="1-4-1" class="menu-item-custom-page-title">Meetup in Ljubljana 2018</el-menu-item>
      </el-submenu>
    </el-submenu>
    <el-menu-item index="3" @click="routerNavigate('/dashboard/blog')">
      <i class="el-icon-edit"></i>
      <span slot="title">Blog</span>
    </el-menu-item>
    <el-menu-item index="4" @click="routerNavigate('/dashboard/settings')">
      <i class="el-icon-setting"></i>
      <span slot="title">Settings</span>
    </el-menu-item>
    <el-menu-item index="5" @click="toggleCollapse">
      <i class="el-icon-arrow-right" v-if="isCollapse"></i>
      <i class="el-icon-arrow-left" v-if="!isCollapse"></i>
      <span slot="title" v-if="isCollapse">Expand</span>
      <span slot="title" v-if="!isCollapse">Collapse</span>
    </el-menu-item>
  </el-menu>
</template>

<style lang="less" scoped>
.dashboard-sidebar {
  display: inline-block;
  &:not(.el-menu--collapse) {
    width: 200px;
  }
  .menu-item-custom-page-title {
    overflow: hidden;
    text-overflow: ellipsis;
  }
}
</style>

<script>
export default {
  data () {
    return {
      isCollapse: true
    }
  },
  methods: {
    // Because of styling issues, the routing had to be done programmatically (instead of using router-link)
    routerNavigate (path) {
      this.$router.push(path)
    },
    toggleCollapse () {
      this.isCollapse = !this.isCollapse
    }
  }
}
</script>
