<script lang="ts" setup>
import type { VbenFormProps } from '@vben/common-ui';

import type { VxeGridProps } from '#/adapter/vxe-table';
import type { AdminLogItem } from '#/api/core/system/admin_log';

import { Page } from '@vben/common-ui';

import { VbenIcon } from '@vben-core/shadcn-ui';

import dayjs from 'dayjs';

import { useVbenVxeGrid } from '#/adapter/vxe-table';
import { getLogs } from '#/api/core/system/admin_log';
import UtcDataFromTime from '#/components/common/utcDataFromTime.vue';

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      fieldName: 'operate_name',
      label: '操作名称',
    },
    {
      component: 'Input',
      fieldName: 'path',
      label: '路由',
    },
    {
      component: 'RangePicker',
      fieldName: 'RangePicker',
      label: '日志时间',
    },
    {
      component: 'Select',
      componentProps: {
        allowClear: true,
        filterOption: true,
        options: [
          {
            label: 'POST',
            value: 'POST',
          },
          {
            label: 'GET',
            value: 'GET',
          },
          {
            label: 'DELETE',
            value: 'DELETE',
          },
        ],
        placeholder: '请选择',
        showSearch: true,
      },
      fieldName: 'method',
      label: '方法',
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
const gridOptions: VxeGridProps<AdminLogItem> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: '序号', type: 'seq', width: 60, treeNode: true },
    { align: 'left', title: '说明', field: 'operate_name', width: 120 },
    { field: 'method', title: '方法' },
    { field: 'path', title: '路由', width: 180 },
    { field: 'ip', title: 'ip信息' },
    { field: 'system_real_name', title: '操作人' },
    { field: 'response_time_ms', title: '请求耗时(ms)' },
    { field: 'response_status', title: '请求状态' },
    { field: 'error_message', title: '错误信息' },
    { field: 'user_agent', title: '代理' },
    { field: 'created_at', title: '创建时间', slots: { default: 'date' } },
  ],
  exportConfig: {},
  height: 'auto',
  keepSource: true,
  proxyConfig: {
    ajax: {
      query: async ({ page, sort }, formValule) => {
        const data = await getLogs({
          page: page.currentPage,
          page_size: page.pageSize,
          sort_by: sort.field,
          sort_order: sort.order,
          ...formValule,
          start_time: formValule.RangePicker?.[0]
            ? dayjs(formValule.RangePicker?.[0]).unix()
            : undefined,
          end_time: formValule.RangePicker?.[1]
            ? dayjs(formValule.RangePicker?.[1]).unix()
            : undefined,
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
const [Grid] = useVbenVxeGrid({
  gridOptions,
  formOptions,
});
</script>
<template>
  <Page auto-content-height>
    <Grid table-title="" table-title-help="">
      <template #icon="{ row }">
        <VbenIcon :icon="row.meta.icon" class="w-[200px]" />
      </template>
      <template #date="{ row }">
        <UtcDataFromTime :time="row.created_at" />
      </template>
    </Grid>
  </Page>
</template>
