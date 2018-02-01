<template>
  <div class="dashboard-settings">
    <h2>Site settings</h2>
    <div class="dashboard-site-settings">
      <div class="dashboard-site-setting-field" v-for="field in siteFields" :key="field.name">
        <el-input :type="field.type" :placeholder="field.alias" :value="field.value" v-if="field.type === 'text'">
          <template slot="prepend">{{ field.alias }}</template>
        </el-input>
        <div class="dashboard-settings-favicon-upload-section" v-if="field.name === 'favicon'">
          <h4>Site favicon:</h4>
          <div class="dashboard-settings-favicon-upload-wrapper">
            <el-upload
              class="dashboard-settings-favicon-upload"
              :action="'/settings/' + field.alias"
              :show-file-list="false"
              :on-success="handleFaviconSuccess"
              :before-upload="beforeFaviconUpload">
              <img v-if="faviconUrl" :src="faviconUrl" class="favicon">
              <i v-else class="el-icon-plus dashboard-settings-favicon-upload-icon"></i>
            </el-upload>
          </div>
        </div>
      </div>
      <h2>Other settings</h2>
      <div class="dashboard-site-setting-field" v-for="field in otherFields" :key="field.name">
        <el-input :type="field.type" :placeholder="field.alias" :value="field.value" v-if="field.type === 'text'">
          <template slot="prepend">{{ field.alias }}</template>
        </el-input>
        <div class="dashboard-settings-favicon-upload-section" v-if="field.name === 'favicon'">
          <h4>Site favicon:</h4>
          <div class="dashboard-settings-favicon-upload-wrapper">
            <el-upload
              class="dashboard-settings-favicon-upload"
              :action="'/settings/' + field.alias"
              :show-file-list="false"
              :on-success="handleFaviconSuccess"
              :before-upload="beforeFaviconUpload">
              <img v-if="faviconUrl" :src="faviconUrl" class="favicon">
              <i v-else class="el-icon-plus dashboard-settings-favicon-upload-icon"></i>
            </el-upload>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data () {
    return {
      faviconUrl: null,
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
  },
  methods: {
    handleFaviconSuccess (res, file) {
      this.$message.success('Favicon successfully uploaded!')
      this.faviconUrl = URL.createObjectURL(file.raw)
    },
    beforeFaviconUpload (file) {
      const isJPG = file.type === 'image/ico'
      const isLt2M = file.size / 1024 / 1024 < 2

      if (!isJPG) {
        this.$message.error('Avatar picture must be ico format!')
      }
      if (!isLt2M) {
        this.$message.error('Avatar picture size can not exceed 2MB!')
      }
      return isJPG && isLt2M
    }
  }
}
</script>

<style lang="less" scoped>
.dashboard-settings {
  .dashboard-site-settings {
    .dashboard-settings-favicon-upload-wrapper {
      width: 64px;
      height: 64px;
      border: 1px dashed #000000;
      border-radius: 6px;
      cursor: pointer;
      position: relative;
      overflow: hidden;
    }
    .dashboard-settings-favicon-upload-wrapper:hover {
      border-color: #409EFF;
    }
    .dashboard-settings-favicon-upload-icon {
      font-size: 28px;
      color: #8c939d;
      width: 64px;
      height: 64px;
      line-height: 64px;
      text-align: center;
    }
    .favicon {
      width: 64px;
      height: 64px;
      display: block;
    }
    .dashboard-site-setting-field {
      margin-bottom: 20px;
    }
  }
}
</style>
