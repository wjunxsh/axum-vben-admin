<script lang="ts" setup>
import type { VbenFormProps } from '@vben/common-ui';

import type { VxeGridProps } from '#/adapter/vxe-table';
import type { DataDictionaryItem } from '#/api/core/system/data_dictionary';

import { Page, useVbenDrawer, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button, Tag } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getDataDictionaries } from '#/api/core/system/data_dictionary';
import UtcDataFromTime from '#/components/common/utcDataFromTime.vue';
import ConfigDataDictionary from '#/components/system/ConfigDataDictionary.vue';
import EditDataDictionary from '#/components/system/editDataDictionary.vue';

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      fieldName: 'name',
      label: '字典',
    },
    {
      component: 'Select',
      componentProps: {
        allowClear: true,
        options: [
          {
            label: '启用',
            value: 1,
          },
          {
            label: '停用',
            value: 0,
          },
        ],
        placeholder: '请选择',
      },
      fieldName: 'status',
      label: '状态',
    },
  ],
  // 控制表单是否显示折叠按钮
  showCollapseButton: true,
  submitButtonOptions: {
    content: '查询',
  },
  // 是否在字段值改变时提交表单
  submitOnChange: false,
  // 按下回车时是否提交表单
  submitOnEnter: true,
};
const gridOptions: VxeGridProps<DataDictionaryItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { title: '名称', field: 'name', width: 100 },
    { field: 'code', sortable: true, title: '编码' },
    {
      field: 'status',
      sortable: true,
      title: '状态',
      slots: { default: 'status' },
    },
    { field: 'remark', sortable: true, title: '备注信息' },
    {
      field: '创建时间',
      sortable: true,
      title: '创建时间',
      slots: { default: 'date' },
    },
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
  proxyConfig: {
    ajax: {
      query: async ({ page, sort }, formValues) => {
        const data = await getDataDictionaries({
          page: page.currentPage,
          page_size: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
          ...formValues,
        });
        return { items: data.items, total: data.pagination.total };
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
    // export: true,
    // import: true,
    refresh: { code: 'query' },
    zoom: true,
  },
};
const [Grid, gridApi] = useVbenVxeGrid({
  formOptions,
  gridOptions,
});
const [Modal, editModalApi] = useVbenModal({
  connectedComponent: EditDataDictionary,
});
const [ConfigDrawer, configItemDrawerApi] = useVbenDrawer({
  connectedComponent: ConfigDataDictionary,
});
const addMenu = (parent_id: number) => {
  editModalApi.setData({ values: { parent_id } }).open();
};
const openEdit = (data: DataDictionaryItem) => {
  editModalApi
    .setData({
      values: {
        id: data?.id,
        name: data?.name,
        remark: data?.remark,
        code: data?.code,
        status: data?.status,
      },
    })
    .open();
};
const openConfigItem = (data: DataDictionaryItem) => {
  configItemDrawerApi
    .setData({
      values: {
        data_dictionary_id: data.id,
      },
    })
    .open();
};
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="提示">
      <template #toolbar-tools>
        <Button class="mr-2" type="primary" @click="addMenu(0)"> 增加 </Button>
      </template>
      <template #status="{ row }">
        <Tag color="success" v-if="row.status === 1">启用</Tag>
        <Tag color="default" v-else-if="row.status === 0">停用</Tag>
      </template>
      <template #date="{ row }">
        <UtcDataFromTime :time="row.created_at" />
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #action="{ row }">
        <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
        <Button type="link" @click="() => openConfigItem(row)"> 配置项 </Button>
      </template>
    </Grid>
    <Modal @refresh="() => gridApi.reload()" />
    <ConfigDrawer @refresh="() => gridApi.reload()" />
  </Page>
</template>
