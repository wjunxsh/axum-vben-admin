<script lang="ts" setup>
import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { saveRole } from '#/api';

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
      fieldName: 'role_name',
      label: '角色名称',
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
            label: 'valid',
            value: 'valid',
          },
          {
            label: 'invalid',
            value: 'invalid',
          },
        ],
        placeholder: '请选择',
        showSearch: true,
      },
      fieldName: 'status',
      label: '状态',
      rules: 'required',
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
    await saveRole({
      id: formData.id ?? 0,
      role_name: formData.role_name,
      status: formData.status,
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
