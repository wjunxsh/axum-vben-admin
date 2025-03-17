<script lang="ts" setup>
import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { saveApi } from '#/api';

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
      formItemClass: 'col-span-6',
      dependencies: {
        show: false,
        triggerFields: ['id'],
      },
    },
    {
      component: 'Input',
      fieldName: 'path',
      label: '路径',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'description',
      label: '描述',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'group',
      label: '分组',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'api_key',
      label: '权限标识',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Select',
      formItemClass: 'col-span-6',
      componentProps: {
        allowClear: true,
        width: '100%',
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
    const formData = await formApi.getValues();
    await saveApi({
      id: formData.id ?? 0,
      description: formData.description,
      method: formData.method,
      path: formData.path,
      api_key: formData.api_key,
      group: formData.group,
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
