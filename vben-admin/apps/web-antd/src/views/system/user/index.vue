<script lang="ts" setup>
import type { VbenFormProps } from '@vben/common-ui';

import type { VxeGridProps } from '#/adapter/vxe-table';
import type { UserItem } from '#/api';

import { Page, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button, Tag } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getSystemUsers } from '#/api';
import EditUser from '#/components/system/editUser.vue';

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      fieldName: 'real_name',
      label: '真实姓名',
    },
    {
      component: 'Select',
      componentProps: {
        allowClear: true,
        options: [
          {
            label: '在职',
            value: 'valid',
          },
          {
            label: '离职',
            value: 'invalid',
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
const gridOptions: VxeGridProps<UserItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { field: 'real_name', sortable: true, title: '真实姓名' },
    { field: 'user_name', sortable: true, title: '登录名' },
    {
      field: 'status',
      sortable: true,
      title: '状态',
      slots: { default: 'status' },
    },
    {
      field: 'avatar',
      sortable: true,
      title: '头像',
      slots: { default: 'avatar' },
    },
    {
      field: 'roles',
      sortable: true,
      title: '角色',
      slots: { default: 'roles' },
    },
    {
      field: 'is_admin',
      sortable: true,
      title: '是否管理员',
      slots: { default: 'admin' },
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
      query: async ({ page, sort }, formVlaues) => {
        const data = await getSystemUsers({
          page: page.currentPage,
          pageSize: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
          ...formVlaues,
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
  connectedComponent: EditUser,
});
const addMenu = (parent_id: number) => {
  editModalApi.setData({ values: { parent_id } }).open();
};
const openEdit = (data: UserItem) => {
  editModalApi
    .setData({
      values: {
        id: data?.id,
        user_name: data?.user_name,
        real_name: data?.real_name,
        status: data?.status,
        avatar: data?.avatar,
        password: '',
        is_admin: data?.is_admin === 1,
        role_ids: data.roles ? data?.roles.map((role) => role.id) || [] : [],
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
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #status="{ row }">
        <Tag color="success" v-if="row.status === 'valid'">在职</Tag>
        <Tag color="default" v-if="row.status === 'invalid'">离职</Tag>
      </template>
      <template #admin="{ row }">
        <Tag color="success" v-if="row.is_admin === 1">管理员</Tag>
        <Tag color="default" v-if="row.is_admin === 0">普通职员</Tag>
      </template>
      <template #roles="{ row }">
        <Tag color="success" v-for="(role, key) in row.roles" :key="key">
          {{ role.role_name }}
        </Tag>
      </template>
      <template #action="{ row }">
        <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
      </template>
    </Grid>
    <Modal @refresh="() => gridApi.reload()" />
  </Page>
</template>
