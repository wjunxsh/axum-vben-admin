<script lang="ts" setup>
import type { VxeGridProps } from '#/adapter/vxe-table';
import type { RoleItem } from '#/api';

import { Page, useVbenDrawer, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button, Tag } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getSystemRoles } from '#/api';
import Authority from '#/components/system/authority.vue';
import EditRole from '#/components/system/editRole.vue';

const gridOptions: VxeGridProps<RoleItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { align: 'left', title: '角色', field: 'role_name' },
    { align: 'left', title: '角色ID', field: 'id' },
    {
      field: 'status',
      sortable: true,
      title: '状态',
      slots: { default: 'status' },
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
      query: async ({ page, sort }) => {
        const data = await getSystemRoles({
          page: page.currentPage,
          pageSize: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
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
});
const [Modal, editModalApi] = useVbenModal({
  connectedComponent: EditRole,
});
const [AuthorityDrawer, drawerApi] = useVbenDrawer({
  // 连接抽离的组件
  connectedComponent: Authority,
  // placement: 'left',
});
const addRole = (parent_id: number) => {
  editModalApi.setData({ values: { parent_id } }).open();
};
const openEdit = (data: RoleItem) => {
  editModalApi
    .setData({
      values: {
        id: data?.id,
        role_name: data?.role_name,
        status: data?.status,
      },
    })
    .open();
};
const openAccess = (data: RoleItem) => {
  drawerApi.setState({ overlayBlur: 5 }).setData(data).open();
};
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="提示">
      <template #toolbar-tools>
        <Button class="mr-2" type="primary" @click="addRole(0)"> 增加 </Button>
      </template>
      <template #status="{ row }">
        <Tag color="success" v-if="row.status === 'valid'">有效</Tag>
        <Tag color="default" v-if="row.status === 'invalid'">无效</Tag>
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #action="{ row }">
        <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
        <Button type="link" @click="() => openAccess(row)"> 授权 </Button>
      </template>
    </Grid>
    <AuthorityDrawer @refresh="() => gridApi.reload()" />
    <Modal @refresh="() => gridApi.reload()" />
  </Page>
</template>
