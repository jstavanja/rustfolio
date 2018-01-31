<template>
  <el-menu class="dashboard-header" mode="horizontal">
    <el-menu-item active index="1" @click="routerNavigate('/dashboard')">Rustfolio Dashboard</el-menu-item>
    <el-menu-item index="2" @click="routerNavigate('/')">Go to site</el-menu-item>
  </el-menu>
</template>

<script>
export default {
  methods: {
    // Because of styling issues, the routing had to be done programmatically (instead of using router-link)
    routerNavigate (path) {
      this.$router.push(path)
    }
  }
}
</script>
