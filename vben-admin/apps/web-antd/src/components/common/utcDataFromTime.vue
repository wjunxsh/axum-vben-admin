<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue';

import dayjs from 'dayjs';
import timezone from 'dayjs/plugin/timezone';
import utc from 'dayjs/plugin/utc';

const props = defineProps<{
  format?: string;
  time: number;
}>();
dayjs.extend(utc);
dayjs.extend(timezone);
const date = ref<string>('');
onMounted(() => {
  if (props.time) {
    date.value = dayjs(props.time * 1000)
      .tz('UTC')
      .format(props.format || 'YYYY-MM-DD HH:mm:ss');
  }
});
watch(
  () => props.time,
  (time) => {
    if (time) {
      date.value = dayjs(time)
        .tz('UTC')
        .format(props.format || 'YYYY-MM-DD HH:mm:ss');
    }
  },
);
</script>
<template>
  {{ date }}
</template>
