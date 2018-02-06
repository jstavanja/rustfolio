<template>
  <div class="dashboard-settings">
    <h2>Site settings</h2>
    <div class="dashboard-site-settings">
      <dashboard-site-settings-list v-for="(fieldSection, idx) in settingsFields" :fields="fieldSection" :key="idx"/>
    </div>
  </div>
</template>

<script>
import axios from 'axios'
import dashboardSiteSettingsList from './DashboardSettings/DashboardSiteSettingsList'

export default {
  data () {
    return {
      // Dummy data
      settingsFields: {}
    }
  },
  components: {
    dashboardSiteSettingsList
  },
  created () {
    // TODO: Fetch fields from the DB into a structure similar to settingsFields in data
    axios.get('/dashboard/settings')
      .then((response) => {
        this.settingsFields = response.data
        console.log(this.settingsFields)
      })
  }
}
</script>
