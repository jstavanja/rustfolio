<template>
  <div class="dashboard-settings-image-upload-section" >
    <h4>{{ field.alias }}</h4>
    <div class="dashboard-settings-image-upload-wrapper">
      <el-upload
        class="dashboard-settings-image-upload"
        :action="'/settings/' + field.alias"
        :show-file-list="false"
        :on-success="handleUploadSuccess"
        :before-upload="beforeUpload">
        <img v-if="imageUrl" :src="imageUrl" class="image">
        <i v-else class="el-icon-plus dashboard-settings-image-upload-icon"></i>
      </el-upload>
    </div>
  </div>
</template>

<script>
export default {
  props: ['field'],
  data () {
    return {
      imageUrl: null
    }
  },
  methods: {
    handleUploadSuccess (res, file) {
      this.$message.success('Image successfully uploaded!')
      this.imageUrl = URL.createObjectURL(file.raw)
    },
    beforeUpload (file) {
      const isPNG = file.type === 'image/png'
      const isLt2M = file.size / 1024 / 1024 < 2

      if (!isPNG) {
        this.$message.error('Image must be in png format!')
      }
      if (!isLt2M) {
        this.$message.error('Image size can not exceed 2MB!')
      }
      return isPNG && isLt2M
    }
  }
}
</script>

<style lang="less" scoped>
.dashboard-site-settings {
  .dashboard-site-setting-field {
    margin-bottom: 20px;
    .dashboard-settings-image-upload-wrapper {
      width: 64px;
      height: 64px;
      border: 1px dashed #000000;
      border-radius: 6px;
      cursor: pointer;
      position: relative;
      overflow: hidden;
      &:hover {
        border-color: #409EFF;
      }
      .dashboard-settings-image-upload {
        .dashboard-settings-image-upload-icon {
          font-size: 28px;
          color: #8c939d;
          width: 64px;
          height: 64px;
          line-height: 64px;
          text-align: center;
        }
        .image {
          width: 64px;
          height: 64px;
          display: block;
        }
      }
    }
  }
}
</style>
