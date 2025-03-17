<script lang="ts" setup>
import type { VxeGridProps } from '#/adapter/vxe-table';
import type { MenuItem } from '#/api';

import { Page, useVbenModal } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import { Button } from 'ant-design-vue';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getMenusApi } from '#/api';
import EditMenu from '#/components/system/editMenu.vue';

const gridOptions: VxeGridProps<MenuItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 50, treeNode: true },
    { align: 'left', title: 'title', width: 100, field: 'title' },
    { field: 'name', sortable: true, title: 'name' },
    { field: 'path', sortable: true, title: 'path' },
    {
      field: 'icon',
      sortable: true,
      title: 'icon',
      slots: { default: 'icon' },
    },
    {
      field: 'action',
      fixed: 'right',
      slots: { default: 'action' },
      title: '操作',
      width: 180,
    },
  ],
  treeConfig: {
    parentField: 'parent_id',
    rowField: 'id',
    transform: true,
  },
  exportConfig: {},
  height: 'auto',
  keepSource: true,
  proxyConfig: {
    ajax: {
      query: async ({ page, sort }) => {
        const data = await getMenusApi({
          page: page.currentPage,
          pageSize: page.pageSize,
          sortBy: sort.field,
          sortOrder: sort.order,
        });
        return { items: data, total: data.length };
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
const expandAll = () => {
  gridApi.grid?.setAllTreeExpand(true);
};

const collapseAll = () => {
  gridApi.grid?.setAllTreeExpand(false);
};
const [Modal, editModalApi] = useVbenModal({
  connectedComponent: EditMenu,
});
const addMenu = (parent_id: number) => {
  editModalApi.setData({ values: { parent_id } }).open();
};
const openEdit = (data: MenuItem) => {
  const meta: string[] = [];
  if (data.meta.keepAlive) {
    meta.push('keep_alive');
  }
  if (data.meta.hideInMenu) {
    meta.push('hide_in_menu');
  }
  if (data.meta.hideInTab) {
    meta.push('hide_in_tab');
  }
  if (data.meta.affixTab) {
    meta.push('affix_tab');
  }
  if (data.meta.openInNewWindow) {
    meta.push('open_in_new_window');
  }
  editModalApi
    .setData({
      values: {
        id: data?.id,
        title: data?.title,
        name: data?.name,
        path: data?.path,
        component: data?.component,
        link: data?.link,
        icon: data?.meta.icon,
        parent_id: data?.parent_id,
        meta,
      },
    })
    .open();
};
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="提示">
      <template #toolbar-tools>
        <Button class="mr-2" type="primary" @click="expandAll">
          展开全部
        </Button>
        <Button class="mr-2" type="primary" @click="collapseAll">
          折叠全部
        </Button>
        <AccessControl :codes="['system:menus:save']" type="code">
          <Button class="mr-2" type="primary" @click="addMenu(0)">
            增加
          </Button>
        </AccessControl>
        <!-- <Button class="mr-2" type="primary" @click="addMenu(0)"> 增加 </Button> -->
      </template>
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #action="{ row }">
        <Button type="link" @click="() => openEdit(row)"> 编辑 </Button>
        <Button type="link" @click="() => addMenu(row.id)">添加</Button>
      </template>
    </Grid>
    <Modal @refresh="() => gridApi.reload()" />
  </Page>
</template>
