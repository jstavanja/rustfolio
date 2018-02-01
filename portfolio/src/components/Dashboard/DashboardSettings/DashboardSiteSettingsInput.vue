<template>
  <el-input :type="field.type" :value="field.value" v-if="field.type === 'text'">
    <template slot="prepend">
      {{ field.alias }}
    </template>
  </el-input>
</template>

<script>
export default {
  props: ['field']
}
</script>
