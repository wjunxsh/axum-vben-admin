<script lang="ts" setup>
import type { VbenFormProps } from '@vben/common-ui';

import type { VxeGridProps } from '#/adapter/vxe-table';
import type { ApiItem } from '#/api';
import type { ConfigItem } from '#/api/core/system/config';

import { Page, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getConfigs } from '#/api/core/system/config';
import UtcDataFromTime from '#/components/common/utcDataFromTime.vue';
import EditConfig from '#/components/system/editConfig.vue';

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      fieldName: 'name',
      label: '参数名',
    },
    {
      component: 'Input',
      fieldName: 'key',
      label: '参数键',
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
const gridOptions: VxeGridProps<ApiItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { align: 'left', title: '名称', field: 'name', width: 100 },
    { field: 'key', sortable: true, title: '参数键' },
    { field: 'value', sortable: true, title: '参数值' },
    { field: 'remark', sortable: true, title: '备注' },
    {
      field: 'created_at',
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
      query: async ({ page, sort }, formValule) => {
        const data = await getConfigs({
          page: page.currentPage,
          page_size: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
          ...formValule,
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
  gridOptions,
  formOptions,
});
const [Modal, editModalApi] = useVbenModal({
  connectedComponent: EditConfig,
});
const addConfig = () => {
  editModalApi.open();
};
const openEdit = (data: ConfigItem) => {
  editModalApi
    .setData({
      values: {
        id: data?.id,
        name: data?.name,
        key: data?.key,
        value: data?.value,
        remark: data?.remark,
      },
    })
    .open();
};
</script>
<template>
  <Page auto-content-height>
    <Grid>
      <template #toolbar-tools>
        <!-- <AccessControl :codes="['system:menus:save']" type="code">
          <Button class="mr-2" type="primary" @click="addMenu(0)">
            增加
          </Button>
        </AccessControl> -->
        <Button class="mr-2" type="primary" @click="addConfig()"> 增加 </Button>
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #date="{ row }">
        <UtcDataFromTime :time="row.created_at" />
      </template>
      <template #action="{ row }">
        <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
      </template>
    </Grid>
    <Modal @refresh="() => gridApi.reload()" />
  </Page>
</template>
