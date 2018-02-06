<template>
  <div class="dashboard-site-settings">
    <div class="dashboard-site-setting-field" v-for="field in fields" :key="field.name">
      <dashboard-site-settings-input v-if="field.field_type === 'text'" :field="field"/>
      <dashboard-site-settings-upload v-else-if="field.field_type === 'upload'" :field="field"/>
    </div>
  </div>
</template>

<script>
import dashboardSiteSettingsInput from './DashboardSiteSettingsInput'
import dashboardSiteSettingsUpload from './DashboardSiteSettingsUpload'
export default {
  props: ['fields'],
  components: {
    dashboardSiteSettingsInput,
    dashboardSiteSettingsUpload
  }
}
</script>

<style lang="less" scoped>
.dashboard-site-setting-field {
  margin-bottom: 20px;
}
</style>
