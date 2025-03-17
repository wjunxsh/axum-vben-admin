<script lang="ts" setup>
import { useVbenDrawer } from '@vben/common-ui';

import DataDisctionaryTable from './DataDisctionaryTable.vue';

const emit = defineEmits<{
  refresh: [];
}>();

const [Drawer, drawerApi] = useVbenDrawer({
  onCancel() {
    drawerApi.close();
  },
  async onConfirm() {
    emit('refresh');
  },
});
</script>

<template>
  <Drawer draggable class="w-[800px]" title="字典配置" title-tooltip="字典配置">
    <DataDisctionaryTable
      :data-dictionary-id="drawerApi.getData().values.data_dictionary_id"
    />
  </Drawer>
</template>
