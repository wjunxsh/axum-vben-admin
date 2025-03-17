<script lang="ts" setup>
import type { Meta } from '#/api';

import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { addMenuApi, getMenusTreeApi } from '#/api';

const emit = defineEmits<{
  refresh: [];
}>();
const [CustomLayoutForm, formApi] = useVbenForm({
  // 所有表单项共用，可单独在表单内覆盖
  commonConfig: {
    // 所有表单项
    componentProps: {
      class: 'w-full',
    },
  },

  layout: 'horizontal',
  schema: [
    {
      component: 'InputNumber',
      fieldName: 'id',
      defaultValue: 0,
      label: 'ID',
      formItemClass: 'col-span-3',
      dependencies: {
        show: false,
        triggerFields: ['id'],
      },
    },
    {
      component: 'ApiTreeSelect',
      formItemClass: 'col-span-3',
      // 对应组件的参数
      componentProps: {
        // 菜单接口
        api: getMenusTreeApi,
        // 菜单接口转options格式
        labelField: 'title',
        valueField: 'id',
        childrenField: 'children',
      },
      // 字段名
      fieldName: 'parent_id',
      // 界面显示的label
      label: '父级菜单',
    },
    {
      component: 'Input',
      fieldName: 'title',
      label: '菜单标题',
      formItemClass: 'col-span-3',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'path',
      label: '路径',
      formItemClass: 'col-span-3',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'name',
      label: '菜单名称',
      formItemClass: 'col-span-3',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'component',
      label: '组件',
      formItemClass: 'col-span-3',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'link',
      label: '外链',
      formItemClass: 'col-span-3',
    },
    {
      component: 'IconPicker',
      fieldName: 'icon',
      label: '图标',
      formItemClass: 'col-span-3',
      rules: 'required',
    },
    {
      component: 'CheckboxGroup',
      componentProps: {
        name: 'meta',
        options: [
          {
            label: '页面缓存',
            value: 'keep_alive',
          },
          {
            label: '隐藏菜单',
            value: 'hide_in_menu',
          },
          {
            label: '隐藏标签',
            value: 'hide_in_tab',
          },
          {
            label: '固定标签页',
            value: 'affix_tab',
          },
          {
            label: '新窗口打开',
            value: 'open_in_new_window',
          },
        ],
      },
      formItemClass: 'col-span-6',
      fieldName: 'meta',
      label: '多选组',
    },
  ],
  // 一共三列
  wrapperClass: 'grid-cols-6',
  showDefaultActions: false,
});
const [Modal, modalApi] = useVbenModal({
  onCancel() {
    modalApi.close();
  },
  async onConfirm() {
    const result = await formApi.validateAndSubmitForm();
    if (!result) {
      return;
    }
    const meta: Meta = {
      title: result.title,
      icon: result.icon,
    };
    const formData = await formApi.getValues();
    formData.meta?.forEach((item: string) => {
      if (item === 'keep_alive') {
        meta.keepAlive = true;
      }
      if (item === 'hide_in_menu') {
        meta.hideInMenu = true;
      }
      if (item === 'hide_in_tab') {
        meta.hideInTab = true;
      }
      if (item === 'affix_tab') {
        meta.affixTab = true;
      }
      if (item === 'open_in_new_window') {
        meta.openInNewWindow = true;
      }
    });
    await addMenuApi({
      id: formData.id ?? 0,
      title: formData.title,
      name: formData.name,
      path: formData.path,
      component: formData.component,
      link: formData.link ?? '',
      parent_id: formData.parent_id,
      children: [],
      meta,
    });
    // message.info('onConfirm');
    modalApi.close();
    emit('refresh');
  },
  onOpenChange(isOpen: boolean) {
    if (isOpen) {
      const { values } = modalApi.getData<Record<string, any>>();
      if (values) {
        formApi.setValues(values);
      }
    }
  },
  onOpened() {
    // message.info('onOpened：打开动画结束');
  },
});
</script>

<template>
  <Modal draggable class="w-[800px]" title="编辑菜单" title-tooltip="修改菜单">
    <CustomLayoutForm />
  </Modal>
</template>
