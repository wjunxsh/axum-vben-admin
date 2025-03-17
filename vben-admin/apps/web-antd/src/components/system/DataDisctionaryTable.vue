<script lang="ts" setup>
import type { VxeGridProps } from '#/adapter/vxe-table';
import type { DataDictionaryConfigItem } from '#/api/core/system/data_dictionary_config';

import { Page, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button, Tag } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import {
  deleteDataDictionaryConfig,
  getDataDictionaryConfigs,
} from '#/api/core/system/data_dictionary_config';

import EditDataDictionaryConfig from './editDataDictionaryConfig.vue';

const props = withDefaults(
  defineProps<{
    dataDictionaryId: number;
  }>(),
  { dataDictionaryId: 0 },
);
const gridOptions: VxeGridProps<DataDictionaryConfigItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { align: 'left', title: '配置项名', field: 'label' },
    { align: 'left', title: '配置项值', field: 'value' },
    {
      field: 'status',
      sortable: true,
      title: '状态',
      slots: { default: 'status' },
    },
    { align: 'left', title: '备注', field: 'remark' },
    {
      field: 'action',
      fixed: 'right',
      slots: { default: 'action' },
      title: '操作',
      width: 180,
    },
  ],
  exportConfig: {},
  height: 'auto',
  keepSource: true,
  pagerConfig: {
    autoHidden: true,
  },
  proxyConfig: {
    ajax: {
      query: async ({ page, sort }) => {
        const data = await getDataDictionaryConfigs({
          page: page.currentPage,
          page_size: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
          data_dictionary_id: props.dataDictionaryId,
        });
        return { items: data };
      },
    },
    sort: true,
  },
  sortConfig: {
    defaultSort: { field: 'name', order: 'desc' },
    remote: true,
  },
  toolbarConfig: {
    // custom: true,
    // import: true,
    // refresh: { code: 'query' },
    // zoom: true,
  },
};
const [Grid, gridApi] = useVbenVxeGrid({
  gridOptions,
});
const [Model, editModalApi] = useVbenModal({
  connectedComponent: EditDataDictionaryConfig,
});
const editClick = (row: DataDictionaryConfigItem) => {
  editModalApi
    .setData({ values: row, data_dictionary_id: props.dataDictionaryId })
    .open();
};
const addClick = () => {
  editModalApi
    .setData({ values: {}, data_dictionary_id: props.dataDictionaryId })
    .open();
};
const deleteClick = async (row: DataDictionaryConfigItem) => {
  gridApi.setLoading(true);
  await deleteDataDictionaryConfig(row.id);
  gridApi.setLoading(false);
  gridApi.reload();
};
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="提示">
      <template #toolbar-tools>
        <Button class="mr-2" type="primary" @click="addClick"> 增加 </Button>
      </template>
      <template #status="{ row }">
        <Tag color="success" v-if="row.status === 1">启用</Tag>
        <Tag color="default" v-if="row.status === 0">停用</Tag>
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #action="{ row }">
        <Button type="link" @click="editClick(row)"> 编辑 </Button>
        <Button type="link" @click="deleteClick(row)"> 删除 </Button>
      </template>
    </Grid>
    <Model title="编辑" @refresh="gridApi.reload()" />
  </Page>
</template>
