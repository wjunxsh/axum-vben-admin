<script lang="ts" setup>
import type { VbenFormProps } from '@vben/common-ui';

import type { VxeGridProps } from '#/adapter/vxe-table';
import type { ApiItem } from '#/api';

import { AccessControl } from '@vben/access';
import { Page, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getApis } from '#/api';
import EditApi from '#/components/system/editApi.vue';

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      fieldName: 'description',
      label: '说明',
    },
    {
      component: 'Input',
      fieldName: 'path',
      label: '路由',
    },
    {
      component: 'Input',
      fieldName: 'api_key',
      label: '权限标识',
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
    { title: '序号', type: 'seq', width: 60, treeNode: true },
    { align: 'left', title: '说明', field: 'description', width: 180 },
    { field: 'method', sortable: true, title: '方法' },
    { field: 'path', sortable: true, title: '路由' },
    { field: 'group', sortable: true, title: '分组' },
    { field: 'api_key', sortable: true, title: '权限标识' },
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
        const data = await getApis({
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
  connectedComponent: EditApi,
});
const addMenu = (parent_id: number) => {
  editModalApi.setData({ values: { parent_id } }).open();
};
const openEdit = (data: ApiItem) => {
  editModalApi
    .setData({
      values: {
        id: data?.id,
        description: data?.description,
        method: data?.method,
        path: data?.path,
        api_key: data?.api_key,
        group: data?.group,
      },
    })
    .open();
};
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="提示">
      <template #toolbar-tools>
        <AccessControl :codes="['system:api:save']" type="code">
          <Button class="mr-2" type="primary" @click="addMenu(0)">
            增加
          </Button>
        </AccessControl>
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #action="{ row }">
        <AccessControl :codes="['system:api:save']" type="code">
          <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
        </AccessControl>
      </template>
    </Grid>
    <Modal @refresh="() => gridApi.reload()" />
  </Page>
</template>
