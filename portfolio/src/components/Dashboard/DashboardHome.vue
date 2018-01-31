<template>
  <div class="dashboard-home">
    <el-row>
      <h2>Hello, Administrator!</h2>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="12" v-for="(notification, index) in notifications" :key="index">
        <el-card class="box-card">
          <div slot="header" class="clearfix">
            <span>{{ notification.title }}</span>
            <el-button type="text" class="card-dismiss">Dismiss</el-button>
          </div>
          <div class="text item">
            {{ notification.text }}
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script>
export default {
  data () {
    return {
      notifications: [
        {
          title: 'Ena',
          text: 'Test of the first notification.'
        },
        {
          title: 'Dva',
          text: 'The followup notification.'
        }
      ]
    }
  }
}
</script>

<style lang="less" scoped>
.dashboard-home {
  .box-card {
    .card-dismiss {
      float: right;
      padding: 3px 0;
    }
    .text {
      font-size: 14px;
    }

    .item {
      margin-bottom: 18px;
    }

    .clearfix:before,
    .clearfix:after {
      display: table;
      content: "";
    }
    .clearfix:after {
      clear: both
    }
  }
}
</style>
