<template>
  <el-input :type="field.field_type" :value="field.value" v-if="field.field_type === 'text'">
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
