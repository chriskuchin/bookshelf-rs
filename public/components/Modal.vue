<template>
  <div :class="['modal', { 'is-active': show }]">
    <div class="modal-background" @click="toggle"></div>
    <div class="modal-content">
      <div class="box">
        <slot></slot>
      </div>
    </div>
  </div>
</template>


<script>
export default {
  emits: ['close', 'open'],
  data: function () {
    return {
      show: false,
    }
  },
  methods: {
    toggle: function () {
      this.show = !this.show

      if (this.show) {
        this.$emit('open')
      } else {
        this.$emit('close')
      }
    }
  }
}
</script>