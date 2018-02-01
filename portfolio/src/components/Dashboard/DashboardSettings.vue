<template>
  <div class="dashboard-settings">
    <h2>Site settings</h2>
    <div class="dashboard-site-settings">
      <dashboard-site-settings-list v-for="(fieldSection, idx) in settingsFields" :fields="fieldSection" :key="idx"/>
    </div>
  </div>
</template>

<script>
import dashboardSiteSettingsList from './DashboardSettings/DashboardSiteSettingsList'
export default {
  data () {
    return {
      // Dummy data
      settingsFields: {
        siteFields: [
          {
            name: 'title',
            alias: 'Site title',
            type: 'text',
            value: 'Rustfolio test'
          },
          {
            name: 'favicon',
            alias: 'Favicon',
            type: 'upload',
            value: 'Some picture'
          }
        ],
        otherFields: [
          {
            name: 'owner',
            alias: 'Owner display name',
            type: 'text',
            value: 'Jaka S.'
          }
        ]
      }
    }
  },
  components: {
    dashboardSiteSettingsList
  },
  created () {
    // TODO: Fetch fields from the DB into a structure similar to settingsFields in data
  }
}
</script>
