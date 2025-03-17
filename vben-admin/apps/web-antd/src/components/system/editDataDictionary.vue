<script lang="ts" setup>
import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { saveDataDictonary } from '#/api/core/system/data_dictionary';

const emit = defineEmits<{
  refresh: [];
}>();
const [DataDictionaryForm, form] = useVbenForm({
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
      fieldName: 'name',
      label: '字典名称',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'code',
      label: '字典编码',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'RadioGroup',
      formItemClass: 'col-span-6',
      componentProps: {
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
      },
      fieldName: 'status',
      label: '状态',
    },
    {
      component: 'Input',
      fieldName: 'remark',
      label: '备注',
      formItemClass: 'col-span-6',
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
    const result = await form.validateAndSubmitForm();
    if (!result) {
      return;
    }
    const formData = await form.getValues();
    await saveDataDictonary({
      id: formData.id ?? 0,
      name: formData.name,
      remark: formData.remark,
      code: formData.code,
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
        form.setValues(values);
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
    <DataDictionaryForm />
  </Modal>
</template>
