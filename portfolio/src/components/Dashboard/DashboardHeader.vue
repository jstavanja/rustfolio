<template>
  <el-menu class="dashboard-header" mode="horizontal">
    <el-menu-item disabled index="1">Rustfolio Dashboard</el-menu-item>
    <el-menu-item index="2" class="navbar-link-site-wrapper"><router-link to="/" class="navbar-link-site">Go to site</router-link></el-menu-item>
  </el-menu>
</template>

<style lang="less" scoped>
.dashboard-header {
  .navbar-link-site-wrapper {
    .navbar-link-site {
      height: 100%;
    }
  }
}
</style>
